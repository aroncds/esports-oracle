// use crate::chain::{
//     // models::EventInsertable,
//     fields::Args
// };

// pub enum Error {
//     ArgumentInvalid,
//     DatabaseError(String)
// }

// pub trait BlockEventExecutor {
//     fn run(&self, event: &EventInsertable, conn: &mut PgConnection) -> Result<(), Error>;
// }

// pub struct OnMatchCreated;

// impl BlockEventExecutor for OnMatchCreated {
//     fn run(&self, event: &EventInsertable, conn: &mut PgConnection) -> Result<(), Error> {
//         if let Args::MatchCreated(x) = &event.params {

//             let m = MatchInsertable {
//                 game_id: WH256(x.game_id),
//                 oracle: WH256(x.game_id),
//                 expire_time: x.expire_time as i64,
//                 bet_count: 0,
//                 state: MatchState::Created,
//                 master_player: None,
//                 external_game_id: Some(WH256(x.external_game_id)),
//             };
            
//             return Ok(());
//         }

//         Err(Error::ArgumentInvalid)
//     }
// }

// pub struct OnBetCreated; 

// impl BlockEventExecutor for OnBetCreated {

//     fn run(&self, event: &EventInsertable, conn: &mut PgConnection) -> Result<(), Error> {
//         if let Args::BetCreated(x) = &event.params {


//             return Ok(());
//         }

//         Err(Error::ArgumentInvalid)
//     }
// }