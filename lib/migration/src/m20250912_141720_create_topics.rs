use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Topics;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Topics::Table)
                    .col(uuid(Topics::TopicId).primary_key())
                    .col(uuid(Topics::UserId))
                    .col(text(Topics::Title))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("topics_user_id_idx")
                    .table(Topics::Table)
                    .col(Topics::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Topics::Table).to_owned())
            .await
    }
}
