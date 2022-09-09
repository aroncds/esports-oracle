use log::{info, error};
use clap::Parser;
use dotenv::dotenv;

#[macro_use]
extern crate diesel;
extern crate serde;
extern crate oracle_data;

mod contract;
mod settings;
mod events;
mod database;
mod matches;

use events::collector::Collector;

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
    dotenv().ok();

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

    let web3 = contract::create_http_web3();

    match web3 {
        Ok(w) => {
            let mut collector = Collector::new(w, 5000);
            collector.init().await?;
            collector.handle().await?;
        },
        Err(_) => error!("Failed to connect to ws!")
    }

    info!("Finalizing");

    Ok(())
}

