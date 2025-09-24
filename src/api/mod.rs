use axum::{routing::get, Router};

pub fn create_app() -> Router {
    Router::new().route("/hello", get(hello))
}

async fn hello() -> &'static str {
    "Hello, UniQuant!"
}