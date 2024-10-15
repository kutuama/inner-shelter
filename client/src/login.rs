use leptos::*;
use serde::Serialize;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, RequestCredentials, Response};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast; // Add this import for JsCast

#[derive(Serialize)]
struct LoginData {
    username: String,
    password: String,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());

    let login = move |_| {
        let username_value = username.get().clone();
        let password_value = password.get().clone();

        spawn_local(async move {
            let login_data = LoginData {
                username: username_value,
                password: password_value,
            };

            // Serialize the login data to JSON
            let body = serde_json::to_string(&login_data).unwrap();

            // Prepare the request
            let opts = RequestInit::new();
            opts.set_method("POST"); // Use set_method instead of method
            opts.set_body(&JsValue::from_str(&body)); // Use set_body instead of body
            opts.set_mode(RequestMode::Cors); // Use set_mode instead of mode
            opts.set_credentials(RequestCredentials::Include); // Use set_credentials instead of credentials

            let request = web_sys::Request::new_with_str_and_init(
                "http://innershelter.org:8080/auth/login",
                &opts,
            )
            .unwrap();

            // Set the request headers
            request
                .headers()
                .set("Content-Type", "application/json")
                .unwrap();

            // Perform the fetch request
            let window = web_sys::window().unwrap();
            let response = JsFuture::from(window.fetch_with_request(&request)).await;

            match response {
                Ok(response_value) => {
                    let response: Response = response_value.dyn_into().unwrap(); // Ensure JsCast is in scope
                    if response.ok() {
                        web_sys::console::log_1(&"Login successful. Token is set in httpOnly cookie.".into());
                    } else if response.status() == 401 {
                        web_sys::console::error_1(&"Session expired. Please log in again.".into());
                    } else {
                        web_sys::console::error_1(
                            &format!("Login failed: {:?}", response.status()).into(),
                        );
                    }
                }
                Err(err) => {
                    web_sys::console::error_1(&format!("Request error: {:?}", err).into());
                }
            }
        });
    };

    view! {
        <div>
            <h2>"Login"</h2>
            <input
                type="text"
                placeholder="Username"
                on:input=move |e| username.set(event_target_value(&e))
            />
            <input
                type="password"
                placeholder="Password"
                on:input=move |e| password.set(event_target_value(&e))
            />
            <button on:click=login>"Login"</button>
        </div>
    }
}
