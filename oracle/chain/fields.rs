use serde::Deserialize;
use serde::Serialize;

use web3::ethabi::LogParam;
use web3::types::H256;

use super::types::Event;

type EventParams = Vec<LogParam>;

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchCreatedParams {
    pub game_id: H256,
    pub expire_time: u64,
    pub external_game_id: H256
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BetCreatedParams {
    pub game_id: H256
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Args {
    MatchCreated(MatchCreatedParams),
    BetCreated(BetCreatedParams)
}

impl Args {
    pub fn create(event_name: &String, params: &EventParams) -> Option<Args> {
        match event_name.into() {
            Event::MatchCreated => Some(Args::MatchCreated(MatchCreatedParams::from(params))),
            Event::BetCreated => Some(Args::BetCreated(BetCreatedParams::from(params))),
            Event::MatchFinished => None
        }
    }
}

impl From<&EventParams> for MatchCreatedParams {
    fn from(x: &EventParams) -> Self {
        let game_id: [u8; 32] = x.get(3)
            .unwrap()
            .clone()
            .value
            .into_fixed_bytes()
            .unwrap()
            .try_into()
            .unwrap();

        let expire_time = x.get(8)
            .unwrap()
            .clone()
            .value
            .into_uint()
            .unwrap()
            .as_u64();

        let mut external_game_id: [u8; 32] = [0; 32];

        external_game_id.copy_from_slice(
            &x.get(4)
                .unwrap()
                .clone()
                .value
                .into_fixed_bytes()
                .unwrap()[0..32]
        );

        Self {
            game_id: game_id.into(),
            expire_time,
            external_game_id: external_game_id.into(),
        }
    }
}

impl From<&EventParams> for BetCreatedParams {
    fn from(x: &EventParams) -> Self {
        let game_id: [u8; 32] = x.get(2)
            .unwrap()
            .clone()
            .value
            .into_fixed_bytes()
            .unwrap()
            .try_into()
            .unwrap();
        
        Self {
            game_id: game_id.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_event_params_to_match_created_params() {
        let params = EventParams::new();
    }
}