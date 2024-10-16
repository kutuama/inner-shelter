use crate::domain::models::User;
use crate::infrastructure::api_client::ApiClient;
use shared::api::auth::{LoginData, RegisterData};
use leptos::{RwSignal, SignalSet};
use web_sys;

#[derive(Clone)]
pub struct AuthService {
    api_client: ApiClient,
}

impl AuthService {
    pub fn new(api_client: ApiClient) -> Self {
        Self { api_client }
    }

    pub async fn login(&self, username: String, password: String, user_signal: RwSignal<Option<User>>) {
        let login_data = LoginData { username: username.clone(), password };
        match self.api_client.login(login_data).await {
            Ok(token) => {
                let user = User {
                    username,
                    token: Some(token),
                };
                user_signal.set(Some(user));
            }
            Err(err) => {
                web_sys::console::error_1(&format!("Login failed: {}", err).into());
            }
        }
    }

    pub async fn register(&self, username: String, password: String) {
        let register_data = RegisterData { username, password };
        match self.api_client.register(register_data).await {
            Ok(_) => {
                web_sys::console::log_1(&"Registration successful".into());
            }
            Err(err) => {
                web_sys::console::error_1(&format!("Registration failed: {}", err).into());
            }
        }
    }
}
