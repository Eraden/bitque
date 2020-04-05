use seed::Method;
use wasm_bindgen::prelude::*;

use jirs_data::{UpdateIssuePayload, WsMsg};

use crate::shared::host_client;
use crate::Msg;
use seed::prelude::Closure;
use std::sync::Once;
use wasm_bindgen::JsCast;

pub async fn fetch_current_project(host_url: String) -> Result<Msg, Msg> {
    match host_client(host_url, "/project") {
        Ok(client) => client.fetch_string(Msg::CurrentProjectResult).await,
        Err(e) => Err(Msg::InternalFailure(e)),
    }
}

pub async fn fetch_current_user(host_url: String) -> Result<Msg, Msg> {
    match host_client(host_url, "/currentUser") {
        Ok(client) => client.fetch_string(Msg::CurrentUserResult).await,
        Err(e) => Err(Msg::InternalFailure(e)),
    }
}

pub async fn update_issue(
    host_url: String,
    id: i32,
    payload: UpdateIssuePayload,
) -> Result<Msg, Msg> {
    match host_client(host_url, format!("/issues/{id}", id = id).as_str()) {
        Ok(client) => {
            client
                .method(Method::Put)
                .header("Content-Type", "application/json")
                .body_json(&payload)
                .fetch_string(Msg::IssueUpdateResult)
                .await
        }
        Err(e) => return Ok(Msg::InternalFailure(e)),
    }
}

pub async fn delete_issue(host_url: String, id: i32) -> Result<Msg, Msg> {
    match host_client(host_url, format!("/issues/{id}", id = id).as_str()) {
        Ok(client) => {
            client
                .method(Method::Delete)
                .header("Content-Type", "application/json")
                .fetch_string(Msg::IssueDeleteResult)
                .await
        }
        Err(e) => return Ok(Msg::InternalFailure(e)),
    }
}

pub struct WebSocket {
    ws: web_sys::WebSocket,
    queue: Vec<WsMsg>,
}

impl Default for WebSocket {
    fn default() -> WebSocket {
        use js_sys::*;
        use seed::prelude::*;
        use web_sys::*;

        let native = web_sys::WebSocket::new("ws://localhost:5000/ws/").unwrap();
        native.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let onmessage_callback =
            Closure::wrap(Box::new(move |e: MessageEvent| {}) as Box<dyn FnMut(MessageEvent)>);
        native.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        //     seed::log!("error event: {:?}", e);
        // }) as Box<dyn FnMut(ErrorEvent)>);
        // native.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        // onerror_callback.forget();

        let cloned_ws = native.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            seed::log!("socket opened");
            match cloned_ws.send_with_str("ping") {
                Ok(_) => seed::log!("message successfully sent"),
                Err(err) => seed::log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);
        native.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        Self {
            ws: native,
            queue: vec![],
        }
    }
}

impl WebSocket {
    pub fn send_with_u8_array(&self, buffer: &[u8]) {
        use seed::*;
        self.ws
            .send_with_u8_array(buffer)
            .unwrap_or_else(|e| error!(e));
    }

    pub fn send(&mut self) {
        use bincode;
        for msg in self.queue.iter() {
            let encoded: Vec<u8> = bincode::serialize(msg).unwrap();
            self.send_with_u8_array(encoded.as_slice());
        }
        self.queue.clear();
    }
}

static INIT_WS: Once = Once::new();
static mut WS: Option<WebSocket> = None;

pub fn ws() -> &'static mut WebSocket {
    unsafe {
        INIT_WS.call_once(|| WS = Some(WebSocket::default()));

        let ws_ping = Box::new(|| match WS.as_mut().map(|ws| ws.ws.ready_state()) {
            Some(0) => {}
            Some(1) => {
                ws_send(WsMsg::Ping);
                WS.as_mut().unwrap().send();
            }
            _ => {
                WS = Some(WebSocket::default());
            }
        }) as Box<dyn Fn()>;
        seed::set_interval(ws_ping, 10_000);

        WS.as_mut().unwrap()
    }
}

// pub fn ws_received() {}
//
pub fn ws_send(msg: jirs_data::WsMsg) {
    ws().queue.push(msg);
}
