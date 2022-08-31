use async_trait::async_trait;

#[derive(Debug)]
pub enum Error {
    NotFound,

    ResponseForbidden,
}

#[derive(Debug, Clone)]
pub enum State {
    Waiting,
    Running,
    Finished
}

#[derive(Debug, Clone)]
pub struct MatchInfo<T> {
    pub id: T,

    pub created_at: u64,

    pub players: Vec<String>,

    pub state: State
}

#[async_trait]
pub trait MatchData<T> {
    async fn get_match_state(id: T) -> Result<State, Error>;
    async fn get_match_result(id: T) -> Result<String, Error>;
    async fn get_match_info(id: T) -> Result<MatchInfo<T>, Error>;
}