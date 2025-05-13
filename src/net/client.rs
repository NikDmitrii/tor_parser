use fake_user_agent::get_rua;
use reqwest::blocking::Client;
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, HeaderMap, HeaderValue
};

pub fn create_client() -> Client {
    let mut headers = HeaderMap::new();

    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"),
    );
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));
    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .default_headers(headers)
        .user_agent(get_rua())
        .build()
        .unwrap();

    client
}
