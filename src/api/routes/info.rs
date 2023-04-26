use crate::nft_info;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/info?<collection>")]
pub async fn get(collection: &str) -> Result<Json<nft_info::NFTInfo>, status::BadRequest<String>> {
    let info = nft_info::all(collection)
        .await
        .map_err(|err| status::BadRequest(Some(err.to_string())))?;

    return Ok(Json(info));
}
