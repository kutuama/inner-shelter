use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError(msg) => {
                log::error!("Database error: {}", msg);
                HttpResponse::InternalServerError().body("Database error")
            },
            AppError::AuthError(msg) => {
                log::warn!("Authentication error: {}", msg);
                HttpResponse::Unauthorized().body("Authentication failed")
            },
            AppError::ValidationError(msg) => {
                log::warn!("Validation error: {}", msg);
                HttpResponse::BadRequest().body(msg.clone())
            },
            AppError::InternalError => {
                log::error!("Internal server error");
                HttpResponse::InternalServerError().body("Internal server error")
            },
        }
    }
}
