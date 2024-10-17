use async_trait::async_trait;
use crate::errors::AppError;
use crate::domain::entities::PositionUpdate;
use futures_util::Stream;
use redis::Msg;
use std::pin::Pin;

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn get_position(&self, username: &str) -> Result<(i32, i32), AppError>;
    async fn set_position(&self, username: &str, x: i32, y: i32) -> Result<(), AppError>;
    async fn publish_position_update(&self, update: &PositionUpdate) -> Result<(), AppError>;
    async fn subscribe(&self) -> Result<Pin<Box<dyn Stream<Item = Result<Msg, redis::RedisError>> + Send>>, AppError>;
}
