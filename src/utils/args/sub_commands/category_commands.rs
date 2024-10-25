use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};

#[derive(Debug, Subcommand)]
pub enum CategorySubcommand {
    Show(GetCategoryByUrlName),

    GetCategoryByName(GetCategoryByName),

    Create(CreateCategory),

    Update(UpdateCategory),

    Delete(DeleteCategory),

    ShowAll,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetCategoryByUrlName {
    pub url_name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetCategoryByName {
    pub name: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateCategory {
    pub name: String,
    pub url_name: String,
    pub description: String,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateCategory {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub published: bool,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteCategory {
    pub id: i32,
}