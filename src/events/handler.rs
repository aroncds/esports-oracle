use std::future::{self, Ready};
use std::collections::HashMap;

use log::info;
use web3::types::Log;
use web3::ethabi::{RawLog, Token};
use web3::{
    futures::StreamExt,
    transports::WebSocket,
};

use crate::contract;
use crate::events::types::Event;

use diesel::RunQueryDsl;
use crate::database::{
    models::Event as EventDB,
    conn::{connect, DatabaseError}
};

// fn convert_data(value: Token) -> Value {
//     match value {
//         Token::Address(x) => Value::Array(x.0),
//         Token::Array(x) => Value::Array(x),
//         Token::Bool(x) => Value::Bool(x),
//         Token::Bytes(x) => Value::Array(x),
//         Token::Int(x) => Value::Number(x),
//         Token::String(x) => Value::String(x),
//         Token::FixedBytes(x) => Value::Array(x),
//         Token::FixedArray(x) => Value::Array(x),
//         Token::Uint(x) => Value::Number(x.0),
//         Token::Tuple(x) => Value::Array(x)
//     }
// }

fn event_save_db(event_name: String, block_height: u64, event_params: HashMap<String, Token>) {
    use crate::database::schema::oracle_event::dsl::*;

    let mut conn = connect().expect("Failed");

    let event_db = EventDB {
        name: &event_name,
        block_number: block_height as i64,
        params: serde_json::from_str("{}").unwrap()
    };

    diesel::insert_into(oracle_event)
        .values(&event_db)
        .execute(&mut conn);
}

fn process_event(event: &web3::ethabi::Event) -> impl Fn(Result<Log, web3::Error>) -> Ready<()> + '_ {
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

            event_save_db(event.name.clone(), x.block_number.unwrap().as_u64(), params);
        }

        future::ready(())
    }
}

pub async fn subscribe_events(w: web3::Web3<WebSocket>) -> web3::contract::Result<()> {
    let contract = contract::create_platform_contract(&w)?;

    let subs = tokio::try_join!(
        contract::subscribe(&w, &contract, Event::MatchCreated),
        contract::subscribe(&w, &contract, Event::BetCreated)
    )?;

    let subscriptions = tokio::spawn(async move {
        let match_created = contract.abi().event("MatchCreated").unwrap();
        let bet_created = contract.abi().event("BetCreated").unwrap();

        tokio::join!(
            subs.0.for_each(process_event(match_created)),
            subs.1.for_each(process_event(bet_created))
        );
    });

    _ = tokio::join!(subscriptions);

    Ok(())
}