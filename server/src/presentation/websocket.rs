use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web::web::Payload;
use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use crate::config::Config;
use crate::infrastructure::authentication;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

pub async fn ws_handler(
    req: HttpRequest,
    stream: Payload,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    // Extract the access_token from cookies
    let token = extract_access_token(&req);

    // Validate token
    let username = if let Some(token) = token {
        match authentication::validate_jwt(&token, config.jwt_secret.as_bytes()) {
            Ok(claims) => claims.sub,
            Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
        }
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let (response, session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    actix_rt::spawn(async move {
        if let Err(e) = ws_session(session, &mut msg_stream, config.get_ref().clone(), username).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    });

    Ok(response)
}

fn extract_access_token(req: &HttpRequest) -> Option<String> {
    req.cookie("access_token").map(|cookie| cookie.value().to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct MoveCommand {
    action: String,
    dx: i32,
    dy: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PositionUpdate {
    action: String,
    username: String,
    x: i32,
    y: i32,
}

async fn ws_session(
    mut session: Session,
    msg_stream: &mut MessageStream,
    config: Config,
    username: String,
) -> Result<(), Error> {
    // Connect to Redis
    let client = redis::Client::open(config.redis_url.clone()).map_err(|e| {
        eprintln!("Failed to create Redis client: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // Connection for publishing and data manipulation
    let mut conn = client.get_async_connection().await.map_err(|e| {
        eprintln!("Failed to connect to Redis: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // Connection for subscribing
    let mut pubsub_conn = client.get_async_connection().await.map_err(|e| {
        eprintln!("Failed to connect to Redis for pubsub: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?.into_pubsub();

    pubsub_conn.subscribe("game_channel").await.map_err(|e| {
        eprintln!("Failed to subscribe to Redis channel: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let mut redis_stream = pubsub_conn.on_message();

    // Initialize player position if not set
    let key = format!("player:{}", username);
    let exists: bool = conn.exists(&key).await.map_err(|e| {
        eprintln!("Failed to check if key exists: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let (mut x, mut y): (i32, i32) = if exists {
        conn.hget(&key, &["x", "y"]).await.map_err(|e| {
            eprintln!("Failed to get position from Redis: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?
    } else {
        (1, 1)
    };

    // Notify other clients of the new player
    let position_update = PositionUpdate {
        action: "update_position".to_string(),
        username: username.clone(),
        x,
        y,
    };
    let message = serde_json::to_string(&position_update).unwrap();

    // Publish to Redis
    let _: () = conn.publish::<_, _, ()>("game_channel", message.clone()).await.map_err(|e| {
        eprintln!("Failed to publish to Redis: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // Send the initial position to the current client
    if session.text(message).await.is_err() {
        // Client disconnected
        return Ok(());
    }

    loop {
        tokio::select! {
            // Handle messages from the client
            Some(msg) = msg_stream.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        eprintln!("Received message from client: {}", text);
                        let text = text.to_string();
                        // Parse the movement command
                        if let Ok(move_cmd) = serde_json::from_str::<MoveCommand>(&text) {
                            eprintln!("Parsed movement command: {:?}", move_cmd);
                            if move_cmd.action == "move" {
                                x += move_cmd.dx;
                                y += move_cmd.dy;
                                // Ensure the position stays within bounds
                                x = x.max(1).min(10);
                                y = y.max(1).min(10);

                                // Update position in Redis
                                let _: () = conn.hset_multiple(&key, &[("x", x), ("y", y)]).await.map_err(|e| {
                                    eprintln!("Failed to update position in Redis: {}", e);
                                    actix_web::error::ErrorInternalServerError(e)
                                })?;

                                // Publish the updated position to all clients
                                let position_update = PositionUpdate {
                                    action: "update_position".to_string(),
                                    username: username.clone(),
                                    x,
                                    y,
                                };
                                let message = serde_json::to_string(&position_update).unwrap();
                                let _: () = conn.publish::<_, _, ()>("game_channel", message).await.map_err(|e| {
                                    eprintln!("Failed to publish to Redis: {}", e);
                                    actix_web::error::ErrorInternalServerError(e)
                                })?;
                            }
                        } else {
                            eprintln!("Failed to parse movement command");
                        }
                    },
                    Ok(Message::Close(_)) => {
                        // Optionally, remove player from Redis or mark as offline
                        let _ = session.close(None).await;
                        break;
                    },
                    Err(e) => {
                        eprintln!("WebSocket error: {:?}", e);
                        break;
                    },
                    _ => {}
                }
            },

            // Handle messages from Redis
            Some(msg) = redis_stream.next() => {
                let payload: String = match msg.get_payload() {
                    Ok(payload) => payload,
                    Err(e) => {
                        eprintln!("Failed to get payload from Redis message: {}", e);
                        continue;
                    }
                };

                // Send the message to the client
                if session.text(payload).await.is_err() {
                    // Client disconnected
                    break;
                }
            }

            else => break,
        }
    }

    Ok(())
}
