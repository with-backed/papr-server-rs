mod graphql;
pub mod models;
pub mod papr_subgraph;
pub mod schema;

use self::models::{NewTwab, Twab};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use reservoir_nft::oracle::OracleResponse;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_twabs(conn: &mut PgConnection, oracle_responses: &Vec<OracleResponse>) -> Vec<Twab> {
    use crate::schema::twabs;

    let mut n: Vec<NewTwab> = Vec::new();
    for res in oracle_responses {
        n.push(NewTwab {
            token_address: &res.message.id,
            // TODO: figure out how to get currency address
            currency_address: &"",
            // I don't love this cast, but postgres doesn't support unsigned ints
            timestamp: res.message.timestamp as i64,
            price: res.price,
        });
    }

    diesel::insert_into(twabs::table)
        .values(&n)
        .get_results(conn)
        .expect("Error saving new twabs")
}
