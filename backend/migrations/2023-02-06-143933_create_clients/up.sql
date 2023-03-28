-- Your SQL goes here

CREATE TABLE clients (
    id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    balance REAL NOT NULL,
    date_of_birth TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
)