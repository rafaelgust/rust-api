use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role_id: i32,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub role_id: &'a i32,
    pub published: &'a bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser<'a> {
    pub id: &'a i32,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
    pub email: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct RemoveUser<'a> {
    pub id: &'a i32,
    pub published: &'a bool,
}
