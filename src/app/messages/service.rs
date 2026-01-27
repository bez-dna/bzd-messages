use async_nats::jetstream::Context;
use bzd_messages_api::events::message::Type;
use sea_orm::{DbConn, TransactionTrait as _};

use crate::app::{
    error::AppError,
    messages::{
        events,
        repo::{self, MessageModel, MessageStreamModel, MessageUserModel},
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

    let message = MessageModel::new(current_user.user_id, req.text, req.code.to_string());
    let message = repo::create_message(&tx, message).await?;

    if let Some(message_id) = req.message_id {
        let source_message = repo::get_message_by_id(&tx, message_id).await?;

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

        repo::create_message_user(
            &tx,
            MessageUserModel::new(source_message.message_id, message.user_id),
        )
        .await?;
    } else {
        repo::create_message_user(
            &tx,
            MessageUserModel::new(message.message_id, message.user_id),
        )
        .await?;
    };

    tx.commit().await?;

    // TODO: нужно сделать чтобы оно не терялось (и инкриз и отсылку эвентов.. аутбокс?)

    repo::increase_stream_messages_count(db, message.message_id).await?;
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
        pub code: Uuid,
        pub message_id: Option<Uuid>,
    }

    pub struct Response {
        pub message: message::Model,
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
