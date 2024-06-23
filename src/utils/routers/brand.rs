use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::{Json, IntoResponse},
    Router,
    extract::Path,
};
use serde_json::json;

use crate::utils::response::ApiResponse;
use crate::utils::models::brand::Brand;
use crate::utils::ops::brand_ops::{self, BrandResult};
use crate::utils::args::commands::BrandCommand;
use crate::utils::args::sub_commands::brand_commands::{BrandSubcommand, CreateBrand, DeleteBrand, GetBrandByUrlName, UpdateBrand as UpdateBrandCommand, BrandPagination};
use crate::utils::constants::{BRAND_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};

pub fn get_brand_routes() -> Router {
    Router::new()
        .route("/brand/:brand_url_name", get(get_brand))
        .route("/brand", get(get_all_brands))
        .route("/brand/list", post(get_brands))
        .route("/brand", post(new_brand))
        .route("/brand", put(update_brand))
        .route("/brand", delete(delete_brand))
}

async fn get_brand(Path(brand_url_name): Path<String>) -> impl IntoResponse {
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Show(GetBrandByUrlName {
            url_name: brand_url_name,
        }),
    });

    match result {
        Ok(BrandResult::Brand(Some(brand))) => (StatusCode::OK, Json(brand)).into_response(),
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({"error": BRAND_NOT_FOUND}))).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))).into_response(),
    }
}

async fn get_all_brands() -> impl IntoResponse {
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::ShowAll,
    });

    match result {
        Ok(BrandResult::Brands(brands)) => {
            let json_response: ApiResponse<Vec<Brand>> = ApiResponse::new_success_data(brands);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(BRAND_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn get_brands(Json(brands): Json<BrandPagination>) -> impl IntoResponse {
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Pagination(BrandPagination {
            limit: brands.limit,
            last_id: brands.last_id,
            order_by_desc: brands.order_by_desc,
        }),
    });

    match result {
        Ok(BrandResult::Brands(brands)) => {
            let json_response: ApiResponse<Vec<Brand>> = ApiResponse::new_success_data(brands);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(BRAND_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn new_brand(Json(new_brand): Json<CreateBrand>) -> impl IntoResponse {
    let brand = CreateBrand {
        name: new_brand.name.trim().to_string(),
        url_name: new_brand.url_name.trim().to_string(),
        description: new_brand.description.trim().to_string(),
    };

    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Create(brand),
    });

    match result {
        Ok(BrandResult::Message(_)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("Brand '{}' was created", new_brand.name.trim()));
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

async fn update_brand(Json(brand): Json<UpdateBrandCommand>) -> impl IntoResponse {
    let brand = UpdateBrandCommand {
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
        Ok(BrandResult::Brand(Some(brand))) => (StatusCode::ACCEPTED, Json(brand)).into_response(),
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}

async fn delete_brand(Json(brand): Json<DeleteBrand>) -> impl IntoResponse {
    let result = brand_ops::handle_brand_command(BrandCommand {
        command: BrandSubcommand::Delete(DeleteBrand {
            id: brand.id,
        }),
    });

    match result {
        Ok(BrandResult::Message(msg)) => (StatusCode::ACCEPTED, Json(json!({"message": msg}))).into_response(),
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}
