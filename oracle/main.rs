use std::future;
use std::collections::HashMap;

use tokio;
use log::{info, error};
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
        let event = contract.abi().event("MatchCreated").unwrap();

        sub.for_each(|log| {
            match log {
                Ok(x) => {
                    let mut data = HashMap::new();

                    let log = event.parse_log(RawLog {
                        topics: x.topics.clone(),
                        data: x.data.0
                    }).unwrap();

                    log.params.iter().for_each(|i| { data.insert(i.name.clone(), i.value.clone()); });

                    info!("OnMatchCreated - Topic: {:?}, Block: {:?}, Data: {:?}", x.topics.get(0), x.block_number, data);        
                },
                Err(x) => error!("Invalid log: {:?}", x)
            }

            future::ready(())
        }).await;
    }));

    Ok(())
}
