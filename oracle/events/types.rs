use web3::types::H256;
use hex_literal::hex;

/// Names
const MATCH_CREATED: &'static str  = "MatchCreated";
const BET_CREATED: &'static str  = "BetCreated";
const MATCH_FINISHED: &'static str = "MatchFinished";

/// Hex keccack256
const ON_MATCH_CREATED: [u8; 32] = hex!("e305bb6965bd4c391d63fd3798b907578aaedfb039342115cf18db4c0b3c07b0");
const ON_BET_CREATED: [u8; 32] = hex!("fa298d3e4326da969389ce9252a0c4c47e5d093a8ebfe43b7ae8236484a3c8fd");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    MatchCreated,
    BetCreated,
    MatchFinished
}

impl From<[u8; 32]> for Event {
    fn from(value: [u8; 32]) -> Self {
        match value {
            ON_MATCH_CREATED => Event::MatchCreated,
            ON_BET_CREATED => Event::BetCreated,
            _ => panic!("Invalid value")
        }
    }
}

impl From<Event> for [u8; 32] {
    fn from(value: Event) -> Self {
        match value {
            Event::MatchCreated => ON_MATCH_CREATED,
            Event::BetCreated => ON_BET_CREATED,
            _ => panic!("Invalid value")
        }
    }
}


impl From<Event> for H256 {
    fn from(value: Event) -> Self {
        match value {
            Event::MatchCreated => ON_MATCH_CREATED.into(),
            Event::BetCreated => ON_BET_CREATED.into(),
            _ => panic!("Invalid value")
        }
    }
}

impl PartialEq<H256> for Event {
    fn eq(&self, other: &H256) -> bool {
        let ev: H256 = (*self).into();
        ev.eq(other)
    }
}

impl From<Event> for &str {
    fn from(value: Event) -> Self {
        match value {
            Event::MatchCreated => MATCH_CREATED,
            Event::BetCreated => BET_CREATED,
            Event::MatchFinished => MATCH_FINISHED,
            _ => panic!("Invalid value")
        }
    }
}

impl From<&String> for Event {
    fn from(value: &String) -> Self {
        match value.as_str() {
            MATCH_CREATED => Event::MatchCreated,
            BET_CREATED => Event::BetCreated,
            MATCH_FINISHED => Event::MatchFinished,
            _ => panic!("Invalid value")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_string_to_event() {
        let value = String::from("MatchCreated");
        let event: Event = (&value).into();
        assert_eq!(event, Event::MatchCreated);
    }
}