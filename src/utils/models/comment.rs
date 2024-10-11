use diesel::prelude::*;
use uuid::Uuid;

use crate::utils::models::user::user::User;

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(User))]
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
use super::user::user::UserCommentResponse;

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub user: UserCommentResponse,
}

// Request
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct InsertCommentRequest<'a> {
    pub text: Cow<'a, str>,
    pub product_id: Cow<'a, str>,
    pub user_id: Cow<'a, str>
}

#[derive(Deserialize)]
pub struct UpdateCommentRequest<'a> {
    pub id: Cow<'a, str>,
    pub text: Cow<'a, str>
}

#[derive(Deserialize)]
pub struct DeleteCommentRequest<'a> {
    pub id: Cow<'a, str>
}

#[derive(Deserialize)]
pub struct CommentPaginationRequest<'a> {
    pub limit: Option<i8>,
    pub product_id: Option<Cow<'a, str>>, 
    pub last_id: Option<Cow<'a, str>>,
    pub order_by_desc: Option<bool>
}