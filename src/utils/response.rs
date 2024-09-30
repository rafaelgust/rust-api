use serde::Serialize;

use axum::http::StatusCode;
use serde_json::Value;
use crate::utils::utf8_json::Utf8Json;

pub type BaseResponse = (StatusCode, Utf8Json<Value>);

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn new_success_data(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data: Some(data),
            message: None,
        }
    }

    pub fn new_success_message(message: impl ToString) -> Self {
        Self {
            status: "success".to_string(),
            data: None,
            message: Some(message.to_string()),
        }
    }

    pub fn new_error(message: impl ToString) -> Self {
        Self {
            status: "error".to_string(),
            data: None,
            message: Some(message.to_string()),
        }
    }
}