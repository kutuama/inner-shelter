use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration as ChronoDuration};
use crate::domain::auth::Claims;
use crate::errors::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::AuthError(e.to_string()))
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    verify(password, hashed)
        .map_err(|e| AppError::AuthError(e.to_string()))
}

pub fn generate_jwt(username: &str, secret: &[u8]) -> Result<String, AppError> {
    let expiration = Utc::now() + ChronoDuration::hours(1);
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration.timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
        .map_err(|e| AppError::AuthError(e.to_string()))
}

