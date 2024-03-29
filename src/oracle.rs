use crate::reservoir;
use crate::{papr_subgraph, redis as papr_redis};
use redis::Commands;
use reservoir_nft::oracle;
use reservoir_nft::oracle::OracleResponse;
use serde::{Deserialize, Serialize};
use serde_json;

const SEVEN_DAYS_SECONDS: u32 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize, Debug)]
pub struct OracleInfo {
    collection: String,
    lower: OracleResponse,
    twab: OracleResponse,
}

fn write_oracle_infos_redis(controller: &str, infos: &Vec<OracleInfo>) -> Result<(), eyre::Error> {
    let mut con = papr_redis::get_connection()?;

    let key = redis_oracle_info_key(controller);
    con.set::<String, String, ()>(key, serde_json::to_string(infos)?)?;

    Ok(())
}

fn redis_oracle_info_key(controller: &str) -> String {
    format!(
        "{}:allowed_collateral_oracle_info",
        controller.to_lowercase()
    )
}

pub fn get_cached_oracle_infos(controller: &str) -> Result<Vec<OracleInfo>, eyre::Error> {
    let mut con = papr_redis::get_connection()?;
    let key = redis_oracle_info_key(controller);
    let info: String = con.get(key)?;
    let info: Vec<OracleInfo> = serde_json::from_str(&info)?;
    Ok(info)
}

pub async fn cache_oracle_values(controller: &str) -> Result<(), eyre::Error> {
    let reservoir_client = reservoir::client();
    let papr_controller = papr_subgraph::client::Client::default()
        .controller_by_id(controller.to_string())
        .await?
        .ok_or(eyre::eyre!("controller not found in subgraph"))?;
    let mut infos = Vec::new();
    for a in papr_controller.allowed_collateral.iter() {
        let lower = reservoir_client
            .max_collection_bid(
                &a.token.id,
                oracle::PriceKind::Lower,
                Some(&papr_controller.underlying.id),
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
        infos.push(OracleInfo {
            collection: a.token.id.clone(),
            lower,
            twab,
        });
    }
    write_oracle_infos_redis(controller, &infos)?;

    Ok(())
}
