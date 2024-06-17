#[macro_use]
extern crate rocket;
extern crate diesel;

use utils::routers::brand;
use utils::routers::category;
use utils::routers::comment;
use utils::routers::product;

mod utils;
mod schema;

use rocket::Rocket;
use rocket::Build;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![index])
    .mount(brand::URI, routes![
        brand::get_brand, 
        brand::get_brands,
        brand::get_all_brands, 
        brand::new_brand, 
        brand::update_brand,
        brand::delete_brand,
        ])
    .mount(category::URI, routes![
        category::get_category, 
        category::get_all_categories, 
        category::new_category, 
        category::update_category,
        category::delete_category,
        ])
    .mount(comment::URI, routes![
        comment::get_comment_by_product_id, 
        comment::get_all_comments, 
        comment::get_comments, 
        comment::new_comment,
        comment::update_comment,
        comment::delete_comment,
        ])
    .mount(product::URI, routes![
        utils::routers::product::get_product_by_id, 
        utils::routers::product::get_products,
        utils::routers::product::get_all_products, 
        utils::routers::product::new_product, 
        utils::routers::product::update_product,
        utils::routers::product::delete_product,
        ])
}

#[rocket::main]
async fn main() {
    rocket().launch().await.expect("Failed to launch Rocket");
}