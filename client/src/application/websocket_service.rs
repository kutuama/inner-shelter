use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent, Event, ErrorEvent, BinaryType};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct WebSocketService {
    ws: Rc<WebSocket>,
    is_open: Rc<RefCell<bool>>,
    message_queue: Rc<RefCell<Vec<String>>>,
    on_open: Rc<RefCell<Option<Box<dyn Fn()>>>>,
}

impl WebSocketService {
    pub fn connect(url: &str) -> Result<Self, String> {
        let ws = WebSocket::new(url).map_err(|err| {
            format!("Failed to create WebSocket: {:?}", err)
        })?;

        ws.set_binary_type(BinaryType::Arraybuffer);

        let service = Self {
            ws: Rc::new(ws),
            is_open: Rc::new(RefCell::new(false)),
            message_queue: Rc::new(RefCell::new(Vec::new())),
            on_open: Rc::new(RefCell::new(None)),
        };

        let is_open_clone = service.is_open.clone();
        let message_queue_clone = service.message_queue.clone();
        let on_open_clone = service.on_open.clone();
        let ws_clone = service.ws.clone();

        // Set onopen callback to update the is_open flag and send queued messages
        let onopen_callback = Closure::wrap(Box::new(move |_e: Event| {
            *is_open_clone.borrow_mut() = true;

            // Send all queued messages
            let mut queue = message_queue_clone.borrow_mut();
            for msg in queue.drain(..) {
                if let Err(err) = ws_clone.send_with_str(&msg) {
                    web_sys::console::error_1(&format!("Failed to send queued message: {:?}", err).into());
                }
            }

            // Call user-provided on_open callback if set
            if let Some(ref callback) = *on_open_clone.borrow() {
                callback();
            }
        }) as Box<dyn FnMut(Event)>);

        service.ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        Ok(service)
    }

    pub fn send(&self, message: &str) -> Result<(), String> {
        if *self.is_open.borrow() {
            self.ws.send_with_str(message).map_err(|err| {
                format!("Failed to send message: {:?}", err)
            })
        } else {
            // Queue the message if the connection is not open yet
            self.message_queue.borrow_mut().push(message.to_string());
            Ok(())
        }
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

    pub fn set_on_open<F>(&self, callback: F)
    where
        F: 'static + Fn(),
    {
        *self.on_open.borrow_mut() = Some(Box::new(callback));
    }
}
