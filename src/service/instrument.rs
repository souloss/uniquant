use std::sync::Arc;
use crate::db::repositories::Repository;
use crate::error::AppError;
use crate::db::repositories::instrument::InstrumentRepository;
use crate::dto::instrument::{CreateInstrumentRequest, UpdateInstrumentRequest, InstrumentResponse};

pub struct InstrumentService {
    repo: Arc<InstrumentRepository>,
}

impl InstrumentService {
    pub fn new(repo: Arc<InstrumentRepository>) -> Self {
        Self { repo }
    }

    /// 创建交易标的
    /// 业务规则：同一个交易所的 symbol 不能重复
    pub async fn create(&self, new_instrument: CreateInstrumentRequest) -> Result<InstrumentResponse, AppError> {
        let exchange = new_instrument.exchange.as_ref();
        let symbol = new_instrument.symbol.as_ref();

        // 业务逻辑检查
        if let Some(_existing) = self.repo.find_by_exchange_and_symbol(exchange, symbol).await
            .map_err(AppError::DatabaseError)? {
            return Err(AppError::Conflict{resource:exchange.to_string(), identifier:symbol.to_string()});
        }

        // 调用 Repo 创建
        let model = self.repo.create(new_instrument.into()).await
            .map_err(AppError::DatabaseError)?;

        tracing::info!(instrument_id = model.id, "Created new instrument");
        Ok(model.into())
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<InstrumentResponse>, AppError> {
        self.repo.find_by_id(id).await
            .map_err(AppError::DatabaseError)
            .map(|model| model.map(InstrumentResponse::from))
    }

    pub async fn get_all(&self) -> Result<Vec<InstrumentResponse>, AppError> {
        self.repo.find_all().await
            .map_err(AppError::DatabaseError)
            .map(|models| models.into_iter().map(InstrumentResponse::from).collect())
    }

    pub async fn update(&self, active: UpdateInstrumentRequest) -> Result<InstrumentResponse, AppError> {
        let updated_model = self.repo.update(active.into()).await
            .map_err(AppError::DatabaseError)?;
        
        tracing::info!(instrument_id = updated_model.id, "Updated instrument");
        Ok(updated_model.into())
    }

    pub async fn delete(&self, id: i32) -> Result<bool, AppError> {
        let deleted = self.repo.delete(id).await
            .map_err(AppError::DatabaseError)?;

        if deleted {
            tracing::info!(instrument_id = id, "Deleted instrument");
        }
        Ok(deleted)
    }
}