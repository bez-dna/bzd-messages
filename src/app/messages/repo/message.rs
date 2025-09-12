use chrono::Utc;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub text: String,
    pub code: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(user_id: Uuid, text: String, code: String) -> Self {
        let now = Utc::now().naive_utc();
        let message_id = Uuid::now_v7();

        Self {
            message_id,
            user_id,
            text,
            code,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
