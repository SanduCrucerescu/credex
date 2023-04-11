-- Your SQL goes here

CREATE TABLE clients (
    id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    balance REAL NOT NULL,
    date_of_birth TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
)