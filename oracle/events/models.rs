use diesel::PgConnection;
use diesel::RunQueryDsl;

use crate::database::schema::oracle_event;
use crate::database::schema::oracle_event::dsl::*;
use crate::events::parameters::Args;

pub enum Error {
    FailedToSave,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = oracle_event)]
pub struct Event<'a> {
    /// Evevnt block height
    pub block_number: i64,
    /// Event name
    pub name: &'a str,
    /// params
    pub params: Args,

    pub executed: bool
}

impl<'a> Event<'a> {
    pub fn save(&self, conn: &mut PgConnection) -> Result<(), Error> {
        let result = diesel::insert_into(oracle_event)
            .values(self)
            .execute(conn);

        result.map_err(|x| Error::FailedToSave)?; 

        Ok(())
    }
}