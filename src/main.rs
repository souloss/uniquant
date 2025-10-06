use std::sync::Arc;

pub mod api;
pub mod core;
pub mod db;
pub mod dto;
pub mod service;
pub mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    core::logging::logger::init();
    // 初始化配置 & 日志
    let config = core::config::AppConfig::bootstrap()?;
    let config = Arc::new(config.clone());
    core::logging::init();

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