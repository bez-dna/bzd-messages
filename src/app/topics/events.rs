use async_nats::{HeaderMap, jetstream::Context};
use bytes::BytesMut;
use bzd_messages_api::events::topic_user::Type;
use prost::Message;

use crate::app::{
    error::AppError,
    topics::{repo::TopicUserModel, settings::EventsSettings},
};

pub async fn topic_user(
    js: &Context,
    settings: &EventsSettings,
    topic_user: &TopicUserModel,
    tp: Type,
) -> Result<(), AppError> {
    let subject = settings.topics_users.subject.clone();
    let mut buf = BytesMut::new();
    let payload: bzd_messages_api::events::TopicUser = topic_user.into();
    payload.encode(&mut buf)?;

    let mut headers = HeaderMap::new();
    headers.append("ce_type", tp.to_string());

    js.publish_with_headers(subject, headers, buf.into())
        .await?;

    Ok(())
}

mod topic_user {
    use crate::app::{grpc::ToProtoTimestamp as _, topics::repo::TopicUserModel};

    impl From<&TopicUserModel> for bzd_messages_api::events::TopicUser {
        fn from(topic_user: &TopicUserModel) -> Self {
            Self {
                topic_user_id: Some(topic_user.topic_user_id.into()),
                user_id: Some(topic_user.user_id.into()),
                topic_id: Some(topic_user.topic_id.into()),
                created_at: topic_user.created_at.to_option_proto(),
                updated_at: topic_user.updated_at.to_option_proto(),
            }
        }
    }
}
