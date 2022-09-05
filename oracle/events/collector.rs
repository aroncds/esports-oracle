use tokio::time;

use web3::types::{BlockNumber, U64};
use web3::{Web3, types::FilterBuilder};
use web3::transports::WebSocket;

use super::types::Event;
use crate::contract;

pub struct Collector {
    space_time: u64,
    block_number: u64,
    provider: web3::Web3<WebSocket>,
}

impl Collector {

    pub fn new(ws: Web3<WebSocket>, space_time: u64) -> Self {
        Self {
            space_time,
            block_number: 0,
            provider: ws,
        }
    }

    async fn get_current_block(&self) -> Result<U64, web3::Error> {
        let block_number = self.provider.eth().block_number().await?;

        Ok(block_number)
    }

    async fn handle(&self) -> Result<(), web3::contract::Error> {
        let platform = contract::create_platform_contract(&self.provider)?;

        loop {
            let current_block = U64::from(self.block_number);
            let newest_block = self.get_current_block().await?;

            let filter = FilterBuilder::default()
                .address(vec![platform.address()])
                .from_block(BlockNumber::Number(current_block))
                .to_block(BlockNumber::Number(newest_block))
                .topics(Some(vec![Event::MatchCreated.into()]), None, None, None)
                .build();

            let filter = self.provider
                .eth_filter()
                .create_logs_filter(filter).await?;

            let stream = filter.stream(time::Duration::from_secs(1));

            time::sleep(time::Duration::from_millis(self.space_time)).await;
        }

        Ok(())
    }
}