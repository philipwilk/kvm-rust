use sea_orm_migration::prelude::*;

use crate::identities::{VdisksVmUuid, Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VdisksVmUuid::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VdisksVmUuid::VDiskUuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VdisksVmUuid::BootOrder).small_unsigned().not_null())
                    
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VdisksVmUuid::Table).to_owned())
            .await
    }
}
