use diesel::prelude::*;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: Uuid,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment<'a> {
    pub id: &'a Uuid,
    pub text: &'a str,
    pub product_id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::comments)]
pub struct UpdateComment<'a> {
    pub id: &'a Uuid,
    pub text: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::comments)]
pub struct RemoveComment<'a> {
    pub id: &'a Uuid,
    pub published: &'a bool,
}

// Response
#[derive(Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub product_id: String,
    pub user_id: String
}

// Request
#[derive(Serialize, Deserialize)]
pub struct InsertCommentRequest<'a> {
    pub text: String, 
    pub product_id: &'a str, 
    pub user_id: &'a str
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCommentRequest<'a> {
    pub id: &'a str, 
    pub text: String, 
}

#[derive(Deserialize)]
pub struct DeleteCommentRequest<'a> {
    pub id: &'a str
}

#[derive(Serialize, Deserialize)]
pub struct CommentPaginationRequest<'a> {
    pub limit: Option<i8>, 
    pub last_id: Option<&'a str>, 
    pub order_by_desc: Option<bool>
}