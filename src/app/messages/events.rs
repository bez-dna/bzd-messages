use async_nats::jetstream::Context;
use bytes::BytesMut;
use prost::Message;

use crate::app::{
    error::AppError,
    messages::{repo, settings::EventsSettings},
};

pub async fn message(
    js: &Context,
    settings: &EventsSettings,
    message: &repo::message::Model,
) -> Result<(), AppError> {
    let subject = settings.message.subject.clone();
    let mut buf = BytesMut::new();
    let payload: bzd_messages_api::events::Message = message.into();
    payload.encode(&mut buf)?;

    js.publish(subject, buf.into()).await?;

    Ok(())
}

mod message {
    use prost_types::Timestamp;

    use crate::app::messages::repo;

    impl From<&repo::message::Model> for bzd_messages_api::events::Message {
        fn from(message: &repo::message::Model) -> Self {
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
            }
        }
    }
}
