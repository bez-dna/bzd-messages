use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Topics {
    Table,
    TopicId,
    Title,
    UserId,
}

#[derive(DeriveIden)]
pub enum Messages {
    Table,
    MessageId,
    Text,
    UserId,
    Code,
}

#[derive(DeriveIden)]
pub enum Streams {
    Table,
    StreamId,
    Text,
    MessageId,
    MessagesCount,
}

#[derive(DeriveIden)]
pub enum MessagesStreams {
    Table,
    MessageStreamId,
    MessageId,
    StreamId,
}

#[derive(DeriveIden)]
pub enum MessagesTopics {
    Table,
    MessageTopicId,
    MessageId,
    TopicId,
}

#[derive(DeriveIden)]
pub enum StreamsUsers {
    Table,
    StreamUserId,
    StreamId,
    UserId,
}
