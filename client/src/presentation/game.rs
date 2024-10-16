use leptos::*;
use leptos::html::Div;
use web_sys::{console, KeyboardEvent};
use crate::application::websocket_service::WebSocketService;
use serde_json::Value;
use std::collections::HashMap;
use serde::Deserialize;

// Define the grid size at the module level
const GRID_SIZE: i32 = 10;

#[derive(Clone, Debug, Deserialize)]
struct PositionUpdate {
    action: String,
    username: String,
    x: i32,
    y: i32,
}

#[component]
pub fn GamePage(websocket_service: WebSocketService, username: String) -> impl IntoView {
    // Create signals for the player's position
    let player_x = create_rw_signal(1i32);
    let player_y = create_rw_signal(1i32);

    // Create a signal to track other players' positions
    let other_players = create_rw_signal(HashMap::<String, (i32, i32)>::new());

    // Reference to the game container for focusing
    let game_container_ref = create_node_ref::<Div>();

    // Focus the game container to receive keyboard events
    create_effect(move |_| {
        if let Some(el) = game_container_ref.get() {
            el.focus().unwrap_or_else(|err| {
                console::error_1(&format!("Failed to focus game container: {:?}", err).into());
            });
        }
    });

    // Handle keydown events to move the player
    let ws_service_clone = websocket_service.clone();
    let player_x_clone = player_x.clone();
    let player_y_clone = player_y.clone();
    let on_keydown = move |e: KeyboardEvent| {
        let key = e.key();
        console::log_1(&format!("Key pressed: {}", key).into());
        let (dx, dy): (i32, i32) = match key.as_str() {
            "ArrowUp" => (0, -1),
            "ArrowDown" => (0, 1),
            "ArrowLeft" => (-1, 0),
            "ArrowRight" => (1, 0),
            _ => (0, 0),
        };

        if dx != 0 || dy != 0 {
            // Update the player's position immediately
            let new_x = (player_x_clone.get() + dx).max(1).min(GRID_SIZE);
            let new_y = (player_y_clone.get() + dy).max(1).min(GRID_SIZE);
            player_x_clone.set(new_x);
            player_y_clone.set(new_y);

            // Send the movement command to the server
            let message = serde_json::json!({
                "action": "move",
                "dx": dx,
                "dy": dy,
            });
            if let Err(err) = ws_service_clone.send(&message.to_string()) {
                console::error_1(&format!("Failed to send message: {}", err).into());
            }
        }
    };

    // Handle incoming messages from the server
    let player_x_clone = player_x.clone();
    let player_y_clone = player_y.clone();
    let other_players_clone = other_players.clone();
    let username_clone = username.clone();

    // Set the on_message handler only once
    websocket_service.set_on_message(move |message| {
        console::log_1(&format!("Received message from server: {}", message).into());
        // Parse the message and update the game state
        if let Ok(value) = serde_json::from_str::<Value>(&message) {
            if let Some(action) = value.get("action").and_then(|v| v.as_str()) {
                match action {
                    "update_position" => {
                        if let Ok(pos_update) = serde_json::from_value::<PositionUpdate>(value.clone()) {
                            // Check if the update is for the current player or others
                            if pos_update.username != username_clone {
                                // Update other player's position
                                other_players_clone.update(|players| {
                                    players.insert(pos_update.username.clone(), (pos_update.x, pos_update.y));
                                });
                            } else {
                                // The server might send corrections; update position if necessary
                                player_x_clone.set(pos_update.x);
                                player_y_clone.set(pos_update.y);
                            }
                        } else {
                            console::error_1(&"Failed to deserialize PositionUpdate".into());
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    view! {
        <div node_ref=game_container_ref on:keydown=on_keydown tabindex="0" class="game-container">
            <h2>"Game Page"</h2>
            <p>{move || format!("Player position: ({}, {})", player_x.get(), player_y.get())}</p>
            {move || {
                let player_x = player_x.get();
                let player_y = player_y.get();
                let other_players = other_players.get();

                let mut rows = vec![];

                for row in 1..=GRID_SIZE {
                    let mut cells = vec![];
                    for col in 1..=GRID_SIZE {
                        let is_player = player_x == col && player_y == row;
                        let is_other_player = other_players.values().any(|&(x, y)| x == col && y == row);

                        let cell_class = if is_player {
                            "cell player"
                        } else if is_other_player {
                            "cell other-player"
                        } else {
                            "cell"
                        };

                        cells.push(view! {
                            <div class=cell_class></div>
                        });
                    }
                    rows.push(view! {
                        <div class="row">{cells}</div>
                    });
                }

                view! { <div class="grid">{rows}</div> }
            }}
        </div>
    }
}
