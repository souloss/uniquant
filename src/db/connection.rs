// src/db/connection.rs
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use url::Url;
use std::{ops::Deref, path::Path};
use std::time::Duration;
use tracing::info;
use anyhow::{Result, anyhow};
use migration::{Migrator, MigratorTrait};
use super::error::DbError;

#[derive(Debug, Clone)]
pub struct DbPool {
    connection: DatabaseConnection,
}

impl DbPool {
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("Connecting to database: {}", database_url);
        if database_url.is_empty() {
                return Err(anyhow::anyhow!("Database URL cannot be empty."));
            }

        // --- SQLite 特殊处理：自动创建数据库文件 ---
        if database_url.starts_with("sqlite://") {
            Self::ensure_sqlite_file_exists(database_url)?;
        }

        let mut opt = ConnectOptions::new(database_url.to_string());
        opt.max_connections(20)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(7200))
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Debug);

        let connection = Database::connect(opt).await?;
        info!("Database connection pool established.");
        Ok(Self { connection })
    }

    fn ensure_sqlite_file_exists(database_url: &str) -> Result<()> {
        // 解析 URL
        let url = Url::parse(database_url)
            .map_err(|e| anyhow!("Invalid database URL: {}", e))?;
        
        // 检查 scheme 是否为 'sqlite'
        if url.scheme() != "sqlite" {
            return Err(anyhow!("Invalid SQLite URL scheme: {}", url.scheme()));
        }

        // 手动提取路径部分
        let db_name_or_flag = url.host()
            .map(|host| host.to_string()) // <-- 修改这里：使用 to_string()
            .ok_or_else(|| anyhow!("SQLite URL is missing database name, e.g., 'sqlite://my.db'"))?;
        
        // 判断是否为内存数据库
        if db_name_or_flag == ":memory:" {
            info!("Connecting to an in-memory SQLite database.");
            return Ok(());
        }
        let path = Path::new(&db_name_or_flag);

        if !path.exists() {
            info!("SQLite database file not found at: {:?}. Creating a new one.", url);
            
            std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(path)
                .map_err(|e| anyhow!("Failed to create SQLite database file at {:?}: {}", path, e))?;
            
            info!("Successfully created SQLite database file.");
        } else {
            info!("Found existing SQLite database file at: {:?}.", path);
        }

        Ok(())
    }
    pub async fn run_migrations(&self) -> Result<(), DbError> {
        Migrator::up(&self.connection, None)
            .await
            .map_err(|e| DbError::MigrationError{reason:e.to_string()})?;
        info!("✅ Database migrations applied successfully.");
        Ok(())
    }
}

impl Deref for DbPool {
    type Target = DatabaseConnection;
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}