use rocket::response::status::{Accepted, NotFound, Created};
use rocket::http::uri::Origin;

use rocket::serde::json::Json;

use crate::utils::response::ApiResponse;

use crate::utils::models::category::Category;
use crate::utils::ops::category_ops::{self, CategoryResult};
use crate::utils::args::commands::CategoryCommand;
use crate::utils::args::sub_commands::category_commands::{CategorySubcommand, CreateCategory, DeleteCategory, GetCategoryByUrlName, UpdateCategory as UpdateCategoryCommand};

use crate::utils::constants::{CATEGORY_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};

pub const URI : Origin<'static> = uri!("/category");

#[get("/<category_url_name>", format = "application/json")]
pub fn get_category(category_url_name: String) ->  Result<Json<Category>, NotFound<String>> {
    
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::Show(GetCategoryByUrlName {
            url_name: category_url_name
        }),
    });

    match result {
        Ok(CategoryResult::Category(Some(category))) => Ok(Json(category)),
        Ok(_) => Err(NotFound(CATEGORY_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}
#[get("/", format = "application/json")]
pub fn get_all_categories() -> Result<Json<Vec<Category>>, NotFound<String>> {
    
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::ShowAll,
    });

    match result {
        Ok(CategoryResult::Categories(category)) => Ok(Json(category)),
        Ok(_) => Err(NotFound(CATEGORY_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[post("/", data = "<new_category>", format = "application/json")]
pub fn new_category(new_category: Json<CreateCategory>) -> Result<Created<String>, NotFound<Json<ApiResponse<String>>>> {

    let category = CreateCategory {
        name: new_category.name.trim().to_string(),
        url_name: new_category.url_name.trim().to_string(),
        description: new_category.description.trim().to_string(),
    };
    
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::Create(category),
    });
    
    match result {
        Ok(CategoryResult::Message(_)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("Category '{}' was created", new_category.name.trim()));

            let json_string = serde_json::to_string(&json_response).unwrap();

            let created_response = Created::new("http://myservice.com/resource.json")
                .tagged_body(json_string);

            Ok(created_response)
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Unexpected result");

            Err(NotFound(Json(json_response)))
        },
        Err(err) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(format!("{}", err.to_string()));

            Err(NotFound(Json(json_response)))
        },
    }
}

#[put("/", data = "<category>", format = "application/json")]
pub fn update_category(category: Json<UpdateCategoryCommand>) -> Result<Accepted<Json<Category>>, NotFound<String>> {
    
    let category = UpdateCategoryCommand{
        id: category.id,
        name: category.name.trim().to_string(),
        url_name: category.url_name.trim().to_string(),
        description: category.description.trim().to_string(),
        published: category.published,
    };
    
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::Update(category),
    });

    match result {
        Ok(CategoryResult::Category(Some(category))) => Ok(Accepted(Json(category))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[delete("/", data = "<category>", format = "application/json")]
pub fn delete_category(category: Json<DeleteCategory>) ->  Result<Accepted<String>, NotFound<String>> {
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::Delete(DeleteCategory {
            id: category.id
        }),
    });

    match result {
        Ok(CategoryResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}