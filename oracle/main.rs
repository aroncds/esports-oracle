use log::{info, error};
use clap::Parser;

#[macro_use]
extern crate diesel;
extern crate serde;

mod contract;
mod settings;
mod events;
mod database;

#[derive(Parser)]
enum SubCommand {
    Start(StOptArgs)
}

#[derive(Parser)]
struct StOptArgs {
    contract_address: String,
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Start(_args) => start().await?,
        _ => error!("Command invalid")
    }

    Ok(())
}

async fn start() -> web3::contract::Result<()> {
    env_logger::init();

    info!("Oracle: Initializing");

    let web3 = contract::create_ws_web3().await;

    match web3 {
        Ok(w) => events::handler::subscribe_events(w).await?,
        Err(_) => error!("Failed to connect to ws!")
    }

    info!("Finalizing");

    Ok(())
}

