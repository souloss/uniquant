use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    body::Body
};
use uuid::Uuid;
use std::sync::Arc;
use tokio::task_local;

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub request_id: String,
    pub lang: String,
}

impl RequestContext {
    pub fn new(request_id: String, lang: String) -> Self {
        Self {
            request_id,
            lang,
        }
    }
}

task_local! {
    pub static CONTEXT: Arc<RequestContext>;
}

pub fn get_request_context() -> Arc<RequestContext> {
    CONTEXT.with(|ctx| ctx.clone())
}

pub async fn request_context_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    // 从 Accept-Language 获取语言，默认 en
    let lang = req
        .headers()
        .get("Accept-Language")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next()) // 只取第一个语言
        .unwrap_or("en")
        .trim()
        .to_string();

    let request_id = Uuid::new_v4().to_string();

    let ctx = Arc::new(RequestContext::new(request_id, lang));

    // 使用 task_local 运行异步任务，将上下文绑定到当前请求
    CONTEXT.scope(ctx, async {
        next.run(req).await
    }).await
}