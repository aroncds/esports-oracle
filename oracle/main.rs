use tokio;
use log::{info, error};
use web3::{
    futures::StreamExt,
    transports::WebSocket,
};

mod contract;
mod settings;
mod types;
mod handler;

use types::Event;

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

        sub.for_each(handler::process_event(on_match_created)).await;
    }));

    Ok(())
}
