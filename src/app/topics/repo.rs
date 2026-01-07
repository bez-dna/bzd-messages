use sea_orm::{
    ActiveModelTrait, ColumnTrait as _, ConnectionTrait, EntityTrait as _, IntoActiveModel as _,
    ModelTrait, QueryFilter as _, sea_query::OnConflict,
};
use uuid::Uuid;

use crate::app::error::AppError;

mod topic;
mod topic_user;

pub type TopicModel = topic::Model;
pub type TopicUserModel = topic_user::Model;

pub async fn create_topic<T: ConnectionTrait>(
    db: &T,
    model: TopicModel,
) -> Result<TopicModel, AppError> {
    let topic = model.into_active_model().insert(db).await?;

    Ok(topic)
}

pub async fn get_topics_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<TopicModel>, AppError> {
    let topics = topic::Entity::find()
        .filter(topic::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn get_topics_by_ids<T: ConnectionTrait>(
    db: &T,
    topic_ids: Vec<Uuid>,
) -> Result<Vec<TopicModel>, AppError> {
    let topics = topic::Entity::find()
        .filter(topic::Column::TopicId.is_in(topic_ids))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn get_topics_users_by_ids_and_user_id<T: ConnectionTrait>(
    db: &T,
    topic_ids: Vec<Uuid>,
    user_id: Uuid,
) -> Result<Vec<TopicUserModel>, AppError> {
    let topics = topic_user::Entity::find()
        .filter(topic_user::Column::TopicId.is_in(topic_ids))
        .filter(topic_user::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn get_topic_by_id<T: ConnectionTrait>(
    db: &T,
    topic_id: Uuid,
) -> Result<TopicModel, AppError> {
    let topic = topic::Entity::find_by_id(topic_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(topic)
}

pub async fn get_topic_user_by_id<T: ConnectionTrait>(
    db: &T,
    topic_user_id: Uuid,
) -> Result<TopicUserModel, AppError> {
    let topic_user = topic_user::Entity::find_by_id(topic_user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(topic_user)
}

pub async fn create_topic_user<T: ConnectionTrait>(
    db: &T,
    model: TopicUserModel,
) -> Result<TopicUserModel, AppError> {
    topic_user::Entity::insert(model.clone().into_active_model())
        .on_conflict(
            OnConflict::columns([topic_user::Column::TopicId, topic_user::Column::UserId])
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;

    let topic_user = topic_user::Entity::find()
        .filter(topic_user::Column::TopicId.eq(model.topic_id))
        .filter(topic_user::Column::UserId.eq(model.user_id))
        .one(db)
        .await?
        .ok_or(AppError::Unreachable)?;

    Ok(topic_user)
}

pub async fn delete_topic_user<T: ConnectionTrait>(
    db: &T,
    topic_user: TopicUserModel,
) -> Result<(), AppError> {
    topic_user.delete(db).await?;

    Ok(())
}
