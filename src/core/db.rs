use sea_orm::{Database, DatabaseConnection};

pub async fn connect(db_url: &str) -> anyhow::Result<DatabaseConnection> {
    let db = Database::connect(db_url).await?;
    Ok(db)
}