pub mod instrument;

use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, DbErr, DeleteResult, EntityTrait, IntoActiveModel, PaginatorTrait, PrimaryKeyTrait, QueryFilter
};
use async_trait::async_trait;

/// 通用 CRUD Trait
#[async_trait]
pub trait Repository<E>
where
    E: EntityTrait,
    E::Model: IntoActiveModel<E::ActiveModel> + Send + Sync,
    E::ActiveModel: ActiveModelTrait<Entity = E> + Send + Sync,
{
    /// 获取数据库连接
    fn conn(&self) -> &DatabaseConnection;

    /// 创建
    async fn create(&self, active: E::ActiveModel) -> Result<E::Model, DbErr> {
        active.insert(self.conn()).await
    }

    /// 根据主键查找
    async fn find_by_id(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) 
        -> Result<Option<E::Model>, DbErr> 
    {
        E::find_by_id(id).one(self.conn()).await
    }

    /// 查找所有
    async fn find_all(&self) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(self.conn()).await
    }

    /// 更新
    async fn update(&self, active: E::ActiveModel) -> Result<E::Model, DbErr> {
        active.update(self.conn()).await
    }

    /// 删除
    async fn delete(&self, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<bool, DbErr> {
        let res = E::delete_by_id(id).exec(self.conn()).await?;
        Ok(res.rows_affected > 0)
    }

    // --- 常见扩展方法 ---
    /// 条件查询
    async fn find_by_condition(&self, cond: Condition) -> Result<Vec<E::Model>, DbErr> {
        E::find().filter(cond).all(self.conn()).await
    }

    /// 根据条件查找第一个匹配的记录
    async fn find_one_by_condition(&self, condition: Condition) -> Result<Option<E::Model>, DbErr> {
        E::find().filter(condition).one(self.conn()).await
    }

    /// 分页查询
    async fn paginate(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<Vec<E::Model>, DbErr> {
        let paginator = E::find().paginate(self.conn(), per_page);
        let models = paginator.fetch_page(page).await?;
        Ok(models)
    }

    /// 检查某条记录是否存在
    async fn exists(&self, cond: Condition) -> Result<bool, DbErr> {
        Ok(E::find().filter(cond).one(self.conn()).await?.is_some())
    }

    /// Upsert（如果存在则更新，否则插入）
    async fn upsert(&self, active: E::ActiveModel) -> Result<E::Model, DbErr> {
        match active.clone().update(self.conn()).await {
            Ok(model) => Ok(model),
            Err(_) => active.insert(self.conn()).await,
        }
    }

    // --- 计数功能 ---
    /// 统计所有记录总数
    async fn count(&self) -> Result<u64, DbErr> {
        E::find().count(self.conn()).await
    }
    /// 根据条件批量删除
    async fn delete_many(&self, condition: Condition) -> Result<DeleteResult, DbErr> {
        E::delete_many().filter(condition).exec(self.conn()).await
    }
}