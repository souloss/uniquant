use crate::core::config::AppConfig;
use tracing_appender::non_blocking;
use tracing_subscriber::{prelude::*, EnvFilter};
use tracing_subscriber::{fmt};

// 简单控制台日志记录器
pub mod logger {
    use std::sync::{Arc, OnceLock};
    use tracing::{info, warn, error, debug, trace};
    use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, Registry};

    /// 日志模块
    #[derive(Clone, Debug)]
    pub struct Logger {
        dispatch: Arc<tracing::Dispatch>,
    }

    impl Logger {
        /// 简单初始化（只输出到 stdout）
        pub fn init_simple() -> Self {
            let filter = EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"));

            let fmt_layer = fmt::layer()
                .pretty()
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_ansi(true);

            let dispatch = tracing::Dispatch::new(
                Registry::default().with(filter).with(fmt_layer)
            );

            Logger {
                dispatch: Arc::new(dispatch),
            }
        }

        /// 直接打印 info
        pub fn info(&self, msg: impl AsRef<str>) {
            tracing::dispatcher::with_default(&self.dispatch, || {
                info!("{}", msg.as_ref());
            });
        }

        /// 直接打印 debug
        pub fn debug(&self, msg: impl AsRef<str>) {
            tracing::dispatcher::with_default(&self.dispatch, || {
                debug!("{}", msg.as_ref());
            });
        }

        /// 直接打印 warn
        pub fn warn(&self, msg: impl AsRef<str>) {
            tracing::dispatcher::with_default(&self.dispatch, || {
                warn!("{}", msg.as_ref());
            });
        }

        /// 直接打印 error
        pub fn error(&self, msg: impl AsRef<str>) {
            tracing::dispatcher::with_default(&self.dispatch, || {
                error!("{}", msg.as_ref());
            });
        }

        /// 直接打印 trace
        pub fn trace(&self, msg: impl AsRef<str>) {
            tracing::dispatcher::with_default(&self.dispatch, || {
                trace!("{}", msg.as_ref());
            });
        }
    }

    // 全局静态，只存放可共享的 Logger
    static LOGGER: OnceLock<Logger> = OnceLock::new();
    /// 初始化全局 Logger，main 调用
    pub fn init() {
        let logger = Logger::init_simple();
        LOGGER.set(logger).expect("Logger 已初始化");
    }
    /// 获取全局 Logger
    pub fn global_logger() -> &'static Logger {
        LOGGER.get().expect("Logger 未初始化")
    }
}

/// 初始化完整日志系统（控制台 + 文件输出 + 可选OTEL）
pub fn init() {
    let cfg = &AppConfig::global().logging;
    
    // 设置日志级别过滤器
    let level_filter = EnvFilter::try_new(&cfg.level)
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    // 创建控制台日志层, 生产时可以将 pretty 改为 json
    let stdout_layer = fmt::layer()
        .pretty()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(true);
    
    // 3) 可选的 file 层（非阻塞）
    let file_layer_opt = if let Some(dir) = &cfg.directory {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("无法创建日志目录 {}: {}", dir, e);
            None
        } else {
            // 使用 rolling::never 简单演示（不做轮转策略，轮转可交给 logrotate）
            let appender = tracing_appender::rolling::never(dir, "app.log");
            let (non_blocking_writer, _guard) = non_blocking(appender);
            // 注意：_guard 需要保留在作用域里防止丢弃；此处因为我们没有返回它，Guard 会在函数结束被 drop。
            // 如果你希望 guard 存活整个程序生命周期，请把它保存在某个静态变量或传回 caller。
            //
            // 我这里仍创建 layer；如果你想在 long-running 程序把 guard 保活，请把 guard 存到静态 OnceLock。
            let file_layer = fmt::layer()
                .json()
                .with_writer(non_blocking_writer)
                .with_current_span(true)
                .with_span_list(true)
                .with_target(true);

            Some(file_layer)
        }
    } else {
        None
    };

    // 把不同组合的层在 match 分支中一次性注册（避免 registry = registry.with(...) 赋值引发的类型问题）
    // 之后可能会新增 OTEL 等层
    match file_layer_opt {
        Some(file_layer_opt) => {
            tracing_subscriber::registry()
                .with(level_filter)
                .with(stdout_layer)
                .with(file_layer_opt)
                .init();
        }
        None => {
            tracing_subscriber::registry()
                .with(level_filter)
                .with(stdout_layer)
                .init();
        }
    }
}