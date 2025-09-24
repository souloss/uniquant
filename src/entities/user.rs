use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 哈希密码
    pub password_hash: String,

    /// 用户角色（普通用户/超级用户）
    pub role: String,

    /// 创建时间
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 以后可以加：#[sea_orm(has_many = "super::user_favorite_instrument::Entity")]
}

impl ActiveModelBehavior for ActiveModel {}