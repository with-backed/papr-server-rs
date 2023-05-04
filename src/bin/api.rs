use papr_server_rs::api::routes::{info, oracle};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            info::get,
            oracle::oracle_info_for_controller_allowed_collateral
        ],
    )
}
