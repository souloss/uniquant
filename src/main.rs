use std::{path::Path, sync::Arc};

use crate::i18n::{fluent::FluentBackend, GlobalI18n};

pub mod api;
pub mod core;
pub mod db;
pub mod dto;
pub mod service;
pub mod error;
pub mod i18n;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    core::logging::logger::init();
    // 初始化配置 & 日志
    let config = core::config::AppConfig::bootstrap()?;
    let config = Arc::new(config.clone());
    core::logging::init();

    let locales_path = Path::new(config.configs_dir.as_str()).join("locales");
    let i18n_backend = FluentBackend::new(locales_path.as_os_str().to_str().unwrap(), "en").unwrap();
    GlobalI18n::get().init_with_backend(i18n_backend)?;
    
    // 初始化数据库连接池
    let db = db::connection::DbPool::new(&config.database.db_url).await?;
    db.run_migrations().await?;
    
    let repo = Arc::new(db);

    // 初始化服务工厂
    let service_factory = service::factory::ServiceFactory::new(repo.clone());
    let service_factory = Arc::new(service_factory);

    // 构建 app
    let app = api::WebServer::new(config, service_factory)?;

    app.start().await?;

    Ok(())
}