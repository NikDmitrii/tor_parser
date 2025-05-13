pub mod bridges;
mod client;
mod doh;
mod decode;

pub use bridges::{extract_bridges, parse_html};
use client::create_client;
use doh::get_doh_ips;
use decode::decode_response;