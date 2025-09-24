use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "instrument")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 交易所 ID（如 Binance, NYSE, NASDAQ）
    pub exchange: String,

    /// 标的符号（如 BTC/USDT, AAPL）
    pub symbol: String,

    /// 标的类型（如 stock, crypto, etf, future）
    pub asset_type: String,

    /// 标的名称（可选，如 Apple Inc.）
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 以后可以加：#[sea_orm(has_many = "super::kline::Entity")]
}

impl ActiveModelBehavior for ActiveModel {}