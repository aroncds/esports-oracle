table! {
    oracle_block (id) {
        id -> Int4,
        oracle -> Varchar,
        block_number -> Int8,
        state -> Int4,
    }
}

table! {
    oracle_event (id) {
        id -> Int4,
        name -> Varchar,
        block_number -> Int8,
        params -> Text,
        executed -> Bool,
    }
}

table! {
    oracle_match (id) {
        id -> Int4,
        oracle -> Varchar,
        game_id -> Varchar,
        expire_time -> Int8,
        external_game_id -> Nullable<Varchar>,
        master_player -> Nullable<Varchar>,
        bet_count -> Int4,
        state -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    oracle_block,
    oracle_event,
    oracle_match,
);
