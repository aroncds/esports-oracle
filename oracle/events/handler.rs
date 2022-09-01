use std::future::{self, Ready};
use std::collections::HashMap;

use log::{info, error};
use web3::types::Log;
use web3::ethabi::{RawLog, Token};
use web3::{
    futures::StreamExt,
    transports::WebSocket,
};

use crate::contract;
use crate::events::types::Event;
use crate::events::models::{Event as EventDB, Error};
use crate::events::parameters::{Args, MatchCreatedParams, BetCreatedParams};
use crate::database::conn::connect;

type EventParams = HashMap<String, Token>;

fn create_args(event_name: &String, params: &EventParams) -> Option<Args> {
    match event_name.into() {
        Event::MatchCreated => Some(Args::MatchCreated(MatchCreatedParams::from(params))),
        Event::BetCreated => Some(Args::BetCreated(BetCreatedParams::from(params))),
        Event::MatchFinished => None
    }
}

fn create_event(
    event_name: String,
    block_height: u64,
    event_params: EventParams
) {
    let mut conn = connect().expect("Failed");

    let event_db = EventDB {
        name: &event_name,
        block_number: block_height as i64,
        params: create_args(&event_name, &event_params).unwrap()
    };

    match event_db.save(&mut conn) {
        Ok(()) => {
            info!("{:?} - Block: {:?}, Data: {:?}",
                event_db.name,
                event_db.block_number,
                event_params
            );
        },
        Err(Error::FailedToSave) => {
            error!("Failed to save the event on database!");
        }
    }
}

fn process_event(event: &web3::ethabi::Event) -> impl Fn(Result<Log, web3::Error>) -> Ready<()> + '_ {
    |log| {
        if let Ok(x) = log {
            let log = event.parse_log(RawLog {
                topics: x.topics.clone(), data: x.data.0 }).unwrap();

            let mut params: EventParams = HashMap::new();

            log.params.iter().for_each(|i| {
                params.insert(i.name.clone(), i.value.clone()); });
    
            create_event(
                event.name.clone(),
                x.block_number.unwrap().as_u64(),
                params
            );
        }

        future::ready(())
    }
}

pub async fn subscribe_events(w: web3::Web3<WebSocket>) -> web3::contract::Result<()> {
    let contract = contract::create_platform_contract(&w)?;

    let subs = tokio::try_join!(
        contract::subscribe(&w, &contract, Event::MatchCreated),
        contract::subscribe(&w, &contract, Event::BetCreated)
    )?;

    let subscriptions = tokio::spawn(async move {
        let match_created = contract.abi().event("MatchCreated").unwrap();
        let bet_created = contract.abi().event("BetCreated").unwrap();

        tokio::join!(
            subs.0.for_each(process_event(match_created)),
            subs.1.for_each(process_event(bet_created))
        );
    });

    _ = tokio::join!(subscriptions);

    Ok(())
}
