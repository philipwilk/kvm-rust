use sea_orm_migration::prelude::*;

use crate::identities::{Vdevs, Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vdevs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vdevs::Uuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Vdevs::Address).text().not_null())
                    .col(ColumnDef::new(Vdevs::Host).custom(Uuid::Uuid).not_null())
                    .col(ColumnDef::new(Vdevs::Type).text().not_null())
                    .col(ColumnDef::new(Vdevs::AddressEnd).unsigned().not_null())
                    .col(ColumnDef::new(Vdevs::IsSystem).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vdevs::Table).to_owned())
            .await
    }
}
