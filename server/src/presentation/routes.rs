use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::infrastructure::db::get_db_session;
use crate::infrastructure::repository::create_user_repository;
use crate::application::{login, register};
use crate::config::Config;
use crate::errors::AppError;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login)
        .service(register::register);
}

pub async fn start_server() -> Result<(), AppError> {
    let config = Config::new();

    // Attempt to get the database session
    let db_session = get_db_session(&config.db_url).await?;
    let user_repository = create_user_repository(db_session.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://innershelter.org:8081")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(user_repository.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")
    .map_err(|_e| AppError::InternalError)?
    .run()
    .await
    .map_err(|_e| AppError::InternalError)
}
