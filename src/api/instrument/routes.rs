use crate::api::instrument::handler::InstrumentHandler;
use crate::service::instrument::InstrumentService;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn routes(service: Arc<InstrumentService>) -> Router {
    Router::new()
        .route("/instruments", post(InstrumentHandler::create))
        .route("/instruments", get(InstrumentHandler::get_all))
        .route("/instruments/{id}", get(InstrumentHandler::get_by_id))
        .route("/instruments/{id}", put(InstrumentHandler::update))
        .route("/instruments/{id}", delete(InstrumentHandler::delete))
        .with_state(service)
}
