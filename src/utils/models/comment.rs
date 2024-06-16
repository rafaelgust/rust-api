use diesel::prelude::*;
use uuid::Uuid;

use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize)]
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
#[derive(Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub text: String,
    pub created_at: NaiveDateTime,
    pub product_id: String,
    pub user_id: String
}