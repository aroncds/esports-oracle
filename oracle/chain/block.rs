use std::collections::HashMap;

use log::info;
use serde_json::json;
use async_trait::async_trait;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::{NotSet, Set};

use web3::contract::Contract;
use web3::transports::Http;
use web3::types::H256;
use web3::types::Log;
use web3::ethabi::RawLog;

use super::types::Event;
use super::error::Result;
use super::fields::Args;
use crate::database::event;

type EventDict = HashMap<H256, web3::ethabi::Event>;

#[async_trait]
pub trait LogDataHandler {
    async fn save(&self, platform: &Contract<Http>) -> Result<()>;
}

pub struct BlockData(pub Vec<Log>);

#[async_trait]
impl LogDataHandler for BlockData {
    async fn save(&self, platform: &Contract<Http>) -> Result<()> {
        if self.0.len() == 0 {
            return Ok(());
        }

        let conn = crate::database::CONNECTION.get().await.clone();
        let events = get_events_map(platform);

        let events: Vec<event::ActiveModel> = (&self.0)
            .into_iter()
            .map(|x| create_event_db(&events, &x))
            .collect();

        event::Entity::insert_many(events).exec(&*conn).await?;

        Ok(())
    }
}

fn get_events_map(platform: &Contract<Http>) -> EventDict {
    let mut events: EventDict = HashMap::new();

    events.insert(Event::MatchCreated.into(), platform.abi().event("MatchCreated").unwrap().clone());
    events.insert(Event::BetCreated.into(), platform.abi().event("BetCreated").unwrap().clone());

    events
}

fn create_event_db(events: &EventDict, x: &Log) -> event::ActiveModel {
    let event = events.get(x.topics.get(0).unwrap()).unwrap();
    
    let log = event.parse_log(RawLog {
        topics: x.topics.clone(),
        data: x.data.0.clone()
    }).unwrap();

    info!("{:?} - GameID: {:?}", event.name, log.params);

    event::ActiveModel {
        id: NotSet,
        name: Set(event.name.clone()),
        block_number: Set(x.block_number.unwrap().as_u64() as i64),
        params: Set(json!(Args::create(&event.name, &log.params))),
        executed: Set(false)
    }
}
