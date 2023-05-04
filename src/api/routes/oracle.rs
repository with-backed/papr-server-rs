use crate::oracle;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/oracleInfoForAllowedCollateral?<controller>")]
pub async fn oracle_info_for_controller_allowed_collateral(
    controller: &str,
) -> Result<Json<Vec<oracle::OracleInfo>>, status::BadRequest<String>> {
    let info = oracle::get_cached_oracle_infos(controller)
        .map_err(|err| status::BadRequest(Some(err.to_string())))?;

    return Ok(Json(info));
}
