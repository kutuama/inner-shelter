use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent, Event, ErrorEvent, BinaryType};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct WebSocketService {
    ws: Rc<WebSocket>,
}

impl WebSocketService {
    pub fn connect(url: &str) -> Result<Self, String> {
        let ws = WebSocket::new(url).map_err(|err| {
            format!("Failed to create WebSocket: {:?}", err)
        })?;

        ws.set_binary_type(BinaryType::Arraybuffer);

        Ok(Self { ws: Rc::new(ws) })
    }

    pub fn send(&self, message: &str) -> Result<(), String> {
        self.ws.send_with_str(message).map_err(|err| {
            format!("Failed to send message: {:?}", err)
        })
    }

    pub fn set_on_message<F>(&self, callback: F)
    where
        F: 'static + Fn(String),
    {
        let callback = Rc::new(RefCell::new(callback));
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let message = text.as_string().unwrap_or_default();
                (callback.borrow())(message);
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        self.ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }

    pub fn set_on_error<F>(&self, callback: F)
    where
        F: 'static + Fn(),
    {
        let callback = Rc::new(RefCell::new(callback));
        let onerror_callback = Closure::wrap(Box::new(move |_e: ErrorEvent| {
            (callback.borrow())();
        }) as Box<dyn FnMut(ErrorEvent)>);

        self.ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }

    pub fn set_on_open<F>(&self, callback: F)
    where
        F: 'static + Fn(),
    {
        let callback = Rc::new(RefCell::new(callback));
        let onopen_callback = Closure::wrap(Box::new(move |_e: Event| {
            (callback.borrow())();
        }) as Box<dyn FnMut(Event)>);

        self.ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    pub fn set_on_close<F>(&self, callback: F)
    where
        F: 'static + Fn(),
    {
        let callback = Rc::new(RefCell::new(callback));
        let onclose_callback = Closure::wrap(Box::new(move |_e: Event| {
            (callback.borrow())();
        }) as Box<dyn FnMut(Event)>);

        self.ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
    }
}
