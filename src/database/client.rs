use diesel::pg::PgConnection;
use std::env;
use once_cell::sync::Lazy;
use diesel::prelude::*;

static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

pub struct Client {
    pub connection: PgConnection,
}


impl Default for Client {
    fn default() -> Self {
        Self {
            connection: PgConnection::establish(&DATABASE_URL).unwrap_or_else(|_| panic!("Error connecting to database")),
        }
    }
}