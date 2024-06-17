use crate::utils::db::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;
use log::info;
use uuid::Uuid;
use crate::utils::models::product::{NewProduct, UpdateProduct, Product};
use crate::schema::products::dsl::*;
use crate::utils::args::commands::ProductCommand;
use crate::utils::args::sub_commands::product_commands::{ProductSubcommand, GetProductById as GetProductByIdCommand, CreateProduct as CreateProductCommand, UpdateProduct as UpdateProductCommand, DeleteProduct as DeleteProductCommand, ProductPagination as ProductPaginationCommand};

pub enum ProductResult {
    Product(Option<Product>),
    Products(Vec<Product>),
    Message(String),
}

pub fn handle_product_command(product: ProductCommand) -> Result<ProductResult, Error> {
    let connection = &mut establish_connection();
    let command = product.command;
    match command {
        ProductSubcommand::GetProductById(product) => show_product_by_id(product, connection).map(ProductResult::Product),
        ProductSubcommand::Create(product) => create_product(product, connection).map(ProductResult::Message),
        ProductSubcommand::Update(product) => update_product_by_id(product, connection).map(ProductResult::Product),
        ProductSubcommand::Delete(delete_entity) => delete_product_by_id(delete_entity, connection).map(ProductResult::Message),
        ProductSubcommand::Pagination(pagination) => product_pagination(pagination, connection).map(ProductResult::Products),
        ProductSubcommand::ShowAll => show_products(connection).map(ProductResult::Products),
    }
}

fn show_product_by_id(product: GetProductByIdCommand, connection: &mut PgConnection) -> Result<Option<Product>, Error> {
    info!("Showing product: {:?}", product);
    let product_result = products.find(product.id).filter(published.eq(true)).first(connection).optional();
    product_result
}

fn create_product(product: CreateProductCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Creating product: {:?}", product);

    let uuid = Uuid::now_v7();

    let new_product = NewProduct {
        id: &uuid,
        name: &product.name,
        url_name: &product.url_name,
        description: &product.description,
        image: product.image.as_deref(),
        brand_id: product.brand_id,
        category_id: product.category_id,
        published: true,
    };

    let result = diesel::insert_into(products).values(new_product).execute(connection);

    match result {
        Ok(_) => Ok(format!("Product created with id: {}", uuid)),
        Err(err) => Err(Error::QueryBuilderError(format!("Creating product error: {:?}", err).into())),
    }
}

fn update_product_by_id(product: UpdateProductCommand, connection: &mut PgConnection) -> Result<Option<Product>, Error> {
    info!("Updating product: {:?}", product);

    let update_product = UpdateProduct {
        id: &product.id,
        name: product.name.as_deref(),
        url_name: product.url_name.as_deref(),
        description: product.description.as_deref(),
        image: product.image.as_deref(),
        brand_id: product.brand_id,
        category_id: product.category_id,
        published: product.published,
    };

    let result = diesel::update(products.find(product.id)).set(update_product).returning(Product::as_returning()).get_result(connection).optional();
    result
}

fn delete_product_by_id(product: DeleteProductCommand, connection: &mut PgConnection) -> Result<String, Error> {
    info!("Deleting product: {:?}", product);

    let num_deleted = diesel::update(products.find(product.id).filter(published.eq(true))).set(published.eq(false)).execute(connection)?;

    match num_deleted {
        0 => Err(Error::NotFound),
        _ => Ok("Product deleted".to_string()),
    }
}

fn product_pagination(pagination: ProductPaginationCommand, connection: &mut PgConnection) -> Result<Vec<Product>, diesel::result::Error> {
    info!("Pagination: {:?}", pagination);

    let limit = pagination.limit.unwrap_or(10);
    let last_id = pagination.last_id;
    let order_by_desc = pagination.order_by_desc.unwrap_or(true);

    let mut query = products.filter(published.eq(true)).into_boxed();

    if let Some(last_id_value) = last_id {
        if order_by_desc {
            query = query.filter(id.lt(last_id_value));
        } else {
            query = query.filter(id.gt(last_id_value));
        }
    }

    query = if order_by_desc { query.order(created_at.desc()) } else { query.order(created_at.asc()) };

    query.limit(limit as i64).load::<Product>(connection)
}

fn show_products(connection: &mut PgConnection) -> Result<Vec<Product>, Error> {
    info!("Displaying all products");
    products.filter(published.eq(true)).order(id.desc()).load::<Product>(connection)
}
