use sea_orm::{DbConn, TransactionTrait as _};

use crate::app::{error::AppError, messages::repo};

pub async fn create_message(
    db: &DbConn,
    req: create_message::Request,
) -> Result<create_message::Response, AppError> {
    let tx = db.begin().await?;

    let message = repo::message::Model::new(req.user_id, req.text, req.code);
    let message = repo::create_message(&tx, message).await?;

    match req.tp {
        create_message::Type::TopicIds(topic_ids) => {
            let topics =
                repo::get_topics_by_ids_and_user_id(&tx, topic_ids.clone(), req.user_id).await?;

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
        }
        create_message::Type::MessageId(message_id) => {
            let source_message = repo::get_message_by_id(&tx, message_id)
                .await?
                .ok_or(AppError::NotFound)?;

            // TODO: нужно воткнуть проверку чтобы не создавать 2 более уровень вложенности пока нет саммари.

            let stream = repo::stream::Model::new(message_id, source_message.text.clone());
            let stream = repo::create_stream(&tx, stream)
                .await?
                .ok_or(AppError::Unreachable)?;

            repo::create_message_stream(
                &tx,
                repo::message_stream::Model::new(source_message.message_id, stream.stream_id),
            )
            .await?;

            repo::create_message_stream(
                &tx,
                repo::message_stream::Model::new(message.message_id, stream.stream_id),
            )
            .await?;

            repo::create_stream_user(
                &tx,
                repo::stream_user::Model::new(stream.stream_id, req.user_id),
            )
            .await?;

            repo::create_stream_user(
                &tx,
                repo::stream_user::Model::new(stream.stream_id, source_message.user_id),
            )
            .await?;
        }
    };

    tx.commit().await?;

    Ok(create_message::Response { message })
}

pub mod create_message {
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::messages::repo;

    #[derive(Validate)]
    pub struct Request {
        pub user_id: Uuid,
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
        pub message: repo::message::Model,
        // pub stream: Option<repo::stream::Model>,
    }
}
