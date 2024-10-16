use leptos::*;
use leptos::html::Div;
use web_sys::{console, KeyboardEvent};

#[component]
pub fn GamePage() -> impl IntoView {
    // Define the grid size
    const GRID_SIZE: usize = 10;

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
    let on_keydown = move |e: KeyboardEvent| {
        let key = e.key();
        match key.as_str() {
            "ArrowUp" => {
                player_y.update(|y| if *y > 1 { *y -= 1 });
            }
            "ArrowDown" => {
                player_y.update(|y| if *y < GRID_SIZE { *y += 1 });
            }
            "ArrowLeft" => {
                player_x.update(|x| if *x > 1 { *x -= 1 });
            }
            "ArrowRight" => {
                player_x.update(|x| if *x < GRID_SIZE { *x += 1 });
            }
            _ => {}
        }
    };

    // Generate the grid
    let grid = move || {
        let mut rows = vec![];

        for row in 1..=GRID_SIZE {
            let mut cells = vec![];
            for col in 1..=GRID_SIZE {
                let is_player = player_x.get() == col && player_y.get() == row;
                let cell_class = if is_player { "cell player" } else { "cell" };

                cells.push(view! {
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
            <p>{format!("Player position: ({}, {})", player_x.get(), player_y.get())}</p>
            {grid()}
        </div>
    }
}
