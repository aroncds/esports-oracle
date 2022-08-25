use web3::api::SubscriptionStream;
use web3::types::Log;
use web3::Transport;
use web3::contract::Contract;
use web3::types::{H160, FilterBuilder};
use web3::transports::{Http, WebSocket};

use crate::events::Event;
use crate::settings::{HTTP_PROVIDER, WS_PROVIDER, PLATFORM_ADDRESS};

pub fn create_http_web3() -> web3::Result<web3::Web3<Http>> {
    let transport = Http::new(HTTP_PROVIDER)?;

    Ok(web3::Web3::new(transport))
}

pub async fn create_ws_web3() -> web3::Result<web3::Web3<WebSocket>> {
    let transport = WebSocket::new(WS_PROVIDER).await?;

    Ok(web3::Web3::new(transport))
}

fn create_contract<T>(w: &web3::Web3<T>, address: H160, json: &[u8]) -> web3::contract::Result<Contract<T>>
    where T: Transport
{
    Ok(Contract::from_json(w.eth(), address, json)?)
}

pub fn create_oracle_contract<T>(w: &web3::Web3<T>, address: H160) -> web3::contract::Result<Contract<T>>
    where T: Transport
{
    create_contract(
        w,
        address,
        include_bytes!("./abi/Oracle.json")
    )
}

pub fn create_platform_contract<T>(w: &web3::Web3<T>, ) -> web3::contract::Result<Contract<T>>
    where T: Transport
{
    create_contract(w, PLATFORM_ADDRESS.into(), include_bytes!("./abi/GoodGame.json"))
}

pub async fn subscribe(
    w: &web3::Web3<WebSocket>,
    contract: &Contract<WebSocket>,
    topic: Event
) -> web3::Result<SubscriptionStream<WebSocket, Log>> {
    let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .topics(Some(vec![topic.into()]), None, None, None);
    
    let sub = w.eth_subscribe()
        .subscribe_logs(filter.build())
        .await?;

    Ok(sub)
}