-- Your SQL goes here

CREATE TABLE transactions (
    id VARCHAR(255) NOT NULL,
    sender_id VARCHAR(255) NOT NULL,
    receiver_id VARCHAR(255) NOT NULL,
    amount REAL NOT NULL,
    withdrawal_time TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
    -- FOREIGN KEY (client_id) REFERENCES clients(id)
)