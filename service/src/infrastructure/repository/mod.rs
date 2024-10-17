pub mod scylla_user_repository;
pub mod redis_game_repository;

use crate::domain::user_repository::UserRepository;
use scylla::Session;
use std::sync::Arc;

pub fn create_user_repository(session: Arc<Session>) -> Arc<dyn UserRepository> {
    Arc::new(scylla_user_repository::ScyllaUserRepository::new(session))
}

use crate::game::components::game_repository::GameRepository;
use crate::config::Config;
use crate::infrastructure::repository::redis_game_repository::RedisGameRepository;

pub fn create_game_repository(config: &Config) -> Result<Arc<dyn GameRepository>, crate::errors::AppError> {
    let repo = RedisGameRepository::new(config)?;
    Ok(Arc::new(repo))
}
