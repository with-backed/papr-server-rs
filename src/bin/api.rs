use papr_server_rs::api::routes::{info, oracle};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::str::FromStr;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::All)
        .allowed_methods(
            vec!["Get"]
                .into_iter()
                .map(|s| FromStr::from_str(s).unwrap())
                .collect(),
        )
        .allowed_headers(AllowedHeaders::All)
        .allow_credentials(true)
        .to_cors()
        .expect("Error initializing CORS");

    rocket::build()
        .mount(
            "/",
            routes![
                info::get,
                oracle::oracle_info_for_controller_allowed_collateral
            ],
        )
        .attach(cors)
}
