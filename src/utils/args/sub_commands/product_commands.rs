use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum ProductSubcommand {
    GetProductById(GetProductById),
    Create(CreateProduct),
    Update(UpdateProduct),
    Delete(DeleteProduct),
    ShowAll,
    Pagination(ProductPagination),
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetProductById {
    pub id: Uuid,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateProduct {
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateProduct {
    pub id: Uuid,
    pub name: Option<String>,
    pub url_name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub brand_id: Option<i32>,
    pub category_id: Option<i32>,
    pub published: Option<bool>,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteProduct {
    pub id: Uuid,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct ProductPagination {
    pub limit: Option<i8>,
    pub last_id: Option<Uuid>,
    pub order_by_desc: Option<bool>,
}
