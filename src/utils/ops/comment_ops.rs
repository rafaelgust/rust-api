use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;

use uuid::Uuid;

use crate::utils::models::comment::{NewComment, UpdateComment, Comment};
use crate::schema::comments::dsl::*;

use crate::utils::args::commands::CommentCommand;
use crate::utils::args::sub_commands::comment_commands::{CommentSubcommand, 
    GetCommentByProductId as GetCommentByProductIdCommand,
    GetCommentById as GetCommentByIdCommand,
    CreateComment as CreateCommentCommand, 
    UpdateComment as UpdateCommentCommand, 
    DeleteComment as DeleteCommentCommand,
    CommentPagination as CommentPaginationCommand
};

pub enum CommentResult {
    Comment(Option<Comment>),
    Comments(Vec<Comment>),
    Message(String),
}

pub fn handle_comment_command(comment: CommentCommand) -> Result<CommentResult, Error> {
    let connection = &mut establish_connection();
    let command = comment.command;
    match command {
        CommentSubcommand::GetCommentByProductId(comment) => {
            show_comments_by_product_id(comment, connection).map(CommentResult::Comments)
        }
        CommentSubcommand::GetCommentById(comment) => {
            show_comment_by_id(comment, connection).map(CommentResult::Comment)
        }
        CommentSubcommand::Create(comment) => {
            create_comment(comment, connection).map(CommentResult::Message)
        }
        CommentSubcommand::Update(comment) => {
            update_comment_by_id(comment, connection).map(CommentResult::Comment)
        }
        CommentSubcommand::Delete(delete_entity) => {
            delete_comment_by_id(delete_entity, connection).map(CommentResult::Message)
        }
        CommentSubcommand::Pagination(pagination) => {
            comment_pagination(pagination, connection).map(CommentResult::Comments)
        }
        CommentSubcommand::ShowAll => {
            show_comments(connection).map(CommentResult::Comments)
        }
    }
}

fn show_comments_by_product_id(comment: GetCommentByProductIdCommand, connection: &mut PgConnection) -> Result<Vec<Comment>, Error> {
    info!("Showing comment: {:?}", comment);
    
    //select no comentario pelo id retornado apenas ele

    let result = comments
        .filter(product_id.eq(comment.product_id))
        .filter(published.eq(true))
        .order(id.desc())
        .load::<Comment>(connection);

    result
}

fn show_comment_by_id(comment: GetCommentByIdCommand, connection: &mut PgConnection) -> Result<Option<Comment>, Error> {
    info!("Showing comment: {:?}", comment);

    let result = comments
        .filter(id.eq(comment.id))
        .filter(published.eq(true))
        .first::<Comment>(connection)
        .optional();

    match result {
        Ok(comment) => Ok(comment),
        Err(err) => Err(err),
    }
}

fn create_comment(comment: CreateCommentCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating comment: {:?}", comment);

    let uuid = Uuid::now_v7();

    let new_comment = NewComment {
        id: &uuid,
        text: &comment.text,
        product_id: &comment.product_id,
        user_id: &comment.user_id,
        published: true,
    };

    match diesel::insert_into(comments)
        .values(&new_comment)
        .execute(connection)
    {
        Ok(_) => {
            let success_message = format!("Comment created successfully!");
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Error creating comment: {}", e);
            Err(Error::QueryBuilderError(error_message.into()))
        }
    }
}

fn update_comment_by_id(comment: UpdateCommentCommand, connection: &mut PgConnection) -> Result<Option<Comment>, Error> {
    info!("Updating comment: {:?}", comment);

    let update_comment = UpdateComment {
        id: &comment.id,
        text: &comment.text
    };

    let result = diesel::update(comments.find(comment.id))
        .set(update_comment)
        .returning(Comment::as_returning())
        .get_result(connection)
        .optional();

    match result {
        Ok(comment) => Ok(comment),
        Err(err) => Err(err),
    }
}

fn delete_comment_by_id(comment: DeleteCommentCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting comment: {:?}", comment);

    let num_deleted = diesel::update(comments.find(comment.id).filter(published.eq(true)))
        .set(published.eq(false))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Comment deleted".to_string()),
    }
}

fn comment_pagination(pagination: CommentPaginationCommand, connection: &mut PgConnection) -> Result<Vec<Comment>, diesel::result::Error> {
    info!("Pagination: {:?}", pagination);

    let limit = pagination.limit.unwrap_or(10);
    let last_id = pagination.last_id;
    let order_by_desc = pagination.order_by_desc.unwrap_or(true);

    let mut query = comments
        .filter(published.eq(true))
        .into_boxed();
    
    if let Some(last_id_value) = last_id {
        if order_by_desc {
            query = query.filter(id.lt(last_id_value));
        } else {
            query = query.filter(id.gt(last_id_value));
        }
    }

    query = if order_by_desc {
        query.order(created_at.desc())
    } else {
        query.order(created_at.asc())
    };

    query
        .limit(limit as i64)
        .load::<Comment>(connection)
}

fn show_comments(connection: &mut PgConnection) -> Result<Vec<Comment>, Error> {
    info!("Displaying all comments");

    let result = comments
        .filter(published.eq(true))
        .order(id.desc())
        .load::<Comment>(connection);

    result
}