use axum::{
    extract::Path, http::StatusCode, response::Json, routing::{delete, get, post, put}, Router
};
use serde_json::json;

use crate::utils::{
    args::sub_commands::brand_commands::GetBrandByName, response::BaseResponse, utf8_json::Utf8Json
};

use crate::utils::{
    args::{
        commands::BrandCommand,
        sub_commands::brand_commands::{BrandPagination, BrandSubcommand, CreateBrand, DeleteBrand, GetBrandByUrlName, UpdateBrand as UpdateBrandCommand}
    }, constants::{BRAND_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT}, models::brand::{InsertBrandRequest, UpdateBrandRequest}, ops::brand_ops::{self, BrandResult}, response::ApiResponse
};

pub struct BrandRoutes;

impl BrandRoutes {
    pub fn get_routes() -> Router {
        Router::new()
            .route("/brand/:url_name", get(Self::get_brand_by_url_name))
            .route("/brand/name/:name", get(Self::get_brand_by_name))
            .route("/brand", get(Self::get_all_brands))
            .route("/brand/list", post(Self::get_brands))
            .route("/brand", post(Self::new_brand))
            .route("/brand", put(Self::update_brand))
            .route("/brand", delete(Self::delete_brand))
    }

    async fn get_brand_by_url_name(Path(url_name): Path<String>) -> BaseResponse {
        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::Show(GetBrandByUrlName { url_name }),
        });

        Self::handle_brand_result(result)
    }

    async fn get_brand_by_name(Path(name): Path<String>) -> BaseResponse {
        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::GetBrandByName(GetBrandByName { name }),
        });

        Self::handle_brands_result(result)
    }

    async fn get_all_brands() -> BaseResponse {
        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::ShowAll,
        });

        Self::handle_brands_result(result)
    }

    async fn get_brands(Json(brands): Json<BrandPagination>) -> BaseResponse {
        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::Pagination(BrandPagination {
                limit: brands.limit,
                last_id: brands.last_id,
                order_by_desc: brands.order_by_desc,
            }),
        });

        Self::handle_brands_result(result)
    }

    async fn new_brand(Json(new_brand): Json<InsertBrandRequest<'_>>) -> BaseResponse {
        let brand = CreateBrand {
            name: new_brand.name.trim().to_string(),
            url_name: new_brand.url_name.trim().to_string(),
            description: new_brand.description.trim().to_string(),
        };

        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::Create(brand),
        });

        Self::handle_message_result(result, StatusCode::CREATED, StatusCode::UNAUTHORIZED)
    }

    async fn update_brand(Json(brand): Json<UpdateBrandRequest<'_>>) -> BaseResponse {

        let update_brand = UpdateBrandCommand  {
            id: brand.id,
            name: brand.name.as_deref().map(String::from),
            url_name: brand.url_name.as_deref().map(String::from),
            description: brand.description.as_deref().map(String::from),
            published: brand.published,
        };
    
        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::Update(update_brand),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::NOT_ACCEPTABLE)
    }

    async fn delete_brand(Json(brand): Json<DeleteBrand>) -> BaseResponse {
        let result = brand_ops::handle_brand_command(BrandCommand {
            command: BrandSubcommand::Delete(DeleteBrand {
                id: brand.id,
            }),
        });

        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }

    fn handle_brand_result(result: Result<BrandResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(BrandResult::Brand(Some(brand))) => {
                let json_response = ApiResponse::new_success_data(brand);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(BRAND_NOT_FOUND.to_string());
                (StatusCode::NOT_FOUND, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_brands_result(result: Result<BrandResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(BrandResult::Brands(brands)) => {
                let json_response = ApiResponse::new_success_data(brands);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(BRAND_NOT_FOUND.to_string());
                (StatusCode::NOT_FOUND, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_message_result(result: Result<BrandResult, diesel::result::Error>, status_success: StatusCode, status_fail: StatusCode) -> BaseResponse {
        match result {
            Ok(BrandResult::Message(result)) => {
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
