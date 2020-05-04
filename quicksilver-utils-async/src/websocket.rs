//! # websocket
//!
//! An async websocket client that can send and recieve. The
//! `WebSocket` is cloneable, so reading and writing can happen
//! on separate futures.
use bytes::Bytes;
use url::Url;

#[derive(Debug)]
pub enum WebSocketError {
    NativeError(String),
    StateInit,
    StateError(String),
    StateClosed,
}

#[derive(Debug)]
pub enum WebSocketMessage {
    String(String),
    Binary(Bytes),
}

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
type WebSocketInner = crate::web_sys::websocket::AsyncWebSocket;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
type WebSocketInner = crate::std_web::websocket::AsyncWebSocket;

#[cfg(not(target_arch = "wasm32"))]
type WebSocketInner = crate::desktop::websocket::AsyncWebSocket;

#[derive(Clone)]
pub struct WebSocket {
    inner: WebSocketInner,
}

// TODO: switch to http::Uri
// TODO: switch to async_trait..
impl WebSocket {
    pub async fn connect(url: &Url) -> Result<Self, WebSocketError> {
        let inner = WebSocketInner::connect(url).await?;
        Ok(WebSocket { inner })
    }

    pub async fn send(&self, msg: &WebSocketMessage) -> Result<(), WebSocketError> {
        self.inner.send(msg).await
    }

    pub async fn receive(&self) -> Result<WebSocketMessage, WebSocketError> {
        self.inner.receive().await
    }

    pub async fn close(&self) -> Result<(), WebSocketError> {
        self.inner.close().await
    }
}
