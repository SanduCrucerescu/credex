-- Your SQL goes here

CREATE TABLE clients (
    client_id VARCHAR NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    date_of_birth TIMESTAMP NOT NULL,
    PRIMARY KEY (client_id)
);

INSERT INTO clients (client_id, name, email, password, date_of_birth)
VALUES ('C001', 'John Doe', 'johndoe@email.com', 'password123', '1990-01-01 00:00:00');
INSERT INTO clients (client_id, name, email, password, date_of_birth)
VALUES ('C002', 'Jane Doe', 'janedoe@email.com', 'password456', '1985-05-15 12:30:00');
INSERT INTO clients (client_id, name, email, password, date_of_birth)
VALUES ('C003', 'Bob Smith', 'bobsmith@email.com', 'password789', '1995-11-30 08:15:00');
INSERT INTO clients (client_id, name, email, password, date_of_birth)
VALUES ('C004', 'Sarah Johnson', 'sarahjohnson@email.com', 'passwordabc', '1992-07-04 18:00:00');
INSERT INTO clients (client_id, name, email, password, date_of_birth)
VALUES ('C005', 'Tom Lee', 'tomlee@email.com', 'passworddef', '1988-02-14 10:45:00');
INSERT INTO clients (client_id, name, email, password, date_of_birth)
VALUES ('C006', 'Mary Kim', 'marykim@email.com', 'passwordghi', '1998-09-20 14:20:00');
