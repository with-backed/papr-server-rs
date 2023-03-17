use crate::schema::twabs;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Twab {
    pub id: i32,
    pub currency_address: String,
    pub token_address: String,
    pub updated: chrono::NaiveDateTime,
    pub price: f64,
}


#[derive(Insertable)]
#[diesel(table_name = twabs)]
pub struct NewTwab<'a> {
    pub currency_address: &'a str,
    pub token_address: &'a str,
    pub updated: chrono::NaiveDateTime,
    pub price: f64,
}