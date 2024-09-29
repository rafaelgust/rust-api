use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::Json,
    Router,
    extract::Path,
};
use serde_json::json;

use crate::utils::{models::comment::Comment, response::ApiResponse};
use crate::utils::models::comment::{CommentPaginationRequest, CommentResponse, DeleteCommentRequest, InsertCommentRequest, UpdateCommentRequest};
use crate::utils::ops::comment_ops::{self, CommentResult};
use crate::utils::args::commands::CommentCommand;
use crate::utils::args::sub_commands::comment_commands::{
    CommentSubcommand, CreateComment, DeleteComment, GetCommentByProductId, UpdateComment as UpdateCommentCommand, CommentPagination
};
use crate::utils::constants::{COMMENT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};
use crate::utils::cryptography::{base32hex_to_uuid, uuid_to_base32hex};

type BaseResponse = (StatusCode, Json<serde_json::Value>);

pub struct CommentRoutes;

impl CommentRoutes {

    pub fn get_routes() -> Router {
        Router::new()
            .route("/comment/:comment_id", get(Self::get_comment_by_comment_id))
            .route("/comment", get(Self::get_all_comments))
            .route("/comment/list", post(Self::get_comments))
            .route("/comment", post(Self::new_comment))
            .route("/comment", put(Self::update_comment))
            .route("/comment", delete(Self::delete_comment))
    }

    async fn get_comment_by_comment_id(Path(comment_id): Path<String>) -> BaseResponse {
        let product_id_uuid = match Self::decode_base32hex(&comment_id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::GetCommentByProductId(GetCommentByProductId {
                product_id: product_id_uuid,
            }),
        });
    
        Self::handle_comment_result(result)
    }

    async fn get_all_comments() -> BaseResponse {
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::ShowAll,
        });
    
        Self::handle_comments_result(result)
    }

    async fn get_comments(Json(comments): Json<CommentPaginationRequest<'_>>) -> BaseResponse {
        let last_id_uuid = match Self::decode_base32hex(&comments.last_id.unwrap_or_default()) {
            Ok(uuid) => Some(uuid),
            Err(err) => return Self::handle_decode_error(err),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Pagination(CommentPagination {
                limit: comments.limit,
                last_id: last_id_uuid,
                order_by_desc: comments.order_by_desc,
            }),
        });
    
        Self::handle_comments_result(result)
    }

    async fn new_comment(Json(new_comment): Json<InsertCommentRequest<'_>>) -> BaseResponse {
        let comment = CreateComment {
            text: new_comment.text.trim().to_string(),
            product_id: Self::decode_base32hex(&new_comment.product_id).unwrap(),
            user_id: Self::decode_base32hex(&new_comment.user_id).unwrap(),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Create(comment),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }

    async fn update_comment(Json(comment): Json<UpdateCommentRequest<'_>>) -> BaseResponse {
        let comment = UpdateCommentCommand {
            id: Self::decode_base32hex(&comment.id).unwrap(),
            text: comment.text.trim().to_string(),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Update(comment),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }
    
    async fn delete_comment(Json(comment): Json<DeleteCommentRequest<'_>>) -> BaseResponse {
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Delete(DeleteComment {
                id: Self::decode_base32hex(&comment.id).unwrap(),
            }),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }

    fn decode_base32hex(id: &str) -> Result<uuid::Uuid, String> {
        base32hex_to_uuid(id).map_err(|e| e.to_string())
    }
    
    fn create_comment_response(comment: Comment) -> CommentResponse {

        CommentResponse {
            id: uuid_to_base32hex(comment.id),
            text: comment.text,
            created_at: comment.created_at,
            user_id: uuid_to_base32hex(comment.user_id)
        }
    }

    fn handle_decode_error(err: String) -> BaseResponse {
        eprintln!("Error decoding base32hex to UUID: {}", err);
        (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR})))
    }

    fn handle_comment_result(result: Result<CommentResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(CommentResult::Comment(Some(comment))) => {
                let response = Self::create_comment_response(comment);
                (StatusCode::OK, Json(json!(response)))
            },
            Ok(_) => (StatusCode::NOT_FOUND, Json(json!({"error": COMMENT_NOT_FOUND}))),
            Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))),
        }
    }

    fn handle_comments_result(result: Result<CommentResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(CommentResult::Comments(result)) => {
                let comments_response: Vec<CommentResponse> = result
                    .into_iter()
                    .map(|comment| Self::create_comment_response(comment))
                    .collect();

                let json_response = ApiResponse::new_success_data(comments_response);
                (StatusCode::OK, Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(COMMENT_NOT_FOUND.to_string());
                (StatusCode::NO_CONTENT, Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(json_response)))
            },
        }
    }

    fn handle_message_result(result: Result<CommentResult, diesel::result::Error>, status_success: StatusCode, status_fail: StatusCode) -> BaseResponse {
        match result {
            Ok(CommentResult::Message(result)) => {
                let json_response: ApiResponse<()> = ApiResponse::new_success_message(result);
                (status_success, Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(UNEXPECTED_RESULT.to_string());
                (status_fail, Json(json!(json_response)))
            },
            Err(err) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(err.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(json_response)))
            },
        }
    }

}
