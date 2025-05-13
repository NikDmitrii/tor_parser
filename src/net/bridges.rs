use std::error::Error;
use url::Url;
use super::decode_response;
use super::create_client;
use super::get_doh_ips;

pub fn parse_html(url: &Url) -> Result<String, Box<dyn Error>> {
    // резолвим IP через Cloudflare DoH
    let hostname = url.host_str().ok_or("Не удалось получить host из URL")?;
    let doh_ips = get_doh_ips(hostname)?;
    //println!("IP от DoH: {:?}", doh_ips);

    let client = create_client();
    let response = client
        .get(url.clone())
        .send()?;

    let actual_ip = response.remote_addr().ok_or("Не удалось получить IP")?;

    //println!("Фактический IP соединения: {}", actual_ip.ip());

    if !doh_ips.contains(&actual_ip.ip()) {
        return Err("⚠️ Подозрение на DNS spoofing! IP не совпадает с DoH".into());
    }
    let text = decode_response(response)?;
    Ok(text)
}

pub fn extract_bridges(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}
