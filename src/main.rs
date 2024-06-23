#![windows_subsystem = "windows"]

use axum::{routing::get, Router};

use tokio::net::TcpListener;

use utils::routers::brand::get_brand_routes;
use utils::routers::category::get_category_routes;
use utils::routers::comment::get_comment_routes;
use utils::routers::product::get_product_routes;

mod utils;
mod schema;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Unable to connect to the server");

    let routes = Router::new()
    .route("/", get(root))
    .merge(get_brand_routes())
    .merge(get_category_routes())
    .merge(get_comment_routes())
    .merge(get_product_routes());
    
    axum::serve(listener, routes)
        .await
        .expect("Error serving application");
}