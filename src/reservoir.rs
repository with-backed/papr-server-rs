use once_cell::sync::Lazy;
use reservoir_nft::client;
use std::env;

static RESERVOIR_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));

pub fn client() -> client::Client {
    client::Client::new(client::Chain::Ethereum, RESERVOIR_API_KEY.to_string())
}
