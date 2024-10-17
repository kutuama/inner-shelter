use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web::web::Payload;
use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use crate::config::Config;
use crate::infrastructure::authentication;
use crate::application::game_service::GameService;
use crate::game::entities::MoveCommand;
use std::sync::Arc;

pub async fn ws_handler(
    req: HttpRequest,
    stream: Payload,
    config: web::Data<Config>,
    game_service: web::Data<Arc<GameService>>,
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

    let game_service = game_service.get_ref().clone();

    actix_rt::spawn(async move {
        if let Err(e) = ws_session(session, &mut msg_stream, username, game_service).await {
            eprintln!("WebSocket error: {:?}", e);
        }
    });

    Ok(response)
}

fn extract_access_token(req: &HttpRequest) -> Option<String> {
    req.cookie("access_token").map(|cookie| cookie.value().to_string())
}

async fn ws_session(
    mut session: Session,
    msg_stream: &mut MessageStream,
    username: String,
    game_service: Arc<GameService>,
) -> Result<(), Error> {
    // Get initial position
    let position_update = match game_service.get_initial_position(&username).await {
        Ok(update) => update,
        Err(e) => {
            eprintln!("Error getting initial position: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };

    // Send the initial position to the current client
    let message = serde_json::to_string(&position_update).unwrap();

    if session.text(message).await.is_err() {
        // Client disconnected
        return Ok(());
    }

    // Subscribe to game updates
    let mut redis_stream = match game_service.subscribe().await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Error subscribing to game channel: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    };

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
                            if let Err(e) = game_service.handle_move(&username, move_cmd).await {
                                eprintln!("Error handling move: {:?}", e);
                            }
                        } else {
                            eprintln!("Failed to parse movement command");
                        }
                    },
                    Ok(Message::Close(_)) => {
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
            Some(msg_result) = redis_stream.next() => {
                match msg_result {
                    Ok(msg) => {
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
                    },
                    Err(e) => {
                        eprintln!("Error in Redis stream: {}", e);
                        break;
                    }
                }
            }

            else => break,
        }
    }

    Ok(())
}