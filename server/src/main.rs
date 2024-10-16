use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use scylla::SessionBuilder;
use std::sync::Arc;
use tokio::sync::Mutex;

mod auth;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cassandra_uri = "127.0.0.1:9042";
    let session = SessionBuilder::new()
        .known_node(cassandra_uri)
        .build()
        .await
        .expect("Cassandra connection failed");
    let session = Arc::new(Mutex::new(session));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://innershelter.org:8081")
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .app_data(web::Data::new(session.clone()))
            .service(
                web::scope("/auth")
                .configure(auth::login::init_routes)
                .configure(auth::register::init_routes),
            )
            .route("/ws", web::get().to(handlers::websocket::websocket_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
