use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("Failed to handle with web3 contract function {0}.")]
    Web3Contract(#[from] web3::contract::Error),

    #[error("Failed to handle with web3 function {0}")]
    Web3(#[from] web3::Error),

    #[error("Failed to insert data {0}")]
    DbError(#[from] sea_orm::DbErr)
}

pub type Result<T> = std::result::Result<T, ChainError>;
