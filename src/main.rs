use axum::{routing::{get, post}, middleware, Router, http::StatusCode};
use axum::response::Response;

use serde_json::json;
use tokio::net::TcpListener;

use utils::routers::brand::get_brand_routes;
use utils::routers::category::get_category_routes;
use utils::routers::comment::get_comment_routes;
use utils::routers::product::get_product_routes;

use utils::auth;

mod utils;
mod schema;

async fn root() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!({
            "success": true,
            "data": "Welcome to the API"
        }).to_string())
        .unwrap()
}

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Unable to connect to the server");

    let routes = Router::new()
    .route("/", get(root))
    .route("/signin", post(auth::jwt::sign_in))
    .route(
        "/protected/",
        get(auth::services::hello).layer(middleware::from_fn(auth::jwt::authorize)),
    )
    .merge(get_brand_routes())
    .merge(get_category_routes())
    .merge(get_comment_routes())
    .merge(get_product_routes());
    
    axum::serve(listener, routes)
        .await
        .expect("Error serving application");
}