// @generated automatically by Diesel CLI.

diesel::table! {
    twabs (id) {
        id -> Int4,
        currency_address -> Bpchar,
        token_address -> Bpchar,
        updated -> Timestamp,
        price -> Float8,
    }
}
