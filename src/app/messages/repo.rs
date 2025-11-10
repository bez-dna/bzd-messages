use sea_orm::{
    ActiveModelTrait as _, ColumnTrait as _, ConnectionTrait, EntityTrait as _,
    IntoActiveModel as _, JoinType, QueryFilter as _, QuerySelect as _, sea_query::OnConflict,
};
use uuid::Uuid;

use crate::app::{error::AppError, topics::repo::topic_user};

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

pub async fn get_messages_by_ids<T: ConnectionTrait>(
    db: &T,
    message_ids: Vec<Uuid>,
) -> Result<Vec<message::Model>, AppError> {
    let messages = message::Entity::find()
        .filter(message::Column::MessageId.is_in(message_ids))
        .all(db)
        .await?;

    Ok(messages)
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

pub async fn get_topics_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
) -> Result<Vec<topic::Model>, AppError> {
    let topics = topic::Entity::find()
        .join(
            JoinType::InnerJoin,
            topic::Entity::belongs_to(topic_user::Entity)
                .to(topic_user::Column::TopicId)
                .from(topic::Column::TopicId)
                .into(),
        )
        .filter(topic_user::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(topics)
}

pub async fn get_messages_by_topic_ids<T: ConnectionTrait>(
    db: &T,
    topic_ids: Vec<Uuid>,
) -> Result<Vec<message::Model>, AppError> {
    let messages = message::Entity::find()
        .join(
            JoinType::InnerJoin,
            message::Entity::belongs_to(message_topic::Entity)
                .to(message_topic::Column::MessageId)
                .from(message::Column::MessageId)
                .into(),
        )
        .filter(message_topic::Column::TopicId.is_in(topic_ids))
        .all(db)
        .await?;

    Ok(messages)
}
