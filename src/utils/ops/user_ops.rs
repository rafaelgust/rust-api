use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;
use uuid::Uuid;
use crate::utils::models::user::user::{NewUser, UpdateUser, User};
use crate::schema::users::dsl::*;
use crate::utils::args::commands::UserCommand;
use crate::utils::args::sub_commands::user_commands::{UserSubcommand, Auth as GetUserByEmailCommand, CreateUser as CreateUserCommand, UpdateUser as UpdateUserCommand, DeleteUser as DeleteUserCommand};

pub enum UserResult {
    User(Option<User>),
    Message(String),
}

pub fn handle_user_command(user: UserCommand) -> Result<UserResult, Error> {
    let connection = &mut establish_connection();
    let command = user.command;
    match command {
        UserSubcommand::GetUserByEmail(user) => show_user_by_email(user, connection).map(UserResult::User),
        UserSubcommand::Create(user) => create_user(user, connection).map(UserResult::Message),
        UserSubcommand::Update(user) => update_user_by_id(user, connection).map(UserResult::User),
        UserSubcommand::Delete(delete_entity) => delete_user_by_id(delete_entity, connection).map(UserResult::Message),
    }
}

fn show_user_by_email(user: GetUserByEmailCommand, connection: &mut PgConnection) -> Result<Option<User>, Error> {
    info!("Showing user: {:?}", user);
    
    let user_result = users
        .filter(email.eq(user.email).and(published.eq(true)))
        .first::<User>(connection)
        .optional();
        
    user_result
}

fn create_user(user: CreateUserCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating user: {:?}", user);

    let uuid = Uuid::now_v7();

    let new_user = NewUser {
        id: &uuid,
        username: &user.username,
        email: &user.email,
        password: &user.password_hash,
        role_id: &user.role_id,
        published: true,
    };

    let result = diesel::insert_into(users).values(new_user).execute(connection);

    match result {
        Ok(_) => Ok(format!("User created with id: {}", uuid)),
        Err(err) => Err(Error::QueryBuilderError(format!("Creating user error: {:?}", err).into())),
    }
}

fn update_user_by_id(user: UpdateUserCommand, connection: &mut PgConnection) -> Result<Option<User>, Error> {
    info!("Updating user: {:?}", user);

    let update_user = UpdateUser {
        id: &user.id,
        username: user.username.as_deref(),
        email: user.email.as_deref(),
        password: user.password_hash.as_deref(),
        role_id: user.role_id,
        published: user.published,
    };

    let result = diesel::update(users.find(user.id)).set(update_user).returning(User::as_returning()).get_result(connection).optional();
    result
}

fn delete_user_by_id(user: DeleteUserCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting user: {:?}", user);

    let num_deleted = diesel::update(users.find(user.id).filter(published.eq(true))).set(published.eq(false)).execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("User deleted".to_string()),
    }
}
