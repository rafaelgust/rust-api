use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;

use diesel::dsl::exists;
use diesel::select;

use crate::utils::models::brand::{NewBrand, UpdateBrand, Brand};
use crate::schema::brands::dsl::*;

use crate::utils::args::commands::BrandCommand;
use crate::utils::args::sub_commands::brand_commands::{BrandSubcommand, 
    GetBrandByUrlName as GetBrandByUrlNameCommand, 
    CreateBrand as CreateBrandCommand, 
    UpdateBrand as UpdateBrandCommand, 
    DeleteBrand as DeleteBrandCommand,
    BrandPagination as BrandPaginationCommand
};

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
            show_brand_by_url_name(brand, connection).map(BrandResult::Brand)
        }
        BrandSubcommand::Create(brand) => {
            create_brand(brand, connection).map(BrandResult::Message)
        }
        BrandSubcommand::Update(brand) => {
            update_brand_by_id(brand, connection).map(BrandResult::Brand)
        }
        BrandSubcommand::Delete(delete_entity) => {
            delete_brand_by_id(delete_entity, connection).map(BrandResult::Message)
        }
        BrandSubcommand::Pagination(pagination) => {
            brand_pagination(pagination, connection).map(BrandResult::Brands)
        }
        BrandSubcommand::ShowAll => {
            show_brands(connection).map(BrandResult::Brands)
        }
    }
}

fn show_brand_by_url_name(brand: GetBrandByUrlNameCommand, connection: &mut PgConnection) -> Result<Option<Brand>, Error> {
    info!("Showing brand: {:?}", brand);
    
    let brand_result = brands
        .filter(url_name.eq(brand.url_name).and(published.eq(true)))
        .select(Brand::as_select())
        .first(connection)
        .optional();

    brand_result
}

fn create_brand(brand: CreateBrandCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating brand: {:?}", brand);

    // Check
     let exists_url_name: bool = select(exists(brands.filter(url_name.eq(&brand.url_name))))
        .get_result(connection)
        .map_err(|err| Error::from(err))?;

    if exists_url_name {
        return Err(Error::QueryBuilderError("Brand with this URL name already exists".into()));
    }

    let new_brand = NewBrand {
        name: &brand.name,
        url_name: &brand.url_name,
        description: &brand.description,
        published: &true,
    };

    let result = diesel::insert_into(brands)
                        .values(new_brand)
                        .execute(connection)
                        .optional();

    match result {
        Ok(brand) => Ok(format!("Creating brand: {:?}", brand)),
        Err(err) => Err(Error::QueryBuilderError(format!("Creating brand error: {:?}",err).into()))
    }
}

fn update_brand_by_id(brand: UpdateBrandCommand, connection: &mut PgConnection) -> Result<Option<Brand>, Error> {
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

fn delete_brand_by_id(brand: DeleteBrandCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting brand: {:?}", brand);

    let num_deleted = diesel::update(brands.find(brand.id).filter(published.eq(true)))
        .set(published.eq(false))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Brand deleted".to_string()),
    }
}

fn brand_pagination(pagination: BrandPaginationCommand, connection: &mut PgConnection) -> Result<Vec<Brand>, diesel::result::Error> {
    info!("Pagination: {:?}", pagination);

    let limit = pagination.limit.unwrap_or(10);
    let last_id = pagination.last_id;
    let order_by_desc = pagination.order_by_desc.unwrap_or(true);

    let mut query = brands
        .filter(published.eq(true))
        .into_boxed(); // Converts to a boxed query for conditional appending

    if let Some(last_id_value) = last_id {
        if order_by_desc {
            query = query.filter(id.lt(last_id_value));
        } else {
            query = query.filter(id.gt(last_id_value));
        }
    }

    query = if order_by_desc {
        query.order(id.desc())
    } else {
        query.order(id.asc())
    };

    query
        .limit(limit)
        .load::<Brand>(connection)
}

fn show_brands(connection: &mut PgConnection) -> Result<Vec<Brand>, Error> {
    info!("Displaying all brands");

    let result = brands
        .filter(published.eq(true))
        .order(id.desc())
        .load::<Brand>(connection);

    result
}