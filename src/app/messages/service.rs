use async_nats::jetstream::Context;
use bzd_messages_api::events::message_topic::Type;
use sea_orm::{DbConn, TransactionTrait as _};

use crate::app::{
    error::AppError,
    messages::{
        events,
        repo::{self, MessageModel, MessageStreamModel, MessageTopicModel, MessageUserModel},
        settings::MessagesSettings,
    },
};

pub async fn create_message(
    db: &DbConn,
    _js: &Context,
    _settings: &MessagesSettings,
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
            MessageUserModel::new(source_message.message_id, message.user_id, false),
        )
        .await?;

        repo::create_message_user(
            &tx,
            MessageUserModel::new(source_message.message_id, source_message.user_id, false),
        )
        .await?;

        // TODO: скорее всего стоит вытащить из транзакции, т.к. это по сути неявный лок на запись,
        // а если много юзеров будет постить в один стрим, это будет растягивать транзакцию, а это будет сильнее пул
        // утилизировать
        repo::increase_stream_messages_count(db, message_id).await?;
    } else {
        repo::create_message_user(
            &tx,
            MessageUserModel::new(message.message_id, message.user_id, true),
        )
        .await?;
    };

    tx.commit().await?;

    // TODO: нужно сделать чтобы оно не терялось (и инкриз и отсылку эвентов.. аутбокс?)
    // events::publish_message(db, js, &settings.events, message.message_id, Type::Created).await?;

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

    use crate::app::messages::repo::MessageModel;

    pub struct Request {
        pub message_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub messages: Vec<MessageModel>,
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

    use crate::app::messages::repo::MessageModel;

    pub struct Request {
        pub message_id: Uuid,
    }

    pub struct Response {
        pub message: MessageModel,
    }
}

pub async fn get_message_messages(
    db: &DbConn,
    req: get_message_messages::Request,
    settings: &MessagesSettings,
) -> Result<get_message_messages::Response, AppError> {
    let message = repo::get_message_by_id(db, req.message_id).await?;
    let stream = repo::find_stream_by_message_id(db, req.message_id).await?;

    let limit = settings.limits.message;

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

pub async fn get_user_messages(
    db: &DbConn,
    req: get_user_messages::Request,
    settings: &MessagesSettings,
) -> Result<get_user_messages::Response, AppError> {
    let limit = settings.limits.user;

    let mut messages_users =
        repo::get_messages_users_by_user_id(db, req.user_id, req.cursor_message_id, limit + 1)
            .await?;

    let cursor_message_user =
        if messages_users.len() > usize::try_from(limit).map_err(|_| AppError::Unreachable)? {
            messages_users.pop()
        } else {
            None
        };

    Ok(get_user_messages::Response {
        messages_users,
        cursor_message_user,
    })
}

pub mod get_user_messages {
    use uuid::Uuid;

    use crate::app::messages::repo::MessageUserModel;

    pub struct Request {
        pub user_id: Uuid,
        pub cursor_message_id: Option<Uuid>,
    }

    pub struct Response {
        pub messages_users: Vec<MessageUserModel>,
        pub cursor_message_user: Option<MessageUserModel>,
    }
}

pub async fn get_streams(
    db: &DbConn,
    req: get_streams::Request,
) -> Result<get_streams::Response, AppError> {
    let streams = repo::get_streams_by_message_ids(db, req.message_ids).await?;

    Ok(get_streams::Response { streams })
}

pub mod get_streams {
    use uuid::Uuid;

    use crate::app::messages::repo::{MessageUserModel, StreamModel};

    pub struct Request {
        pub message_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub streams: Vec<(StreamModel, Vec<MessageUserModel>)>,
    }
}

pub async fn get_messages_users(
    db: &DbConn,
    req: get_messages_users::Request,
) -> Result<get_messages_users::Response, AppError> {
    let messages_users = repo::get_messages_users_by_message_ids(db, req.message_ids).await?;

    Ok(get_messages_users::Response { messages_users })
}

pub mod get_messages_users {
    use uuid::Uuid;

    use crate::app::messages::repo::MessageUserModel;

    pub struct Request {
        pub message_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub messages_users: Vec<MessageUserModel>,
    }
}

pub async fn get_messages_topics(
    db: &DbConn,
    req: get_messages_topics::Request,
) -> Result<get_messages_topics::Response, AppError> {
    let messages_topics = repo::get_messages_topics_by_message_ids(db, req.message_ids).await?;

    Ok(get_messages_topics::Response { messages_topics })
}

pub mod get_messages_topics {
    use uuid::Uuid;

    use crate::app::messages::repo::MessageTopicModel;

    pub struct Request {
        pub message_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub messages_topics: Vec<MessageTopicModel>,
    }
}

pub async fn create_message_topic(
    db: &DbConn,
    js: &Context,
    settings: &MessagesSettings,
    req: create_message_topic::Request,
) -> Result<create_message_topic::Response, AppError> {
    let current_user = req.current_user.ok_or(AppError::Forbidden)?;

    let message = repo::get_message_by_id(db, req.message_id).await?;
    current_user.check_access(message.user_id)?;

    let topic = repo::get_topic_by_id(db, req.topic_id).await?;
    current_user.check_access(topic.user_id)?;

    let message_topic = repo::create_message_topic(
        db,
        MessageTopicModel::new(message.message_id, topic.topic_id),
    )
    .await?;

    events::message_topic(js, &settings.events, &message_topic, Type::Created).await?;

    Ok(create_message_topic::Response { message_topic })
}

pub mod create_message_topic {
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::{current_user::CurrentUser, messages::repo::MessageTopicModel};

    #[derive(Validate)]
    pub struct Request {
        pub current_user: Option<CurrentUser>,
        pub message_id: Uuid,
        pub topic_id: Uuid,
    }

    pub struct Response {
        pub message_topic: MessageTopicModel,
    }
}

pub async fn delete_message_topic(
    db: &DbConn,
    js: &Context,
    settings: &MessagesSettings,
    req: delete_message_topic::Request,
) -> Result<(), AppError> {
    let current_user = req.current_user.ok_or(AppError::Forbidden)?;

    let message_topic = repo::get_message_topic_by_id(db, req.message_topic_id).await?;
    let message = repo::get_message_by_id(db, message_topic.message_id).await?;
    current_user.check_access(message.user_id)?;

    repo::delete_message_topic(db, message_topic.clone()).await?;

    events::message_topic(js, &settings.events, &message_topic, Type::Deleted).await?;

    Ok(())
}

pub mod delete_message_topic {
    use uuid::Uuid;

    use crate::app::current_user::CurrentUser;

    pub struct Request {
        pub current_user: Option<CurrentUser>,
        pub message_topic_id: Uuid,
    }
}
