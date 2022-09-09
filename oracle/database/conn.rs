use diesel::{prelude::PgConnection, Connection};
use crate::settings;

#[derive(Debug)]
pub enum DatabaseError {
    InvalidUrl,
    FailedToConnect,
}

pub fn establish_connection() -> Result<PgConnection, DatabaseError> {
    let db_url = settings::get_database_url();

    let connection = PgConnection::establish(&db_url)
        .map_err(|x| DatabaseError::FailedToConnect)?;

    Ok(connection)
}
