use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Topics {
    Table,
    TopicId,
    Title,
    UserId,
}
