use crate::errors::AppError;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_user_by_username(&self, username: &str) -> Result<Option<String>, AppError>;
    async fn create_user(&self, username: &str, password: String) -> Result<(), AppError>;
}
