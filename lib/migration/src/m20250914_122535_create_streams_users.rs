use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::StreamsUsers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(StreamsUsers::Table)
                    .col(uuid(StreamsUsers::StreamUserId).primary_key())
                    .col(uuid(StreamsUsers::UserId))
                    .col(uuid(StreamsUsers::StreamId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("streams_users_stream_id_user_id_udx")
                    .unique()
                    .table(StreamsUsers::Table)
                    .col(StreamsUsers::UserId)
                    .col(StreamsUsers::StreamId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("streams_users_user_id_idx")
                    .table(StreamsUsers::Table)
                    .col(StreamsUsers::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("streams_users_stream_id_idx")
                    .table(StreamsUsers::Table)
                    .col(StreamsUsers::StreamId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StreamsUsers::Table).to_owned())
            .await
    }
}
