use actix_web::{HttpRequest, HttpResponse, Error, web};
use actix_ws::{Message, Session, MessageStream};
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::game::GameState;
use crate::infrastructure::authentication::validate_token;

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    game_state: web::Data<Arc<Mutex<GameState>>>,
) -> Result<HttpResponse, Error> {
    // Extract access_token cookie
    let token = match req.cookie("access_token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    // Validate the token
    let username = match validate_token(&token) {
        Ok(username) => username,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;

    let game_state = game_state.get_ref().clone();

    // Spawn a task to handle the websocket connection
    actix_rt::spawn(async move {
        // Add player to the game state with their session
        {
            let mut state = game_state.lock().unwrap();
            state.add_player(username.clone(), session.clone());
        }

        // Handle incoming messages
        if let Err(e) = ws_session(username.clone(), game_state.clone(), session, msg_stream).await {
            log::error!("WebSocket session error: {:?}", e);
        }

        // Remove player from the game state when the connection closes
        {
            let mut state = game_state.lock().unwrap();
            state.remove_player(&username);
        }
    });

    Ok(response)
}

async fn ws_session(
    username: String,
    game_state: Arc<Mutex<GameState>>,
    session: Session,
    mut msg_stream: MessageStream,
) -> Result<(), Error> {
    // Handle incoming messages
    while let Some(Ok(msg)) = msg_stream.next().await {
        match msg {
            Message::Text(text) => {
                // Convert ByteString to String
                let text_str = text.to_string();

                // Process the message
                handle_message(&username, text_str, &game_state).await;
            }
            Message::Close(reason) => {
                session.close(reason).await.unwrap();
                break;
            }
            _ => (),
        }
    }

    Ok(())
}

async fn handle_message(
    username: &str,
    msg: String,
    game_state: &Arc<Mutex<GameState>>,
) {
    // Process input and update game state
    let input: Value = serde_json::from_str(&msg).unwrap_or_default();

    {
        let mut state = game_state.lock().unwrap();
        state.process_input(username, input);

        // Retrieve updated positions
        let positions = state.get_positions();
        let positions_json = serde_json::to_string(&positions).unwrap();

        // Broadcast updated positions to all connected clients
        for (client_username, client_session) in &mut state.sessions {
            // Send the positions JSON to each client
            if let Err(e) = client_session.text(positions_json.clone()).await {
                log::error!("Error sending message to {}: {:?}", client_username, e);
            }
        }
    }
}
