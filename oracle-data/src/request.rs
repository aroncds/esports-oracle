use async_trait::async_trait;

pub enum Error {
    InvalidServer,
}

#[async_trait]
pub trait DataRequest<T> {
    async fn get_game_info<B>(id: T) -> Result<B, Error>;
    async fn get_game_result<B>(id: T) -> Result<B, Error>;
}