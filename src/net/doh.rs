use std::error::Error;
use std::net::IpAddr;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

pub fn get_doh_ips(host: &str) -> Result<Vec<IpAddr>, Box<dyn Error>> {
    let config = ResolverConfig::cloudflare_https();
    let resolver = Resolver::new(config, ResolverOpts::default())?;
    let doh_ips: Vec<_> = resolver.lookup_ip(host)?.iter().collect();
    if doh_ips.is_empty() {
        return Err("DoH не вернул ни одного IP".into());
    }
    Ok(doh_ips)
}
