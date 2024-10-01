use axum::{
    extract::Path, http::StatusCode, response::Json, routing::{delete, get, post, put}, Router
};
use serde_json::json;

use crate::utils::{
    response::BaseResponse,
    utf8_json::Utf8Json,
};

use crate::utils::{
    models::{
        brand::{Brand, BrandProductResponse},
        category::{Category, CategoryProductResponse},
        product::{Product, ProductPaginationRequest, ProductResponse, DeleteProductRequest, InsertProductRequest, UpdateProductRequest}
    },
    response::ApiResponse,
    ops::product_ops::{self, ProductResult},
    args::{
        commands::ProductCommand,
        sub_commands::product_commands::{ProductSubcommand, CreateProduct, DeleteProduct, GetProductById, GetProductByIdUrlName, UpdateProduct as UpdateProductCommand, ProductPagination}
    },
    constants::{PRODUCT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT},
    cryptography::{base32hex_to_uuid, uuid_to_base32hex}
};

pub struct ProductRoutes;

impl ProductRoutes {

    pub fn get_routes() -> Router {
        Router::new()
            .route("/p/:url_name", get(Self::get_product_by_url_name))
            .route("/product/:id", get(Self::get_product_by_id))
            .route("/product", get(Self::get_all_products))
            .route("/product/list", post(Self::get_products))
            .route("/product", post(Self::new_product))
            .route("/product", put(Self::update_product))
            .route("/product", delete(Self::delete_product))
    }

    async fn get_product_by_url_name(Path(url_name): Path<String>) -> BaseResponse {
        let result = product_ops::handle_product_command(ProductCommand {
            command: ProductSubcommand::GetProductByIdUrlName(GetProductByIdUrlName { url_name }),
        });

        Self::handle_product_result(result)
    }

    async fn get_product_by_id(Path(id): Path<String>) -> BaseResponse {
        let product_id_uuid = match Self::decode_base32hex(&id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };

        let result = product_ops::handle_product_command(ProductCommand {
            command: ProductSubcommand::GetProductById(GetProductById { id: product_id_uuid }),
        });

        Self::handle_product_result(result)
    }

    async fn get_all_products() -> BaseResponse {
        let result = product_ops::handle_product_command(ProductCommand {
            command: ProductSubcommand::ShowAll,
        });

        Self::handle_products_result(result)
    }

    async fn get_products(Json(products): Json<ProductPaginationRequest<'_>>) -> BaseResponse {
        let last_id_uuid = match products.last_id {
            Some(ref last_id) => match Self::decode_base32hex(last_id) {
                Ok(uuid) => Some(uuid),
                Err(err) => return Self::handle_decode_error(err),
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

        Self::handle_products_result(result)
    }

    async fn new_product(Json(new_product): Json<InsertProductRequest<'_>>) -> BaseResponse {
        let product = CreateProduct {
            name: new_product.name.trim().to_string(),
            url_name: new_product.url_name.trim().to_string(),
            description: new_product.description.trim().to_string(),
            image: Some(new_product.image.expect("REASON").to_string()),
            brand_id: new_product.brand_id,
        };

        let result = product_ops::handle_product_command(ProductCommand {
            command: ProductSubcommand::Create(product),
        });

        Self::handle_message_result(result , StatusCode::CREATED, StatusCode::UNAUTHORIZED)
    }

    async fn update_product(Json(product): Json<UpdateProductRequest<'_>>) -> BaseResponse {
        let product_id_uuid = match Self::decode_base32hex(&product.id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };

        let update_product = UpdateProductCommand {
            id: product_id_uuid,
            name: product.name.as_deref().map(String::from),
            url_name: product.url_name.as_deref().map(String::from),
            description: product.description.as_deref().map(String::from),
            image: Some(product.image.expect("REASON").to_string()),
            brand_id: product.brand_id,
            published: product.published,
        };

        let result = product_ops::handle_product_command(ProductCommand {
            command: ProductSubcommand::Update(update_product),
        });

        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::NOT_ACCEPTABLE)
    }

    async fn delete_product(Json(product): Json<DeleteProductRequest<'_>>) -> BaseResponse {
        let product_id_uuid = match Self::decode_base32hex(&product.id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };

        let result = product_ops::handle_product_command(ProductCommand {
            command: ProductSubcommand::Delete(DeleteProduct { id: product_id_uuid }),
        });

        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }

    fn decode_base32hex(id: &str) -> Result<uuid::Uuid, String> {
        base32hex_to_uuid(id).map_err(|e| e.to_string())
    }
    
    fn create_product_response(product: Product, brand: Option<Brand>, categories: Vec<Category>) -> ProductResponse {
        let brand_product_response = brand.map(|b| BrandProductResponse {
            name: b.name,
            url_name: b.url_name,
        });

        let category_product_responses = categories.into_iter().map(|c| CategoryProductResponse {
            name: c.name,
            url_name: c.url_name,
        }).collect();

        ProductResponse {
            id: uuid_to_base32hex(product.id),
            name: product.name,
            url_name: product.url_name,
            description: product.description,
            image: product.image,
            brand: brand_product_response,
            categories: Some(category_product_responses),
        }
    }

    fn handle_decode_error(err: String) -> BaseResponse {
        eprintln!("Error decoding base32hex to UUID: {}", err);
        (StatusCode::NOT_FOUND, Utf8Json(json!({"error": FETCH_ERROR})))
    }

    fn handle_product_result(result: Result<ProductResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(ProductResult::Product(Some((product, brand, categories)))) => {
                let product_response = Self::create_product_response(product, brand, categories);
                let json_response = ApiResponse::new_success_data(product_response);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(PRODUCT_NOT_FOUND.to_string());
                (StatusCode::NOT_FOUND, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_products_result(result: Result<ProductResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(ProductResult::Products(products)) => {
                let products_responses: Vec<ProductResponse> = products
                    .into_iter()
                    .filter_map(|opt_product_brand| opt_product_brand.map(|(product, brand, categories)| Self::create_product_response(product, brand, categories)))
                    .collect();

                let json_response = ApiResponse::new_success_data(products_responses);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(PRODUCT_NOT_FOUND.to_string());
                (StatusCode::NO_CONTENT, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_message_result(result: Result<ProductResult, diesel::result::Error>, status_success: StatusCode, status_fail: StatusCode) -> BaseResponse {
        match result {
            Ok(ProductResult::Message(result)) => {
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