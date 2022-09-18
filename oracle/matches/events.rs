use std::sync::Arc;

use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait,
    DatabaseConnection,
    QueryFilter,
    ColumnTrait
};

use crate::chain::{fields::Args};
use crate::database::matches;
use super::error::MatchError;

#[async_trait::async_trait]
pub trait BlockEventExecutor: Sync + Send {
    async fn run(&self, event: &Args, db: Arc<DatabaseConnection>) -> Result<(), MatchError>;
}

pub struct OnMatchCreated;

#[async_trait::async_trait]
impl BlockEventExecutor for OnMatchCreated {
    async fn run(&self, args: &Args, db: Arc<DatabaseConnection>) -> Result<(), MatchError> {
        if let Args::MatchCreated(x) = args {

            let m = matches::ActiveModel {
                id: NotSet,
                game_id: Set(x.game_id.to_string()),
                oracle: Set(x.game_id.to_string()),
                expire_time: Set(x.expire_time as i64),
                external_game_id: Set(x.external_game_id.to_string()),
                state: Set(1),
                master_player: NotSet
            };

            matches::Entity::insert(m).exec(&*db).await?;

            return Ok(());
        }

        Err(MatchError::ArgumentInvalid)
    }
}

pub struct OnBetCreated; 

#[async_trait::async_trait]
impl BlockEventExecutor for OnBetCreated {
    async fn run(&self, args: &Args, db: Arc<DatabaseConnection>) -> Result<(), MatchError> {
        if let Args::BetCreated(x) = args {

            let game_id = x.game_id;

            let m = matches::Entity::find()
                .filter(matches::Column::GameId.eq(game_id.to_string()))
                .one(&*db).await?;

            if let Some(x) = m {
                if let None = x.master_player {
                    
                }
            }

            return Ok(());
        }

        Err(MatchError::ArgumentInvalid)
    }
}
