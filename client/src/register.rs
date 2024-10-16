use leptos::*;
use serde::Serialize;
use gloo_net::http::Request;

#[derive(Serialize)]
struct RegisterData {
    username: String,
    password: String,
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());

    let create_account = move |_| {
        let username_value = username.get().clone();
        let password_value = password.get().clone();

        spawn_local(async move {
            let account_data = RegisterData {
                username: username_value,
                password: password_value,
            };

            match Request::post("http://innershelter.org:8080/register")
                .json(&account_data)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if response.ok() {
                        web_sys::console::log_1(&"Account created successfully".into());
                    } else {
                        web_sys::console::error_1(
                            &format!("Failed to create account: {:?}", response.status()).into(),
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
            <h2>"Create Account"</h2>
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
            <button on:click=create_account>"Create Account"</button>
        </div>
    }
}
