use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("Failed to instantiate a contract.")]
    ContractFailedToLoad,

    #[error("Failed to get block height")]
    BlockHeightFailed,

    #[error("Failed to load logs from height {0}")]
    FailedToLoadLogs(u64),

    #[error("It is not possible to insert event entry: {0}")]
    EventInsertFailed(String)
}
