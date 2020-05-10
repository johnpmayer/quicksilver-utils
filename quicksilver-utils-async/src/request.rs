use async_trait::async_trait;
use bytes::Bytes;
use http::Uri;
use log::{debug, trace};
use protobuf::Message;

#[derive(Debug)]
pub enum RequestError {
    NativeError(String),
}

pub type Result<T> = std::result::Result<T, RequestError>;

#[cfg(not(target_arch = "wasm32"))]
use crate::desktop::request as platform;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use crate::std_web::request;

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
use crate::web_sys::request;

#[async_trait]
pub trait ServiceClient {
    fn new() -> Self;

    fn set_auth_token(&mut self, auth_token: &str);

    async fn post_raw(&self, uri: Uri, request_body: Bytes) -> Result<Bytes>;

    async fn post_proto<RequestT, ResponseT>(
        &self,
        uri: Uri,
        request_payload: &RequestT,
    ) -> Result<ResponseT>
    where
        RequestT: Message,
        ResponseT: Message,
    {
        let request_body: Bytes = From::from(request_payload.write_to_bytes().unwrap());
        trace!("Request bytes: {:?}", request_body);
        let response_body: Bytes = self.post_raw(uri, request_body).await?;
        trace!("Response bytes: {:?}", response_body);
        let response_payload =
            protobuf::parse_from_carllerche_bytes::<ResponseT>(&response_body).unwrap();
        Ok(response_payload)
    }
}

pub use platform::ServiceClientImpl;
