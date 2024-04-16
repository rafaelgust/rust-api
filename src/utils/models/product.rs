use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub description: &'a str,
    pub image: Option<&'a str>,
    pub brand_id: Option<&'a i32>,
    pub category_id: &'a i32,
    pub published: &'a bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::products)]
pub struct UpdateProduct<'a> {
    pub id: &'a i32,
    pub name: Option<&'a str>,
    pub url_name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub image: Option<&'a str>,
    pub brand_id: Option<&'a i32>,
    pub category_id: Option<&'a i32>,
    pub published: Option<&'a bool>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::products)]
pub struct RemoveProduct<'a> {
    pub id: &'a i32,
    pub published: Option<&'a bool>,
}