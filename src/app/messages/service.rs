use async_nats::jetstream::Context;
use bzd_messages_api::events::message::Type;
use sea_orm::{DbConn, TransactionTrait as _};

use crate::app::{
    error::AppError,
    messages::{
        events,
        repo::{self, MessageModel, MessageStreamModel, StreamUserModel},
        settings::MessagesSettings,
    },
};

pub async fn create_message(
    db: &DbConn,
    js: &Context,
    settings: &MessagesSettings,
    req: create_message::Request,
) -> Result<create_message::Response, AppError> {
    let current_user = req.current_user.ok_or(AppError::Forbidden)?;

    let tx = db.begin().await?;

    let message = MessageModel::new(current_user.user_id, req.text, req.code);
    let message = repo::create_message(&tx, message).await?;

    let stream = match req.tp {
        create_message::Type::TopicIds(topic_ids) => {
            let topics =
                repo::get_topics_by_ids_and_user_id(&tx, topic_ids.clone(), current_user.user_id)
                    .await?;

            if topics.len() < 1 || topics.len() != topic_ids.len() {
                return Err(AppError::Other);
            }

            for topic_id in topic_ids {
                repo::create_message_topic(
                    &tx,
                    repo::message_topic::Model::new(message.message_id, topic_id),
                )
                .await?;
            }

            None
        }
        create_message::Type::MessageId(message_id) => {
            let source_message = repo::get_message_by_id(&tx, message_id).await?;

            // TODO: нужно воткнуть проверку чтобы не создавать 2 более уровень вложенности пока нет саммари.

            let stream = repo::stream::Model::new(message_id, source_message.text.clone());
            let stream = repo::create_stream(&tx, stream)
                .await?
                .ok_or(AppError::Unreachable)?;

            repo::create_message_stream(
                &tx,
                MessageStreamModel::new(source_message.message_id, stream.stream_id),
            )
            .await?;

            repo::create_message_stream(
                &tx,
                MessageStreamModel::new(message.message_id, stream.stream_id),
            )
            .await?;

            repo::create_stream_user(
                &tx,
                StreamUserModel::new(stream.stream_id, current_user.user_id),
            )
            .await?;

            repo::create_stream_user(
                &tx,
                StreamUserModel::new(stream.stream_id, source_message.user_id),
            )
            .await?;

            Some(stream)
        }
    };

    tx.commit().await?;

    // TODO: нужно сделать чтобы оно не терялось (и инкриз и отсылку эвентов.. аутбокс?)

    if let Some(stream) = stream.clone() {
        repo::increase_stream_messages_count(db, stream.stream_id).await?;
    }

    events::publish_message(db, js, &settings.events, message.message_id, Type::Created).await?;

    Ok(create_message::Response { message })
}

pub mod create_message {
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::{current_user::CurrentUser, messages::repo::message};

    #[derive(Validate)]
    pub struct Request {
        pub current_user: Option<CurrentUser>,
        #[validate(length(min = 2))]
        pub text: String,
        #[validate(length(min = 2))]
        pub code: String,
        pub tp: Type,
        // pub message_id: Option<Uuid>,
    }

    pub enum Type {
        TopicIds(Vec<Uuid>),
        MessageId(Uuid),
    }

    pub struct Response {
        pub message: message::Model,
        // pub stream: Option<repo::stream::Model>,
    }
}

pub async fn get_messages(
    db: &DbConn,
    req: get_messages::Request,
) -> Result<get_messages::Response, AppError> {
    let messages = repo::get_messages_by_ids(db, req.message_ids).await?;

    Ok(get_messages::Response { messages })
}

pub mod get_messages {
    use uuid::Uuid;

    use crate::app::messages::repo::message;

    pub struct Request {
        pub message_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub messages: Vec<message::Model>,
    }
}

pub async fn get_message(
    db: &DbConn,
    req: get_message::Request,
) -> Result<get_message::Response, AppError> {
    let message = repo::get_message_by_id(db, req.message_id).await?;

    Ok(get_message::Response { message })
}

pub mod get_message {
    use uuid::Uuid;

    use crate::app::messages::repo::message;

    pub struct Request {
        pub message_id: Uuid,
    }

    pub struct Response {
        pub message: message::Model,
    }
}

pub async fn get_message_messages(
    db: &DbConn,
    req: get_message_messages::Request,
    settings: &MessagesSettings,
) -> Result<get_message_messages::Response, AppError> {
    let message = repo::get_message_by_id(db, req.message_id).await?;
    let stream = repo::find_stream_by_message_id(db, req.message_id).await?;

    let limit = settings.message_messages_limit;

    let mut messages = match stream {
        Some(stream) => {
            repo::get_messages_by_stream_id(
                db,
                stream.stream_id,
                req.cursor_message_id,
                (limit + 1) as u64,
            )
            .await?
        }
        None => vec![message],
    };

    let cursor_message = if messages.len() > limit as usize {
        Some(messages.remove(0))
    } else {
        None
    };

    messages.reverse();

    Ok(get_message_messages::Response {
        messages,
        cursor_message,
    })
}

pub mod get_message_messages {
    use uuid::Uuid;

    use crate::app::messages::repo::message;

    pub struct Request {
        pub message_id: Uuid,
        pub cursor_message_id: Option<Uuid>,
    }

    pub struct Response {
        pub messages: Vec<message::Model>,
        pub cursor_message: Option<message::Model>,
    }
}
