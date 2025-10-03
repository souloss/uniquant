use crate::dto::response::APIResponse;
use crate::dto::instrument::{
    CreateInstrumentRequest, InstrumentResponse, UpdateInstrumentRequest,
};
use crate::error::AppError;
use crate::service::instrument::{InstrumentService};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;

pub struct InstrumentHandler;

impl InstrumentHandler {
    pub async fn create(
        State(service): State<Arc<InstrumentService>>,
        Json(req): Json<CreateInstrumentRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let cmd = CreateInstrumentRequest {
            exchange: req.exchange,
            symbol: req.symbol,
            asset_type: req.asset_type,
            name: req.name,
        };

        let model = service.create(cmd).await?;
        let response: InstrumentResponse = model.into();

        Ok((axum::http::StatusCode::CREATED, Json(APIResponse::success(response))))
    }

    pub async fn get_by_id(
        State(service): State<Arc<InstrumentService>>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, AppError> {
        let model = service.get_by_id(id).await?;
        let model = model.ok_or(AppError::NotFound {
            resource: "Instrument".to_string(),
            identifier: id.to_string(),
        })?;
        let response: InstrumentResponse = model.into();
        Ok(Json(APIResponse::success(response)))
    }

    pub async fn get_all(
        State(service): State<Arc<InstrumentService>>,
    ) -> Result<impl IntoResponse, AppError> {
        let models = service.get_all().await?;
        let responses: Vec<InstrumentResponse> = models.into_iter().map(Into::into).collect();
        Ok(Json(APIResponse::success(responses)))
    }

    pub async fn update(
        State(service): State<Arc<InstrumentService>>,
        Json(req): Json<UpdateInstrumentRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let model = service.update(req).await?;
        let response: InstrumentResponse = model.into();
        Ok(Json(APIResponse::success(response)))
    }
    pub async fn delete(
        State(service): State<Arc<InstrumentService>>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, AppError> {
        let deleted = service.delete(id).await?;
        if !deleted {
            return Err(AppError::NotFound {
                resource: "Instrument".to_string(),
                identifier: id.to_string(),
            });
        }
        Ok(axum::http::StatusCode::NO_CONTENT)
    }
}