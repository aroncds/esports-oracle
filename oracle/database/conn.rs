use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::settings;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = settings::get_database_url();

    let connection = Database::connect(db_url).await?;

    Ok(connection)
}
