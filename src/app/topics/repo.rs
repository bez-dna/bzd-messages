use sea_orm::{
    ActiveModelTrait, ColumnTrait as _, ConnectionTrait, EntityTrait as _, IntoActiveModel as _,
    QueryFilter as _,
};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod topic;

pub async fn create_topic<T: ConnectionTrait>(
    db: &T,
    model: topic::Model,
) -> Result<topic::Model, AppError> {
    let topic = model.into_active_model().insert(db).await?;

    Ok(topic)
}

pub async fn get_topics_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<topic::Model>, AppError> {
    let topics = topic::Entity::find()
        .filter(topic::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(topics)
}
