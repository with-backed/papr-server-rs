use diesel::pg::PgConnection;
use diesel::prelude::*;
use once_cell::sync::Lazy;
use std::env;

static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
