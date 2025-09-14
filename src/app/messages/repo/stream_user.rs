use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "streams_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub stream_user_id: Uuid,
    pub user_id: Uuid,
    pub stream_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(stream_id: Uuid, user_id: Uuid) -> Self {
        let now = Utc::now().naive_utc();
        let stream_user_id = Uuid::now_v7();

        Self {
            stream_user_id,
            stream_id,
            user_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
