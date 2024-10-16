use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration as ChronoDuration};
use crate::domain::auth::Claims;

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap()
}

pub fn generate_jwt(username: &str, secret: &[u8]) -> String {
    let expiration = Utc::now() + ChronoDuration::hours(1);
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration.timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap()
}
