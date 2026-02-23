use sea_orm::{
    ActiveModelTrait as _, ColumnTrait as _, ConnectionTrait, EntityTrait as _,
    IntoActiveModel as _, JoinType, ModelTrait as _, QueryFilter as _, QueryOrder as _,
    QuerySelect as _, QueryTrait as _, prelude::Expr, sea_query::OnConflict,
};
use uuid::Uuid;

use crate::app::error::AppError;

pub mod message;
pub mod message_stream;
pub mod message_topic;
pub mod message_user;
pub mod stream;
pub mod topic;

pub type MessageModel = message::Model;
pub type TopicModel = topic::Model;
pub type MessageStreamModel = message_stream::Model;
pub type MessageTopicModel = message_topic::Model;
pub type MessageUserModel = message_user::Model;
pub type StreamModel = stream::Model;

pub async fn create_message<T: ConnectionTrait>(
    db: &T,
    model: MessageModel,
) -> Result<MessageModel, AppError> {
    let message = model.into_active_model().insert(db).await?;

    Ok(message)
}

pub async fn get_message_by_id<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<MessageModel, AppError> {
    let message = message::Entity::find_by_id(message_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

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

pub async fn create_message_user<T: ConnectionTrait>(
    db: &T,
    model: MessageUserModel,
) -> Result<(), AppError> {
    message_user::Entity::insert(model.into_active_model())
        .on_conflict(OnConflict::new().do_nothing().to_owned())
        .do_nothing()
        .exec(db)
        .await?;

    Ok(())
}

pub async fn find_stream_by_message_id<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<Option<stream::Model>, AppError> {
    let stream = stream::Entity::find()
        .filter(stream::Column::MessageId.eq(message_id))
        .one(db)
        .await?;

    Ok(stream)
}

pub async fn get_messages_by_stream_id<T: ConnectionTrait>(
    db: &T,
    stream_id: Uuid,
    cursor_message_id: Option<Uuid>,
    limit: u64,
) -> Result<Vec<message::Model>, AppError> {
    let messages = message::Entity::find()
        .join(
            JoinType::InnerJoin,
            message::Entity::belongs_to(message_stream::Entity)
                .to(message_stream::Column::MessageId)
                .from(message::Column::MessageId)
                .into(),
        )
        .filter(message_stream::Column::StreamId.eq(stream_id))
        .apply_if(cursor_message_id, |query, v| {
            query.filter(message::Column::MessageId.lte(v))
        })
        .cursor_by(message::Column::MessageId)
        .last(limit)
        .all(db)
        .await?;

    Ok(messages)
}

pub async fn increase_stream_messages_count<T: ConnectionTrait>(
    db: &T,
    message_id: Uuid,
) -> Result<(), AppError> {
    stream::Entity::update_many()
        .col_expr(
            stream::Column::MessagesCount,
            Expr::col(stream::Column::MessagesCount).add(1),
        )
        .filter(stream::Column::MessageId.eq(message_id))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn get_messages_users_by_user_id<T: ConnectionTrait>(
    db: &T,
    user_id: Uuid,
    cursor_message_id: Option<Uuid>,
    limit: u64,
) -> Result<Vec<MessageUserModel>, AppError> {
    let messages_users = message_user::Entity::find()
        .filter(message_user::Column::UserId.eq(user_id))
        .filter(message_user::Column::IsOwned.eq(true))
        .apply_if(cursor_message_id, |query, v| {
            query.filter(message_user::Column::MessageUserId.lte(v))
        })
        .order_by_desc(message_user::Column::MessageUserId)
        .limit(limit)
        .all(db)
        .await?;

    Ok(messages_users)
}

pub async fn get_streams_by_message_ids<T: ConnectionTrait>(
    db: &T,
    message_ids: Vec<Uuid>,
) -> Result<Vec<(StreamModel, Vec<MessageUserModel>)>, AppError> {
    let streams = stream::Entity::find()
        .select_with(message_user::Entity)
        .join(
            JoinType::LeftJoin,
            stream::Entity::belongs_to(message_user::Entity)
                .to(message_user::Column::MessageId)
                .from(stream::Column::MessageId)
                .into(),
        )
        .filter(stream::Column::MessageId.is_in(message_ids))
        .all(db)
        .await?;

    Ok(streams)
}

pub async fn get_messages_users_by_message_ids<T: ConnectionTrait>(
    db: &T,
    message_ids: Vec<Uuid>,
) -> Result<Vec<MessageUserModel>, AppError> {
    let messages_users = message_user::Entity::find()
        .filter(message_user::Column::MessageId.is_in(message_ids))
        .all(db)
        .await?;

    Ok(messages_users)
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

pub async fn get_messages_topics_by_message_ids_and_topics_ids<T: ConnectionTrait>(
    db: &T,
    message_ids: Vec<Uuid>,
    topic_ids: Vec<Uuid>,
) -> Result<Vec<MessageTopicModel>, AppError> {
    let messages_topics = message_topic::Entity::find()
        .filter(message_topic::Column::MessageId.is_in(message_ids))
        .filter(message_topic::Column::TopicId.is_in(topic_ids))
        .all(db)
        .await?;

    Ok(messages_topics)
}

pub async fn create_message_topic<T: ConnectionTrait>(
    db: &T,
    model: MessageTopicModel,
) -> Result<MessageTopicModel, AppError> {
    message_topic::Entity::insert(model.clone().into_active_model())
        .on_conflict(
            OnConflict::columns([
                message_topic::Column::MessageId,
                message_topic::Column::TopicId,
            ])
            .do_nothing()
            .to_owned(),
        )
        .do_nothing()
        .exec(db)
        .await?;

    let message_topic = message_topic::Entity::find()
        .filter(message_topic::Column::TopicId.eq(model.topic_id))
        .filter(message_topic::Column::MessageId.eq(model.message_id))
        .one(db)
        .await?
        .ok_or(AppError::Unreachable)?;

    Ok(message_topic)
}

pub async fn get_message_topic_by_id<T: ConnectionTrait>(
    db: &T,
    message_topic_id: Uuid,
) -> Result<MessageTopicModel, AppError> {
    let message_topic = message_topic::Entity::find_by_id(message_topic_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(message_topic)
}

pub async fn delete_message_topic<T: ConnectionTrait>(
    db: &T,
    message_topic: MessageTopicModel,
) -> Result<(), AppError> {
    message_topic.delete(db).await?;

    Ok(())
}
