use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub url_name: &'a str,
    pub description: &'a str,
    pub image: Option<&'a str>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::products)]
pub struct UpdateProduct<'a> {
    pub id: &'a Uuid,
    pub name: Option<&'a str>,
    pub url_name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub image: Option<&'a str>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: Option<bool>,
}

// Response
#[derive(Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub published: bool,
}

// Request
#[derive(Serialize, Deserialize)]
pub struct InsertProductRequest {
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub id: String,
    pub name: Option<String>,
    pub url_name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: Option<bool>,
}

#[derive(Deserialize)]
pub struct DeleteProductRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProductPaginationRequest {
    pub limit: Option<i8>,
    pub last_id: Option<String>,
    pub order_by_desc: Option<bool>,
}
