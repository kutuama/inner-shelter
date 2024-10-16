use actix_web::{post, web, HttpResponse, Responder};
use crate::domain::auth::LoginData;
use crate::infrastructure::authentication;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::TryStreamExt;
use crate::config::Config;

#[post("/login")]
pub async fn login(
    login_data: web::Json<LoginData>,
    session: web::Data<Arc<Mutex<scylla::Session>>>,
    config: web::Data<Config>,
) -> impl Responder {
    let username = login_data.username.clone();
    let password = login_data.password.clone();

    let session = session.lock().await;

    let query = "SELECT password FROM inner_shelter.users WHERE username = ?";
    let prepared = session.prepare(query).await.unwrap();
    let result = session.execute_iter(prepared, (username.clone(),)).await.unwrap();
    let mut rows = result.into_typed::<(String,)>();

    if let Some(row) = rows.try_next().await.unwrap() {
        let stored_password = row.0;

        if authentication::verify_password(&password, &stored_password) {
            let token = authentication::generate_jwt(&username, config.jwt_secret.as_bytes());
            return HttpResponse::Ok().body(format!("Login successful, token: {}", token));
        }
    }

    HttpResponse::Unauthorized().body("Invalid credentials")
}
