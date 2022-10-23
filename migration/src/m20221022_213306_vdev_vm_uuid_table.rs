use sea_orm_migration::prelude::*;

use crate::identities::{VdevsVmUuid, Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VdevsVmUuid::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VdevsVmUuid::Uuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VdevsVmUuid::DeviceOrder).small_unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VdevsVmUuid::Table).to_owned())
            .await
    }
}
