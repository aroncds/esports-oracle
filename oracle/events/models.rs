use web3::ethabi::LogParam;

use crate::database::schema::oracle_event;
use super::parameters::Args;

#[derive(Debug, Insertable)]
#[diesel(table_name = oracle_event)]
pub struct Event {
    /// Evevnt block height
    pub block_number: i64,
    /// Event name
    pub name: String,
    /// params
    pub params: Args,

    pub executed: bool
}

impl Event {

    pub fn new(name: &String, block_number: u64, params: &Vec<LogParam>) -> Self {
        Self {
            name: name.clone(),
            block_number: block_number as i64,
            params: Args::create(&String::from(name), params).unwrap(),
            executed: false
        }
    }
}
