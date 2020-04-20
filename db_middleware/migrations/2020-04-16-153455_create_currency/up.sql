-- Your SQL goes here
CREATE TABLE currency (
    id SERIAL PRIMARY KEY,
    currency VARCHAR NOT NULL,
    description VARCHAR NOT NULL
)