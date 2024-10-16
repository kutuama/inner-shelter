use actix_web::{post, web, HttpResponse, Responder};
use crate::domain::auth::LoginData;
use crate::infrastructure::authentication;
use std::sync::Arc;
use futures::TryStreamExt;
use crate::config::Config;
use crate::errors::AppError;
use scylla::Session;

#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginData>,
    session: web::Data<Arc<Session>>,
    config: web::Data<Config>,
) -> Result<impl Responder, AppError> {
    let username = login_data.username.clone();
    let password = login_data.password.clone();

    let query = "SELECT password FROM inner_shelter.users WHERE username = ?";
    let prepared = session.prepare(query).await
        .map_err(|e| AppError::DbError(e.to_string()))?;
    let result = session.execute_iter(prepared, (username.clone(),)).await
        .map_err(|e| AppError::DbError(e.to_string()))?;
    let mut rows = result.into_typed::<(String,)>();

    if let Some(row) = rows.try_next().await
        .map_err(|e| AppError::DbError(e.to_string()))? {
        let stored_password = row.0;

        let is_valid = authentication::verify_password(&password, &stored_password)
            .map_err(|e| AppError::AuthError(e.to_string()))?;

        if is_valid {
            let token = authentication::generate_jwt(&username, config.jwt_secret.as_bytes())?;
            return Ok(HttpResponse::Ok().body(format!("Login successful, token: {}", token)));
        } else {
            return Err(AppError::AuthError("Invalid credentials".into()));
        }
    }

    Err(AppError::AuthError("Invalid credentials".into()))
}
