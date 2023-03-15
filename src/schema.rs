// @generated automatically by Diesel CLI.

diesel::table! {
    twabs (id) {
        id -> Varchar,
        timestamp -> Int8,
        price -> Float8,
    }
}
