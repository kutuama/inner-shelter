use leptos::*;
use leptos::html::Div;
use crate::presentation::login::LoginPage;
use crate::presentation::register::RegisterPage;
use crate::presentation::game::GamePage; // Add this import
use crate::application::auth_service::AuthService;
use crate::application::websocket_service::WebSocketService;
use crate::domain::models::User;
use web_sys::console;

#[component]
pub fn HomePage(auth_service: AuthService) -> impl IntoView {
    let active_tab = create_rw_signal("login".to_string());
    let user = create_rw_signal(None::<User>);

    let websocket_service = create_rw_signal(None::<WebSocketService>);

    let container_ref = create_node_ref::<Div>();

    // Focus the container to receive keyboard events
    create_effect(move |_| {
        if let Some(el) = container_ref.get() {
            el.focus().unwrap_or_else(|err| {
                console::error_1(&format!("Failed to focus element: {:?}", err).into());
            });
        }
    });

    // Establish WebSocket connection when the user logs in
    create_effect(move |_| {
        if let Some(_user) = user.get() {
            // User is logged in, connect to WebSocket
            let ws_url = "ws://innershelter.org:8080/ws";
            match WebSocketService::connect(ws_url) {
                Ok(ws_service) => {
                    let ws_clone = ws_service.clone();
                    ws_service.set_on_open(move || {
                        console::log_1(&"WebSocket connection opened".into());
                    });

                    ws_service.set_on_message(move |message| {
                        console::log_1(&format!("Received message: {}", message).into());
                        // Handle incoming messages (e.g., update game state)
                    });

                    ws_service.set_on_error(move || {
                        console::error_1(&"WebSocket error occurred".into());
                    });

                    ws_service.set_on_close(move || {
                        console::log_1(&"WebSocket connection closed".into());
                    });

                    websocket_service.set(Some(ws_clone));
                }
                Err(err) => {
                    console::error_1(&format!("WebSocket connection failed: {}", err).into());
                }
            }
        }
    });

    // Remove the global keydown handler from HomePage
    // Keyboard events will be handled in GamePage

    let select_login = move |_| active_tab.set("login".to_string());
    let select_create_account = move |_| active_tab.set("create".to_string());

    let auth_service_clone = auth_service.clone();
    let user_signal = user.clone();
    let active_tab_signal = active_tab.clone();

    view! {
        <div node_ref=container_ref tabindex="0">
            <h1>"Welcome to Inner Shelter"</h1>
            {if user.get().is_some() {
                if let Some(ws_service) = websocket_service.get() {
                    // User is logged in, display the game page
                    view! {
                        <div>
                            <p>{format!("Logged in as {}", user.get().unwrap().username)}</p>
                            <GamePage websocket_service=ws_service.clone() />
                        </div>
                    }.into_view()
                } else {
                    // WebSocketService not available yet
                    view! {
                        <p>"Connecting to game server..."</p>
                    }.into_view()
                }
            } else {
                // User is not logged in, display login/create account forms
                view! {
                    <div>
                        <div class="tab-menu">
                            <button on:click=select_login>"Login"</button>
                            <button on:click=select_create_account>"Create Account"</button>
                        </div>

                        <div class="tab-content">
                            {match active_tab_signal.get().as_str() {
                                "login" => view! { <LoginPage auth_service=auth_service_clone.clone() user_signal=user_signal.clone() /> }.into_view(),
                                "create" => view! { <RegisterPage auth_service=auth_service_clone.clone() /> }.into_view(),
                                _ => view! { <LoginPage auth_service=auth_service_clone.clone() user_signal=user_signal.clone() /> }.into_view(),
                            }}
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
