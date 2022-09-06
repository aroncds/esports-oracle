use std::collections::HashMap;
use tokio::time;

use log::info;

use web3::types::Log;
use web3::ethabi::{RawLog, Token};
use web3::types::{BlockNumber, U64};
use web3::{Web3, types::FilterBuilder};
use web3::transports::Http;

use super::types::Event;
use crate::contract;

fn process_event(event: &web3::ethabi::Event, x: &Log) {
    let log = event.parse_log(RawLog {
        topics: x.topics.clone(), data: x.data.0.clone() }).unwrap();

    super::handler::create_event(
        event.name.clone(),
        x.block_number.unwrap().as_u64(),
        log.params
    );
}

pub struct Collector {
    space_time: u64,
    block_number: u64,
    provider: web3::Web3<Http>,
}

impl Collector {

    pub fn new(ws: Web3<Http>, space_time: u64) -> Self {
        Self {
            space_time,
            block_number: 0,
            provider: ws,
        }
    }

    async fn get_current_block(&self) -> Result<U64, web3::Error> {
        self.provider.eth().block_number().await
    }

    pub async fn init(&mut self) -> Result<(), web3::Error> {
        self.block_number = self.get_current_block().await?.as_u64();
        Ok(())
    }

    pub async fn handle(&mut self) -> Result<(), web3::contract::Error> {
        let platform = contract::create_platform_contract(&self.provider)?;

        loop {
            let mut current_block = U64::from(self.block_number);
            let newest_block = self.get_current_block().await?;

            while current_block.as_u64() < newest_block.as_u64() {
                info!("Collecting events from {:?}", current_block);

                let filter = FilterBuilder::default()
                    .address(vec![platform.address()])
                    .from_block(BlockNumber::Number(current_block))
                    .to_block(BlockNumber::Number(current_block))
                    .topics(Some(vec![Event::MatchCreated.into(), Event::BetCreated.into()]), None, None, None)
                    .build();

                let filter = self.provider.eth().logs(filter).await?;

                let ev_match_created = platform.abi().event("MatchCreated").unwrap();
        
                let query = filter
                    .iter()
                    .filter(|x| Event::MatchCreated.eq(x.topics.get(0).unwrap()));

                for x in query {
                    process_event(ev_match_created, x);
                }

                let ev_bt = platform.abi().event("BetCreated").unwrap();
        
                let query = filter
                    .iter()
                    .filter(|x| Event::BetCreated.eq(x.topics.get(0).unwrap()));

                for x in query {
                    process_event(ev_bt, x);
                }

                current_block += U64::from(1u64);
            }

            self.block_number = newest_block.as_u64();

            time::sleep(time::Duration::from_millis(self.space_time)).await;
        }
    }
}
