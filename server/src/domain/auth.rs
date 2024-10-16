use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}
