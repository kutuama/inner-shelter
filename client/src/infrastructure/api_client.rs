use shared::api::auth::{LoginData, RegisterData};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, RequestCredentials, Response};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn login(&self, login_data: LoginData) -> Result<String, String> {
        let body = serde_json::to_string(&login_data).map_err(|e| e.to_string())?;

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&JsValue::from_str(&body));
        opts.set_mode(RequestMode::Cors);
        opts.set_credentials(RequestCredentials::Include);

        let request = web_sys::Request::new_with_str_and_init(
            &format!("{}/login", self.base_url),
            &opts,
        )
        .map_err(|e| e.as_string().unwrap_or("Request creation failed".into()))?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|e| e.as_string().unwrap_or("Header setting failed".into()))?;

        let window = web_sys::window().ok_or("No global `window` exists")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| e.as_string().unwrap_or("Fetch failed".into()))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|_| "Failed to cast to Response".to_string())?;
        if resp.ok() {
            let text_js = JsFuture::from(resp.text().map_err(|_| "Failed to get text".to_string())?)
                .await
                .map_err(|_| "Failed to await text".to_string())?;
            let text = text_js.as_string().ok_or("Response text is not a string".to_string())?;
            
            // Assuming the server sends "Login successful, token: {token}"
            if let Some(token_start) = text.find("token: ") {
                let token = text[token_start + 7..].trim().to_string();
                Ok(token)
            } else {
                Err("Token not found in response".into())
            }
        } else {
            Err(format!("HTTP error: {}", resp.status()))
        }
    }

    pub async fn register(&self, register_data: RegisterData) -> Result<(), String> {
        let body = serde_json::to_string(&register_data).map_err(|e| e.to_string())?;

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&JsValue::from_str(&body));
        opts.set_mode(RequestMode::Cors);
        opts.set_credentials(RequestCredentials::Include);

        let request = web_sys::Request::new_with_str_and_init(
            &format!("{}/register", self.base_url),
            &opts,
        )
        .map_err(|e| e.as_string().unwrap_or("Request creation failed".into()))?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|e| e.as_string().unwrap_or("Header setting failed".into()))?;

        let window = web_sys::window().ok_or("No global `window` exists")?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| e.as_string().unwrap_or("Fetch failed".into()))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|_| "Failed to cast to Response".to_string())?;
        if resp.ok() {
            Ok(())
        } else {
            Err(format!("HTTP error: {}", resp.status()))
        }
    }
}
