use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::schema::oracle_match;

use super::fields::{WH256, MatchState};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = oracle_match)]
pub struct Match<'a> {
    pub oracle: WH256,
    pub game_id: WH256,
    pub expire_time: i64,
    pub external_game_id: Option<WH256>,
    pub master_player: Option<&'a str>,
    pub bet_count: i32,
    pub state: MatchState,
}

impl<'a> Match<'a> {

    pub fn is_valid_time_to_start(&self) -> bool {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        self.expire_time > since_the_epoch.as_millis().try_into().unwrap()
    }

}

// fn execute_match_created(&self, platform: &Contract<Http>) {

//     let matches = oracle_match
//         .filter(game_id.eq(WH256(event.game_id.clone())))
//         .limit(1)
//         .load::<Match>(connection)
//         .map_err(|x| Error::DBFailed)?;

//     if matches.len() != 0 {
//         return Err(Error::MatchAlreadyExist);
//     }

//     let m = Match {
//         game_id: WH256(event.game_id),
//         oracle: WH256(event.game_id),
//         expire_time: event.expire_time as i64,
//         bet_count: 0,
//         state: MatchState::Created,
//         master_player: None,
//         external_game_id: Some(WH256(event.external_game_id)),
//     };

//     diesel::insert_into(oracle_match)
//         .values(&m)
//         .execute(connection);
    
//     Ok(m)
// }
