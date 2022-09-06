use std::future::{self, Ready};
use std::collections::HashMap;

use log::{info, error};
use web3::types::Log;
use web3::ethabi::{RawLog, Token, LogParam};
use web3::{
    futures::StreamExt,
};

use crate::events::types::Event;
use crate::events::models::{Event as EventDB, Error};
use crate::events::parameters::{Args, MatchCreatedParams, BetCreatedParams};
use crate::database::conn::connect;

type EventParams = Vec<LogParam>;

fn create_args(event_name: &String, params: &EventParams) -> Option<Args> {
    match event_name.into() {
        Event::MatchCreated => Some(Args::MatchCreated(MatchCreatedParams::from(params))),
        Event::BetCreated => Some(Args::BetCreated(BetCreatedParams::from(params))),
        Event::MatchFinished => None
    }
}

pub fn create_event(
    event_name: String,
    block_height: u64,
    event_params: EventParams
) {
    let mut conn = connect().expect("Failed");

    let event_db = EventDB {
        name: &event_name,
        block_number: block_height as i64,
        params: create_args(&event_name, &event_params).unwrap(),
        executed: false
    };

    match event_db.save(&mut conn) {
        Ok(()) => {
            info!("{:?} - Block: {:?}, Data: {:?}",
                event_db.name,
                event_db.block_number,
                event_db.params
            );
        },
        Err(Error::FailedToSave) => {
            error!("Failed to save the event on database!");
        }
    }
}

// fn process_event(event: &web3::ethabi::Event) -> impl Fn(Result<Log, web3::Error>) -> Ready<()> + '_ {
//     |log| {
//         if let Ok(x) = log {
//             let log = event.parse_log(RawLog {
//                 topics: x.topics.clone(), data: x.data.0 }).unwrap();

//             let mut params: EventParams = HashMap::new();

//             log.params.iter().for_each(|i| {
//                 params.insert(i.name.clone(), i.value.clone()); });
    
//             create_event(
//                 event.name.clone(),
//                 x.block_number.unwrap().as_u64(),
//                 params
//             );
//         }

//         future::ready(())
//     }
// }
