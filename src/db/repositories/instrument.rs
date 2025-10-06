use std::sync::Arc;

use sea_orm::*;
use crate::db::connection::DbPool;
use crate::db::repositories::Repository;
use entities::instrument;

pub struct InstrumentRepository {
    db: Arc<DbPool>,
}

impl InstrumentRepository {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { db }
    }
    
    /// 检查交易所和标的代码是否已存在
    pub async fn find_by_exchange_and_symbol(
        &self,
        exchange: &str,
        symbol: &str,
    ) -> Result<Option<instrument::Model>, DbErr> {
        let condition = Condition::all()
            .add(instrument::Column::Exchange.eq(exchange))
            .add(instrument::Column::Symbol.eq(symbol));
        self.find_one_by_condition(condition).await
    }
}

// 关键：实现通用 Repository trait
#[async_trait::async_trait]
impl Repository<instrument::Entity> for InstrumentRepository {
    fn conn(&self) -> &DatabaseConnection {
        &self.db
    }
}