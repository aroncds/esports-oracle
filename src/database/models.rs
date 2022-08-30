use crate::database::schema::oracle_event;

#[derive(Insertable)]
#[table_name = "oracle_event"]
pub struct Event<'a> {
    /// Evevnt block height
    pub block_number: i64,
    /// Event name
    pub name: &'a str,
    /// params
    pub params: serde_json::Value
}
