use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use once_cell::sync::Lazy;
use papr_server_rs::{database, papr_subgraph};
use reservoir_nft::{
    client,
    oracle::{self},
};
use std::env;

static RESERVOIR_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));
static ETHEREUM_ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    let reservoir_client =
        reservoir_nft::client::Client::new(client::Chain::Ethereum, RESERVOIR_API_KEY.clone());
    let controllers = vec!["0x3b29c19ff2fcea0ff98d0ef5b184354d74ea74b0".to_string()];
    let result = papr_subgraph::client::Client::default()
        .allowed_collateral_for_controllers(controllers)
        .await?;
    let mut twabs: Vec<database::models::NewTwab> = Vec::new();
    for a in result {
        // get 7 day time weighted average max collection bid
        let res = reservoir_client
            .max_collection_bid(
                &a.token.id,
                oracle::PriceKind::Twap,
                None,
                Some(60 * 60 * 24 * 7),
            )
            .await?;
        twabs.push(database::models::NewTwab {
            token_address: a.token.id,
            currency_address: ETHEREUM_ZERO_ADDRESS.to_string(),
            created_at: NaiveDateTime::from_timestamp_opt(
                res.message.timestamp.try_into().unwrap(),
                0,
            )
            .ok_or(eyre::eyre!(
                "Timestamp {} out of range",
                res.message.timestamp
            ))?,
            price: res.price,
        });
    }
    diesel::insert_into(database::schema::twabs::table)
        .values(&twabs)
        .execute(&mut database::connection::establish_connection())?;
    Ok(())
}
