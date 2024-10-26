use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Title,
    Completed,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(uuid(Todo::Id).not_null().primary_key())
                    .col(string(Todo::Title).not_null())
                    .col(boolean(Todo::Completed).not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}