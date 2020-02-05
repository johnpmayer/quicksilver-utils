
#[derive(Debug)]
pub enum RequestError {
    NativeError(String),
}

#[cfg(not(target_arch = "wasm32"))]
use crate::desktop::request;

pub async fn get_resource(url: &str) -> Result<String, RequestError> {
    request::get_resource(url).await
}