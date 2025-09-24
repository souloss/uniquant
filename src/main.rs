use uniquant::core::{config, logger};
use uniquant::web;

use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置 & 日志
    let config = config::AppConfig::load();
    logger::init();

    // 构建 app
    let app = web::create_app();

    // 监听地址
    let addr: SocketAddr = config.server.addr.parse()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("🚀 UniQuant running on {}", addr);

    // 启动服务
    axum::serve(listener, app).await?;

    Ok(())
}