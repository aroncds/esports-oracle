// use std::convert::TryInto;
// use std::time::{SystemTime, UNIX_EPOCH};

// use crate::database::schema::oracle_match;

// use web3::types::H256;

// pub enum MatchState {
//     Created,
//     Started,
//     Running,
//     Finished,
//     Recused,
//     Cancelled
// }

// pub struct MatchInsertable {
//     pub oracle: H256,
//     pub game_id: H256,
//     pub expire_time: i64,
//     pub external_game_id: Option<H256>,
//     pub master_player: Option<String>,
//     pub bet_count: i32,
//     pub state: MatchState,
// }

// pub struct MatchQueryable {
//     pub id: i32,
//     pub oracle: H256,
//     pub game_id: H256,
//     pub expire_time: i64,
//     pub external_game_id: Option<H256>,
//     pub master_player: Option<String>,
//     pub bet_count: i32,
//     pub state: MatchState,
// }

// impl MatchQueryable {

//     pub fn is_valid_time_to_start(&self) -> bool {
//         let start = SystemTime::now();
//         let since_the_epoch = start
//             .duration_since(UNIX_EPOCH)
//             .expect("Time went backwards");
        
//         self.expire_time > since_the_epoch.as_millis().try_into().unwrap()
//     }

// }
