use actix_web::{post, web, HttpResponse, Responder};
use scylla::Session;
use tokio::sync::Mutex;
use std::sync::Arc;
use futures::TryStreamExt;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    register_data: web::Json<RegisterData>,
    session: web::Data<Arc<Mutex<Session>>>,
) -> impl Responder {
    let username = register_data.username.clone();
    let password = register_data.password.clone();

    let session = session.lock().await;

    // Check if the username already exists
    let check_query = "SELECT username FROM inner_shelter.users WHERE username = ?";
    let prepared_check = session.prepare(check_query).await.unwrap();
    let result_check = session
        .execute_iter(prepared_check, (username.clone(),))
        .await
        .unwrap();

    if result_check
        .into_typed::<(String,)>()
        .try_next()
        .await
        .unwrap()
        .is_some()
    {
        return HttpResponse::BadRequest().body("Username already taken");
    }

    // Hash the password
    let hashed_password = hash(&password, DEFAULT_COST).unwrap();

    // Insert the new user into the database
    let insert_query = "INSERT INTO inner_shelter.users (username, password) VALUES (?, ?)";
    let prepared_insert = session.prepare(insert_query).await.unwrap();
    session
        .execute_iter(prepared_insert, (username, hashed_password))
        .await
        .unwrap();

    HttpResponse::Ok().body("User created successfully")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}