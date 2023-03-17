// @generated automatically by Diesel CLI.

diesel::table! {
    twabs (id) {
        id -> Int4,
        currency_address -> Bpchar,
        token_address -> Bpchar,
        created_at -> Timestamp,
        price -> Float8,
    }
}
