use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::infrastructure::db;
use crate::application::{login, register};
use crate::config::Config;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login)
        .service(register::register);
}

pub async fn start_server() -> std::io::Result<()> {
    let config = Config::new();
    let db_session = db::get_db_session(&config.db_url).await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://innershelter.org:8081")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(db_session.clone()))
            .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
