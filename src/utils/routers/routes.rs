use axum::{http::StatusCode, middleware, routing::{get, post}, Router};
use axum::response::Response;
use serde_json::json;


use crate::utils::{self, auth};
use utils::auth::handlers::authorize;

use super::category::CategoryRoutes;
use super::comment::CommentRoutes;
use super::brand::BrandRoutes;
use super::product::ProductRoutes;

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

    let category_routes = CategoryRoutes::get_routes();
    let comment_routes = CommentRoutes::get_routes();
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