use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum CommentSubcommand {
    GetCommentByProductId(GetCommentByProductId),

    Create(CreateComment),

    Update(UpdateComment),

    Delete(DeleteComment),

    ShowAll,

    Pagination(CommentPagination),
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetCommentByProductId {
    pub product_id: Uuid,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateComment {
    pub text: String,
    pub product_id: Uuid,
    pub user_id: Uuid
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateComment {
    pub id: i32,
    pub text: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteComment {
    pub id: i32,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CommentPagination {
    pub limit: Option<i64>, 
    pub last_id: Option<i32>, 
    pub order_by_desc: Option<bool>
}