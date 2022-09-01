-- Your SQL goes here

CREATE TABLE oracle_event (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL,
    block_number bigint NOT NULL,
    params text NOT NULL
);

CREATE TABLE oracle_collector (
    id serial PRIMARY KEY,
    block_number bigint NOT NULL,
)

CREATE TABLE oracle_match (
    id serial PRIMARY KEY,
    game_id varchar(80) NOT NULL,
    expire_time bigint NOT NULL,
    external_game_id varchar(80),
    master_player varchar(80)
)