// @generated automatically by Diesel CLI.

diesel::table! {
    twabs (id) {
        id -> Int4,
        #[sql_name = "currency address"]
        currency_address -> Bpchar,
        #[sql_name = "token address"]
        token_address -> Bpchar,
        timestamp -> Int8,
        price -> Float8,
    }
}
