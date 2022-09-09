use tokio::time;

use log::info;

use web3::contract::Contract;
use web3::types::{H256, H160};
use web3::types::{BlockNumber, U64};
use web3::{Web3, types::FilterBuilder};
use web3::transports::Http;

use super::types::Event;
use super::processor::Processor;
use crate::contract;
use crate::settings;

pub struct Collector {
    space_time: u64,
    block_number: u64,
    provider: web3::Web3<Http>,
    oracle: H256
}

impl Collector {

    pub fn new(ws: Web3<Http>, space_time: u64) -> Self {
        let oracle = H160::from(settings::ORACLE_ADDRESS);
        let oracle = H256::from(oracle);

        Self {
            space_time,
            block_number: 0,
            provider: ws,
            oracle
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

                self.request_events(&platform, current_block).await?
                    .save_events(&platform);
                    //.process_events();

                current_block += U64::from(1u64);
            }

            self.block_number = newest_block.as_u64();

            time::sleep(time::Duration::from_millis(self.space_time)).await;
        }
    }

    async fn request_events(&self, platform: &Contract<Http>, block: U64) -> Result<Processor, web3::contract::Error> {
        let filter = FilterBuilder::default()
            .address(vec![platform.address()])
            .from_block(BlockNumber::Number(block))
            .to_block(BlockNumber::Number(block))
            .topics(Some(vec![Event::MatchCreated.into(), Event::BetCreated.into()]), Some(vec![self.oracle]), None, None)
            .build();

        let logs = self.provider.eth().logs(filter).await?;

        Ok(Processor(logs))
    }
}
