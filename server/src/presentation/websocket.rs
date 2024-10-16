use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web::web::Payload;
use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use crate::config::Config;
use crate::infrastructure::authentication;
use redis::AsyncCommands;

pub async fn ws_handler(
    req: HttpRequest,
    stream: Payload,
    config: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    // Extract the access_token from cookies
    let token = extract_access_token(&req);

    // Validate token
    let _username = if let Some(token) = token {
        match authentication::validate_jwt(&token, config.jwt_secret.as_bytes()) {
            Ok(claims) => claims.sub,
            Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
        }
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let (response, session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    actix_rt::spawn(async move {
        if let Err(e) = ws_session(session, &mut msg_stream, config.get_ref().clone()).await {
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
    config: Config,
) -> Result<(), Error> {
    // Connect to Redis
    let client = redis::Client::open(config.redis_url.clone()).map_err(|e| {
        eprintln!("Failed to create Redis client: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // Connection for publishing
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

    loop {
        tokio::select! {
            // Handle messages from the client
            Some(msg) = msg_stream.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        let text = text.to_string();
                        // Publish the message to Redis
                        if let Err(e) = conn.publish::<_, _, i32>("game_channel", text).await {
                            eprintln!("Failed to publish to Redis: {}", e);
                            break;
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
