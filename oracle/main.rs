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
    let subs = tokio::try_join!(
        contract::subscribe(&w, &contract, Event::MatchCreated),
        contract::subscribe(&w, &contract, Event::BetCreated)
    )?;

    let subscriptions = tokio::spawn(async move {
        let match_created = contract.abi().event("MatchCreated").unwrap();
        let bet_created = contract.abi().event("MatchCreated").unwrap();

        tokio::join!(
            subs.0.for_each(handler::process_event(match_created)),
            subs.1.for_each(handler::process_event(bet_created))
        );
    });

    _ = tokio::join!(subscriptions);

    Ok(())
}
