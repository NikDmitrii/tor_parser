use clap::Parser;
use reqwest::blocking::Client;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::IpAddr;
use std::process::Command;
use url::Url;

use trust_dns_resolver::{
    Resolver,
    config::{ResolverConfig, ResolverOpts},
};

fn restart_or_start_tor() -> Result<(), Box<dyn Error>> {
    let status = Command::new("systemctl")
        .arg("is-active")
        .arg("tor")
        .output()?;

    let output = String::from_utf8_lossy(&status.stdout);

    if output.trim() == "active" {
        println!("🔄 Tor активен, перезапускаем...");
        let result = Command::new("sudo")
            .arg("systemctl")
            .arg("restart")
            .arg("tor")
            .status()?;
        if !result.success() {
            return Err("Не удалось перезапустить Tor".into());
        }
        println!("✅ Tor перезапущен");
    } else {
        println!("⚠️ Tor не активен, пробуем запустить...");
        let result = Command::new("sudo")
            .arg("systemctl")
            .arg("start")
            .arg("tor")
            .status()?;
        if !result.success() {
            return Err("Не удалось запустить Tor".into());
        }
        println!("✅ Tor запущен");
    }

    Ok(())
}

fn get_doh_ips(host: &str) -> Result<Vec<IpAddr>, Box<dyn Error>> {
    let config = ResolverConfig::cloudflare_https();
    let resolver = Resolver::new(config, ResolverOpts::default())?;
    let doh_ips: Vec<_> = resolver.lookup_ip(host)?.iter().collect();
    if doh_ips.is_empty() {
        return Err("DoH не вернул ни одного IP".into());
    }
    Ok(doh_ips)
}
fn parse_html(url: &Url) -> Result<String, Box<dyn Error>> {
    // резолвим IP через Cloudflare DoH
    let hostname = url.host_str().ok_or("Не удалось получить host из URL")?;
    let doh_ips = get_doh_ips(hostname)?;
    //println!("IP от DoH: {:?}", doh_ips);

    let client = create_doh_client();
    let response = client.get(url.clone()).send()?;
    let actual_ip = response.remote_addr().ok_or("Не удалось получить IP")?;

    //println!("Фактический IP соединения: {}", actual_ip.ip());

    if !doh_ips.contains(&actual_ip.ip()) {
        return Err("⚠️ Подозрение на DNS spoofing! IP не совпадает с DoH".into());
    }
    Ok(response.text()?)
}

fn extract_bridges(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}

fn update_torrc(path: &str, bridges: &Vec<String>) -> Result<(), Box<dyn Error>> {
    const USE_BRIDGE: &str = "UseBridges 1";
    const BRIDGE_BEGIN: &str = "Bridge ";
    let input_file = File::open(path)?;
    let reader = BufReader::new(input_file);
    let temp_path = format!("{}.tmp", path);
    let mut temp_file = File::create(&temp_path)?;
    let mut use_bridges_found = false;
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed == USE_BRIDGE {
            if use_bridges_found {
                eprintln!("Повтор строки '{}'", USE_BRIDGE);
            } else {
                use_bridges_found = true;
                writeln!(temp_file, "{}", line)?;
                for bridge in bridges {
                    writeln!(temp_file, "Bridge {}", bridge)?;
                }
            }
        } else if trimmed.starts_with(BRIDGE_BEGIN) {
            continue;
        } else {
            writeln!(temp_file, "{}", line)?;
        }
    }
    if !use_bridges_found {
        eprintln!("Строка '{}' не найдена", USE_BRIDGE);
    }
    fs::rename(temp_path, path)?;
    Ok(())
}

fn create_doh_client() -> Client {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    client
}

#[derive(Parser)]
#[command(name = "torrc-updater")]
#[command(about = "Обновляет torrc с мостами", version = "1.0")]
struct Args {
    #[arg(short, long, default_value = "/etc/tor/torrc")]
    torrc: String,

    #[arg(short, long)]
    print: bool,

    #[arg(short, long)]
    dry_run: bool,

    #[arg(short = 'r', long = "restart")]
    restart: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let url = Url::parse("https://torscan-ru.ntc.party/relays.txt")?;
    let html = parse_html(&url)?;
    let bridges = extract_bridges(&html);

    if args.print || args.dry_run {
        println!("мосты:");
        for i in &bridges {
            println!("{i}");
        }
    }

    if args.dry_run {
        println!("Мосты не были записаны в файл, так как --dry-run активирован");
        return Ok(());
    }

    update_torrc(&args.torrc, &bridges)?;

    println!("Мосты обновлены");

    if args.restart {
        restart_or_start_tor()?;
    }

    Ok(())
}
