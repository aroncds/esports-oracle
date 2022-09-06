-- Your SQL goes here

CREATE TABLE oracle_event (
    id serial PRIMARY KEY,
    name varchar(100) NOT NULL,
    block_number bigint NOT NULL,
    params text NOT NULL,
    executed boolean DEFAULT false NOT NULL
);

CREATE TABLE oracle_block (
    id serial PRIMARY KEY,
    oracle varchar(80) NOT NULL,
    block_number bigint NOT NULL,
    state int DEFAULT 0 NOT NULL
);

CREATE TABLE oracle_match (
    id serial PRIMARY KEY,
    oracle varchar(80) NOT NULL,
    game_id varchar(80) NOT NULL,
    expire_time bigint NOT NULL,
    external_game_id varchar(80),
    master_player varchar(80),
    bet_count int DEFAULT 0 NOT NULL,
    state int DEFAULT 0 NOT NULL
);