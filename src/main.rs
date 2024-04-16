#[macro_use]
extern crate rocket;
extern crate diesel;

use utils::routers::brand;
use utils::routers::category;

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
}

#[rocket::main]
async fn main() {
    rocket().launch().await.expect("Failed to launch Rocket");
}