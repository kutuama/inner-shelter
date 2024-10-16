use leptos::*;
use leptos::html::Div;
use web_sys::{console, KeyboardEvent};
use crate::application::websocket_service::WebSocketService;
use serde_json::Value;

// Define the grid size at the module level
const GRID_SIZE: usize = 10;

#[component]
pub fn GamePage(websocket_service: WebSocketService) -> impl IntoView {
    // Create signals for the player's position
    let player_x = create_rw_signal(1usize);
    let player_y = create_rw_signal(1usize);

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
        let (dx, dy) = match key.as_str() {
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
    let ws_service_clone2 = websocket_service.clone();
    let player_x_clone = player_x.clone();
    let player_y_clone = player_y.clone();

    create_effect(move |_| {
        ws_service_clone2.set_on_message(move |message| {
            // Parse the message and update the game state
            if let Ok(value) = serde_json::from_str::<Value>(&message) {
                if let Some(action) = value.get("action").and_then(|v| v.as_str()) {
                    match action {
                        "update_position" => {
                            if let Some(x) = value.get("x").and_then(|v| v.as_u64()) {
                                player_x_clone.set(x as usize);
                            }
                            if let Some(y) = value.get("y").and_then(|v| v.as_u64()) {
                                player_y_clone.set(y as usize);
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
    });

    // Generate the grid
    let grid = move || {
        let mut rows = vec![];

        for row in 1..=GRID_SIZE {
            let mut cells = vec![];
            for col in 1..=GRID_SIZE {
                // Wrap signal accesses in closures
                let is_player = move || player_x.get() == col && player_y.get() == row;
                let cell_class = move || if is_player() { "cell player" } else { "cell" };

                cells.push(view! {
                    // Use closures in the view to ensure reactivity
                    <div class=cell_class></div>
                });
            }
            rows.push(view! {
                <div class="row">{cells}</div>
            });
        }

        view! { <div class="grid">{rows}</div> }
    };

    view! {
        <div node_ref=game_container_ref on:keydown=on_keydown tabindex="0" class="game-container">
            <h2>"Game Page"</h2>
            // Wrap signal accesses in a closure
            <p>{move || format!("Player position: ({}, {})", player_x.get(), player_y.get())}</p>
            {grid()}
        </div>
    }
}
