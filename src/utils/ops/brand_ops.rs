use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;

use crate::utils::models::brand::{NewBrand, UpdateBrand, Brand};
use crate::schema::brands::dsl::*;

use crate::utils::args::commands::BrandCommand;
use crate::utils::args::sub_commands::brand_commands::{BrandSubcommand, GetBrand as GetBrandCommand, CreateBrand as CreateBrandCommand, UpdateBrand as UpdateBrandCommand, DeleteBrand as DeleteBrandCommand};

pub enum BrandResult {
    Brand(Option<Brand>),
    Brands(Vec<Brand>),
    Message(String),
}

pub fn handle_brand_command(brand: BrandCommand) -> Result<BrandResult, Error> {
    let connection = &mut establish_connection();
    let command = brand.command;
    match command {
        BrandSubcommand::Show(brand) => {
            show_brand_by_id(brand, connection).map(BrandResult::Brand)
        }
        BrandSubcommand::Create(brand) => {
            create_brand(brand, connection).map(BrandResult::Message)
        }
        BrandSubcommand::Update(brand) => {
            update_brand(brand, connection).map(BrandResult::Brand)
        }
        BrandSubcommand::Delete(delete_entity) => {
            delete_brand(delete_entity, connection).map(BrandResult::Message)
        }
        BrandSubcommand::ShowAll => {
            show_brands(connection).map(BrandResult::Brands)
        }
    }
}

fn show_brand_by_id(brand: GetBrandCommand, connection: &mut PgConnection) -> Result<Option<Brand>, Error> {
    info!("Showing brand: {:?}", brand);
    
    let brand_result = brands
        .filter(id.eq(brand.id))
        .select(Brand::as_select())
        .first(connection)
        .optional();

    brand_result
}

fn create_brand(brand: CreateBrandCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating brand: {:?}", brand);

    let new_brand = NewBrand {
        name: &brand.name,
        url_name: &brand.url_name,
        description: &brand.description,
        published: &brand.published,
    };

    let result = diesel::insert_into(brands)
                    .values(new_brand)
                    .execute(connection)
                    .optional();

    match result {
        Ok(brand) => Ok(format!("Creating brand: {:?}", brand)),
        Err(err) => Err(err),
    }
}

fn update_brand(brand: UpdateBrandCommand, connection: &mut PgConnection) -> Result<Option<Brand>, Error> {
    info!("Updating brand: {:?}", brand);

    let update_brand = UpdateBrand {
        id: &brand.id,
        name: Some(&brand.name),
        url_name: Some(&brand.url_name), 
        description: Some(&brand.description),
        published: Some(&brand.published),
    };

    let result = diesel::update(brands.find(brand.id))
        .set(update_brand)
        .returning(Brand::as_returning())
        .get_result(connection)
        .optional();

    match result {
        Ok(brand) => Ok(brand),
        Err(err) => Err(err),
    }
}

fn delete_brand(brand: DeleteBrandCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting brand: {:?}", brand);

    let num_deleted = diesel::update(brands.find(brand.id))
        .set(published.eq(brand.published))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Brand deleted".to_string()),
    }
}

fn show_brands(connection: &mut PgConnection) -> Result<Vec<Brand>, Error> {
    info!("Displaying all brands");

    let result = brands
        .load::<Brand>(connection);

    result
}