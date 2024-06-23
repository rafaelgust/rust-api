use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::{Json, IntoResponse},
    Router,
    extract::Path,
};
use serde_json::json;

use crate::utils::response::ApiResponse;
use crate::utils::models::category::Category;
use crate::utils::ops::category_ops::{self, CategoryResult};
use crate::utils::args::commands::CategoryCommand;
use crate::utils::args::sub_commands::category_commands::{CategorySubcommand, CreateCategory, DeleteCategory, GetCategoryByUrlName, UpdateCategory as UpdateCategoryCommand};
use crate::utils::constants::{CATEGORY_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};

pub fn get_category_routes() -> Router {
    Router::new()
        .route("/category/:category_url_name", get(get_category))
        .route("/category", get(get_all_categories))
        .route("/category", post(new_category))
        .route("/category", put(update_category))
        .route("/category", delete(delete_category))
}

async fn get_category(Path(category_url_name): Path<String>) -> impl IntoResponse {
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::Show(GetCategoryByUrlName {
            url_name: category_url_name,
        }),
    });

    match result {
        Ok(CategoryResult::Category(Some(category))) => (StatusCode::OK, Json(category)).into_response(),
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({"error": CATEGORY_NOT_FOUND}))).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))).into_response(),
    }
}

async fn get_all_categories() -> impl IntoResponse {
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::ShowAll,
    });

    match result {
        Ok(CategoryResult::Categories(categories)) => {
            let json_response: ApiResponse<Vec<Category>> = ApiResponse::new_success_data(categories);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(CATEGORY_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
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

    match result {
        Ok(CategoryResult::Message(_)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("Category '{}' was created", new_category.name.trim()));
            (StatusCode::CREATED, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Unexpected result".to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
        Err(err) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
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

    match result {
        Ok(CategoryResult::Category(Some(category))) => (StatusCode::ACCEPTED, Json(category)).into_response(),
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}

async fn delete_category(Json(category): Json<DeleteCategory>) -> impl IntoResponse {
    let result = category_ops::handle_category_command(CategoryCommand {
        command: CategorySubcommand::Delete(DeleteCategory {
            id: category.id,
        }),
    });

    match result {
        Ok(CategoryResult::Message(msg)) => (StatusCode::ACCEPTED, Json(json!({"message": msg}))).into_response(),
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}
