
#[derive(Debug)]
pub enum RequestError {
    NativeError(String),
}

#[cfg(not(target_arch = "wasm32"))]
use crate::desktop::request;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use crate::std_web::request;

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
use crate::web_sys::request;

pub async fn get_resource(url: &str) -> Result<String, RequestError> {
    request::get_resource(url).await
}