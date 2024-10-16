use actix_web::{post, web, HttpResponse, Responder};
use crate::domain::auth::RegisterData;
use crate::infrastructure::authentication;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::TryStreamExt;
use crate::errors::AppError; // Import AppError
use scylla::Session;

#[post("/register")]
pub async fn register(
    register_data: web::Json<RegisterData>,
    session: web::Data<Arc<Mutex<Session>>>,
) -> Result<impl Responder, AppError> { // Change return type
    let username = register_data.username.clone();
    let password = authentication::hash_password(&register_data.password)
        .map_err(|e| AppError::AuthError(e.to_string()))?; // Handle hashing error

    let session = session.lock().await;

    let check_query = "SELECT username FROM inner_shelter.users WHERE username = ?";
    let prepared_check = session.prepare(check_query).await
        .map_err(|e| AppError::DbError(e.to_string()))?;
    let result_check = session.execute_iter(prepared_check, (username.clone(),)).await
        .map_err(|e| AppError::DbError(e.to_string()))?;

    if result_check.into_typed::<(String,)>().try_next().await
        .map_err(|e| AppError::DbError(e.to_string()))?.is_some() {
        return Err(AppError::ValidationError("Username already taken".into()));
    }

    let insert_query = "INSERT INTO inner_shelter.users (username, password) VALUES (?, ?)";
    let prepared_insert = session.prepare(insert_query).await
        .map_err(|e| AppError::DbError(e.to_string()))?;
    session.execute_iter(prepared_insert, (username, password)).await
        .map_err(|e| AppError::DbError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("User created successfully"))
}
