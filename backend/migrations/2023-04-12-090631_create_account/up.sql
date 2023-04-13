-- Your SQL goes here

CREATE TABLE accounts (
    acc_id INT NOT NULL UNIQUE,
    client_id VARCHAR NOT NULL UNIQUE,
    balance REAL NOT NULL,
    acc_activation_date TIMESTAMP NOT NULL,
    PRIMARY KEY (acc_id),
    CONSTRAINT fk_clients
        FOREIGN KEY(client_id)
            REFERENCES clients(client_id)
);

INSERT INTO accounts (acc_id, client_id, balance, acc_activation_date)
VALUES (1, 'C001', 1000, '2021-01-01 00:00:00');
INSERT INTO accounts (acc_id, client_id, balance, acc_activation_date)
VALUES (2, 'C002', 5000, '2020-05-15 12:30:00');
INSERT INTO accounts (acc_id, client_id, balance, acc_activation_date)
VALUES (3, 'C003', 200, '2022-11-30 08:15:00');
INSERT INTO accounts (acc_id, client_id, balance, acc_activation_date)
VALUES (4, 'C004', 3500, '2019-07-04 18:00:00');
INSERT INTO accounts (acc_id, client_id, balance, acc_activation_date)
VALUES (5, 'C005', 750, '2020-02-14 10:45:00');
INSERT INTO accounts (acc_id, client_id, balance, acc_activation_date)
VALUES (6, 'C006', 1200, '2018-09-20 14:20:00');
