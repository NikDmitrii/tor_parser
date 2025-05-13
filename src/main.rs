mod cli;
mod net;
mod tor;
mod utils;
use clap::Parser;
use cli::Args;
use net::extract_bridges;
use net::parse_html;
use std::error::Error;
use std::io::{BufRead, Write};
use tor::restart_or_start_tor;
use tor::update_torrc;
use url::Url;

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
