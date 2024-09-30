use axum::{
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Json},
};
use serde::Serialize;

pub struct Utf8Json<T>(pub T);

impl<T> IntoResponse for Utf8Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let body = Json(self.0);

        (headers, body).into_response()
    }
}