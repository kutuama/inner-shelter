use crate::domain::game_repository::GameRepository;
use crate::game::entities::{MoveCommand, PositionUpdate};
use crate::errors::AppError;
use std::sync::Arc;
use futures_util::Stream;
use redis::Msg;
use std::pin::Pin;

pub struct GameService {
    game_repository: Arc<dyn GameRepository>,
}

impl GameService {
    pub fn new(game_repository: Arc<dyn GameRepository>) -> Self {
        Self { game_repository }
    }

    pub async fn handle_move(&self, username: &str, move_cmd: MoveCommand) -> Result<PositionUpdate, AppError> {
        let (mut x, mut y) = self.game_repository.get_position(username).await?;
        if move_cmd.action == "move" {
            x += move_cmd.dx;
            y += move_cmd.dy;
            x = x.max(1).min(10);
            y = y.max(1).min(10);

            self.game_repository.set_position(username, x, y).await?;

            let position_update = PositionUpdate {
                action: "update_position".to_string(),
                username: username.to_string(),
                x,
                y,
            };
            self.game_repository.publish_position_update(&position_update).await?;

            Ok(position_update)
        } else {
            Err(AppError::ValidationError("Invalid action".to_string()))
        }
    }

    pub async fn get_initial_position(&self, username: &str) -> Result<PositionUpdate, AppError> {
        let (x, y) = self.game_repository.get_position(username).await?;
        let position_update = PositionUpdate {
            action: "update_position".to_string(),
            username: username.to_string(),
            x,
            y,
        };
        self.game_repository.publish_position_update(&position_update).await?;

        Ok(position_update)
    }

    pub async fn subscribe(&self) -> Result<Pin<Box<dyn Stream<Item = Result<Msg, redis::RedisError>> + Send>>, AppError> {
        self.game_repository.subscribe().await
    }
}
