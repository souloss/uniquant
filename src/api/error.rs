#![allow(unreachable_patterns)]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::dto::response::APIResponse;
use crate::error::{AppError, code::AppCode};
use crate::api::middleware::context::get_request_context;

impl From<AppCode> for StatusCode {
    fn from(code: AppCode) -> Self {
        match code {
            AppCode::Success => StatusCode::OK,
            AppCode::Validation | AppCode::BadRequest => StatusCode::BAD_REQUEST,
            AppCode::Unauthorized => StatusCode::UNAUTHORIZED,
            AppCode::Forbidden => StatusCode::FORBIDDEN,
            AppCode::NotFound => StatusCode::NOT_FOUND,
            AppCode::Conflict => StatusCode::CONFLICT,
            // 显式列出已知的 5xx
            AppCode::Internal | AppCode::Database => StatusCode::INTERNAL_SERVER_ERROR,
            // 任何尚未显式处理的新变体都进这里
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 从上下文中取语言（task_local）
        let ctx = get_request_context();
        let language = ctx.lang.clone();
        let language = crate::error::message::Language::from_header(&language);

        let app_code = self.app_code();
        let http_status: StatusCode = app_code.into();

        let message = crate::error::message::MessageRenderer::render(&self, language);
        let error_response = APIResponse::<()>::error(app_code, message);

        (http_status, Json(error_response)).into_response()
    }
}
