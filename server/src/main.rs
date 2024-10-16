mod config;
mod domain;
mod infrastructure;
mod application;
mod presentation;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    presentation::routes::start_server().await
}
