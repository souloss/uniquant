use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));

    fmt()
        .with_env_filter(filter)
        .json() // JSON 格式，便于收集到 ELK/ClickHouse
        .init();
}