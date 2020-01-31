//! This file only exists to debug the issues with connecting to wss on desktop, which seems broken

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use async_tls::TlsConnector;
use url::Url;

// use rustls::ClientConfig;

fn main() -> io::Result<()> {
    // let url = Url::parse("https://echo.websocket.org").expect("parse a url");
    let url = Url::parse("https://www.google.com").expect("parse a url");
    // let url = Url::parse("https://www.websocket.org/").expect("parse a url");
    let port = url.port_or_known_default();
    let addr = url.socket_addrs(|| port).expect("url lookup via dns")[0];
    let domain = url.host_str().expect("url host");
    // let domain = "websocket.org";

    // Create a bare bones HTTP GET request
    let http_request = format!("GET / HTTP/1.0\r\nHost: {}\r\n\r\n", domain);

    // let cafile = &options.cafile;

    task::block_on(async move {
        // Create default connector comes preconfigured with all you need to safely connect
        // to remote servers!
        let connector = TlsConnector::default();

        // Open a normal TCP connection, just as you are used to
        let tcp_stream = TcpStream::connect(&addr).await?;

        // Use the connector to start the handshake process.
        // This consumes the TCP stream to ensure you are not reusing it.
        // Awaiting the handshake gives you an encrypted
        // stream back which you can use like any other.
        let mut tls_stream = connector.connect(&domain, tcp_stream)?.await?;

        // We write our crafted HTTP request to it
        tls_stream.write_all(http_request.as_bytes()).await?;

        // And read it all to stdout
        let mut stdout = io::stdout();
        io::copy(&mut tls_stream, &mut stdout).await?;

        // Voila, we're done here!
        Ok(())
    })
}
