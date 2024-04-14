use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};

#[derive(Debug, Subcommand)]
pub enum CommentSubcommand {
    Show(GetComment),

    Create(CreateComment),

    Update(UpdateComment),

    Delete(DeleteComment),

    ShowAll,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetComment {
    pub id: i32,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateComment {
    pub text: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub product_id: i32,
    pub user_id: i32,
    pub published: bool,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateComment {
    pub id: i32,
    pub text: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub published: bool,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteComment {
    pub id: i32,
}
