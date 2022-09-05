use async_trait::async_trait;
use thiserror::Error;
use log::error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("There is not the data in api!")]
    NotFound,

    #[error("Forbidden request: {0}")]
    ResponseForbidden(String),

    #[error("Failed to read cache")]
    CacheUnavailable,

    #[error("Failed to set value in cache")]
    CacheSetFailed,
}

#[derive(Debug, Clone)]
pub enum State {
    Running,
    Finished
}

#[derive(Debug, Clone)]
pub struct MatchInfo<T> {
    pub id: T,

    pub created_at: i64,

    pub players: Vec<String>,

    pub state: State
}

#[async_trait]
pub trait MatchData<T> {
    // async fn get_match_state(&self, id: &T) -> Result<State, Error>;
    // async fn get_match_result(&self, id: &T) -> Result<String, Error>;
    async fn get_match_by_id(&self, id: &T) -> Result<MatchInfo<T>, Error>;
    async fn get_match_by_player_id(&self, player_name: String) -> Result<MatchInfo<T>, Error>;
}