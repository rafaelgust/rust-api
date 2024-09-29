
use std::{env, net::{IpAddr, SocketAddr}, str::FromStr};

use axum::{http::{self, HeaderValue, Method}, Router};

use tower_http::cors::{AllowOrigin, CorsLayer};

use dotenv::dotenv;
use tokio::net::TcpListener;

use utils::{auth, routers::routes::create_router};

mod utils;
mod schema;

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