use web_sys::{BinaryType, MessageEvent, WebSocket};
use js_sys::{ArrayBuffer, Uint8Array};

use std::cell::RefCell;
use std::sync::Arc;

use std::collections::VecDeque;

use std::task::{Poll, Waker};

use futures_util::future::poll_fn;
use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::console;
use url::Url;

use crate::websocket::{WebSocketError, WebSocketMessage};

impl From<JsValue> for WebSocketError {
    fn from(js_value: JsValue) -> Self {
        WebSocketError::NativeError(js_value.as_string().unwrap())
    }
}

enum SocketState {
    Init,
    Open,
    Error(String),
    Closed,
}

struct AsyncWebSocketInner {
    ws: WebSocket,
    state: SocketState,
    waker: Option<Waker>,
    buffer: VecDeque<MessageEvent>,
}

pub struct AsyncWebSocket {
    inner: Arc<RefCell<AsyncWebSocketInner>>,
}

impl Clone for AsyncWebSocket {
    fn clone(&self) -> Self {
        AsyncWebSocket {
            inner: self.inner.clone(),
        }
    }
}

// TODO: look at <https://github.com/najamelan/ws_stream_wasm>
// for examples of handling all of the status codes!

// #[async_trait]
impl AsyncWebSocket {
    // TODO: this needs to take a URL... maybe only support SSL?
    pub async fn connect(url: Url) -> Result<Self, WebSocketError> {
        let ws = WebSocket::new(url.as_str())?;
        ws.set_binary_type(BinaryType::Arraybuffer);
        let async_ws: AsyncWebSocket = {
            let ws = ws.clone();
            let state = SocketState::Init;
            let waker = None;
            let buffer = VecDeque::new();

            let inner = Arc::new(RefCell::new(AsyncWebSocketInner {
                ws,
                state,
                waker,
                buffer,
            }));
            AsyncWebSocket { inner }
        };

        let onopen_callback = {
            let async_ws = async_ws.clone();
            Closure::wrap(Box::new(move |_| {
                console::log_1(&JsValue::from_str("Websocket onopen callback!")); // TODO: debug logging
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.state = SocketState::Open;
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }) as Box<dyn FnMut(JsValue)>)
        };
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onclose_callback = {
            let async_ws = async_ws.clone();
            Closure::wrap(Box::new(move |_| {
                console::log_1(&JsValue::from_str("Websocket onclose callback!")); // TODO: debug logging
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.state = SocketState::Closed;
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }) as Box<dyn FnMut(JsValue)>)
        };
        ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        let onerror_callback = {
            let async_ws = async_ws.clone();
            Closure::wrap(Box::new(move |err: JsValue| {
                console::log_1(&JsValue::from_str("Websocket onerror callback!")); // TODO: debug logging
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.state = SocketState::Error(err.as_string().unwrap());
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }) as Box<dyn FnMut(JsValue)>)
        };
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let onmessage_callback = {
            let async_ws = async_ws.clone();
            Closure::wrap(Box::new(move |ev: MessageEvent| {
                console::log_1(&JsValue::from_str("Websocket onmessage callback!")); // TODO: debug logging
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.buffer.push_back(ev);
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }) as Box<dyn FnMut(MessageEvent)>)
        };
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        poll_fn({
            let async_ws = async_ws.clone();
            move |cx| {
                console::log_1(&JsValue::from_str("Polling"));
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                match &inner.state {
                    SocketState::Init => {
                        inner.waker.replace(cx.waker().clone());
                        Poll::Pending
                    }
                    SocketState::Open => Poll::Ready(Ok(())),
                    SocketState::Error(val) => {
                        Poll::Ready(Err(WebSocketError::StateError(val.clone())))
                    }
                    SocketState::Closed => Poll::Ready(Err(WebSocketError::StateClosed)),
                }
            }
        })
        .await?;

        Ok(async_ws)
    }

    // TODO: should this be a result? what error would there be
    pub fn send(&self, msg: &str) -> Result<(), WebSocketError> {
        console::log_1(&JsValue::from_str("Send"));
        let inner: &mut AsyncWebSocketInner = &mut *self.inner.borrow_mut();
        inner.ws.send_with_str(msg)?;
        Ok(())
    }

    // <https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent>
    pub async fn receive(&self) -> Result<WebSocketMessage, WebSocketError> {
        console::log_1(&JsValue::from_str("Recieve (blocking)"));
        let message_event = poll_fn({
            move |cx| {
                console::log_1(&JsValue::from_str("Polling"));
                let inner: &mut AsyncWebSocketInner = &mut *self.inner.borrow_mut();
                match &inner.state {
                    SocketState::Init => Poll::Ready(Err(WebSocketError::StateInit)),
                    SocketState::Open => {
                        if let Some(ev) = inner.buffer.pop_front() {
                            Poll::Ready(Ok(ev))
                        } else {
                            inner.waker.replace(cx.waker().clone());
                            Poll::Pending
                        }
                    }
                    SocketState::Error(val) => {
                        Poll::Ready(Err(WebSocketError::StateError(val.clone())))
                    }
                    SocketState::Closed => Poll::Ready(Err(WebSocketError::StateClosed)),
                }
            }
        })
        .await?;

        // info!("Websocket message: {:?}", message_event);
        let data: JsValue = message_event.data();
        console::log_1(&data);

        let message = match data.as_string() {
            Some(s) => WebSocketMessage::String(s),
            None => {
                let buf: &ArrayBuffer = data.as_ref().unchecked_ref(); // consider using JsCast::dyn_into for safety?
                let vec: Vec<u8> = Uint8Array::new(buf).to_vec();
                WebSocketMessage::Binary(vec)
            }
        };

        Ok(message)   
    }
}
