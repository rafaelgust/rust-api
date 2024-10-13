use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    GetUserByEmail(EmailAuth),
    GetUserByUserName(UserNameAuth),
    VerifyUserName(UserName),
    Create(CreateUser),
    Update(UpdateUser),
    Delete(DeleteUser),
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct EmailAuth {
    pub email: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UserNameAuth {
    pub username: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UserName {
    pub username: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateUser {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_id: Option<i32>,
    pub published: Option<bool>,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteUser {
    pub id: Uuid,
}