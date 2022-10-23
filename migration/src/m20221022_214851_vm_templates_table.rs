use sea_orm_migration::prelude::*;

use crate::identities::{VmTemplates, Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VmTemplates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VmTemplates::Uuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VmTemplates::FriendlyName).text().not_null())
                    .col(ColumnDef::new(VmTemplates::Memory).unsigned().not_null())
                    .col(ColumnDef::new(VmTemplates::Vcpus).small_integer().not_null())
                    .col(ColumnDef::new(VmTemplates::BootDeviceType).text().not_null())
                    .col(ColumnDef::new(VmTemplates::VDevsUuid).custom(Uuid::Uuid).not_null())
                    .col(ColumnDef::new(VmTemplates::VDisksUuid).custom(Uuid::Uuid).not_null())
                    .col(ColumnDef::new(VmTemplates::Arch).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VmTemplates::Table).to_owned())
            .await
    }
}
