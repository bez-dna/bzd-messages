use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "topics")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub topic_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(user_id: Uuid, title: String) -> Self {
        let now = Utc::now().naive_utc();
        let topic_id = Uuid::now_v7();

        Self {
            topic_id,
            user_id,
            title,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
