use once_cell::sync::Lazy;
use redis;
use std::env;

static REDIS_URL: Lazy<String> = Lazy::new(|| env::var("REDIS_URL").expect("REDIS_URL not set"));

pub fn get_connection() -> Result<redis::Connection, eyre::Error> {
    let client = redis::Client::open(REDIS_URL.to_string())?;
    Ok(client.get_connection()?)
}
