use hex_literal::hex;
use lazy_static::lazy_static;

pub const HTTP_PROVIDER: &'static str = "https://rpc.gglabs.gg/";

pub const WS_PROVIDER: &'static str = "wss://rpc.gglabs.gg/ws";

pub const ORACLE_ADDRESS: [u8; 20] = hex!("C5BB81A1E5353740919CB5CE9fF87dfb3463F608");

pub const PLATFORM_ADDRESS: [u8; 20] = hex!("b0ee3790941196429da7CCAfDb612a95060d49d2");
