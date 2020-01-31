use futures_io::{AsyncRead, AsyncWrite};
use url::Url;

use async_std::net::TcpStream;
use async_tls::TlsConnector;
use soketto::{
    connection::{Error as ConnectionError, Receiver, Sender},
    handshake::{Client, Error as HandshakeError, ServerResponse},
};
use std::cell::RefCell;
use std::io::Error as IoError;
use std::sync::Arc;

use log::debug;

use crate::websocket::{WebSocketError, WebSocketMessage};

#[derive(Clone)]
pub struct AsyncWebSocket {
    sender: Arc<RefCell<Sender<Box<dyn AsyncStream>>>>,
    receiver: Arc<RefCell<Receiver<Box<dyn AsyncStream>>>>,
}

impl From<HandshakeError> for WebSocketError {
    fn from(err: HandshakeError) -> Self {
        WebSocketError::NativeError(format!("Handshake error: {}", err))
    }
}

impl From<ConnectionError> for WebSocketError {
    fn from(err: ConnectionError) -> Self {
        WebSocketError::NativeError(format!("Connection error: {}", err))
    }
}

impl From<IoError> for WebSocketError {
    fn from(err: IoError) -> Self {
        WebSocketError::NativeError(format!("IO Error: {}", err))
    }
}

trait AsyncStream: AsyncRead + AsyncWrite + Unpin {}

impl<T: AsyncRead + AsyncWrite + Unpin> AsyncStream for T {}

impl AsyncWebSocket {
    async fn client(url: &Url) -> Result<Client<'_, Box<dyn AsyncStream>>, WebSocketError> {
        let port = url.port_or_known_default();
        let host = url.host_str().expect("url host");
        let path = url.path();
        let scheme = url.scheme();
        let address = url.socket_addrs(|| port).expect("url lookup via dns")[0];

        let transport_stream = TcpStream::connect(address).await.expect("Connect");

        let boxed_stream: Box<dyn AsyncStream> = if scheme == "wss" {
            // TODO: this hasn't yet been proven to work...
            debug!(
                "Starting TLS handshake for secure websocket with domain {}",
                host
            );
            let connector = TlsConnector::default();
            let handshake = connector.connect(host, transport_stream)?;
            let tls_stream = handshake.await?;
            debug!("Completed TLS handshake");
            Box::new(tls_stream)
        } else {
            Box::new(transport_stream)
        };

        Ok(Client::new(boxed_stream, host, path))
    }

    pub async fn connect(url: &Url) -> Result<Self, WebSocketError> {
        let mut client = AsyncWebSocket::client(url).await?;

        let (sender, receiver) = match client.handshake().await? {
            ServerResponse::Accepted { .. } => client.into_builder().finish(),
            ServerResponse::Redirect { .. } => unimplemented!("follow location URL"),
            ServerResponse::Rejected { .. } => unimplemented!("handle failure"),
        };

        let sender = Arc::new(RefCell::new(sender));
        let receiver = Arc::new(RefCell::new(receiver));

        Ok(AsyncWebSocket { sender, receiver })
    }

    pub async fn send(&self, msg: &str) -> Result<(), WebSocketError> {
        let mut sender = self.sender.borrow_mut();
        sender.send_text(msg).await?;
        sender.flush().await?; // otherwise it just sits there, which is just surprising for casual users
        Ok(())
    }

    pub async fn receive(&self) -> Result<WebSocketMessage, WebSocketError> {
        let data = self.receiver.borrow_mut().receive_data().await?;
        let message = if data.is_binary() {
            let data_slice: &[u8] = data.as_ref();
            WebSocketMessage::Binary(Vec::from(data_slice))
        } else {
            let data_slice: &[u8] = data.as_ref();
            let s = String::from_utf8(Vec::from(data_slice))
                .map_err(|_| WebSocketError::NativeError("invalid ut8".to_string()))?;
            WebSocketMessage::String(s)
        };
        Ok(message)
    }
}
