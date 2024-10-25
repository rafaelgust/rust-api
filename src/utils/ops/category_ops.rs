use crate::utils::db::establish_connection;
use diesel::{prelude::*, select};

use diesel::result::Error;
use log::info;

use diesel::dsl::exists;

use crate::utils::models::category::{NewCategory, UpdateCategory, Category};
use crate::schema::categories::dsl::*;

use crate::utils::args::commands::CategoryCommand;
use crate::utils::args::sub_commands::category_commands::{
    CategorySubcommand, 
    GetCategoryByUrlName as GetCategoryByUrlNameCommand, 
    GetCategoryByName as GetCategoryByNameCommand,
    CreateCategory as CreateCategoryCommand, 
    UpdateCategory as UpdateCategoryCommand, 
    DeleteCategory as DeleteCategoryCommand
};

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
            show_category_by_url_name(category, connection).map(CategoryResult::Category)
        }
        CategorySubcommand::GetCategoryByName(category) => {
            show_category_by_name(category, connection).map(CategoryResult::Categories)
        }
        CategorySubcommand::Create(category) => {
            create_category(category, connection).map(CategoryResult::Message)
        }
        CategorySubcommand::Update(category) => {
            update_category_by_id(category, connection).map(CategoryResult::Category)
        }
        CategorySubcommand::Delete(delete_entity) => {
            delete_category_by_id(delete_entity, connection).map(CategoryResult::Message)
        }
        CategorySubcommand::ShowAll => {
            show_categories(connection).map(CategoryResult::Categories)
        }
    }
}

fn show_category_by_url_name(category: GetCategoryByUrlNameCommand, connection: &mut PgConnection) -> Result<Option<Category>, Error> {
    info!("Showing Category: {:?}", category);
    
    let category_result = categories
        .filter(url_name.eq(category.url_name).and(published.eq(true)))
        .select(Category::as_select())
        .first(connection)
        .optional();

    category_result
}

fn show_category_by_name(brand: GetCategoryByNameCommand, connection: &mut PgConnection) -> Result<Vec<Category>, Error> {
    info!("Showing Category: {:?}", brand);
        
    let search_term = brand.name.trim().to_lowercase();
    let search_pattern = format!("%{}%", search_term);
    
    let query = categories
        .filter(name.ilike(search_pattern).and(published.eq(true)))
        .order(name.desc())
        .load::<Category>(connection);

    query
}

fn create_category(category: CreateCategoryCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating Category: {:?}", category);

    // Check
    let exists_url_name: bool = select(exists(categories.filter(url_name.eq(&category.url_name))))  
        .get_result(connection)
        .map_err(|err| Error::from(err))?;

    if exists_url_name {
        return Err(Error::QueryBuilderError("Category with this URL name already exists".into()));
    }

    let new_category = NewCategory {
        name: &category.name,
        url_name: &category.url_name,
        description: &category.description,
        published: &true,
    };

    let result = diesel::insert_into(categories)
                        .values(new_category)
                        .execute(connection)
                        .optional();

    match result {
        Ok(brand) => Ok(format!("Creating category: {:?}", brand)),
        Err(err) => Err(Error::QueryBuilderError(format!("Creating category error: {:?}",err).into()))
    }
}

fn update_category_by_id(category: UpdateCategoryCommand, connection: &mut PgConnection) -> Result<Option<Category>, Error> {
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

fn delete_category_by_id(category: DeleteCategoryCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting Category: {:?}", category);

    let num_deleted = diesel::update(categories.find(category.id).filter(published.eq(true)))
        .set(published.eq(false))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Category deleted".to_string()),
    }
}

fn show_categories(connection: &mut PgConnection) -> Result<Vec<Category>, Error> {
    info!("Displaying all Categorys");

    let result = categories
        .filter(published.eq(true))
        .order(id.desc())
        .load::<Category>(connection);

    result
}