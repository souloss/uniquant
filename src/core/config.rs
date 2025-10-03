use serde::Deserialize;
use validator::Validate;
use crate::core::logging::logger::global_logger;
use config::{Config, ConfigError, Environment, File};
use std::sync::OnceLock;
use url::Url;

/// -------------------- 子配置 --------------------
#[derive(Debug, Deserialize, Clone, Validate)]
pub struct DatabaseConfig {
    /// 数据库 URL
    #[validate(length(min = 1, message = "数据库 URL 不能为空"))]
    pub db_url: String,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct ServerConfig {
    /// 服务监听地址
    #[validate(length(min = 1, message = "服务地址不能为空"))]
    pub addr: String,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct LoggingConfig {
    /// 日志级别（默认为DEBUG）
    pub level: String,

    /// 日志输出目录（为空则只输出到 stdout）
    pub directory: Option<String>,

    /// 是否启用 otel 支持
    pub enable_otel_logging: bool,

    /// otel 导出器端点地址
    pub otel_exporter_otlp_endpoint: Option<String>,
}

/// -------------------- 应用配置 --------------------
#[derive(Debug, Deserialize, Clone, Validate)]
pub struct AppConfig {
    pub app_name: String,

    #[validate(nested)]
    pub database: DatabaseConfig,

    #[validate(nested)]
    pub server: ServerConfig,

    #[validate(nested)]
    pub logging: LoggingConfig,
}

/// 全局配置实例
static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    /// 构建带默认值的 ConfigBuilder
    fn builder_with_defaults() -> config::ConfigBuilder<config::builder::DefaultState> {
        Config::builder()
            .set_default("app_name","uniquant" ).unwrap()
            .set_default("database.db_url", "sqlite://uniquant.db").unwrap()
            .set_default("server.addr", "127.0.0.1:8333").unwrap()
            .set_default("logging.level", "debug").unwrap()
            .set_default("logging.max_size", 10 * 1024 * 1024).unwrap()
            .set_default("logging.rotation", 3).unwrap()
            .set_default("logging.enable_otel_logging", false).unwrap()
    }

    /// 从文件 + 环境变量加载
    pub fn load() -> Result<Self, ConfigError> {
        let builder = Self::builder_with_defaults()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("UNIQUANT").separator("__"));

        let cfg: Self = builder.build()?.try_deserialize()?;
        cfg.validate().map_err(|e| ConfigError::Message(format!("{e}")))?;
        Ok(cfg)
    }

    /// 仅从环境变量加载（容器化/CI）
    pub fn load_from_env() -> Result<Self, ConfigError> {
        let builder = Self::builder_with_defaults()
            .add_source(Environment::with_prefix("UNIQUANT").separator("__"));

        let cfg: Self = builder.build()?.try_deserialize()?;
        cfg.validate().map_err(|e| ConfigError::Message(format!("{e}")))?;
        Ok(cfg)
    }

    /// 初始化全局配置
    pub fn init(cfg: AppConfig) -> Result<(), ConfigError> {
        GLOBAL_CONFIG.set(cfg).map_err(|_| ConfigError::Message("配置已初始化".to_string()))
    }

    /// 一站式启动（文件+环境加载并注册为全局）
    pub fn bootstrap() -> Result<&'static Self, ConfigError> {
        let cfg = Self::load()?;
        let _ = Self::init(cfg).map_err(|e| ConfigError::Message(e.to_string()))?;
        Self::log_summary(&Self::global());
        Ok(Self::global())
    }

    /// 获取全局配置
    pub fn global() -> &'static Self {
        GLOBAL_CONFIG.get().expect("配置未初始化，请先调用 AppConfig::init()")
    }

    /// 动态打印配置摘要（自动序列化，支持新增字段）
    pub fn log_summary(&self) {
        let masked_db_url = mask_db_url(&self.database.db_url);
        global_logger().info( "✅ 配置加载完成");
        global_logger().info( format!("app_name: {}", self.app_name));
        global_logger().info( format!("database.url: {}", masked_db_url));
        global_logger().info( format!("server: {:#?}", self.server.addr));
        global_logger().info( format!("logging: {:#?}", self.logging));
    }
}

/// 脱敏函数
fn mask_db_url(url: &str) -> String {
    if let Ok(parsed) = Url::parse(url) {
        let mut sanitized = parsed.clone();
        sanitized.set_username("****").ok();
        sanitized.set_password(Some("****")).ok();
        return sanitized.to_string();
    }
    url.to_string()
}