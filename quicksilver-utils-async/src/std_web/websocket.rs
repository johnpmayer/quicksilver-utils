
use futures_util::future::poll_fn;
use std::sync::Arc;
use std::cell::RefCell;
use std::task::{Poll, Waker};
use std::collections::VecDeque;
use url::Url;

// annoying the re-naming of this package...
use std_web::web::{
    WebSocket, SocketBinaryType, IEventTarget, TypedArray,
    event::{
        SocketOpenEvent, SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, IMessageEvent, SocketMessageData,
    },
};

use crate::websocket::{WebSocketError, WebSocketMessage};

use log::{debug, trace};

// TODO: not DRY
enum SocketState {
    Init,
    Open,
    Error(String),
    Closed,
}

// TODO: not DRY
struct AsyncWebSocketInner {
    ws: WebSocket,
    state: SocketState,
    waker: Option<Waker>,
    buffer: VecDeque<SocketMessageEvent>,
}

pub struct AsyncWebSocket {
    inner: Arc<RefCell<AsyncWebSocketInner>>,
}

impl Clone for AsyncWebSocket {
    fn clone(&self) -> Self { AsyncWebSocket { inner: self.inner.clone() }}
}

impl AsyncWebSocket {
    pub async fn connect(url: &Url) -> Result<Self, WebSocketError> {
        let ws = WebSocket::new(url.as_str()).map_err(|_| WebSocketError::NativeError("Creation".to_string()))?;
        ws.set_binary_type(SocketBinaryType::ArrayBuffer);

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

        ws.add_event_listener({
            let async_ws = async_ws.clone();
            move |_: SocketOpenEvent| {
                trace!("Websocket onopen callback!");
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.state = SocketState::Open;
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }
        });

        ws.add_event_listener({
            let async_ws = async_ws.clone();
            move |_: SocketCloseEvent| {
                trace!("Websocket onclose callback!");
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.state = SocketState::Closed;
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }
        });

        ws.add_event_listener({
            let async_ws = async_ws.clone();
            move |error_event: SocketErrorEvent| {
                trace!("Websocket onerror callback!");
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                let error_message = format!("{:?}", error_event);
                inner.state = SocketState::Error(error_message);
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }
        });

        ws.add_event_listener({
            let async_ws = async_ws.clone();
            move |message_event: SocketMessageEvent| {
                let inner: &mut AsyncWebSocketInner = &mut *async_ws.inner.borrow_mut();
                inner.buffer.push_back(message_event);
                if let Some(waker) = inner.waker.take() {
                    waker.wake()
                }
            }
        });

        poll_fn({
            let async_ws = async_ws.clone();
            move |cx| {
                trace!("Polling");
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

    pub async fn send(&self, msg: &str) -> Result<(), WebSocketError> {
        trace!("Send");
        let inner: &AsyncWebSocketInner = &self.inner.borrow();
        inner.ws.send_text(msg).map_err(|_| WebSocketError::NativeError("Send".to_string()))?;
        Ok(())
    }

    pub async fn receive(&self) -> Result<WebSocketMessage, WebSocketError> {
        let message_event = poll_fn({
            move |cx| {
                trace!("Polling");
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

        let data = message_event.data();
        debug!("{:?}", &data);

        let message = match data {
            SocketMessageData::Text(s) => WebSocketMessage::String(s),
            SocketMessageData::ArrayBuffer(buf) => {
                let t_buffer: TypedArray<u8> = TypedArray::from(buf);
                WebSocketMessage::Binary(t_buffer.to_vec())
            }
            SocketMessageData::Blob(_) => panic!("binary should have been set to array buffer above..."),
        };

        Ok(message)
    }
}