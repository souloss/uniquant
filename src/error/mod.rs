use crate::error::code::AppCode;

// 确保导出了子模块
pub mod code;
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
}