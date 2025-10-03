use serde::Serialize;

/// 应用级别的统一返回码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[repr(i32)]
pub enum AppCode {
    // --- 成功 ---
    /// 操作成功
    Success = 0,

    // --- 客户端错误 (4xxx) ---
    /// 请求参数有误
    BadRequest = 4000,
    /// 未授权
    Unauthorized = 4001,
    /// 禁止访问
    Forbidden = 4003,
    /// 资源未找到
    NotFound = 4004,
    /// 资源冲突
    Conflict = 4009,

    // --- 服务器错误 (5xxx) ---
    /// 内部服务器错误
    InternalError = 5000,
}

impl AppCode {
    /// 获取返回码的整数值
    pub fn code(self) -> i32 {
        self as i32
    }

    /// 获取返回码的默认消息
    pub fn message(self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::Conflict => "Conflict",
            Self::InternalError => "Internal Server Error",
        }
    }
}