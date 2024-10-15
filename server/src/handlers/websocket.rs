use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Payload;
use redis::AsyncCommands;
use redis::Client as RedisClient;
use tokio::sync::broadcast;
use futures_util::StreamExt;
use uuid::Uuid;
use crate::auth::jwt::validate_jwt;

pub async fn websocket_route(
    req: HttpRequest,
    stream: Payload,
    redis_client: web::Data<RedisClient>,
    tx: web::Data<broadcast::Sender<String>>,
) -> impl Responder {
    let query_string = req.query_string();
    let query_params: std::collections::HashMap<String, String> = serde_urlencoded::from_str(query_string).unwrap();

    if let Some(token) = query_params.get("token").map(String::as_str) {
        match validate_jwt(&token, b"your_secret_key") {
            Ok(user_id) => {
                match handle_socket(stream, tx, redis_client, user_id).await {
                    Ok(_) => HttpResponse::Ok().finish(),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            },
            Err(_) => HttpResponse::Unauthorized().body("Invalid Token"),
        }
    } else {
        HttpResponse::BadRequest().body("Missing JWT token")
    }
}

async fn handle_socket(
    mut stream: Payload,
    tx: web::Data<broadcast::Sender<String>>,
    redis_client: web::Data<RedisClient>,
    user_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client_id = Uuid::new_v4().to_string();
    let mut redis_con = redis_client.get_multiplexed_async_connection().await?;

    // WebSocket message handling
    while let Some(Ok(msg)) = stream.next().await {
        if let Ok(text) = std::str::from_utf8(&msg) {
            let _: () = redis_con.set(&client_id, text).await?;
            tx.send(format!("{}: {}", user_id, text)).unwrap();
        }
    }

    Ok(())
}
