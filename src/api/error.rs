#![allow(unreachable_patterns)]

use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use crate::{dto::response::APIResponse, i18n::{supported_locales, t_locale}};
use crate::error::code::AppError;
use accept_language::parse_with_quality;
use unic_langid::{langid, LanguageIdentifier};
use crate::api::middleware::context::get_request_context;

pub fn parse_accept_language(header: &str) -> Vec<LanguageIdentifier> {
    // 1. 带权重解析 → Vec<(lang, q)>
    let ranked = parse_with_quality(header); // e.g. "en-US,en;q=0.9,zh-CN;q=0.8"
    // 2. 转 LanguageIdentifier （失败就跳过）
    ranked
        .into_iter()
        .filter_map(|(lang_str, _q)| lang_str.parse().ok())
        .collect()
}

impl From<&AppError> for StatusCode {
    fn from(err: &AppError) -> Self {
        StatusCode::from_u16(err.http_status())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 从上下文中取语言（task_local）
        // 然后从用户语言以及支持的语言中协商得到最佳语言
        let ctx = get_request_context();
        let user_langs = parse_accept_language(ctx.lang.as_str());
        let supported = supported_locales().unwrap_or_default();
        let best = user_langs
            .iter()
            .find(|l| supported.contains(l)) // 完整匹配
            .or_else(|| {
                // 主区域匹配（zh-CN → zh）
                user_langs
                    .iter()
                    .find_map(|l| supported.iter().find(|s| s.language == l.language))
            })
            .cloned()
            .unwrap_or_else(|| langid!("zh-CN"));

        // 获取状态码和返回消息
        let status = StatusCode::from(&self);
        let message = t_locale(self.as_key(), &best, self.to_args().into()).unwrap_or(self.to_string());

        let error_response = APIResponse::<()>::error(self, message);

        (status, Json(error_response)).into_response()
    }
}
