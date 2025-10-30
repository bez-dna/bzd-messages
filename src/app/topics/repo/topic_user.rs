use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "topics_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub topic_user_id: Uuid,
    pub user_id: Uuid,
    pub topic_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(user_id: Uuid, topic_id: Uuid) -> Self {
        let now = Utc::now().naive_utc();
        let topic_user_id = Uuid::now_v7();

        Self {
            topic_user_id,
            topic_id,
            user_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
