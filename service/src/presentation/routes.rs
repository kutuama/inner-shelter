use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::infrastructure::db::get_db_session;
use crate::infrastructure::repository::{create_user_repository, create_game_repository};
use crate::application::{login, register};
use crate::game::systems::game_service::GameService;
use crate::config::Config;
use crate::errors::AppError;
use crate::presentation::websocket::ws_handler;
use std::sync::Arc;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login)
        .service(register::register)
        .route("/ws", web::get().to(ws_handler));
}

pub async fn start_server() -> Result<(), AppError> {
    let config = Config::new();

    // Attempt to get the database session
    let db_session = get_db_session(&config.db_url).await?;
    let user_repository = create_user_repository(db_session.clone());

    let game_repository = create_game_repository(&config)?;
    let game_service = Arc::new(GameService::new(game_repository.clone()));

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
            .app_data(web::Data::new(game_service.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")
    .map_err(|_e| AppError::InternalError)?
    .run()
    .await
    .map_err(|_e| AppError::InternalError)
}
