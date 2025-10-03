use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Instrument::Table)
                    .if_not_exists()
                    .col(pk_auto(Instrument::Id))
                    .col(string(Instrument::Exchange).not_null())
                    .col(string(Instrument::Symbol).not_null())
                    .col(string(Instrument::AssetType).not_null())
                    .col(string(Instrument::Name).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Instrument::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Instrument {
    Table,
    Id,
    Exchange,
    Symbol,
    AssetType,
    Name,
}