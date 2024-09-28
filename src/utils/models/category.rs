use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Selectable, Serialize)]
#[derive(Queryable, Identifiable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
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

// Response

#[derive(Serialize)]
pub struct CategoryProductResponse {
    pub name: String,
    pub url_name: String,
}