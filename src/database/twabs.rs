use super::client;
use super::models::{NewTwab, Twab};
use super::schema::twabs;
use diesel::prelude::*;

// impl client::Client {
//     pub fn write_twabs(&self, twabs: &Vec<NewTwab>) -> Insertable<Twabs> {
//         let c = *self;
//         diesel::insert_into(twabs::table)
//             .values(twabs)
//     }
// }