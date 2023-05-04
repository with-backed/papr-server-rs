use crate::reservoir;
use crate::{papr_subgraph, redis as papr_redis};
use redis::Commands;
use reservoir_nft::oracle;
use reservoir_nft::oracle::OracleResponse;
use serde::{Deserialize, Serialize};
use serde_json;

const SEVEN_DAYS_SECONDS: u32 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize, Debug)]
struct OracleInfo {
    lower: OracleResponse,
    twab: OracleResponse,
}

fn write_oracle_values(controller: &str, infos: &Vec<OracleInfo>) -> Result<(), eyre::Error> {
    let mut con = papr_redis::get_connection()?;

    let key = redis_oracle_info_key(controller);
    con.set::<String, String, ()>(key, serde_json::to_string(infos)?)?;

    Ok(())
}

fn redis_oracle_info_key(controller: &str) -> String {
    format!("{}:allowed_collateral_oracle_info", controller)
}

pub async fn write_oracle_values_to_redis(controller: &str) -> Result<(), eyre::Error> {
    let reservoir_client = reservoir::client();
    let allowed_collateral = papr_subgraph::client::Client::default()
        .allowed_collateral_for_controllers(vec![controller.to_string()])
        .await?;
    let mut infos = Vec::new();
    for a in allowed_collateral {
        let lower = reservoir_client
            .max_collection_bid(
                &a.token.id,
                oracle::PriceKind::Lower,
                None,
                Some(SEVEN_DAYS_SECONDS),
            )
            .await?;
        let twab = reservoir_client
            .max_collection_bid(
                &a.token.id,
                oracle::PriceKind::Twap,
                None,
                Some(SEVEN_DAYS_SECONDS),
            )
            .await?;
        infos.push(OracleInfo { lower, twab });
    }
    write_oracle_values(controller, &infos)?;

    Ok(())
}
