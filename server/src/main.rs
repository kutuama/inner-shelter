use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use scylla::SessionBuilder;
use std::sync::Arc;
use tokio::sync::Mutex;

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the Cassandra client
    let cassandra_uri = "127.0.0.1:9042";
    let session = SessionBuilder::new()
        .known_node(cassandra_uri)
        .build()
        .await
        .expect("Cassandra connection failed");
    let session = Arc::new(Mutex::new(session));

    // Define the HTTP server and routes
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://innershelter.org:8081")
                    .allow_any_method()  // Allow all HTTP methods
                    .allow_any_header()  // Allow all headers like Content-Type
                    .supports_credentials() // Allow credentials like cookies
            )
            .app_data(web::Data::new(session.clone()))
            .service(web::scope("/auth").configure(auth::init_routes))  // Configures the auth routes
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
