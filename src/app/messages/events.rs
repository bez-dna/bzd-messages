use bytes::BytesMut;
use bzd_messages_api::events::message::Type;
use prost::Message;
use sea_orm::DbConn;
use uuid::Uuid;

use crate::app::{
    error::AppError,
    mess::JS,
    messages::{events::publish_message::Payload, repo, settings::EventsSettings},
};

pub async fn publish_message(
    db: &DbConn,
    js: &JS,
    settings: &EventsSettings,
    message_id: Uuid,
    tp: Type,
) -> Result<(), AppError> {
    let message = repo::get_message_by_id(db, message_id).await?;
    let topics = repo::get_topics_by_message_id(db, message.message_id).await?;

    let subject = settings.message.subject.clone();
    let mut buf = BytesMut::new();
    let payload: bzd_messages_api::events::Message = Payload { message, topics }.into();
    payload.encode(&mut buf)?;

    let mut headers = async_nats::HeaderMap::new();
    headers.append("ce_type", tp.to_string());

    js.publish_with_headers(subject, headers, buf.into())
        .await?;

    Ok(())
}

mod publish_message {
    use prost_types::Timestamp;

    use crate::app::messages::repo::{MessageModel, TopicModel};

    pub struct Payload {
        pub message: MessageModel,
        pub topics: Vec<TopicModel>,
    }

    impl From<Payload> for bzd_messages_api::events::Message {
        fn from(Payload { message, topics }: Payload) -> Self {
            Self {
                message_id: Some(message.message_id.into()),
                text: message.text.clone().into(),
                user_id: Some(message.user_id.into()),
                code: message.code.clone().into(),
                created_at: Some(Timestamp {
                    seconds: message.created_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                updated_at: Some(Timestamp {
                    seconds: message.updated_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                topic_ids: topics.iter().map(|it| it.topic_id.into()).collect(),
            }
        }
    }
}
