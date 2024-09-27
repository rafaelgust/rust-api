use axum::{Extension, Json, response::IntoResponse};
use serde::{Serialize, Deserialize};


use super::models::UserContext;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    username: String, 
    email: String,
    uuid: String
}

pub async fn hello(Extension(current_user): Extension<UserContext>) -> impl IntoResponse {
    Json(current_user.id.to_string())
}