#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub db_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or("my_secret_key".to_string()),
            db_url: std::env::var("DATABASE_URL").unwrap_or("127.0.0.1:9042".to_string()),
            redis_url: std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string()),
        }
    }
}
