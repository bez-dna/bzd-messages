use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "streams")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub stream_id: Uuid,
    pub text: String,
    pub message_id: Uuid,
    pub messages_count: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(message_id: Uuid, text: String) -> Self {
        let now = Utc::now().naive_utc();
        let stream_id = Uuid::now_v7();

        Self {
            stream_id,
            text,
            message_id,
            messages_count: 2,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
