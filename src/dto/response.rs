use serde::{Serialize};
use crate::error::code::AppError;

/// 统一的 API 响应结构
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct APIResponse<T> {
    /// 请求是否成功
    pub success: bool,
    /// 应用级别的返回码
    pub code: i32,
    /// 返回消息
    pub message: String,
    /// 返回的数据，错误时为 null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> APIResponse<T> {
    /// 创建一个成功的响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: AppError::Success.code(),
            message: AppError::Success.description().to_string(),
            data: Some(data),
        }
    }

    /// 创建一个成功的响应，并附带自定义消息
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            code: AppError::Success.code(),
            message: message.into(),
            data: Some(data),
        }
    }

    /// 创建一个失败的响应
    pub fn error(code: AppError, message: impl Into<String>) -> APIResponse<()> {
        APIResponse {
            success: false,
            code: code.code(),
            message: message.into(),
            data: None,
        }
    }
}