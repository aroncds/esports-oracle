use serde::{Deserialize, Serialize};
use diesel::sql_types::Int4;

use crate::database::schema::oracle_match;
use crate::database::schema::oracle_match::dsl::*;

use super::fields::F256;

#[derive(Debug, AsExpression, Deserialize, Serialize)]
#[diesel(sql_type = Int4)]
#[repr(i32)]
pub enum EventState {
    Created = 0,
    Started,
    Running,
    Finished,
    Recused,
    Cancelled
}

#[derive(Debug, Insertable)]
#[diesel(table_name = oracle_match)]
pub struct Match<'a> {
    pub oracle: F256,
    pub game_id: F256,
    pub expire_time: i64,
    pub bet_count: i32,
    pub state: EventState,
    pub external_game_id: Option<F256>,
    pub master_player: Option<&'a str>,
}
