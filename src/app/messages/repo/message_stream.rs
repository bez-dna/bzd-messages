use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "messages_streams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub message_stream_id: Uuid,
    pub message_id: Uuid,
    pub stream_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(message_id: Uuid, stream_id: Uuid) -> Self {
        let now = Utc::now().naive_utc();
        let message_stream_id = Uuid::now_v7();

        Self {
            message_stream_id,
            stream_id,
            message_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
