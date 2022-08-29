use std::future::{self, Ready};
use std::collections::HashMap;

use log::info;
use web3::types::Log;
use web3::ethabi::RawLog;
use web3::{
    futures::StreamExt,
    transports::WebSocket,
};

use crate::contract;
use crate::events::types::Event;

fn process_event(event: &web3::ethabi::Event) -> impl Fn(Result<Log, web3::Error>) -> Ready<()> + '_ {
    |log| {
        if let Ok(x) = log {
            let mut params = HashMap::new();

            let log = event.parse_log(RawLog {
                topics: x.topics.clone(), data: x.data.0 }).unwrap();

            log.params.iter().for_each(|i| {
                params.insert(i.name.clone(), i.value.clone()); });

            info!("{:?} - Topic: {:?}, Block: {:?}, Data: {:?}",
                event.name,
                x.topics.get(0),
                x.block_number,
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