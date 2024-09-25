use axum::{Extension, Json, response::IntoResponse};
use serde::{Serialize, Deserialize};

use crate::auth::models::CurrentUser;
use crate::utils::cryptography::uuid_to_base32hex;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    username: String, 
    email: String,
    uuid: String
}

pub async fn hello(Extension(current_user): Extension<CurrentUser>) -> impl IntoResponse {
    Json(UserResponse {
        uuid: uuid_to_base32hex(current_user.id),
        username: current_user.username,
        email: current_user.email,
    })
}