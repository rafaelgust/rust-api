use std::borrow::Cow;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Database models

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
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

// API Response models

#[derive(Serialize)]
pub struct BrandResponse {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct BrandProductResponse {
    pub name: String,
    pub url_name: String,
}

// API Request models

#[derive(Deserialize)]
pub struct InsertBrandRequest<'a> {
    pub name: Cow<'a, str>,
    pub url_name: Cow<'a, str>,
    pub description: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct UpdateBrandRequest<'a> {
    pub id: i32,
    pub name: Option<Cow<'a, str>>,
    pub url_name: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub published: Option<bool>,
}