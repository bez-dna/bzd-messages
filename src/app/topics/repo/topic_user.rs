use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "topics_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub topic_user_id: Uuid,
    pub user_id: Uuid,
    pub topic_id: Uuid,
    pub rate: Rate,
    pub timing: Timing,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Text", rename_all = "snake_case")]
pub enum Rate {
    Q,
    Qd,
    Qw,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Text", rename_all = "snake_case")]
pub enum Timing {
    Instant,
    Weekdays,
    Weekends,
}

impl Model {
    pub fn new(user_id: Uuid, topic_id: Uuid) -> Self {
        let now = Utc::now().naive_utc();
        let topic_user_id = Uuid::now_v7();
        let rate = Rate::Q;
        let timing = Timing::Instant;

        Self {
            topic_user_id,
            topic_id,
            user_id,
            rate,
            timing,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
