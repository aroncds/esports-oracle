use diesel::{prelude::PgConnection, Connection};
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub enum DatabaseError {
    InvalidUrl,
    FailedToConnect,
}

pub fn connect() -> Result<PgConnection, DatabaseError> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .map_err(|x| DatabaseError::InvalidUrl)?;

    let connection = PgConnection::establish(&db_url)
        .map_err(|x| DatabaseError::FailedToConnect)?;

    Ok(connection)
}
