use axum::{Router, routing::get};
use super::handlers::hello_handler;

pub fn app_router() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
}