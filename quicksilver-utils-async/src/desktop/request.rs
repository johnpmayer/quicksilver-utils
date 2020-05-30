use crate::request::{RequestError, Result, ServiceClient};
use async_trait::async_trait;
use bytes::Bytes;
use http::Uri;

pub struct ServiceClientImpl {
    auth_token: Option<String>,
}

#[async_trait]
impl ServiceClient for ServiceClientImpl {
    // TODO: build a surf client once and re-use?
    fn new() -> Self {
        let auth_token = None;
        ServiceClientImpl { auth_token }
    }

    fn set_auth_token(&mut self, auth_token: &str) {
        self.auth_token = Some(auth_token.to_string())
    }

    async fn post_raw(&self, uri: Uri, request_body: Bytes) -> Result<Bytes> {
        let raw_uri = format!("{}", uri);
        let mut request = surf::post(raw_uri)
            .set_header("Accept", "application/octet-stream")
            .set_header("Content-Type", "application/octet-stream");

        if let Some(auth_token) = &self.auth_token {
            request = request.set_header("Authorization", auth_token);
        }

        request = request.body_bytes(request_body);

        let response_bytes_vec = request
            .recv_bytes()
            .await
            .map_err(|e| RequestError::NativeError(format!("Failed making hyper request {}", e)))?;

        Ok(Bytes::from(response_bytes_vec))
    }
}
