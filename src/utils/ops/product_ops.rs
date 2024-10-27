use crate::utils::db::establish_connection;
use crate::utils::models::brand::Brand;
use crate::schema::{brands, categories, product_categories, products};
use crate::utils::models::category::Category;
use crate::utils::models::product_categories::ProductCategory;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;
use uuid::Uuid;
use crate::utils::models::product::{NewProduct, UpdateProduct, Product};
use crate::utils::args::commands::ProductCommand;
use crate::utils::args::sub_commands::product_commands::{
    ProductSubcommand, 
    GetProductById as GetProductByIdCommand, 
    GetProductByUrlName as GetProductByUrlNameCommand, 
    GetProductByName as GetProductByNameCommand,
    CreateProduct as CreateProductCommand, 
    UpdateProduct as UpdateProductCommand, 
    DeleteProduct as DeleteProductCommand, 
    ProductPagination as ProductPaginationCommand
};

pub enum ProductResult {
    Product(Option<(Product, Option<Brand>, Vec<Category>)>),
    Products(Vec<Option<(Product, Option<Brand>, Vec<Category>)>>),
    Message(String),
}

pub fn handle_product_command(product: ProductCommand) -> Result<ProductResult, Error> {
    let connection = &mut establish_connection();
    let command = product.command;
    match command {
        ProductSubcommand::GetProductById(product) => show_product_by_id(product, connection).map(ProductResult::Product),
        ProductSubcommand::GetProductByUrlName(product) => show_product_by_url_name(product, connection).map(ProductResult::Product),
        ProductSubcommand::GetProductByName(product) => show_product_by_name(product, connection).map(ProductResult::Products),
        ProductSubcommand::Create(product) => create_product(product, connection).map(ProductResult::Message),
        ProductSubcommand::Update(product) => update_product_by_id(product, connection).map(ProductResult::Message),
        ProductSubcommand::Delete(delete_entity) => delete_product_by_id(delete_entity, connection).map(ProductResult::Message),
        ProductSubcommand::Pagination(pagination) => product_pagination(pagination, connection).map(ProductResult::Products),
        ProductSubcommand::ShowAll => show_products(connection).map(ProductResult::Products),
    }
}

fn show_product_by_url_name(product: GetProductByUrlNameCommand, connection: &mut PgConnection) -> Result<Option<(Product, Option<Brand>, Vec<Category>)>, Error> {
    info!("Showing product: {:?}", product);

    let product_result = products::table
        .left_join(brands::table)
        .filter(products::url_name.eq(&product.url_name))
        .filter(products::published.eq(true))
        .select((products::all_columns, brands::all_columns.nullable()))
        .first::<(Product, Option<Brand>)>(connection)
        .optional()?;
    
    if let Some((product, brand)) = product_result {
        let category_ids: Vec<i32> = ProductCategory::belonging_to(&product)
            .select(product_categories::category_id)
            .load::<i32>(connection)?;

        let categories = categories::table
            .filter(categories::id.eq_any(category_ids))
            .load::<Category>(connection)?;

        Ok(Some((product, brand, categories)))
    } else {
        Ok(None)
    }
}

fn show_product_by_name(product: GetProductByNameCommand, connection: &mut PgConnection) -> Result<Vec<Option<(Product, Option<Brand>, Vec<Category>)>>, Error> {
    info!("Showing product: {:?}", product);
    
    let mut result = Vec::new();
    
    let search_term = product.name.trim().to_lowercase();
    let search_pattern = format!("%{}%", search_term);
    
    let product_brands: Vec<(Product, Option<Brand>)> = products::table
        .left_join(brands::table)
        .filter(products::name.ilike(search_pattern))
        .filter(products::published.eq(true))
        .select((products::all_columns, brands::all_columns.nullable()))
        .order(products::name.desc())
        .limit(30)
        .load(connection)?;
        
        for (product, brand) in product_brands {
            let category_ids: Vec<i32> = ProductCategory::belonging_to(&product)
                .select(product_categories::category_id)
                .load::<i32>(connection)?;
    
            let categories = categories::table
                .filter(categories::id.eq_any(category_ids))
                .load::<Category>(connection)?;
    
            result.push(Some((product, brand, categories)));
        }
    
    Ok(result)
}

