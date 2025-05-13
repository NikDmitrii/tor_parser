pub mod bridges;
mod client;
mod doh;

pub use bridges::{extract_bridges, parse_html};
use client::create_client;
use doh::get_doh_ips;
