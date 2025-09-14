use sea_orm::{
    ActiveModelTrait as _, ColumnTrait as _, ConnectionTrait, EntityTrait as _,
    IntoActiveModel as _, QueryFilter as _, sea_query::OnConflict,
};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod message;
pub mod message_stream;
pub mod message_topic;
pub mod stream;
pub mod stream_user;
pub mod topic;

pub async fn create_message<T: ConnectionTrait>(
    db: &T,
    model: message::Model,
) -> Result<message::Model, AppError> {
    let message = model.into_active_model().insert(db).await?;

    Ok(message)
}

pub async fn get_message_by_id<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<Option<message::Model>, AppError> {
    let message = message::Entity::find_by_id(message_id).one(db).await?;

    Ok(message)
}

pub async fn create_stream<T: ConnectionTrait>(
    db: &T,
    model: stream::Model,
) -> Result<Option<stream::Model>, AppError> {
    let message_id = model.message_id;

    stream::Entity::insert(model.into_active_model())
        .on_conflict(
            OnConflict::column(stream::Column::MessageId)
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;

    let stream = stream::Entity::find()
        .filter(stream::Column::MessageId.eq(message_id))
        .one(db)
        .await?;

    Ok(stream)
}

pub async fn create_message_stream<T: ConnectionTrait>(
    db: &T,
    model: message_stream::Model,
) -> Result<(), AppError> {
    message_stream::Entity::insert(model.into_active_model())
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db)
        .await?;
    Ok(())
}

pub async fn create_stream_user<T: ConnectionTrait>(
    db: &T,
    model: stream_user::Model,
) -> Result<(), AppError> {
    stream_user::Entity::insert(model.into_active_model())
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db)
        .await?;

    Ok(())
}

pub async fn get_topics_by_ids_and_user_id<T: ConnectionTrait>(
    db: &T,
    topic_ids: Vec<Uuid>,
    user_id: Uuid,
) -> Result<Vec<topic::Model>, AppError> {
    let topics = topic::Entity::find()
        .filter(topic::Column::UserId.eq(user_id))
        .filter(topic::Column::TopicId.is_in(topic_ids))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn create_message_topic<T: ConnectionTrait>(
    db: &T,
    model: message_topic::Model,
) -> Result<(), AppError> {
    message_topic::Entity::insert(model.into_active_model())
        .exec(db)
        .await?;
    Ok(())
}
