use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub text: String,
    pub date: Option<NaiveDateTime>,
    pub product_id: i32,
    pub user_id: i32,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment<'a> {
    pub text: &'a str,
    pub date: &'a Option<NaiveDateTime>,
    pub product_id: &'a i32,
    pub user_id: &'a i32,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::comments)]
pub struct UpdateComment<'a> {
    pub id: &'a i32,
    pub text: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::comments)]
pub struct RemoveComment<'a> {
    pub id: &'a i32,
    pub published: &'a bool,
}