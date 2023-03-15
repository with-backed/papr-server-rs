use crate::schema::twabs;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Twab {
    pub id: String,
    pub timestamp: i64,
    pub price: f64,
}

#[derive(Insertable)]
#[diesel(table_name = twabs)]
pub struct NewTwab<'a> {
    pub id: &'a str,
    pub timestamp: i64,
    pub price: f64,
}
