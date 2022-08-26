use std::future::{self, Ready};
use std::collections::HashMap;

use tokio;
use log::{info, error};
use web3::types::Log;
use web3::{
    futures::StreamExt,
    transports::WebSocket,
    ethabi::RawLog
};

mod contract;
mod settings;
mod events;

use events::Event;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    env_logger::init();

    info!("Oracle: Initializing");

    initialize_events().await?;

    info!("Finalizing");

    Ok(())
}

async fn initialize_events() -> web3::contract::Result<()> {
    let web3 = contract::create_ws_web3().await;

    match web3 {
        Ok(w) => subscribe_events(w).await?,
        Err(_) => panic!("Failed to connect")
    }

    Ok(())
}

async fn subscribe_events(w: web3::Web3<WebSocket>) -> web3::contract::Result<()> {
    let contract = contract::create_platform_contract(&w)?;
    let sub = contract::subscribe(&w, &contract, Event::MatchCreated).await?;

    _ = tokio::join!(tokio::spawn(async move {
        let on_match_created = contract.abi().event("MatchCreated").unwrap();

        sub.for_each(process_event(on_match_created)).await;
    }));

    Ok(())
}

fn process_event<'a>(event: &'a web3::ethabi::Event) -> impl Fn(Result<Log, web3::Error>) -> Ready<()> + 'a{
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