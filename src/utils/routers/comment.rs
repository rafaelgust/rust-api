use rocket::response::status::{Accepted, NotFound, Created};
use rocket::http::uri::Origin;
use rocket::serde::json::Json;

use crate::utils::response::ApiResponse;
use crate::utils::models::comment::{CommentPaginationRequest, CommentResponse, DeleteCommentRequest, InsertCommentRequest, UpdateCommentRequest};
use crate::utils::ops::comment_ops::{self, CommentResult};
use crate::utils::args::commands::CommentCommand;
use crate::utils::args::sub_commands::comment_commands::{
    CommentSubcommand, CreateComment, DeleteComment, GetCommentByProductId, UpdateComment as UpdateCommentCommand, CommentPagination
};
use crate::utils::constants::{COMMENT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};
use crate::utils::cryptography::{base32hex_to_uuid, uuid_to_base32hex};

pub const URI: Origin<'static> = uri!("/comment");

#[get("/<product_id>", format = "application/json")]
pub fn get_comment_by_product_id(product_id: &str) -> Result<Json<CommentResponse>, NotFound<String>> {
    let product_id_uuid = match base32hex_to_uuid(product_id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return Err(NotFound(COMMENT_NOT_FOUND.to_string()));
        }
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::GetCommentByProductId(GetCommentByProductId {
            product_id: product_id_uuid
        }),
    });

    match result {
        Ok(CommentResult::Comment(Some(comment))) => Ok(Json(CommentResponse {
            id: uuid_to_base32hex(comment.id),
            text: comment.text,
            created_at: comment.created_at,
            product_id: uuid_to_base32hex(comment.product_id),
            user_id: uuid_to_base32hex(comment.user_id)
        })),
        Ok(_) => Err(NotFound(COMMENT_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[get("/", format = "application/json")]
pub fn get_all_comments() -> Result<Json<ApiResponse<Vec<CommentResponse>>>, NotFound<String>> {
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
                user_id: uuid_to_base32hex(comment.user_id)
            }).collect();

            Ok(Json(ApiResponse::new_success_data(comments_with_base32hex)))
        },
        Ok(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(COMMENT_NOT_FOUND.to_string())).unwrap())),
        Err(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(FETCH_ERROR.to_string())).unwrap())),
    }
}

#[post("/list", data = "<comments>", format = "application/json")]
pub fn get_comments(comments: Json<CommentPaginationRequest>) -> Result<Json<ApiResponse<Vec<CommentResponse>>>, NotFound<String>> {
    let last_id_uuid = match base32hex_to_uuid(&comments.last_id.unwrap_or_default()) {
        Ok(uuid) => Some(uuid),
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return Err(NotFound(COMMENT_NOT_FOUND.to_string()));
        }
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
                user_id: uuid_to_base32hex(comment.user_id)
            }).collect();

            Ok(Json(ApiResponse::new_success_data(comments_with_base32hex)))
        },
        Ok(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(COMMENT_NOT_FOUND.to_string())).unwrap())),
        Err(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(FETCH_ERROR.to_string())).unwrap())),
    }
}

#[post("/", data = "<new_comment>", format = "application/json")]
pub fn new_comment(new_comment: Json<InsertCommentRequest>) -> Result<Created<String>, NotFound<Json<ApiResponse<String>>>> {

    let comment = CreateComment {
        text: new_comment.text.trim().to_string(),
        product_id: base32hex_to_uuid(&new_comment.product_id).unwrap(),
        user_id: base32hex_to_uuid(&new_comment.user_id).unwrap()
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Create(comment),
    });

    match result {
        Ok(CommentResult::Message(_)) => {
            let json_response = ApiResponse::<String>::new_success_message(format!("Comment '{}' was created", new_comment.text.trim()));
            let json_string = serde_json::to_string(&json_response).unwrap();
            let created_response = Created::new("http://myservice.com/resource.json").tagged_body(json_string);

            Ok(created_response)
        },
        Ok(_) => Err(NotFound(Json(ApiResponse::new_error(UNEXPECTED_RESULT.to_string())))),
        Err(err) => Err(NotFound(Json(ApiResponse::new_error(format!("{}", err))))),
    }
}

#[put("/", data = "<comment>", format = "application/json")]
pub fn update_comment(comment: Json<UpdateCommentRequest>) -> Result<Accepted<Json<CommentResponse>>, NotFound<String>> {

    let comment = UpdateCommentCommand {
        id: base32hex_to_uuid(&comment.id).unwrap(),
        text: comment.text.trim().to_string()
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Update(comment),
    });

    match result {
        Ok(CommentResult::Comment(Some(comment))) => Ok(Accepted(Json(CommentResponse {
            id: uuid_to_base32hex(comment.id),
            text: comment.text,
            created_at: comment.created_at,
            product_id: uuid_to_base32hex(comment.product_id),
            user_id: uuid_to_base32hex(comment.user_id)
        }))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[delete("/", data = "<comment>", format = "application/json")]
pub fn delete_comment(comment: Json<DeleteCommentRequest>) -> Result<Accepted<String>, NotFound<String>> {

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Delete(DeleteComment { 
            id: base32hex_to_uuid(&comment.id).unwrap() 
        }),
    });

    match result {
        Ok(CommentResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}
