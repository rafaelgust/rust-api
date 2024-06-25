use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::{Json, IntoResponse},
    Router,
    extract::Path,
};
use serde_json::json;

use crate::utils::response::ApiResponse;
use crate::utils::models::product::{ProductPaginationRequest, ProductResponse, DeleteProductRequest, InsertProductRequest, UpdateProductRequest};
use crate::utils::ops::product_ops::{self, ProductResult};
use crate::utils::args::commands::ProductCommand;
use crate::utils::args::sub_commands::product_commands::{ProductSubcommand, CreateProduct, DeleteProduct, GetProductById, UpdateProduct as UpdateProductCommand, ProductPagination};
use crate::utils::constants::{PRODUCT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};
use crate::utils::cryptography::{base32hex_to_uuid, uuid_to_base32hex};

pub fn get_product_routes() -> Router {
    Router::new()
        .route("/product/:id", get(get_product_by_id))
        .route("/product", get(get_all_products))
        .route("/product/list", post(get_products))
        .route("/product", post(new_product))
        .route("/product", put(update_product))
        .route("/product", delete(delete_product))
}

async fn get_product_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let product_id_uuid = match base32hex_to_uuid(&id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return (StatusCode::NOT_FOUND, Json(json!({"error": PRODUCT_NOT_FOUND}))).into_response();
        }
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::GetProductById(GetProductById {
            id: product_id_uuid,
        }),
    });

    match result {
        Ok(ProductResult::Product(Some(product))) => {
            let response = ProductResponse {
                id: uuid_to_base32hex(product.id),
                name: product.name,
                url_name: product.url_name,
                description: product.description,
                image: product.image,
                brand_id: product.brand_id,
                category_id: product.category_id,
                created_at: product.created_at,
                published: product.published,
            };
            (StatusCode::OK, Json(response)).into_response()
        },
        Ok(_) => (StatusCode::NOT_FOUND, Json(json!({"error": PRODUCT_NOT_FOUND}))).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))).into_response(),
    }
}

async fn get_all_products() -> impl IntoResponse {
    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::ShowAll,
    });

    match result {
        Ok(ProductResult::Products(products)) => {
            let products_with_base32hex: Vec<ProductResponse> = products.into_iter().map(|product| ProductResponse {
                id: uuid_to_base32hex(product.id),
                name: product.name,
                url_name: product.url_name,
                description: product.description,
                image: product.image,
                brand_id: product.brand_id,
                category_id: product.category_id,
                created_at: product.created_at,
                published: product.published,
            }).collect();

            let json_response: ApiResponse<Vec<ProductResponse>> = ApiResponse::new_success_data(products_with_base32hex);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(PRODUCT_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn get_products(Json(products): Json<ProductPaginationRequest<'_>>) -> impl IntoResponse {
    let last_id_uuid = match products.last_id {
        Some(ref last_id) => match base32hex_to_uuid(last_id) {
            Ok(uuid) => Some(uuid),
            Err(err) => {
                eprintln!("Error decoding base32hex to UUID: {}", err);
                return (StatusCode::NOT_FOUND, Json(json!({"error": PRODUCT_NOT_FOUND}))).into_response();
            }
        },
        None => None,
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Pagination(ProductPagination {
            limit: products.limit,
            last_id: last_id_uuid,
            order_by_desc: products.order_by_desc,
        }),
    });

    match result {
        Ok(ProductResult::Products(products)) => {
            let products_with_base32hex: Vec<ProductResponse> = products.into_iter().map(|product| ProductResponse {
                id: uuid_to_base32hex(product.id),
                name: product.name,
                url_name: product.url_name,
                description: product.description,
                image: product.image,
                brand_id: product.brand_id,
                category_id: product.category_id,
                created_at: product.created_at,
                published: product.published,
            }).collect();

            let json_response: ApiResponse<Vec<ProductResponse>> = ApiResponse::new_success_data(products_with_base32hex);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(PRODUCT_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn new_product(Json(new_product): Json<InsertProductRequest<'_>>) -> impl IntoResponse {
    let product = CreateProduct {
        name: new_product.name.trim().to_string(),
        url_name: new_product.url_name.trim().to_string(),
        description: new_product.description.trim().to_string(),
        image: Some(new_product.image.expect("REASON").to_string()),
        brand_id: new_product.brand_id,
        category_id: new_product.category_id,
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Create(product),
    });

    match result {
        Ok(ProductResult::Message(_)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("Product '{}' was created", new_product.name.trim()));
            (StatusCode::CREATED, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(UNEXPECTED_RESULT.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
        Err(err) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn update_product(Json(product): Json<UpdateProductRequest<'_>>) -> impl IntoResponse {
    let product_id_uuid = match base32hex_to_uuid(&product.id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))).into_response();
        }
    };

    let update_product = UpdateProductCommand {
        id: product_id_uuid,
        name: product.name.as_deref().map(String::from),
        url_name: product.url_name.as_deref().map(String::from),
        description: product.description.as_deref().map(String::from),
        image: Some(product.image.expect("REASON").to_string()),
        brand_id: product.brand_id,
        category_id: product.category_id,
        published: product.published,
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Update(update_product),
    });

    match result {
        Ok(ProductResult::Product(Some(product))) => {
            let response = ProductResponse {
                id: uuid_to_base32hex(product.id),
                name: product.name,
                url_name: product.url_name,
                description: product.description,
                image: product.image,
                brand_id: product.brand_id,
                category_id: product.category_id,
                created_at: product.created_at,
                published: product.published,
            };
            (StatusCode::ACCEPTED, Json(response)).into_response()
        },
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}

async fn delete_product(Json(product): Json<DeleteProductRequest<'_>>) -> impl IntoResponse {
    let product_id_uuid = match base32hex_to_uuid(&product.id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))).into_response();
        }
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Delete(DeleteProduct {
            id: product_id_uuid,
        }),
    });

    match result {
        Ok(ProductResult::Message(msg)) => (StatusCode::ACCEPTED, Json(json!({"message": msg}))).into_response(),
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}