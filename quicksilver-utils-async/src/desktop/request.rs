use super::tls::client_config;
use crate::request::{RequestError, Result, ServiceClient};
use async_trait::async_trait;
use bytes::Bytes;
use http::Uri;
use hyper::{client::connect::HttpConnector, Body, Client, Request, StatusCode};
use hyper_rustls::HttpsConnector;
use log::debug;
use std::sync::Arc;

pub struct ServiceClientImpl {
    hyper_client: Client<HttpsConnector<HttpConnector>>,
    auth_token: Option<String>,
}

fn hyper_client() -> Client<HttpsConnector<HttpConnector>> {
    // TODO: hyper_rustls provides rustls-native-certs behind feature flag
    let config = client_config();
    let mut http_connector = HttpConnector::new();
    http_connector.enforce_http(false);
    let https_connector: HttpsConnector<HttpConnector> =
        From::from((http_connector, Arc::new(config)));
    Client::builder().build::<_, hyper::Body>(https_connector)
}

#[async_trait]
impl ServiceClient for ServiceClientImpl {
    fn new() -> Self {
        let hyper_client = hyper_client();
        let auth_token = None;
        ServiceClientImpl {
            hyper_client,
            auth_token,
        }
    }

    fn set_auth_token(&mut self, auth_token: &str) {
        self.auth_token = Some(auth_token.to_string())
    }

    async fn post_raw(&self, uri: Uri, request_body: Bytes) -> Result<Bytes> {
        let body: Body = From::from(request_body);
        let mut builder = Request::builder()
            .method(hyper::Method::POST)
            .uri(uri)
            .header("Accept", "application/octet-stream")
            .header("Content-Type", "application/octet-stream");
        if let Some(auth_token) = &self.auth_token {
            builder = builder.header(http::header::AUTHORIZATION, auth_token)
        }
        let request = builder.body(body).expect("request built");
        let response =
            self.hyper_client.request(request).await.map_err(|e| {
                RequestError::NativeError(format!("Failed making hyper request {}", e))
            })?;
        let status = response.status();
        debug!("Status code: {}", status);
        match status {
            StatusCode::OK => {
                let response_payload =
                    hyper::body::to_bytes(response.into_body())
                        .await
                        .map_err(|e| {
                            RequestError::NativeError(format!(
                                "Failed to get response bytes: {}",
                                e
                            ))
                        })?;
                Ok(response_payload)
            }
            _ => Err(RequestError::NativeError(format!(
                "Non-200 response: {}",
                status
            ))),
        }
    }
}
