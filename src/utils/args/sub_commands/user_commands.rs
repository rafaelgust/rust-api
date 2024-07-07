use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    GetUserByEmail(Auth),
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteUser),
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct Auth {
    pub email: String,
}
#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: i32
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateUser {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role_id: Option<i32>,
    pub published: Option<bool>,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteUser {
    pub id: Uuid,
}