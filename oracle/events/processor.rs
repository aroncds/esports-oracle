use std::collections::HashMap;

use diesel::RunQueryDsl;

use web3::contract::Contract;
use web3::transports::Http;
use web3::types::H256;
use web3::types::Log;
use web3::ethabi::RawLog;

use super::types::Event;
use super::models::Event as EventDB;

use crate::database::schema::oracle_event::dsl::*;

type EventDict = HashMap<H256, web3::ethabi::Event>;

#[repr(transparent)]
pub struct Processor(pub Vec<Log>);

impl Processor {

    fn get_events(&self, platform: &Contract<Http>) -> EventDict {
        let mut events: EventDict = HashMap::new();

        events.insert(Event::MatchCreated.into(), platform.abi().event("MatchCreated").unwrap().clone());
        events.insert(Event::BetCreated.into(), platform.abi().event("BetCreated").unwrap().clone());

        events
    }

    pub fn save_events(&self, platform: &Contract<Http>) -> &Self {
        let mut conn = crate::database::conn::establish_connection().unwrap();
        let events = self.get_events(platform);

        let events: Vec<EventDB> = (&self.0)
            .into_iter()
            .map(|x| process_event(&events, &x))
            .collect();

        diesel::insert_into(oracle_event)
            .values(events)
            .execute(&mut conn)
            .expect("Failed to save events");

        self
    }
}

fn process_event(events: &EventDict, x: &Log) -> EventDB {
    let event = events.get(x.topics.get(0).unwrap()).unwrap();
    
    let log = event.parse_log(RawLog {
        topics: x.topics.clone(),
        data: x.data.0.clone()
    }).unwrap();

    EventDB::new(
        &event.name,
        x.block_number.unwrap().as_u64(),
        &log.params
    )
}
