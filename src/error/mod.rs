use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::dto::response::APIResponse;
use crate::error::app_code::AppCode;

// 确保导出了子模块
pub mod app_code;
pub mod message;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    // 结构体变体，用于携带结构化数据
    #[error("Not found")]
    NotFound { resource: String, identifier: String },

    #[error("Unauthorized")]
    Unauthorized(String),

    #[error("Forbidden")]
    Forbidden(String),

    // 结构体变体，用于携带结构化数据
    #[error("Conflict")]
    Conflict { resource: String, identifier: String },

    #[error("Bad request")]
    BadRequest(String),

    #[error("Internal error")]
    InternalError(String),

    #[error("Database error")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl AppError {
    /// 将业务错误映射到应用返回码
    pub fn app_code(&self) -> AppCode {
        match self {
            Self::ValidationError(_) => AppCode::BadRequest,
            Self::NotFound { .. } => AppCode::NotFound, // 使用 `..` 忽略字段
            Self::Unauthorized(_) => AppCode::Unauthorized,
            Self::Forbidden(_) => AppCode::Forbidden,
            Self::Conflict { .. } => AppCode::Conflict, // 使用 `..` 忽略字段
            Self::BadRequest(_) => AppCode::BadRequest,
            Self::InternalError(_) => AppCode::InternalError,
            Self::DatabaseError(_) => AppCode::InternalError,
        }
    }

    /// 获取对应的 HTTP 状态码
    pub fn http_status_code(&self) -> StatusCode {
        self.app_code().into()
    }
}

// 将 AppCode 转换为 HTTP 状态码
impl From<AppCode> for StatusCode {
    fn from(code: AppCode) -> Self {
        match code {
            AppCode::Success => StatusCode::OK,
            AppCode::BadRequest => StatusCode::BAD_REQUEST,
            AppCode::Unauthorized => StatusCode::UNAUTHORIZED,
            AppCode::Forbidden => StatusCode::FORBIDDEN,
            AppCode::NotFound => StatusCode::NOT_FOUND,
            AppCode::Conflict => StatusCode::CONFLICT,
            AppCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// 将 AppError 转换为统一的 API 响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 由于无法获取请求，我们只能使用默认语言
        let language = crate::error::message::Language::English;
        let app_code = self.app_code();
        let http_status = self.http_status_code();
        
        // 使用渲染器生成最终的、翻译好的消息
        let message = crate::error::message::MessageRenderer::render(&self, language);

        let error_response = APIResponse::<()>::error(app_code, message);

        (http_status, Json(error_response)).into_response()
    }
}