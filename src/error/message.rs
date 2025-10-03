// src/error/message.rs
use once_cell::sync::Lazy;
use tera::{Tera, Context};
use crate::error::{AppError, app_code::AppCode};

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    /// 从 Accept-Language 头部解析语言
    pub fn from_header(header: &str) -> Self {
        if header.to_lowercase().starts_with("zh") {
            Self::Chinese
        } else {
            Self::English
        }
    }
}

/// 消息渲染器
pub struct MessageRenderer;

// 创建一个全局的 Tera 实例，并内嵌所有模板
static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    
    // 定义所有错误消息模板
    // Tera 使用 {{ var }} 语法
    let templates = vec![
        ("not_found_en", "The {{ resource }} with identifier '{{ identifier }}' was not found."),
        ("not_found_zh", "未找到资源 {{ resource }}，标识符 '{{ identifier }}' 不存在。"),
        ("conflict_en", "The {{ resource }} with identifier '{{ identifier }}' already exists."),
        ("conflict_zh", "资源 {{ resource }}，标识符 '{{ identifier }}' 已存在。"),
        ("bad_request_en", "Bad request: {{ message }}."),
        ("bad_request_zh", "请求错误: {{ message }}。"),
        ("internal_error_en", "An internal server error occurred. Please try again later."),
        ("internal_error_zh", "服务器内部错误，请稍后重试。"),
    ];

    for (name, content) in templates {
        tera.add_raw_template(name, content).expect("Failed to add template");
    }
    
    // Tera 在开发时可以自动重载，生产环境关闭
    tera.autoescape_on(vec!["html"]);
    tera
});

impl MessageRenderer {
    /// 根据错误和语言渲染最终的消息
    pub fn render(error: &AppError, language: Language) -> String {
        let code = error.app_code();
        
        // 根据语言和错误码选择模板名称
        let template_name = match (code, language) {
            (AppCode::NotFound, Language::English) => "not_found_en",
            (AppCode::NotFound, Language::Chinese) => "not_found_zh",
            (AppCode::Conflict, Language::English) => "conflict_en",
            (AppCode::Conflict, Language::Chinese) => "conflict_zh",
            (AppCode::BadRequest, Language::English) => "bad_request_en",
            (AppCode::BadRequest, Language::Chinese) => "bad_request_zh",
            (AppCode::InternalError, Language::English) => "internal_error_en",
            (AppCode::InternalError, Language::Chinese) => "internal_error_zh",
            // 默认回退到英文内部错误
            _ => "internal_error_en",
        };

        // 构建 Tera Context
        let mut context = Context::new();
        match error {
            AppError::NotFound { resource, identifier } => {
                context.insert("resource", resource);
                context.insert("identifier", identifier);
            }
            AppError::Conflict { resource, identifier } => {
                context.insert("resource", resource);
                context.insert("identifier", identifier);
            }
            AppError::ValidationError(msg) | AppError::Unauthorized(msg) | AppError::Forbidden(msg) | AppError::BadRequest(msg) | AppError::InternalError(msg) => {
                context.insert("message", msg);
            }
            AppError::DatabaseError(db_err) => {
                tracing::error!("Database error: {:?}", db_err);
                context.insert("message", "A database error occurred.");
            }
        }
        
        // 渲染模板
        TEMPLATES
            .render(template_name, &context)
            .unwrap_or_else(|e| {
                // 渲染失败时的回退
                tracing::error!("Failed to render error template '{}': {}", template_name, e);
                format!("An error occurred while rendering the error message.")
            })
    }
}