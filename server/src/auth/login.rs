use actix_web::{post, web, HttpResponse, Responder};
use actix_web::cookie::{Cookie, SameSite, time::Duration};
use scylla::Session;
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::{Utc, Duration as ChronoDuration};
use futures::TryStreamExt;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

// JWT Secret Key
pub const SECRET_KEY: &[u8] = b"my_secret_key"; // Replace with a secure key in production

// Struct for JWT claims
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

#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginData>,
    session: web::Data<Arc<Mutex<Session>>>,
) -> impl Responder {
    let username = login_data.username.clone();
    let password = login_data.password.clone();

    let session = session.lock().await;

    // Prepare and execute the query
    let query = "SELECT password FROM inner_shelter.users WHERE username = ?";
    let prepared = session.prepare(query).await.unwrap();
    let result = session.execute_iter(prepared, (username.clone(),)).await.unwrap();
    let mut rows = result.into_typed::<(String,)>();

    if let Some(row) = rows.try_next().await.unwrap() {
        let stored_password = row.0;

        // Verify the password using bcrypt
        if verify(&password, &stored_password).unwrap() {
            // Generate JWT token with 1-hour expiration
            let expiration = Utc::now() + ChronoDuration::hours(1);
            let claims = Claims {
                sub: username.clone(),
                exp: expiration.timestamp() as usize,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(SECRET_KEY),
            )
            .unwrap();

            let cookie = Cookie::build("access_token", token)
                .http_only(true)
                .secure(false)
                .same_site(SameSite::Lax)
                .path("/")
                .max_age(Duration::seconds(3600))
                .finish();

            return HttpResponse::Ok()
                .cookie(cookie)
                .body("Login successful, token set in httpOnly cookie");
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}