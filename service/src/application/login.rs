use actix_web::{post, web, HttpResponse, Responder};
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use shared::api::auth::LoginData;
use crate::infrastructure::authentication;
use crate::domain::user_repository::UserRepository;
use crate::config::Config;
use crate::errors::AppError;
use std::sync::Arc;

#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginData>,
    user_repo: web::Data<Arc<dyn UserRepository>>,
    config: web::Data<Config>,
) -> Result<impl Responder, AppError> {
    // Validate login data
    login_data.validate().map_err(|e| AppError::ValidationError(e))?;

    let username = login_data.username.clone();
    let password = login_data.password.clone();

    let stored_password = match user_repo.find_user_by_username(&username).await? {
        Some(pw) => pw,
        None => return Err(AppError::AuthError("Invalid credentials".into())),
    };

    let is_valid = authentication::verify_password(&password, &stored_password)
        .map_err(|e| AppError::AuthError(e.to_string()))?;

    if is_valid {
        let token = authentication::generate_jwt(&username, config.jwt_secret.as_bytes())?;
        let token_clone = token.clone();
        let cookie = Cookie::build("access_token", token)
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Lax)
            .path("/")
            .max_age(Duration::seconds(3600))
            .finish();
        Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body(format!("Login successful, token: {}", token_clone)))
    } else {
        Err(AppError::AuthError("Invalid credentials".into()))
    }
}
