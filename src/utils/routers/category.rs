use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::{Json, IntoResponse},
    Router,
    extract::Path,
};
use serde_json::json;

use crate::utils::{
    response::BaseResponse,
    utf8_json::Utf8Json,
};

use crate::utils::{models::category::CategoryProductResponse, response::ApiResponse};
use crate::utils::models::category::Category;
use crate::utils::ops::category_ops::{self, CategoryResult};
use crate::utils::args::commands::CategoryCommand;
use crate::utils::args::sub_commands::category_commands::{CategorySubcommand, CreateCategory, DeleteCategory, GetCategoryByUrlName, UpdateCategory as UpdateCategoryCommand};
use crate::utils::constants::{CATEGORY_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};

pub struct CategoryRoutes;

impl CategoryRoutes {

    pub fn get_routes() -> Router {
        Router::new()
            .route("/category/:category_url_name", get(Self::get_category))
            .route("/category", get(Self::get_all_categories))
            .route("/category", post(Self::new_category))
            .route("/category", put(Self::update_category))
            .route("/category", delete(Self::delete_category))
    }

    async fn get_category(Path(category_url_name): Path<String>) -> impl IntoResponse {
        let result = category_ops::handle_category_command(CategoryCommand {
            command: CategorySubcommand::Show(GetCategoryByUrlName {
                url_name: category_url_name,
            }),
        });

        Self::handle_category_product_result(result)
    }
    
    async fn get_all_categories() -> impl IntoResponse {
        let result = category_ops::handle_category_command(CategoryCommand {
            command: CategorySubcommand::ShowAll,
        });
    
        Self::handle_categories_product_result(result)
    }
    
    async fn new_category(Json(new_category): Json<CreateCategory>) -> impl IntoResponse {
        let category = CreateCategory {
            name: new_category.name.trim().to_string(),
            url_name: new_category.url_name.trim().to_string(),
            description: new_category.description.trim().to_string(),
        };
    
        let result = category_ops::handle_category_command(CategoryCommand {
            command: CategorySubcommand::Create(category),
        });
    
        Self::handle_message_result(result, StatusCode::CREATED, StatusCode::UNAUTHORIZED)
    }
    
    async fn update_category(Json(category): Json<UpdateCategoryCommand>) -> impl IntoResponse {
        let category = UpdateCategoryCommand {
            id: category.id,
            name: category.name.trim().to_string(),
            url_name: category.url_name.trim().to_string(),
            description: category.description.trim().to_string(),
            published: category.published,
        };
    
        let result = category_ops::handle_category_command(CategoryCommand {
            command: CategorySubcommand::Update(category),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }
    
    async fn delete_category(Json(category): Json<DeleteCategory>) -> impl IntoResponse {
        let result = category_ops::handle_category_command(CategoryCommand {
            command: CategorySubcommand::Delete(DeleteCategory {
                id: category.id,
            }),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }
    

    fn create_category_product_response(category: Category) -> CategoryProductResponse {

        CategoryProductResponse {
            name: category.name,
            url_name: category.url_name,
        }
    }

    fn handle_category_product_result(result: Result<CategoryResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(CategoryResult::Category(Some(category))) => {
                let response = Self::create_category_product_response(category);
                (StatusCode::OK, Utf8Json(json!(response)))
            },
            Ok(_) => (StatusCode::NOT_FOUND, Utf8Json(json!({"error": CATEGORY_NOT_FOUND}))),
            Err(_) => (StatusCode::NOT_FOUND, Utf8Json(json!({"error": FETCH_ERROR}))),
        }
    }

    fn handle_categories_product_result(result: Result<CategoryResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(CategoryResult::Categories(result)) => {
                let categories_responses: Vec<CategoryProductResponse> = result
                    .into_iter()
                    .map(|category| Self::create_category_product_response(category))
                    .collect();

                let json_response = ApiResponse::new_success_data(categories_responses);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(CATEGORY_NOT_FOUND.to_string());
                (StatusCode::NO_CONTENT, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_message_result(result: Result<CategoryResult, diesel::result::Error>, status_success: StatusCode, status_fail: StatusCode) -> BaseResponse {
        match result {
            Ok(CategoryResult::Message(result)) => {
                let json_response: ApiResponse<()> = ApiResponse::new_success_message(result);
                (status_success, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(UNEXPECTED_RESULT.to_string());
                (status_fail, Utf8Json(json!(json_response)))
            },
            Err(err) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(err.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

}
