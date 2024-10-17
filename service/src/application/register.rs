use actix_web::{post, web, HttpResponse, Responder};
use shared::api::auth::RegisterData;
use crate::infrastructure::authentication;
use crate::domain::user_repository::UserRepository;
use crate::errors::AppError;
use std::sync::Arc;

#[post("/register")]
pub async fn register(
    register_data: web::Json<RegisterData>,
    user_repo: web::Data<Arc<dyn UserRepository>>,
) -> Result<impl Responder, AppError> {
    // Validate registration data
    register_data.validate().map_err(|e| AppError::ValidationError(e))?;

    let username = register_data.username.clone();
    let password = authentication::hash_password(&register_data.password)
        .map_err(|e| AppError::AuthError(e.to_string()))?;

    // Check if user already exists
    if user_repo.find_user_by_username(&username).await?.is_some() {
        return Err(AppError::ValidationError("Username already taken".into()));
    }

    // Create new user
    user_repo.create_user(&username, password).await?;

    Ok(HttpResponse::Ok().body("User created successfully"))
}
