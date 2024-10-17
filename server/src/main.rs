mod config;
mod game;
mod domain;
mod infrastructure;
mod application;
mod presentation;
mod errors;

use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env::set_var("RUST_LOG", "info"); // Set default log level
    env_logger::init();

    // Start the server and handle potential AppError
    match presentation::routes::start_server().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Server failed to start: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Server failed to start"))
        }
    }
}
