use tokio;
use log::{info, error};

#[macro_use]
extern crate diesel;
extern crate serde;

mod contract;
mod settings;
mod events;
mod database;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    env_logger::init();

    info!("Oracle: Initializing");

    connect_web3().await?;

    info!("Finalizing");

    Ok(())
}

async fn connect_web3() -> web3::contract::Result<()> {
    let web3 = contract::create_ws_web3().await;

    match web3 {
        Ok(w) => events::handler::subscribe_events(w).await?,
        Err(_) => error!("Failed to connect to ws!")
    }

    Ok(())
}

