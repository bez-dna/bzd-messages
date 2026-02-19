use async_nats::{HeaderMap, jetstream::Context};
use bytes::BytesMut;
use bzd_messages_api::events::message::Type;
use prost::Message;

use crate::app::{
    error::AppError,
    messages::{repo::MessageTopicModel, settings::EventsSettings},
};

pub async fn message_topic(
    js: &Context,
    settings: &EventsSettings,
    message_topic: &MessageTopicModel,
    tp: Type,
) -> Result<(), AppError> {
    let subject = settings.messages_topics.subject.clone();
    let mut buf = BytesMut::new();
    let payload: bzd_messages_api::events::MessageTopic = message_topic.into();
    payload.encode(&mut buf)?;

    let mut headers = HeaderMap::new();
    headers.append("ce_type", tp.to_string());

    js.publish_with_headers(subject, headers, buf.into())
        .await?;

    Ok(())
}

mod message_topic {
    use crate::app::{grpc::ToProtoTimestamp as _, messages::repo::MessageTopicModel};

    impl From<&MessageTopicModel> for bzd_messages_api::events::MessageTopic {
        fn from(message_topic: &MessageTopicModel) -> Self {
            Self {
                message_topic_id: Some(message_topic.message_topic_id.into()),
                message_id: Some(message_topic.message_id.into()),
                topic_id: Some(message_topic.topic_id.into()),
                created_at: message_topic.created_at.to_option_proto(),
                updated_at: message_topic.updated_at.to_option_proto(),
            }
        }
    }
}
