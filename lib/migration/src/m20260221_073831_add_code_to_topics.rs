use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Topics;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Topics::Table)
                    .add_column_if_not_exists(text(Topics::Code))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("topics_code_user_id_udx")
                    .unique()
                    .table(Topics::Table)
                    .col(Topics::Code)
                    .col(Topics::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Topics::Table)
                    .drop_column(Topics::Code)
                    .to_owned(),
            )
            .await
    }
}
