use serde::{Deserialize, Serialize};
use validator::Validate;
use entities::instrument;

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateInstrumentRequest {
    #[validate(length(min = 1, max = 50))]
    pub exchange: String,
    #[validate(length(min = 1, max = 20))]
    pub symbol: String,
    #[validate(length(min = 1, max = 20))]
    pub asset_type: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct UpdateInstrumentRequest {
    pub id: i32,
    #[validate(length(min = 1, max = 50))]
    pub exchange: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub symbol: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub asset_type: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct InstrumentResponse {
    pub id: i32,
    pub exchange: String,
    pub symbol: String,
    pub asset_type: String,
    pub name: String,
}

// 用于分页查询的过滤器
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct InstrumentFilter {
    pub exchange: Option<String>,
    pub symbol: Option<String>,
    pub asset_type: Option<String>,
}


// CreateInstrumentRequest -> ActiveModel
impl From<CreateInstrumentRequest> for instrument::ActiveModel {
    fn from(req: CreateInstrumentRequest) -> Self {
        Self {
            id: sea_orm::ActiveValue::NotSet,
            exchange: sea_orm::ActiveValue::Set(req.exchange),
            symbol: sea_orm::ActiveValue::Set(req.symbol),
            asset_type: sea_orm::ActiveValue::Set(req.asset_type),
            name: sea_orm::ActiveValue::Set(req.name),
        }
    }
}

// UpdateInstrumentRequest -> ActiveModel
impl From<UpdateInstrumentRequest> for instrument::ActiveModel {
    fn from(req: UpdateInstrumentRequest) -> Self {
        Self {
            // ⚠️ 注意：这里 id 通常需要单独传入，而不是从 DTO 填
            id: Default::default(),
            exchange: match req.exchange {
                Some(v) => sea_orm::ActiveValue::Set(v),
                None => Default::default(),
            },
            symbol: match req.symbol {
                Some(v) => sea_orm::ActiveValue::Set(v),
                None => Default::default(),
            },
            asset_type: match req.asset_type {
                Some(v) => sea_orm::ActiveValue::Set(v),
                None => Default::default(),
            },
            name: match req.name {
                Some(v) => sea_orm::ActiveValue::Set(v),
                None => Default::default(),
            },
        }
    }
}

// Model -> InstrumentResponse
impl From<instrument::Model> for InstrumentResponse {
    fn from(model: instrument::Model) -> Self {
        Self {
            id: model.id,
            exchange: model.exchange,
            symbol: model.symbol,
            asset_type: model.asset_type,
            name: model.name,
        }
    }
}

// InstrumentResponse -> Model （可选）
impl From<InstrumentResponse> for instrument::Model {
    fn from(resp: InstrumentResponse) -> Self {
        Self {
            id: resp.id,
            exchange: resp.exchange,
            symbol: resp.symbol,
            asset_type: resp.asset_type,
            name: resp.name,
        }
    }
}