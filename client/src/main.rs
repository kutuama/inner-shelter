use leptos::*;

mod register;
mod login;

use register::RegisterPage;
use login::LoginPage;

#[component]
fn HomePage() -> impl IntoView {
    // Signal to track the active tab
    let active_tab = create_rw_signal("login".to_string());

    let select_create_account = move |_| {
        active_tab.set("create".to_string());
    };

    let select_login = move |_| {
        active_tab.set("login".to_string());
    };

    view! {
        <div>
            <h1>"Welcome to Inner Shelter"</h1>
            <div class="tab-menu">
                <button on:click=select_login>"Login"</button>
                <button on:click=select_create_account>"Create Account"</button>
            </div>

            <div class="tab-content">
                {move || match active_tab.get().as_str() {
                    "login" => view! { <LoginPage /> }.into_view(),
                    "create" => view! { <RegisterPage /> }.into_view(),
                    _ => view! { <LoginPage /> }.into_view(), // Default case
                }}
            </div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| {
        view! { <HomePage /> }
    });
}
