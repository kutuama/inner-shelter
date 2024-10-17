pub mod scylla_user_repository;

use crate::domain::user_repository::UserRepository;
use scylla::Session;
use std::sync::Arc;

pub fn create_user_repository(session: Arc<Session>) -> Arc<dyn UserRepository> {
    Arc::new(scylla_user_repository::ScyllaUserRepository::new(session))
}
