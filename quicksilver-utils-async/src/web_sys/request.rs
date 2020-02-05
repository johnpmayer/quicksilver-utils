
use crate::request::RequestError;

pub async fn get_resource(url: &str) -> Result<String, RequestError> {
    surf::get(url).recv_string().await.map_err(|e| RequestError::NativeError(format!("{}", e)))
}