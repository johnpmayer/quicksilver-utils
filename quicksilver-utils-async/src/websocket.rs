
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
    Binary(Vec<u8>),
}

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
pub type WebSocket = crate::web_sys::websocket::AsyncWebSocket;

#[cfg(not(target_arch = "wasm32"))]
pub type WebSocket = crate::desktop::websocket::AsyncWebSocket;