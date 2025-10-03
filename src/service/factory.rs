// src/service/factory.rs
use std::sync::Arc;
use crate::db::connection::DbPool;
use crate::db::repositories::instrument::InstrumentRepository;
use crate::service::{
    instrument::InstrumentService
};

pub struct ServiceFactory {
    db: Arc<DbPool>,
}

impl ServiceFactory {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { db }
    }

    pub fn instrument_service(&self) -> Arc<InstrumentService> {
        let repo = Arc::new(InstrumentRepository::new(self.db.clone()));
        Arc::new(InstrumentService::new(repo))
    }
}