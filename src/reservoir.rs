use once_cell::sync::Lazy;
use reservoir_nft::client;
use std::env;

static RESERVOIR_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));
static CHAIN_ID: Lazy<String> = Lazy::new(|| env::var("CHAIN_ID").expect("CHAIN not set"));

pub fn client() -> client::Client {
    client::Client::new(
        if CHAIN_ID.to_string() == "5" {
            client::Chain::Goerli
        } else {
            client::Chain::Ethereum
        },
        RESERVOIR_API_KEY.to_string(),
    )
}
