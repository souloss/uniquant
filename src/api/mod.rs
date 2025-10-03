pub mod instrument;

// src/api/mod.rs

use std::sync::Arc;
use std::net::SocketAddr;
use axum::{
    Router,
    routing::get,
};
use tokio::net::TcpListener;
use tokio::signal;
use tracing::{info};
use anyhow::Result;

// 导入所有必要的依赖
use crate::core::config::AppConfig;
use crate::service::factory::ServiceFactory;

/// 封装 Axum Web 服务器的结构体
#[derive(Debug)]
pub struct WebServer {
    app: Router,
    addr: SocketAddr,
}

impl WebServer {
    /// 创建一个新的 WebServer 实例
    pub fn new(config: Arc<AppConfig>, service_factory: Arc<ServiceFactory>) -> Result<Self> {
        // 1. 从工厂获取所有服务
        let instrument_service = service_factory.instrument_service();
        // let strategy_service = service_factory.strategy_service();
        // ... 其他服务

        // 2. 构建应用路由
        let app = Router::new()
            // 健康检查端点（无状态）
            .route("/health", get(health_check))
            // 合并所有业务模块的路由
            .merge(instrument::routes::routes(instrument_service));

        // 3. 解析服务器地址
        let addr: SocketAddr = config.server.addr.parse()?;

        info!("Web server configured to listen on {}", addr);

        Ok(Self { app, addr })
    }

    /// 启动 Web 服务器并处理优雅关闭
    pub async fn start(self) -> Result<()> { // <-- 修改这里
        let listener = TcpListener::bind(self.addr).await?;
        info!("Web server listening on {}", listener.local_addr().unwrap());

        // 启动服务器
        axum::serve(listener, self.app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        info!("Application has shut down gracefully.");
        Ok(())
    }
}

/// 健康检查处理器
async fn health_check() -> &'static str {
    "OK"
}

/// 监听关闭信号（Ctrl+C / SIGTERM）
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("received Ctrl+C, shutting down");
        },
        _ = terminate => {
            info!("received terminate signal, shutting down");
        },
    }
}