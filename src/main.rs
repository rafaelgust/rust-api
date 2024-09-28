
use std::{env, net::{IpAddr, SocketAddr}, str::FromStr};

use axum::{http::{self, HeaderValue, Method, StatusCode}, middleware, routing::{get, post}, Router};
use axum::response::Response;

use log::info;
use tower_http::cors::{AllowOrigin, CorsLayer};

use dotenv::dotenv;
use serde_json::json;
use tokio::net::TcpListener;

use utils::auth::handlers::authorize;
use utils::routers::category::get_category_routes;
use utils::routers::comment::get_comment_routes;
use utils::routers::brand::BrandRoutes;
use utils::routers::product::ProductRoutes;

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
        .route("/signup", get(auth::handlers::sign_out))
        .layer(middleware::from_fn(authorize));

    let public_routes = Router::new()
        .route("/", get(root))
        .route("/create", post(auth::handlers::create_user))
        .route("/signin", post(auth::handlers::sign_in))
        .route("/refresh", post(auth::handlers::refresh_access_token));

    let category_routes = get_category_routes();
    let comment_routes = get_comment_routes();
    let brand_routes = BrandRoutes::get_routes();
    let product_routes = ProductRoutes::get_routes();

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

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Configurar o CORS
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("https://glammount.com".parse::<HeaderValue>().unwrap()))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
        .allow_credentials(true);
    
    let ip = env::var("SERVER_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3333);

    let ip_addr = IpAddr::from_str(&ip).expect("Invalid IP address");
    let addr = SocketAddr::new(ip_addr, port);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to connect to the server");

    println!("Listening on {}:{}...", ip, port);

    let routes = Router::new()
        .nest("/api", create_router())
        .layer(cors);
    
    println!("Server running on http://{}:{}", ip, port);

    axum::serve(listener, routes)
        .await
        .expect("Error serving application");
}