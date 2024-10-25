use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};

#[derive(Debug, Subcommand)]
pub enum BrandSubcommand {
    Show(GetBrandByUrlName),

    GetBrandByName(GetBrandByName),

    Create(CreateBrand),

    Update(UpdateBrand),

    Delete(DeleteBrand),

    ShowAll,

    Pagination(BrandPagination),
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetBrandByUrlName {
    pub url_name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetBrandByName {
    pub name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateBrand {
    pub name: String,
    pub url_name: String,
    pub description: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateBrand {
    pub id: i32,
    pub name: Option<String>,
    pub url_name: Option<String>,
    pub description: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteBrand {
    pub id: i32,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct BrandPagination {
    pub limit: Option<i64>, 
    pub last_id: Option<i32>, 
    pub order_by_desc: Option<bool>
}