use axum::{
    extract::Path, http::StatusCode, middleware, response::Json, routing::{delete, get, post, put}, Extension, Router
};

use serde_json::json;

use crate::utils::{
    args::sub_commands::comment_commands::{GetAmountOfComments, GetCommentById}, auth::{handlers::authorize, models::UserContext}, models::user::user::{User, UserCommentResponse}, response::BaseResponse, utf8_json::Utf8Json
};

use crate::utils::{models::comment::Comment, response::ApiResponse};
use crate::utils::models::comment::{CommentPaginationRequest, CommentResponse, DeleteCommentRequest, InsertCommentRequest, UpdateCommentRequest};
use crate::utils::ops::comment_ops::{self, CommentResult};
use crate::utils::args::commands::CommentCommand;
use crate::utils::args::sub_commands::comment_commands::{
    CommentSubcommand, CreateComment, DeleteComment, GetCommentByProductId, UpdateComment as UpdateCommentCommand, CommentPagination
};
use crate::utils::constants::{COMMENT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};
use crate::utils::cryptography::{base32hex_to_uuid, uuid_to_base32hex};


pub struct CommentRoutes;

impl CommentRoutes {

    pub fn get_routes() -> Router {
        let protected_routes = Router::new()
            .route("/comment", put(Self::update_comment))
            .route("/comment", delete(Self::delete_comment))
            .route("/comment", post(Self::new_comment))
            .layer(middleware::from_fn(authorize));

        let public_routes = Router::new()
            .route("/comment/amount/:product_id", get(Self::get_amount_of_comments))
            .route("/comment/id/:comment_id", get(Self::get_comment_by_id))
            .route("/comment/:product_id/:is_desc", get(Self::get_comments_by_product_id))
            .route("/comment", get(Self::get_all_comments))
            .route("/comment/list", post(Self::get_comments));

        Router::new()
        .merge(protected_routes)
        .merge(public_routes)
    }

    async fn get_amount_of_comments(Path(product_id): Path<String>) -> BaseResponse {
        let product_id_uuid = match Self::decode_base32hex(&product_id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::GetAmountOfComments(GetAmountOfComments {
                product_id: product_id_uuid,
            }),
        });
    
        Self::handle_message_result(result, StatusCode::OK, StatusCode::INTERNAL_SERVER_ERROR)
    }

    async fn get_comments_by_product_id(Path((product_id, is_desc)): Path<(String, String)>) -> BaseResponse {
        let product_id_uuid = match Self::decode_base32hex(&product_id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };

        let order_is_desc = is_desc.parse::<bool>().unwrap_or_default();
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::GetCommentByProductId(GetCommentByProductId {
                product_id: product_id_uuid,
                order_is_desc: order_is_desc,
            }),
        });
    
        Self::handle_comments_result(result)
    }

    async fn get_comment_by_id(Path(comment_id): Path<String>) -> BaseResponse {
        let comment_id_uuid = match Self::decode_base32hex(&comment_id) {
            Ok(uuid) => uuid,
            Err(err) => return Self::handle_decode_error(err),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::GetCommentById(GetCommentById {
                id: comment_id_uuid,
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
        let last_id_uuid = match Self::decode_base32hex(&comments.last_id.unwrap()) {
            Ok(uuid) => Some(uuid),
            Err(err) => return Self::handle_decode_error(err),
        };

        let product_id_uuid = match Self::decode_base32hex(&comments.product_id.unwrap()) {
            Ok(uuid) => Some(uuid),
            Err(err) => return Self::handle_decode_error(err),
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Pagination(CommentPagination {
                limit: comments.limit,
                product_id: product_id_uuid,
                last_id: last_id_uuid,
                order_by_desc: comments.order_by_desc,
            }),
        });
    
        Self::handle_comments_result(result)
    }

    async fn new_comment(
        Extension(current_user): Extension<UserContext>,
        Json(new_comment): Json<InsertCommentRequest>,
    ) -> BaseResponse {
        let comment = CreateComment {
            text: new_comment.text.trim().to_string(),
            product_id: Self::decode_base32hex(&new_comment.product_id).unwrap(),
            user_id: current_user.id,
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Create(comment),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }

    async fn update_comment(
        Extension(current_user): Extension<UserContext>,
        Json(comment): Json<UpdateCommentRequest>,
    ) -> BaseResponse {
        let comment = UpdateCommentCommand {
            id: Self::decode_base32hex(&comment.id).unwrap(),
            text: comment.text.trim().to_string(),
            user_id: current_user.id,
        };
    
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Update(comment),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }
    
    async fn delete_comment(
        Extension(current_user): Extension<UserContext>,
        Json(comment): Json<DeleteCommentRequest>
    ) -> BaseResponse {
        let result = comment_ops::handle_comment_command(CommentCommand {
            command: CommentSubcommand::Delete(DeleteComment {
                id: Self::decode_base32hex(&comment.id).unwrap(),
                user_id: current_user.id,
            }),
        });
    
        Self::handle_message_result(result, StatusCode::ACCEPTED, StatusCode::UNAUTHORIZED)
    }

    fn decode_base32hex(id: &str) -> Result<uuid::Uuid, String> {
        base32hex_to_uuid(id).map_err(|e| e.to_string())
    }
    
    fn create_comment_response(comment: Comment, user: User) -> CommentResponse {

        CommentResponse {
            id: uuid_to_base32hex(comment.id),
            text: comment.text,
            created_at: comment.created_at,
            user: UserCommentResponse {
                username: user.username,
                role_id: user.role_id,
            },
        }
    }

    fn handle_decode_error(err: String) -> BaseResponse {
        eprintln!("Error decoding base32hex to UUID: {}", err);
        (StatusCode::NOT_FOUND, Utf8Json(json!({"error": FETCH_ERROR})))
    }

    fn handle_comment_result(result: Result<CommentResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(CommentResult::Comment(Some((comment, user)))) => {
                let comment_response = Self::create_comment_response(comment, user);
                let json_response = ApiResponse::new_success_data(comment_response);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(COMMENT_NOT_FOUND.to_string());
                (StatusCode::NOT_FOUND, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_comments_result(result: Result<CommentResult, diesel::result::Error>) -> BaseResponse {
        match result {
            Ok(CommentResult::Comments(comments)) => {
                let comments_response: Vec<CommentResponse> = comments
                .into_iter()
                .filter_map(|opt_comments| opt_comments.map(|(comment, user)| Self::create_comment_response(comment, user)))
                .collect();
    
                let json_response = ApiResponse::new_success_data(comments_response);
                (StatusCode::OK, Utf8Json(json!(json_response)))
            },
            Ok(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(COMMENT_NOT_FOUND.to_string());
                (StatusCode::NO_CONTENT, Utf8Json(json!(json_response)))
            },
            Err(_) => {
                let json_response: ApiResponse<()> = ApiResponse::new_error(FETCH_ERROR.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, Utf8Json(json!(json_response)))
            },
        }
    }

    fn handle_message_result(result: Result<CommentResult, diesel::result::Error>, status_success: StatusCode, status_fail: StatusCode) -> BaseResponse {
        match result {
            Ok(CommentResult::Message(result)) => {
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
