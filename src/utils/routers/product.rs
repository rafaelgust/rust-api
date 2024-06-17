use rocket::response::status::{Accepted, NotFound, Created};
use rocket::http::uri::Origin;
use rocket::serde::json::Json;
use crate::utils::response::ApiResponse;
use crate::utils::models::product::{ProductPaginationRequest, ProductResponse, DeleteProductRequest, InsertProductRequest, UpdateProductRequest};
use crate::utils::ops::product_ops::{self, ProductResult};
use crate::utils::args::commands::ProductCommand;
use crate::utils::args::sub_commands::product_commands::{ProductSubcommand, CreateProduct, DeleteProduct, GetProductById, UpdateProduct as UpdateProductCommand, ProductPagination};
use crate::utils::constants::{PRODUCT_NOT_FOUND, FETCH_ERROR, UNEXPECTED_RESULT};
use crate::utils::cryptography::{base32hex_to_uuid, uuid_to_base32hex};

pub const URI: Origin<'static> = uri!("/product");

#[get("/<id>", format = "application/json")]
pub fn get_product_by_id(id: &str) -> Result<Json<ProductResponse>, NotFound<String>> {
    let product_id_uuid = match base32hex_to_uuid(id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return Err(NotFound(PRODUCT_NOT_FOUND.to_string()));
        }
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::GetProductById(GetProductById {
            id: product_id_uuid
        }),
    });

    match result {
        Ok(ProductResult::Product(Some(product))) => Ok(Json(ProductResponse {
            id: uuid_to_base32hex(product.id),
            name: product.name,
            url_name: product.url_name,
            description: product.description,
            image: product.image,
            brand_id: product.brand_id,
            category_id: product.category_id,
            created_at: product.created_at,
            published: product.published
        })),
        Ok(_) => Err(NotFound(PRODUCT_NOT_FOUND.to_string())),
        Err(_) => Err(NotFound(FETCH_ERROR.to_string())),
    }
}

#[get("/", format = "application/json")]
pub fn get_all_products() -> Result<Json<ApiResponse<Vec<ProductResponse>>>, NotFound<String>> {
    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::ShowAll,
    });

    match result {
        Ok(ProductResult::Products(products)) => {
            let products_with_base32hex: Vec<ProductResponse> = products.into_iter().map(|product| ProductResponse {
                id: uuid_to_base32hex(product.id),
                name: product.name,
                url_name: product.url_name,
                description: product.description,
                image: product.image,
                brand_id: product.brand_id,
                category_id: product.category_id,
                created_at: product.created_at,
                published: product.published
            }).collect();

            Ok(Json(ApiResponse::new_success_data(products_with_base32hex)))
        },
        Ok(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(PRODUCT_NOT_FOUND.to_string())).unwrap())),
        Err(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(FETCH_ERROR.to_string())).unwrap())),
    }
}

#[post("/list", data = "<products>", format = "application/json")]
pub fn get_products(products: Json<ProductPaginationRequest>) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, NotFound<String>> {
    let last_id_uuid = match products.last_id {
        Some(ref last_id) => match base32hex_to_uuid(last_id) {
            Ok(uuid) => Some(uuid),
            Err(err) => {
                eprintln!("Error decoding base32hex to UUID: {}", err);
                return Err(NotFound(PRODUCT_NOT_FOUND.to_string()));
            }
        },
        None => None,
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Pagination(ProductPagination {
            limit: products.limit,
            last_id: last_id_uuid,
            order_by_desc: products.order_by_desc,
        }),
    });

    match result {
        Ok(ProductResult::Products(products)) => {
            let products_with_base32hex: Vec<ProductResponse> = products.into_iter().map(|product| ProductResponse {
                id: uuid_to_base32hex(product.id),
                name: product.name,
                url_name: product.url_name,
                description: product.description,
                image: product.image,
                brand_id: product.brand_id,
                category_id: product.category_id,
                created_at: product.created_at,
                published: product.published
            }).collect();

            Ok(Json(ApiResponse::new_success_data(products_with_base32hex)))
        },
        Ok(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(PRODUCT_NOT_FOUND.to_string())).unwrap())),
        Err(_) => Err(NotFound(serde_json::to_string(&ApiResponse::<String>::new_error(FETCH_ERROR.to_string())).unwrap())),
    }
}

#[post("/", data = "<new_product>", format = "application/json")]
pub fn new_product(new_product: Json<InsertProductRequest>) -> Result<Created<String>, NotFound<Json<ApiResponse<String>>>> {
    let product = CreateProduct {
        name: new_product.name.trim().to_string(),
        url_name: new_product.url_name.trim().to_string(),
        description: new_product.description.trim().to_string(),
        image: new_product.image.clone(),
        brand_id: new_product.brand_id,
        category_id: new_product.category_id,
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Create(product),
    });

    match result {
        Ok(ProductResult::Message(_)) => {
            let json_response = ApiResponse::<String>::new_success_message(format!("Product '{}' was created", new_product.name.trim()));
            let json_string = serde_json::to_string(&json_response).unwrap();
            let created_response = Created::new("http://myservice.com/resource.json").tagged_body(json_string);

            Ok(created_response)
        },
        Ok(_) => Err(NotFound(Json(ApiResponse::new_error(UNEXPECTED_RESULT.to_string())))),
        Err(err) => Err(NotFound(Json(ApiResponse::new_error(format!("{}", err))))),
    }
}

#[put("/", data = "<product>", format = "application/json")]
pub fn update_product(product: Json<UpdateProductRequest>) -> Result<Accepted<Json<ProductResponse>>, NotFound<String>> {
    let product_id_uuid = match base32hex_to_uuid(&product.id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return Err(NotFound(FETCH_ERROR.to_string()));
        }
    };

    let update_product = UpdateProductCommand {
        id: product_id_uuid,
        name: product.name.as_deref().map(String::from),
        url_name: product.url_name.as_deref().map(String::from),
        description: product.description.as_deref().map(String::from),
        image: product.image.clone(),
        brand_id: product.brand_id,
        category_id: product.category_id,
        published: product.published,
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Update(update_product),
    });

    match result {
        Ok(ProductResult::Product(Some(product))) => Ok(Accepted(Json(ProductResponse {
            id: uuid_to_base32hex(product.id),
            name: product.name,
            url_name: product.url_name,
            description: product.description,
            image: product.image,
            brand_id: product.brand_id,
            category_id: product.category_id,
            created_at: product.created_at,
            published: product.published
        }))),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}

#[delete("/", data = "<product>", format = "application/json")]
pub fn delete_product(product: Json<DeleteProductRequest>) -> Result<Accepted<String>, NotFound<String>> {
    let product_id_uuid = match base32hex_to_uuid(&product.id) {
        Ok(uuid) => uuid,
        Err(err) => {
            eprintln!("Error decoding base32hex to UUID: {}", err);
            return Err(NotFound(FETCH_ERROR.to_string()));
        }
    };

    let result = product_ops::handle_product_command(ProductCommand {
        command: ProductSubcommand::Delete(DeleteProduct { 
            id: product_id_uuid 
        }),
    });

    match result {
        Ok(ProductResult::Message(msg)) => Ok(Accepted(msg)),
        Ok(_) => Err(NotFound(UNEXPECTED_RESULT.to_string())),
        Err(err) => Err(NotFound(err.to_string())),
    }
}
