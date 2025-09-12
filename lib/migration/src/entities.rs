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
