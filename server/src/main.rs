use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

mod application;
mod infrastructure;
mod game;

use application::websocket::ws_handler;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();

    // Initialize shared game state
    let game_state = web::Data::new(Arc::new(Mutex::new(game::GameState::new())));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(game_state.clone())
            .wrap(actix_cors::Cors::default().allow_any_origin())
            .route("/ws", web::get().to(ws_handler))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
