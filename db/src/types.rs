use diesel::prelude::*;

#[derive(Queryable)]
pub struct Event<'a> {
    /// Evevnt block height
    pub block_height: u64,
    /// Event name
    pub name: &'a str
}

#[derive(Queryable)]
pub struct Match<'a> {
    pub game_id: &'a str,

    pub expire_time: u64,

    pub external_game_id: &'a str
}
