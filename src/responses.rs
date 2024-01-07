use rocket::{serde::json::Json, Responder};
use serde::Serialize;
use std::collections::HashMap; 
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub code: u32,
}

#[derive(Debug, Serialize, Responder)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn create_error(message: &str) -> Error {
        Error::StandardError(Json(ErrorResponse {
            message: message.to_string(),
        }))
    }
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 500, content_type = "json")]
    StandardError(Json<ErrorResponse>),
}

#[derive(Debug, Serialize)]
pub struct EmptyTradKeys {
    pub empty_trad_keys: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct EmptyTradKeysHashMap {
    pub empty_or_missing_trad_keys: HashMap<String, (String, String)>
}
