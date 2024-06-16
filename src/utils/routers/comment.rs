use rocket::response::status::{Accepted, NotFound, Created};
use rocket::http::uri::Origin;

use rocket::serde::json::Json;
use uuid::Uuid;

use crate::utils::response::ApiResponse;

use crate::utils::models::comment::Comment;
use crate::utils::ops::comment_ops::{self, CommentResult};

use crate::utils::args::commands::CommentCommand;
use crate::utils::args::sub_commands::comment_commands::{CommentSubcommand, CreateComment, DeleteComment, GetCommentByProductId, UpdateComment as UpdateCommentCommand, CommentPagination};

use crate::utils::constants::{BRAND_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};

pub const URI : Origin<'static> = uri!("/comment");

#[get("/<product_id>", format = "application/json")]
pub fn get_comment_by_product_id(product_id: String) ->  Result<Json<Comment>, NotFound<String>> {

    let product_id = Uuid::parse_str(&product_id).unwrap();
    
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::GetCommentByProductId(GetCommentByProductId {
            product_id: product_id
        }),
    });

    match result {
        Ok(CommentResult::Comment(Some(comment))) => Ok(Json(comment)),
        Ok(_) => Err(NotFound(BRAND_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}
#[get("/", format = "application/json")]
pub fn get_all_comments() -> Result<Json<ApiResponse<Vec<Comment>>>, NotFound<String>> {
    
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::ShowAll,
    });

    match result {
        Ok(CommentResult::Comments(comment)) => {
            let json_response: ApiResponse<Vec<Comment>> = ApiResponse::new_success_data(comment);
            
            Ok(Json(json_response))
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(BRAND_NOT_FOUND.to_string());

            let json_string = serde_json::to_string(&json_response).unwrap();

            Err(NotFound(json_string))
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());

            let json_string = serde_json::to_string(&json_response).unwrap();

            Err(NotFound(json_string))
        },
    }
}

#[post("/list", data = "<comments>", format = "application/json")]
pub fn get_comments(comments: Json<CommentPagination>) -> Result<Json<ApiResponse<Vec<Comment>>>, NotFound<String>> {
    
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Pagination(
            CommentPagination {
                limit: comments.limit,
                last_id: comments.last_id,
                order_by_desc: comments.order_by_desc,
            }
        ),
    });

    match result {
        Ok(CommentResult::Comments(comment)) => {
            let json_response: ApiResponse<Vec<Comment>> = ApiResponse::new_success_data(comment);
            
            Ok(Json(json_response))
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(BRAND_NOT_FOUND.to_string());

            let json_string = serde_json::to_string(&json_response).unwrap();

            Err(NotFound(json_string))
        },
        Err(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(FETCH_ERROR.to_string());

            let json_string = serde_json::to_string(&json_response).unwrap();

            Err(NotFound(json_string))
        },
    }
}

#[post("/", data = "<new_comment>", format = "application/json")]
pub fn new_comment(new_comment: Json<CreateComment>) -> Result<Created<String>, NotFound<Json<ApiResponse<String>>>> {

    let comment = CreateComment {
        text: new_comment.text.trim().to_string(),
        product_id: new_comment.product_id,
        user_id: new_comment.user_id
    };

    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Create(comment),
    });

    match result {
        Ok(CommentResult::Message(_)) => {
            let json_response: ApiResponse<String> = ApiResponse::new_success_message(format!("Comment '{}' was created", new_comment.text.trim()));

            let json_string = serde_json::to_string(&json_response).unwrap();

            let created_response = Created::new("http://myservice.com/resource.json")
                .tagged_body(json_string);

            Ok(created_response)
        },
        Ok(_) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error("Unexpected result");

            Err(NotFound(Json(json_response)))
        },
        Err(err) => {
            let json_response: ApiResponse<String> = ApiResponse::new_error(format!("{}", err.to_string()));

            Err(NotFound(Json(json_response)))
        },
    }
}

#[put("/", data = "<comment>", format = "application/json")]
pub fn update_comment(comment: Json<UpdateCommentCommand>) -> Result<Accepted<Json<Comment>>, NotFound<String>> {
    
    let comment = UpdateCommentCommand{
        id: comment.id,
        text: comment.text.trim().to_string()
    };
    
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Update(comment),
    });

    match result {
        Ok(CommentResult::Comment(Some(comment))) => Ok(Accepted(Json(comment))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[delete("/", data = "<comment>", format = "application/json")]
pub fn delete_comment(comment: Json<DeleteComment>) ->  Result<Accepted<String>, NotFound<String>> {
    let result = comment_ops::handle_comment_command(CommentCommand {
        command: CommentSubcommand::Delete(DeleteComment {
            id: comment.id
        }),
    });

    match result {
        Ok(CommentResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}