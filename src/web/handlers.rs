use axum::{Json, response::IntoResponse};
use serde_json::json;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn hello_handler() -> impl IntoResponse {
    Json(json!({ "message": "Hello, UniQuant!" }))
}