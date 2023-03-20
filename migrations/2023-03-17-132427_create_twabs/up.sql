CREATE TABLE twabs (
    id SERIAL PRIMARY KEY,
    currency_address CHAR(42) NOT NULL,
    token_address CHAR(42) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    price DOUBLE PRECISION NOT NULL
)
