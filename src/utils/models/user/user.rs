use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role_id: i32,
    pub created_at: NaiveDateTime,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub username: &'a str,
    pub password: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub role_id: &'a i32,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser<'a> {
    pub id: &'a Uuid,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub role_id: Option<i32>,
    pub published: Option<bool>,
}
// Response
#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role_id: i32,
    pub published: bool,
}

#[derive(Serialize)]
pub struct UserCommentResponse {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub role_id: i32,
}

//Request
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct InsertUserRequest<'a> {
    pub username: Cow<'a, str>,
    pub email: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub first_name: Cow<'a, str>,
    pub last_name: Cow<'a, str>,
    pub role_id: i32
}

#[derive(Deserialize)]
pub struct UpdateUserRequest<'a> {
    pub id: Cow<'a, str>,
    pub username: Option<Cow<'a, str>>,
    pub email: Option<Cow<'a, str>>,
    pub password: Option<Cow<'a, str>>,
    pub first_name: Option<Cow<'a, str>>,
    pub last_name: Option<Cow<'a, str>>,
    pub role_id: Option<i32>,
    pub published: Option<bool>
}
#[derive(Deserialize)]
pub struct DeleteUserRequest<'a> {
    pub id: Cow<'a, str>,
}