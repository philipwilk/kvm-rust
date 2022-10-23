use sea_orm_migration::prelude::*;

use crate::identities::{Vms, Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vms::Uuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Vms::FriendlyName).text().not_null())
                    .col(ColumnDef::new(Vms::Memory).unsigned().not_null())
                    .col(ColumnDef::new(Vms::VCpus).small_unsigned().not_null())
                    .col(ColumnDef::new(Vms::BootDeviceType).text().not_null())
                    .col(ColumnDef::new(Vms::VDisksUuid).custom(Uuid::Uuid).not_null())
                    .col(ColumnDef::new(Vms::Template).custom(Uuid::Uuid).not_null())
                    .col(ColumnDef::new(Vms::VDevsUuid).custom(Uuid::Uuid).not_null())
                    .col(ColumnDef::new(Vms::Arch).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vms::Table).to_owned())
            .await
    }
}
