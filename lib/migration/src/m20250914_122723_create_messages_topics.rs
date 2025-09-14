use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::MessagesTopics;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(MessagesTopics::Table)
                    .col(uuid(MessagesTopics::MessageTopicId).primary_key())
                    .col(uuid(MessagesTopics::MessageId))
                    .col(uuid(MessagesTopics::TopicId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_topics_message_id_topic_id_udx")
                    .unique()
                    .table(MessagesTopics::Table)
                    .col(MessagesTopics::MessageId)
                    .col(MessagesTopics::TopicId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_topics_message_id_idx")
                    .table(MessagesTopics::Table)
                    .col(MessagesTopics::MessageId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_topics_topic_id_idx")
                    .table(MessagesTopics::Table)
                    .col(MessagesTopics::TopicId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessagesTopics::Table).to_owned())
            .await
    }
}
