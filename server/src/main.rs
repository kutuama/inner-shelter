mod config;
mod auth;
mod handlers;
mod services;
mod components;

use warp::Filter;
use tokio::sync::broadcast;
use handlers::websocket::handle_socket;

#[tokio::main]
async fn main() {
    let redis_client = config::get_redis_client();
    let (tx, _rx) = broadcast::channel(16);
    let jwt_secret: &[u8] = b"your_secret_key";

    let filter = warp::path("ws")
    .and(warp::ws())
    .and(warp::cookie::optional("jwt"))
    .and_then(move |ws: warp::ws::Ws, jwt: Option<String>| {
        let tx = tx.clone();
        let redis_client = redis_client.clone();
        let jwt_secret = jwt_secret; // Removed `.clone()` here
        async move {
            if let Some(token) = jwt {
                match auth::validate_jwt(&token, &jwt_secret) {
                    Ok(user_id) => Ok(ws.on_upgrade(move |socket| handle_socket(socket, tx, redis_client, user_id))),
                    Err(_) => Err(warp::reject::custom(auth::Unauthorized)),
                }
            } else {
                Err(warp::reject::custom(auth::Unauthorized))
            }
        }
    });

    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}
