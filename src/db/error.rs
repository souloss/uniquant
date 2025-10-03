use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database operation failed: {0}")]
    SeaOrm(#[from] sea_orm::DbErr),

    #[error("Record not found: {entity} with identifier '{id}'")]
    NotFound { entity: &'static str, id: String },

    #[error("Authentication failed: {reason}")]
    AuthenticationFailed { reason: String },

    #[error("Invalid data provided: {field} - {message}")]
    InvalidData { field: String, message: String },

    #[error("A unique constraint was violated: {constraint}")]
    UniqueViolation { constraint: &'static str },

    #[error("MigrationError failed: {reason}")]
    MigrationError { reason: String },
}

impl DbError {
    // 便利构造函数，使调用更简洁
    pub fn user_not_found(id: &str) -> Self {
        Self::NotFound {
            entity: "User",
            id: id.to_string(),
        }
    }

    pub fn crate_not_found(id: &str) -> Self {
        Self::NotFound {
            entity: "Crate",
            id: id.to_string(),
        }
    }
}