use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<String, ()> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "my_secret_key".into());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ).map_err(|_| ())?;

    Ok(token_data.claims.sub)
}
