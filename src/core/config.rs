use serde::Deserialize;
use figment::{Figment, providers::{Env, Format, Toml}};

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_url")]
    pub db_url: String,
}
fn default_db_url() -> String { "sqlite://uniquant.db".to_string() }

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_addr")]
    pub addr: String,
}
fn default_addr() -> String { "127.0.0.1:8333".to_string() }

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("UNIQUANT_").split("__"))
            .extract()
            .expect("配置解析失败")
    }
}