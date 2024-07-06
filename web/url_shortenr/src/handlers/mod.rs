mod url;
use serde::Serialize;
pub(crate) use url::*;

#[derive(Debug, Serialize)]
pub struct ServiceResponse {
    success: bool,
    message: String,
    data: serde_json::Value,
}

impl ServiceResponse {
    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            data: serde_json::Value::Null,
        }
    }
}

impl Default for ServiceResponse {
    fn default() -> Self {
        Self {
            success: true,
            message: "success".to_string(),
            data: serde_json::Value::Null,
        }
    }
}