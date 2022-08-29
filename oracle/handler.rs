use std::future::{self, Ready};
use std::collections::HashMap;

use log::info;
use web3::types::Log;
use web3::ethabi::RawLog;

pub fn process_event(event: &web3::ethabi::Event) -> impl Fn(Result<Log, web3::Error>) -> Ready<()> + '_ {
    |log| {
        if let Ok(x) = log {
            let mut params = HashMap::new();

            let log = event.parse_log(RawLog {
                topics: x.topics.clone(), data: x.data.0 }).unwrap();

            log.params.iter().for_each(|i| {
                params.insert(i.name.clone(), i.value.clone()); });

            info!("{:?} - Topic: {:?}, Block: {:?}, Data: {:?}",
                event.name,
                x.topics.get(0),
                x.block_number,
                params
            );  
        }

        future::ready(())
    }
}