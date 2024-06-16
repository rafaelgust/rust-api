use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub created: NaiveDateTime,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::brands)]
pub struct NewBrand<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub description: &'a str,
    pub published: &'a bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::brands)]
pub struct UpdateBrand<'a> {
    pub id: &'a i32,
    pub name: Option<&'a str>,
    pub url_name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub published: Option<&'a bool>,
}