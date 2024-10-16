use leptos::*;
use crate::application::auth_service::AuthService;

#[component]
pub fn RegisterPage(auth_service: AuthService) -> impl IntoView {
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());

    let on_register = move |_| {
        let username = username.get().clone();
        let password = password.get().clone();
        let auth_service = auth_service.clone();

        spawn_local(async move {
            auth_service.register(username, password).await;
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
            <button on:click=on_register>"Create Account"</button>
        </div>
    }
}
