use rocket::http::uri::Origin;
use rocket::response::{status::Accepted, status::NotFound};
use rocket::serde::json::Json;

use crate::utils::models::brand::Brand;
use crate::utils::ops::brand_ops::{self, BrandResult};
use crate::utils::args::commands::BrandCommand;
use crate::utils::args::sub_commands::brand_commands::{BrandSubcommand, CreateBrand, DeleteBrand, GetBrand, UpdateBrand as UpdateBrandCommand};

use crate::utils::constants::{BRAND_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};

pub const URI : Origin<'static> = uri!("/brand");

#[get("/<brand_id>", format = "application/json")]
pub fn get_brand(brand_id: i32) ->  Result<Json<Brand>, NotFound<String>> {
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Show(GetBrand {
            id: brand_id
        }),
    });

    match result {
        Ok(BrandResult::Brand(Some(brand))) => Ok(Json(brand)),
        Ok(_) => Err(NotFound(BRAND_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}
#[get("/", format = "application/json")]
pub fn get_all_brands() -> Result<Json<Vec<Brand>>, NotFound<String>> {
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::ShowAll,
    });

    match result {
        Ok(BrandResult::Brands(brand)) => Ok(Json(brand)),
        Ok(_) => Err(NotFound(BRAND_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[post("/", data = "<new_brand>", format = "application/json")]
pub fn new_brand(new_brand: Json<CreateBrand>) -> Result<Accepted<String>, NotFound<String>> {

    let brand = CreateBrand {
        name: new_brand.name.trim().to_string(),
        url_name: new_brand.url_name.trim().to_string(),
        description: new_brand.description.trim().to_string(),
        published: true,
    };
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Create(brand),
    });
    
    match result {
        Ok(BrandResult::Message(_)) => Ok(Accepted(format!("Brand '{}' was created", new_brand.name.trim().to_string()))),
        Ok(_) => Err(NotFound(format!("Unable to find brand"))),
        Err(_) => Err(NotFound(format!("An error occurred while fetching brand"))),
    }
}

#[put("/", data = "<brand>", format = "application/json")]
pub fn update_brand(brand: Json<UpdateBrandCommand>) -> Result<Accepted<Json<Brand>>, NotFound<String>> {
    
    let brand = UpdateBrandCommand{
        id: brand.id,
        name: brand.name.trim().to_string(),
        url_name: brand.url_name.trim().to_string(),
        description: brand.description.trim().to_string(),
        published: brand.published,
    };
    
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Update(brand),
    });

    match result {
        Ok(BrandResult::Brand(Some(brand))) => Ok(Accepted(Json(brand))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[delete("/", data = "<brand>", format = "application/json")]
pub fn delete_brand(brand: Json<DeleteBrand>) ->  Result<Accepted<String>, NotFound<String>> {
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Delete(DeleteBrand {
            id: brand.id,
            published: false,
        }),
    });

    match result {
        Ok(BrandResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}