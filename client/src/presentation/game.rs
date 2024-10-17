use leptos::*;
use leptos::html::Div;
use web_sys::{console, KeyboardEvent};
use crate::application::websocket_service::WebSocketService;
use std::collections::HashMap;

const GRID_SIZE: i32 = 10;

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

    // Set the on_message handler
    websocket_service.set_on_message(move |message| {
        console::log_1(&format!("Received message from server: {}", message).into());
        // Parse the message and prepare updates
        if let Ok(positions) = serde_json::from_str::<Vec<(String, i32, i32)>>(&message) {
            let mut new_other_players = HashMap::new();
            let mut new_player_x = None;
            let mut new_player_y = None;

            for (player_username, x, y) in positions {
                if player_username == username_clone {
                    new_player_x = Some(x);
                    new_player_y = Some(y);
                } else {
                    new_other_players.insert(player_username, (x, y));
                }
            }

            // Update player position first
            if let Some(x) = new_player_x {
                player_x_clone.set(x);
            }
            if let Some(y) = new_player_y {
                player_y_clone.set(y);
            }

            // Then update other players
            other_players_clone.set(new_other_players);
        } else {
            console::error_1(&"Failed to parse positions from server".into());
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
                        let is_other_player = other_players.iter().any(|(_, &(x, y))| x == col && y == row);

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
