use web3::Transport;
use web3::contract::Contract;
use web3::types::H160;
use web3::transports::{Http, WebSocket};

use crate::settings::{HTTP_PROVIDER, WS_PROVIDER, PLATFORM_ADDRESS};

fn create_contract<T>(w: &web3::Web3<T>, address: H160, json: &[u8]) -> web3::contract::Result<Contract<T>>
    where T: Transport
{
    Ok(Contract::from_json(w.eth(), address, json)?)
}

pub fn create_http_web3() -> web3::Result<web3::Web3<Http>> {
    let transport = Http::new(HTTP_PROVIDER)?;

    Ok(web3::Web3::new(transport))
}

pub fn create_oracle_contract<T>(w: &web3::Web3<T>, address: H160) -> web3::contract::Result<Contract<T>>
    where T: Transport
{
    create_contract(
        w,
        address,
        include_bytes!("../abi/oracle.json")
    )
}

pub fn create_platform_contract<T>(w: &web3::Web3<T>) -> web3::contract::Result<Contract<T>>
    where T: Transport
{
    create_contract(w, PLATFORM_ADDRESS.into(), include_bytes!("../abi/platform.json"))
}
