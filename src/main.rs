
use std::{env, net::{IpAddr, SocketAddr}, str::FromStr};

use axum::{routing::{get, post}, middleware, Router, http::StatusCode};
use axum::response::Response;

use dotenv::dotenv;
use serde_json::json;
use tokio::net::TcpListener;

use utils::{auth::jwt::authorize, routers::brand::get_brand_routes};
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

pub fn create_router() -> Router {
    let protected_routes = Router::new()
        .route("/protected", get(auth::services::hello))
        .route("/signup", get(auth::jwt::sign_out))
        .layer(middleware::from_fn(authorize));

    let public_routes = Router::new()
        .route("/", get(root))
        .route("/signin", post(auth::jwt::sign_in))
        .route("/refresh", post(auth::jwt::refresh_access_token));

    let brand_routes = get_brand_routes();
    let category_routes = get_category_routes();
    let comment_routes = get_comment_routes();
    let product_routes = get_product_routes();

    Router::new()
        .merge(public_routes)
        .merge(brand_routes)
        .merge(category_routes)
        .merge(comment_routes)
        .merge(product_routes)
        .merge(protected_routes)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let ip_addr = IpAddr::from_str(&ip).expect("Invalid IP address");
    let addr = SocketAddr::new(ip_addr, port);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to connect to the server");

    println!("Listening on {}:{}...", ip, port);

    let routes = Router::new()
        .nest("/api", create_router());
    
    println!("Server running on http://{}:{}", ip, port);

    axum::serve(listener, routes)
        .await
        .expect("Error serving application");
}