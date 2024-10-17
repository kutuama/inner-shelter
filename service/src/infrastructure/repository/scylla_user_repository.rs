use crate::domain::user_repository::UserRepository;
use crate::errors::AppError;
use scylla::Session;
use std::sync::Arc;
use futures_util::stream::TryStreamExt;

pub struct ScyllaUserRepository {
    session: Arc<Session>,
}

impl ScyllaUserRepository {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }
}

#[async_trait::async_trait]
impl UserRepository for ScyllaUserRepository {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<String>, AppError> {
        let query = "SELECT password FROM inner_shelter.users WHERE username = ?";
        let prepared = self.session.prepare(query).await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        let result = self.session.execute_iter(prepared, (username,)).await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        let mut rows = result.into_typed::<(String,)>();

        if let Some(row) = rows.try_next().await
            .map_err(|e| AppError::DbError(e.to_string()))? {
            Ok(Some(row.0))
        } else {
            Ok(None)
        }
    }

    async fn create_user(&self, username: &str, password: String) -> Result<(), AppError> {
        let insert_query = "INSERT INTO inner_shelter.users (username, password) VALUES (?, ?)";
        let prepared_insert = self.session.prepare(insert_query).await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        self.session.execute_iter(prepared_insert, (username, password)).await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(())
    }
}
