use papr_server_rs::api::routes::info;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![info::get])
}
