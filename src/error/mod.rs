pub mod code;

impl From<sea_orm::DbErr> for code::AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(_) => code::AppError::Database {
                message: "Record not found".to_string()
            },
            sea_orm::DbErr::Exec(_) | sea_orm::DbErr::Query(_) => code::AppError::Database {
                message: "SQL execution error".to_string()
            },
            _ => code::AppError::Database {
                message: "Unknown database error".to_string()
            },
        }
    }
}