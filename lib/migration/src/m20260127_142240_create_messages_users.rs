use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::MessagesUsers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(MessagesUsers::Table)
                    .col(uuid(MessagesUsers::MessageUserId).primary_key())
                    .col(uuid(MessagesUsers::MessageId))
                    .col(uuid(MessagesUsers::UserId))
                    .col(boolean(MessagesUsers::IsOwned))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_users_message_id_user_id_udx")
                    .unique()
                    .table(MessagesUsers::Table)
                    .col(MessagesUsers::MessageId)
                    .col(MessagesUsers::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_users_user_id_idx")
                    .table(MessagesUsers::Table)
                    .col(MessagesUsers::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_users_message_id_idx")
                    .table(MessagesUsers::Table)
                    .col(MessagesUsers::MessageId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("messages_users_is_owned_idx")
                    .table(MessagesUsers::Table)
                    .col(MessagesUsers::IsOwned)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessagesUsers::Table).to_owned())
            .await
    }
}
