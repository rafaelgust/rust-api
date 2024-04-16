use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub description: &'a str,
    pub published: &'a bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::categories)]
pub struct UpdateCategory<'a> {
    pub id: &'a i32,
    pub name: Option<&'a str>,
    pub url_name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub published: Option<&'a bool>,
}