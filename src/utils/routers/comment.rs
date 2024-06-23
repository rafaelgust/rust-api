use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::{Json, IntoResponse},
    Router,
    extract::{Path, Json as ExtractJson},
};
use serde_json::json;

use crate::utils::response::ApiResponse;
use crate::utils::models::comment::{CommentPaginationRequest, CommentResponse, DeleteCommentRequest, InsertCommentRequest, UpdateCommentRequest};
use crate::utils::ops::comment_ops::{self, CommentResult};
use crate::utils::args::commands::CommentCommand;
use crate::utils::args::sub_commands::comment_commands::{
    CommentSubcommand, CreateComment, DeleteComment, GetCommentByProductId, UpdateComment as UpdateCommentCommand, CommentPagination
};
use crate::utils::constants::{COMMENT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};
use crate::utils::cryptography::{base32hex_to_uuid, uuid_to_base32hex};

pub fn get_comment_routes() -> Router {
    Router::new()
        .route("/comment/:product_id", get(get_comment_by_product_id))
        .route("/comment", get(get_all_comments))
        .route("/comment/list", post(get_comments))
        .route("/comment", post(new_comment))
        .route("/comment", put(update_comment))
        .route("/comment", delete(delete_comment))
}

async fn get_comment_by_product_id(Path(product_id): Path<String>) -> impl IntoResponse {
    let product_id_uuid = match base32hex_to_uuid(&product_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"error": COMMENT_NOT_FOUND}))).into_response(),
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::GetCommentByProductId(GetCommentByProductId {
            product_id: product_id_uuid,
        }),
    });

    match result {
        Ok(CommentResult::Comment(Some(comment))) => {
            (StatusCode::OK, Json(CommentResponse {
                id: uuid_to_base32hex(comment.id),
                text: comment.text,
                created_at: comment.created_at,
                product_id: uuid_to_base32hex(comment.product_id),
                user_id: uuid_to_base32hex(comment.user_id),
            })).into_response()
        },
        Ok(_) => (StatusCode::NO_CONTENT, Json(json!({"error": COMMENT_NOT_FOUND}))).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": FETCH_ERROR}))).into_response(),
    }
}

async fn get_all_comments() -> impl IntoResponse {
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::ShowAll,
    });

    match result {
        Ok(CommentResult::Comments(comments)) => {
            let comments_with_base32hex: Vec<CommentResponse> = comments.into_iter().map(|comment| CommentResponse {
                id: uuid_to_base32hex(comment.id),
                text: comment.text,
                created_at: comment.created_at,
                product_id: uuid_to_base32hex(comment.product_id),
                user_id: uuid_to_base32hex(comment.user_id),
            }).collect();

            let json_response: ApiResponse<Vec<CommentResponse>> = ApiResponse::new_success_data(comments_with_base32hex);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(COMMENT_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn get_comments(ExtractJson(comments): ExtractJson<CommentPaginationRequest<'_>>) -> impl IntoResponse {
    let last_id_uuid = match base32hex_to_uuid(&comments.last_id.unwrap_or_default()) {
        Ok(uuid) => Some(uuid),
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"error": COMMENT_NOT_FOUND}))).into_response(),
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Pagination(CommentPagination {
            limit: comments.limit,
            last_id: last_id_uuid,
            order_by_desc: comments.order_by_desc,
        }),
    });

    match result {
        Ok(CommentResult::Comments(comments)) => {
            let comments_with_base32hex: Vec<CommentResponse> = comments.into_iter().map(|comment| CommentResponse {
                id: uuid_to_base32hex(comment.id),
                text: comment.text,
                created_at: comment.created_at,
                product_id: uuid_to_base32hex(comment.product_id),
                user_id: uuid_to_base32hex(comment.user_id),
            }).collect();

            let json_response: ApiResponse<Vec<CommentResponse>> = ApiResponse::new_success_data(comments_with_base32hex);
            (StatusCode::OK, Json(json_response)).into_response()
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(COMMENT_NOT_FOUND.to_string());
            (StatusCode::NO_CONTENT, Json(json_response)).into_response()
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
        },
    }
}

async fn new_comment(ExtractJson(new_comment): ExtractJson<InsertCommentRequest<'_>>) -> impl IntoResponse {
    let comment = CreateComment {
        text: new_comment.text.trim().to_string(),
        product_id: base32hex_to_uuid(&new_comment.product_id).unwrap(),
        user_id: base32hex_to_uuid(&new_comment.user_id).unwrap(),
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Create(comment),
    });

    match result {
        Ok(CommentResult::Message(_)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("Comment '{}' was created", new_comment.text.trim()));
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

async fn update_comment(ExtractJson(comment): ExtractJson<UpdateCommentRequest<'_>>) -> impl IntoResponse {
    let comment = UpdateCommentCommand {
        id: base32hex_to_uuid(&comment.id).unwrap(),
        text: comment.text.trim().to_string(),
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Update(comment),
    });

    match result {
        Ok(CommentResult::Comment(Some(comment))) => {
            (StatusCode::ACCEPTED, Json(CommentResponse {
                id: uuid_to_base32hex(comment.id),
                text: comment.text,
                created_at: comment.created_at,
                product_id: uuid_to_base32hex(comment.product_id),
                user_id: uuid_to_base32hex(comment.user_id),
            })).into_response()
        },
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}

async fn delete_comment(ExtractJson(comment): ExtractJson<DeleteCommentRequest<'_>>) -> impl IntoResponse {
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Delete(DeleteComment {
            id: base32hex_to_uuid(&comment.id).unwrap(),
        }),
    });

    match result {
        Ok(CommentResult::Message(msg)) => (StatusCode::ACCEPTED, Json(json!({"message": msg}))).into_response(),
        Ok(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": UNEXPECTED_RESULT}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))).into_response(),
    }
}
