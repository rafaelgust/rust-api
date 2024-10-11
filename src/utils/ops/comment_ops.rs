use crate::schema::{users, comments};

use crate::utils::db::establish_connection;
use crate::utils::models::user::user::User;
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
    Comment(Option<(Comment, User)>),
    Comments(Vec<Option<(Comment, User)>>),
    Message(String),
}

pub fn handle_comment_command(comment: CommentCommand) -> Result<CommentResult, Error> {
    let connection = &mut establish_connection();
    let command = comment.command;

    match command {
        CommentSubcommand::GetCommentByProductId(comment) => 
            show_comments_by_product_id(comment, connection).map(CommentResult::Comments),
        
        CommentSubcommand::GetCommentById(comment) => {
            show_comment_by_id(comment, connection).map(CommentResult::Comment)
        }
        CommentSubcommand::Create(comment) => {
            create_comment(comment, connection).map(|msg| CommentResult::Message(msg))
        }
        CommentSubcommand::Update(comment) => {
            update_comment_by_id(comment, connection).map(|msg| CommentResult::Message(msg))
        }
        CommentSubcommand::Delete(delete_entity) => {
            delete_comment_by_id(delete_entity, connection).map(|msg| CommentResult::Message(msg))
        }
        CommentSubcommand::Pagination(pagination) => {
            comment_pagination(pagination, connection).map(CommentResult::Comments)
        }
        CommentSubcommand::ShowAll => {
            show_comments(connection).map(CommentResult::Comments)
        }
    }
}

fn show_comment_by_id(comment: GetCommentByIdCommand, connection: &mut PgConnection) -> Result<Option<(Comment, User)>, Error> {
    info!("Showing comment: {:?}", comment);

    let result = comments::table
        .left_join(users::table)
        .filter(comments::id.eq(comment.id))
        .filter(comments::published.eq(true))
        .select((comments::all_columns, users::all_columns.nullable()))
        .first::<(Comment, Option<User>)>(connection)
        .optional()?;

    Ok(result.and_then(|(comment, user)| user.map(|user| (comment, user))))
}

fn show_comments_by_product_id(
    comment: GetCommentByProductIdCommand,
    connection: &mut PgConnection
) -> Result<Vec<Option<(Comment, User)>>, Error> {
    info!("Showing comments for product ID: {:?}", comment.product_id);
    
    let mut query = comments::table
        .left_join(users::table)
        .filter(comments::product_id.eq(&comment.product_id))
        .filter(comments::published.eq(true))
        .limit(10 as i64)
        .into_boxed();

    if comment.order_is_desc {
        query = query.order(comments::created_at.desc());
    } else {
        query = query.order(comments::created_at.asc());
    }

    let comments_result = query
        .select((comments::all_columns, users::all_columns.nullable()))
        .load::<(Comment, Option<User>)>(connection)?;

    Ok(comments_result.into_iter().map(|(comment, user)| {
        user.map(|user| (comment, user))
    }).collect())
}

fn show_comments(connection: &mut PgConnection) -> Result<Vec<Option<(Comment, User)>>, Error> {
    info!("Displaying all comments");

    let results = comments::table
        .left_join(users::table)
        .filter(comments::published.eq(true))
        .order(comments::id.desc())
        .select((comments::all_columns, users::all_columns.nullable()))
        .load::<(Comment, Option<User>)>(connection)?;

    Ok(results.into_iter().map(|(comment, user)| {
        user.map(|user| (comment, user))
    }).collect())

}

fn comment_pagination(
    pagination: CommentPaginationCommand, 
    connection: &mut PgConnection
) -> Result<Vec<Option<(Comment, User)>>, diesel::result::Error> {
    info!("Pagination: {:?}", pagination);

    let limit = pagination.limit.unwrap_or(10);
    let product = pagination.product_id.unwrap();
    let last_id = pagination.last_id;
    let order_by_desc = pagination.order_by_desc.unwrap_or(true);

    let mut query = comments::table
        .left_join(users::table)
        .filter(comments::published.eq(true))
        .filter(comments::product_id.eq(product))
        .select((comments::all_columns, users::all_columns.nullable()))
        .into_boxed();

    if let Some(last_id_value) = last_id {
        if order_by_desc {
            query = query.filter(comments::id.lt(last_id_value));
        } else {
            query = query.filter(comments::id.gt(last_id_value));
        }
    }

    query = if order_by_desc {
        query.order(comments::created_at.desc())
    } else {
        query.order(comments::created_at.asc())
    };

    let results = query
        .limit(limit as i64)
        .load::<(Comment, Option<User>)>(connection)?;

    Ok(results.into_iter().map(|(comment, user)| {
        user.map(|user| (comment, user))
    }).collect())
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

fn update_comment_by_id(comment: UpdateCommentCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Updating comment: {:?}", comment);

    let update_comment = UpdateComment {
        id: &comment.id,
        text: &comment.text,
    };

    match diesel::update(comments::table.find(comment.id))
        .set(update_comment)
        .execute(connection) 
    {
        Ok(0) => Err(Error::NotFound), // No comment found
        Ok(_) => {
            let success_message = format!("Comment updated successfully!");
            Ok(success_message)
        },
        Err(e) => {
            let error_message = format!("Error updating comment: {}", e);
            Err(Error::QueryBuilderError(error_message.into()))
        },
    }
}


fn delete_comment_by_id(comment: DeleteCommentCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting comment: {:?}", comment);

    let num_deleted = diesel::update(comments::table.find(comment.id).filter(comments::published.eq(true)))
        .set(comments::published.eq(false))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Comment deleted".to_string()),
    }
}



