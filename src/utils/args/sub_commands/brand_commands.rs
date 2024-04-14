use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};

#[derive(Debug, Subcommand)]
pub enum BrandSubcommand {
    Show(GetBrand),

    Create(CreateBrand),

    Update(UpdateBrand),

    Delete(DeleteBrand),

    ShowAll,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct GetBrand {
    pub id: i32,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct CreateBrand {
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub published: bool,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct UpdateBrand {
    pub id: i32,
    pub name: String,
    pub url_name: String,
    pub description: String,
    pub published: bool,
}

#[derive(Debug, Args, Deserialize, Serialize)]
pub struct DeleteBrand {
    pub id: i32,
}