fn show_product_by_id(product: GetProductByIdCommand, connection: &mut PgConnection) -> Result<Option<(Product, Option<Brand>, Vec<Category>)>, Error> {
    info!("Showing product: {:?}", product);
    let product_result = products::table
        .left_join(brands::table)
        .filter(products::id.eq(product.id))
        .filter(products::published.eq(true))
        .select((products::all_columns, brands::all_columns.nullable()))
        .first::<(Product, Option<Brand>)>(connection)
        .optional()?;

    if let Some((product, brand)) = product_result {
        let category_ids: Vec<i32> = ProductCategory::belonging_to(&product)
            .select(product_categories::category_id)
            .load::<i32>(connection)?;

        let categories = categories::table
            .filter(categories::id.eq_any(category_ids))
            .load::<Category>(connection)?;

        Ok(Some((product, brand, categories)))
    } else {
        Ok(None)
    }
}

fn create_product(
    product: CreateProductCommand, 
    connection: &mut PgConnection
) -> Result<String, Error> {
    info!("Creating product: {:?}", product);

    let uuid = Uuid::now_v7();

    let new_product = NewProduct {
        id: &uuid,
        name: &product.name,
        url_name: &product.url_name,
        description: &product.description,
        image: product.image.as_deref(),
        brand_id: product.brand_id,
        published: true,
    };

    match diesel::insert_into(products::table)
        .values(&new_product)
        .execute(connection)
    {
        Ok(_) => {
            let success_message = format!("Product created successfully!");
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Error creating product: {}", e);
            Err(Error::QueryBuilderError(error_message.into()))
        }
    }
}

fn update_product_by_id(
    product: UpdateProductCommand, 
    connection: &mut PgConnection
) -> Result<String, Error> {
    info!("Updating product: {:?}", product);

    let update_product = UpdateProduct {
        id: &product.id,
        name: product.name.as_deref(),
        url_name: product.url_name.as_deref(),
        description: product.description.as_deref(),
        image: product.image.as_deref(),
        brand_id: product.brand_id,
        published: product.published,
    };

    connection.transaction(|conn| {
        let updated_product = diesel::update(products::table.find(product.id))
            .set(update_product)
            .returning(Product::as_returning())
            .get_result(conn)
            .optional()?;

        match updated_product {
            Some(_) => Ok(format!("Product Updated: {:?}", product.name)),
            None => Err(Error::NotFound),
        }
    })
}

fn delete_product_by_id(
    product: DeleteProductCommand, 
    connection: &mut PgConnection
) -> Result<String, Error> {
    info!("Deleting product: {:?}", product);

    let num_deleted = diesel::update(products::table.find(product.id).filter(products::published.eq(true)))
        .set(products::published.eq(false))
        .execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Product deleted".to_string()),
    }
}

fn product_pagination(
    pagination: ProductPaginationCommand, 
    connection: &mut PgConnection
) -> Result<Vec<Option<(Product, Option<Brand>, Vec<Category>)>>, Error> {
    info!("Pagination: {:?}", pagination);

    let limit = pagination.limit.unwrap_or(10);
    let last_id = pagination.last_id;
    let order_by_desc = pagination.order_by_desc.unwrap_or(true);

    let mut query = products::table
        .left_join(brands::table)
        .filter(products::published.eq(true))
        .select((products::all_columns, brands::all_columns.nullable()))
        .into_boxed();

    if let Some(last_id_value) = last_id {
        if order_by_desc {
            query = query.filter(products::id.lt(last_id_value));
        } else {
            query = query.filter(products::id.gt(last_id_value));
        }
    }

    query = if order_by_desc {
        query.order(products::created_at.desc())
    } else {
        query.order(products::created_at.asc())
    };

    let product_brands: Vec<(Product, Option<Brand>)> = query.limit(limit as i64).load(connection)?;

    let mut result = Vec::new();
    for (product, brand) in product_brands {
        let category_ids: Vec<i32> = ProductCategory::belonging_to(&product)
            .select(product_categories::category_id)
            .load::<i32>(connection)?;

        let categories = categories::table
            .filter(categories::id.eq_any(category_ids))
            .load::<Category>(connection)?;

        result.push(Some((product, brand, categories)));
    }

    Ok(result)
}

fn show_products(connection: &mut PgConnection) -> Result<Vec<Option<(Product, Option<Brand>, Vec<Category>)>>, Error> {
    info!("Displaying all products");

    let product_brands: Vec<(Product, Option<Brand>)> = products::table
        .left_join(brands::table)
        .filter(products::published.eq(true))
        .select((products::all_columns, brands::all_columns.nullable()))
        .order(products::id.desc())
        .load(connection)?;

    let mut result = Vec::new();
    for (product, brand) in product_brands {
        let category_ids: Vec<i32> = ProductCategory::belonging_to(&product)
            .select(product_categories::category_id)
            .load::<i32>(connection)?;

        let categories = categories::table
            .filter(categories::id.eq_any(category_ids))
            .load::<Category>(connection)?;

        result.push(Some((product, brand, categories)));
    }

    Ok(result)
}