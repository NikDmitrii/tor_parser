use fake_user_agent::get_rua;
use reqwest::blocking::Client;

pub fn create_client() -> Client {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(get_rua())
        .build()
        .unwrap();

    client
}
