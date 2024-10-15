use actix_web::{post, web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use scylla::Session;
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::{Utc, Duration};
use futures::TryStreamExt;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

const SECRET_KEY: &[u8] = b"my_secret_key"; // Replace with a secure key in production

#[post("/login")]
async fn login(
    login_data: web::Json<LoginData>,
    session: web::Data<Arc<Mutex<Session>>>,
) -> impl Responder {
    let username = login_data.username.clone();
    let password = login_data.password.clone();

    // Lock the session
    let session = session.lock().await;

    // Prepare and execute the query
    let query = "SELECT password FROM my_keyspace.users WHERE username = ?";

    // Prepare the statement
    let prepared = session.prepare(query).await.unwrap();

    // Execute the query and get an iterator over the results
    let result = session.execute_iter(prepared, (username.clone(),)).await.unwrap();

    // Convert the result into a typed iterator
    let mut rows = result.into_typed::<(String,)>();

    // Fetch the first row
    if let Some(row) = rows.try_next().await.unwrap() {
        let stored_password = row.0;

        if stored_password == password {
            // Generate JWT token
            let expiration = Utc::now() + Duration::hours(24); // Token expires in 24 hours
            let claims = Claims {
                sub: username,
                exp: expiration.timestamp() as usize,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(SECRET_KEY),
            )
            .unwrap();

            // Set the JWT token as an HTTP-only cookie
            let cookie = Cookie::build("jwt", token)
                .http_only(true) // Make the cookie http-only
                .secure(false)    // Disable secure for HTTP (development only)
                .same_site(actix_web::cookie::SameSite::Lax)
                .path("/")       // Set the path where the cookie is accessible
                .finish();

            return HttpResponse::Ok()
                .cookie(cookie) // Set the cookie in the response
                .body("Login successful, token set in httpOnly cookie");
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
