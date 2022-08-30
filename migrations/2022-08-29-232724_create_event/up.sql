-- Your SQL goes here

CREATE TABLE oracle_event (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL,
    block_number bigint NOT NULL,
    params text NOT NULL
);

