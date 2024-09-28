use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::utils::models::brand::Brand;
use super::{brand::BrandProductResponse, category::CategoryProductResponse};

#[derive(Selectable)]
#[derive(Queryable, Associations, Identifiable)]
#[diesel(belongs_to(Brand))]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
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
    pub brand_id: i32,
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
    pub brand: Option<BrandProductResponse>,
    pub categories: Option<Vec<CategoryProductResponse>>
}

// Request
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct InsertProductRequest<'a> {
    pub name: Cow<'a, str>,
    pub url_name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub image: Option<Cow<'a, str>>,
    pub brand_id: i32,
    pub category_id: i32
}

#[derive(Deserialize)]
pub struct UpdateProductRequest<'a> {
    pub id: Cow<'a, str>,
    pub name: Option<Cow<'a, str>>,
    pub url_name: Option<Cow<'a, str>>,
    pub description: Option<Cow<'a, str>>,
    pub image: Option<Cow<'a, str>>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: Option<bool>
}

#[derive(Deserialize)]
pub struct DeleteProductRequest<'a> {
    pub id: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct ProductPaginationRequest<'a> {
    pub limit: Option<i8>,
    pub last_id: Option<Cow<'a, str>>,
    pub order_by_desc: Option<bool>,
}
