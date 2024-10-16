use leptos::*;
use crate::presentation::login::LoginPage;
use crate::presentation::register::RegisterPage;
use crate::application::auth_service::AuthService;
use crate::domain::models::User;
use web_sys::console;

#[component]
pub fn HomePage(auth_service: AuthService) -> impl IntoView {
    let active_tab = create_rw_signal("login".to_string());
    let user = create_rw_signal(None::<User>);

    // Function to log user data to the console
    let log_user = move || {
        if let Some(user) = user.get() {
            let user_info = format!("Username: {}, Token: {:?}", user.username, user.token);
            console::log_1(&user_info.into());
        }
    };

    // Call log_user whenever the user signal is updated
    create_effect(move |_| {
        log_user();
    });

    let select_login = move |_| active_tab.set("login".to_string());
    let select_create_account = move |_| active_tab.set("create".to_string());

    view! {
        <div>
            <h1>"Welcome to Inner Shelter"</h1>
            <div class="tab-menu">
                <button on:click=select_login>"Login"</button>
                <button on:click=select_create_account>"Create Account"</button>
            </div>

            <div class="tab-content">
                {move || match active_tab.get().as_str() {
                    "login" => view! { <LoginPage auth_service=auth_service.clone() user_signal=user.clone() /> }.into_view(),
                    "create" => view! { <RegisterPage auth_service=auth_service.clone() /> }.into_view(),
                    _ => view! { <LoginPage auth_service=auth_service.clone() user_signal=user.clone() /> }.into_view(),
                }}
            </div>
        </div>
    }
}
