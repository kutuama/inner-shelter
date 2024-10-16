use leptos::*;
use crate::application::auth_service::AuthService;
use crate::domain::models::User;

#[component]
pub fn LoginPage(auth_service: AuthService, user_signal: RwSignal<Option<User>>) -> impl IntoView {
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());

    let on_login = move |_| {
        let username = username.get().clone();
        let password = password.get().clone();
        let auth_service = auth_service.clone();
        let user_signal = user_signal.clone();

        spawn_local(async move {
            auth_service.login(username, password, user_signal).await;
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
            <button on:click=on_login>"Login"</button>
        </div>
    }
}
