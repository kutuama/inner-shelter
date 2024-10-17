use actix_web::{HttpRequest, HttpResponse, Error, web};
use actix_ws::{Message, Session};
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

    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    let game_state = game_state.get_ref().clone();

    // Spawn a task to handle the websocket connection
    actix_rt::spawn(async move {
        // Add player to the game state
        {
            let mut state = game_state.lock().unwrap();
            state.add_player(username.clone());
        }

        // Handle incoming messages
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    // Convert ByteString to String
                    let text_str = text.to_string();

                    // Process the message
                    handle_message(&username, text_str, &game_state, &mut session).await;
                }
                Message::Close(reason) => {
                    session.close(reason).await.unwrap();
                    break;
                }
                _ => (),
            }
        }

        // Remove player from the game state when the connection closes
        {
            let mut state = game_state.lock().unwrap();
            state.remove_player(&username);
        }
    });

    Ok(response)
}

async fn handle_message(
    username: &str,
    msg: String,
    game_state: &Arc<Mutex<GameState>>,
    session: &mut Session,
) {
    // Process input and update game state
    let input: Value = serde_json::from_str(&msg).unwrap_or_default();

    {
        let mut state = game_state.lock().unwrap();
        state.process_input(username, input);
    }

    // Retrieve updated positions
    let positions_json = {
        let mut state = game_state.lock().unwrap();
        let positions = state.get_positions();
        serde_json::to_string(&positions).unwrap()
    };

    // Send updated positions back to the client
    session.text(positions_json).await.unwrap();
}
