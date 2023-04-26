use once_cell::sync::Lazy;
use reservoir_nft;
use serde::Serialize;
use std::env;

static RESERVOIR_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("RESERVOIR_API_KEY").expect("RESERVOIR_API_KEY not set"));

const SEVEN_DAYS_SECONDS: u32 = 60 * 60 * 24 * 7;

#[derive(Serialize)]
pub struct NFTInfo {
    pub name: String,
    pub address: String,
    pub created_at: String,
    pub token_count: String,
    pub top_collection_bid: f64,
    pub seven_day_twab: f64,
    pub bids_above_seven_day_twab: i64,
    pub volume: reservoir_nft::collections::VolumeStats,
}

async fn reservoir_collection_info(
    collection: &str,
    client: &reservoir_nft::client::Client,
) -> Result<reservoir_nft::collections::Collection, eyre::Error> {
    let all = client.collections(collection).await?;
    let info = all
        .collections
        .first()
        .ok_or(eyre::eyre!("Collection info not found"))?;

    Ok(info.clone())
}

pub async fn all(collection: &str) -> Result<NFTInfo, eyre::Error> {
    let reservoir_client = reservoir_nft::client::Client::new(
        reservoir_nft::client::Chain::Ethereum,
        RESERVOIR_API_KEY.to_string(),
    );
    let reservoir_info = reservoir_collection_info(collection, &reservoir_client).await?;
    let twab = reservoir_client
        .max_collection_bid(
            collection,
            reservoir_nft::oracle::PriceKind::Twap,
            None,
            Some(SEVEN_DAYS_SECONDS),
        )
        .await?;

    let mut info = NFTInfo {
        name: reservoir_info.name,
        address: collection.to_string(),
        created_at: reservoir_info.created_at,
        token_count: reservoir_info.token_count,
        top_collection_bid: 0.0,
        seven_day_twab: twab.price,
        bids_above_seven_day_twab: 0,
        volume: reservoir_info.volume,
    };

    let depth_response = reservoir_client
        .depth(reservoir_nft::orders::depth::DepthSide::Buy, collection)
        .await?;

    if depth_response.depth.len() == 0 {
        return Ok(info);
    }

    info.top_collection_bid = depth_response.depth.first().unwrap().price;
    info.bids_above_seven_day_twab = depth_response
        .depth
        .iter()
        .filter(|x| x.price > twab.price)
        .map(|x| x.quantity)
        .sum();

    Ok(info)
}
