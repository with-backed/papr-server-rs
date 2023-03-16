-- Your SQL goes here
DROP TABLE twabs;
CREATE TABLE twabs (
    id SERIAL PRIMARY KEY,
    "currency address" CHAR(40) NOT NULL,
    "token address" CHAR(40) NOT NULL,
    timestamp BIGINT NOT NULL,
    price DOUBLE PRECISION NOT NULL
)