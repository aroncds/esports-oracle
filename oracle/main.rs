use std::future;

use tokio;
use log::info;
use web3::{
    futures::StreamExt,
    transports::WebSocket,
    types::Log
};

mod contract;
mod settings;
mod events;

use events::Event;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    env_logger::init();

    info!("Oracle: Initializing");

    let web3 = contract::create_ws_web3().await;

    match web3 {
        Ok(w) => initialize_events(w).await?,
        Err(_) => panic!("Failed")
    }

    Ok(())
}

async fn initialize_events(w: web3::Web3<WebSocket>) -> web3::contract::Result<()> {
    let contract = contract::create_platform_contract(&w)?;
    let sub = contract::subscribe(&w, &contract, Event::MatchCreated).await?;

    _ = tokio::join!(tokio::spawn(async move {
        sub.for_each(|log| {
            let x: Log = log.unwrap();

            info!("OnMatchCreated - Topic: {:?}, Block: {:?}", x.topics.get(0), x.block_number);

            future::ready(())
        }).await;
    }));

    Ok(())
}
