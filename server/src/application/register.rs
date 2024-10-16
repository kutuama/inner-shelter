use actix_web::{post, web, HttpResponse, Responder};
use crate::domain::auth::RegisterData;
use crate::infrastructure::authentication;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::TryStreamExt;

#[post("/register")]
pub async fn register(
    register_data: web::Json<RegisterData>,
    session: web::Data<Arc<Mutex<scylla::Session>>>,
) -> impl Responder {
    let username = register_data.username.clone();
    let password = authentication::hash_password(&register_data.password);

    let session = session.lock().await;

    let check_query = "SELECT username FROM inner_shelter.users WHERE username = ?";
    let prepared_check = session.prepare(check_query).await.unwrap();
    let result_check = session.execute_iter(prepared_check, (username.clone(),)).await.unwrap();

    if result_check.into_typed::<(String,)>().try_next().await.unwrap().is_some() {
        return HttpResponse::BadRequest().body("Username already taken");
    }

    let insert_query = "INSERT INTO inner_shelter.users (username, password) VALUES (?, ?)";
    let prepared_insert = session.prepare(insert_query).await.unwrap();
    session.execute_iter(prepared_insert, (username, password)).await.unwrap();

    HttpResponse::Ok().body("User created successfully")
}
