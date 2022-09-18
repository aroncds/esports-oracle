use sea_orm::DbErr;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum MatchError {
    #[error("Event not found")]
    EventNotFound,

    #[error("Argument invalid for event")]
    ArgumentInvalid,

    #[error("Failed to insert on database: {0}")]
    DbInsertFailed(#[from] DbErr)
}