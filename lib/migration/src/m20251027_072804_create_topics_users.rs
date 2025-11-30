use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::TopicsUsers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(TopicsUsers::Table)
                    .col(uuid(TopicsUsers::TopicUserId).primary_key())
                    .col(uuid(TopicsUsers::UserId))
                    .col(uuid(TopicsUsers::TopicId))
                    .col(text(TopicsUsers::Timing))
                    .col(text(TopicsUsers::Rate))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("topics_users_topic_id_user_id_udx")
                    .unique()
                    .table(TopicsUsers::Table)
                    .col(TopicsUsers::UserId)
                    .col(TopicsUsers::TopicId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("topics_users_user_id_idx")
                    .table(TopicsUsers::Table)
                    .col(TopicsUsers::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("topics_users_topic_id_idx")
                    .table(TopicsUsers::Table)
                    .col(TopicsUsers::TopicId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TopicsUsers::Table).to_owned())
            .await
    }
}
