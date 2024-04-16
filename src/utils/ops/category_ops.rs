use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;

use crate::utils::models::category::{NewCategory, UpdateCategory, Category};
use crate::schema::categories::dsl::*;

use crate::utils::args::commands::CategoryCommand;
use crate::utils::args::sub_commands::category_commands::{CategorySubcommand, GetCategory as GetCategoryCommand, CreateCategory as CreateCategoryCommand, UpdateCategory as UpdateCategoryCommand, DeleteCategory as DeleteCategoryCommand};

pub enum CategoryResult {
    Category(Option<Category>),
    Categories(Vec<Category>),
    Message(String),
}

pub fn handle_category_command(category: CategoryCommand) -> Result<CategoryResult, Error> {
    let connection = &mut establish_connection();
    let command = category.command;
    match command {
        CategorySubcommand::Show(category) => {
            show_category_by_id(category, connection).map(CategoryResult::Category)
        }
        CategorySubcommand::Create(category) => {
            create_category(category, connection).map(CategoryResult::Message)
        }
        CategorySubcommand::Update(category) => {
            update_category(category, connection).map(CategoryResult::Category)
        }
        CategorySubcommand::Delete(delete_entity) => {
            delete_category(delete_entity, connection).map(CategoryResult::Message)
        }
        CategorySubcommand::ShowAll => {
            show_categories(connection).map(CategoryResult::Categories)
        }
    }
}

fn show_category_by_id(category: GetCategoryCommand, connection: &mut PgConnection) -> Result<Option<Category>, Error> {
    info!("Showing Category: {:?}", category);
    
    let category_result = categories
        .filter(id.eq(category.id))
        .select(Category::as_select())
        .first(connection)
        .optional();

    category_result
}

fn create_category(category: CreateCategoryCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating Category: {:?}", category);

    let new_category = NewCategory {
        name: &category.name,
        url_name: &category.url_name,
        description: &category.description,
        published: &category.published,
    };

    let result = diesel::insert_into(categories)
                    .values(new_category)
                    .execute(connection)
                    .optional();

    match result {
        Ok(category) => Ok(format!("Creating Category: {:?}", category)),
        Err(err) => Err(err),
    }
}

fn update_category(category: UpdateCategoryCommand, connection: &mut PgConnection) -> Result<Option<Category>, Error> {
    info!("Updating Category: {:?}", category);

    let update_category = UpdateCategory {
        id: &category.id,
        name: Some(&category.name),
        url_name: Some(&category.url_name), 
        description: Some(&category.description),
        published: Some(&category.published),
    };

    let result = diesel::update(categories.find(category.id))
        .set(update_category)
        .returning(Category::as_returning())
        .get_result(connection)
        .optional();

    match result {
        Ok(category) => Ok(category),
        Err(err) => Err(err),
    }
}

fn delete_category(category: DeleteCategoryCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting Category: {:?}", category);

    let num_deleted = diesel::update(categories.find(category.id))
        .set(published.eq(category.published))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Category deleted".to_string()),
    }
}

fn show_categories(connection: &mut PgConnection) -> Result<Vec<Category>, Error> {
    info!("Displaying all Categorys");

    let result = categories
        .load::<Category>(connection);

    result
}