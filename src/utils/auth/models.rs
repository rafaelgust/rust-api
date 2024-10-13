use std::borrow::Cow;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserContext {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub name: String,
    pub username: String,
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub password_hash: String,
}

pub struct AuthError {
    pub status: String,
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({ "status": "error", "message": self.message }));
        (self.status_code, body).into_response()
    }
}

#[derive(Deserialize)]
pub struct CreateUserData<'a> {
    pub email: Cow<'a, str>,
    pub username: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub first_name: Cow<'a, str>,
    pub last_name: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenData {
    pub refresh_token: String,
}