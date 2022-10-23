use sea_orm_migration::prelude::*;

use crate::identities::{Hosts, Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Hosts::Table)
                    .col(
                        ColumnDef::new(Hosts::Uuid)
                            .custom(Uuid::Uuid)
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Hosts::FriendlyName).text().not_null())
                    .col(ColumnDef::new(Hosts::Memory).unsigned().not_null())
                    .col(ColumnDef::new(Hosts::Cpus).small_unsigned().not_null())
                    .col(ColumnDef::new(Hosts::Arch).text().not_null())
                    .col(ColumnDef::new(Hosts::Ip).text().not_null())
                    .col(ColumnDef::new(Hosts::IsManager).boolean().not_null())
                    .col(ColumnDef::new(Hosts::State).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Hosts::Table).to_owned())
            .await
    }
}
