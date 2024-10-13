use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum CommentSubcommand {
    GetAmountOfComments(GetAmountOfComments),

    GetCommentByProductId(GetCommentByProductId),

    GetCommentById(GetCommentById),

    Create(CreateComment),

    Update(UpdateComment),

    Delete(DeleteComment),

    ShowAll,

    Pagination(CommentPagination),
}
#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetAmountOfComments {
    pub product_id: Uuid,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetCommentByProductId {
    pub product_id: Uuid,
    pub order_is_desc: bool,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetCommentById {
    pub id: Uuid,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateComment {
    pub text: String,
    pub product_id: Uuid,
    pub user_id: Uuid
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateComment {
    pub id: Uuid,
    pub text: String,
    pub user_id: Uuid
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteComment {
    pub id: Uuid,
    pub user_id: Uuid
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CommentPagination {
    pub limit: Option<i8>, 
    pub last_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub order_by_desc: Option<bool>
}