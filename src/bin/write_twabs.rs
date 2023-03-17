use once_cell::sync::Lazy;
use papr_server_rs::{add_twabs, establish_connection, papr_subgraph};
use reservoir_nft::{
    client,
    oracle::{self, OracleResponse},
};
use std::env;

static RESERVOIR_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));

#[tokio::main]
async fn main() -> Result<(), eyre::Error> {
    let mut connection = establish_connection();
    let reservoir_client =
        reservoir_nft::client::Client::new(client::Chain::Ethereum, RESERVOIR_API_KEY.clone());
    let controllers = vec!["0x3b29c19ff2fcea0ff98d0ef5b184354d74ea74b0".to_string()];
    let result = papr_subgraph::client::Client::default()
        .allowed_collateral_for_controllers(controllers)
        .await?;
    let mut oracle_responses: Vec<OracleResponse> = Vec::new();
    for a in result {
        // get 7 day twap max bad
        let res = reservoir_client
            .max_collection_bid(
                &a.token.id,
                oracle::PriceKind::Twap,
                None,
                Some(60 * 60 * 24 * 7),
            )
            .await?;
        println!("collection: {}", a.token.id);
        println!("max bid: {}", res.price);
        oracle_responses.push(res);
    }
    add_twabs(&mut connection, &oracle_responses);
    // TODO: write results to postgres DB
    Ok(())
}
