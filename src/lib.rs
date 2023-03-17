mod graphql;
pub mod models;
pub mod papr_subgraph;
pub mod schema;

use self::models::{NewTwab, Twab};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_twabs(conn: &mut PgConnection, twabs: &Vec<NewTwab>) -> Vec<Twab> {
    use crate::schema::twabs;

    diesel::insert_into(twabs::table)
        .values(twabs)
        .get_results(conn)
        .expect("Error saving new twabs")
}
