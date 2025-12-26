use async_nats::{HeaderMap, jetstream::Context};
use bytes::BytesMut;
use bzd_messages_api::events::topic_user::Type;
use prost::Message;

use crate::app::{
    error::AppError,
    topics::{repo, settings::EventsSettings},
};

pub async fn topic_user(
    js: &Context,
    settings: &EventsSettings,
    topic_user: &repo::topic_user::Model,
    tp: Type,
) -> Result<(), AppError> {
    let subject = settings.topic_user.subject.clone();
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
    use prost_types::Timestamp;

    use crate::app::topics::repo;

    impl From<&repo::topic_user::Model> for bzd_messages_api::events::TopicUser {
        fn from(topic_user: &repo::topic_user::Model) -> Self {
            Self {
                topic_user_id: Some(topic_user.topic_user_id.into()),
                user_id: Some(topic_user.user_id.into()),
                topic_id: Some(topic_user.topic_id.into()),
                created_at: Some(Timestamp {
                    seconds: topic_user.created_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                updated_at: Some(Timestamp {
                    seconds: topic_user.updated_at.and_utc().timestamp(),
                    nanos: 0,
                }),
            }
        }
    }
}
