use leptos::*;
use crate::presentation::home_page::HomePage;
use crate::infrastructure::api_client::ApiClient;
use crate::application::auth_service::AuthService;

mod domain;
mod application;
mod infrastructure;
mod presentation;

fn main() {
    leptos::mount_to_body(|| {
        let api_client = ApiClient::new("http://innershelter.org:8080".to_string());
        let auth_service = AuthService::new(api_client);
        view! { <HomePage auth_service=auth_service.clone() /> }
    });
}
