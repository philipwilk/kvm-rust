use sea_orm_migration::prelude::*;

use crate::identities::{Hosts, RunningVms, Uuid, Vms};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RunningVms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RunningVms::Uuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RunningVms::State).boolean().not_null())
                    .col(
                        ColumnDef::new(RunningVms::Host)
                            .custom(Uuid::Uuid)
                            .not_null(),
                    )
                    .col(ColumnDef::new(RunningVms::IsOrphan).boolean().not_null())
                    .col(
                        ColumnDef::new(RunningVms::VmUuid)
                            .custom(Uuid::Uuid)
                            .not_null(),
                    )
                    .col(ColumnDef::new(RunningVms::Fd).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_Host")
                            .from(RunningVms::Table, RunningVms::Host)
                            .to(Hosts::Table, Hosts::Uuid)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_VmUuid")
                            .from(RunningVms::Table, RunningVms::VmUuid)
                            .to(Vms::Table, Vms::Uuid)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RunningVms::Table).to_owned())
            .await
    }
}
