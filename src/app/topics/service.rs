use sea_orm::DbConn;

use crate::app::{error::AppError, topics::repo};

pub async fn create_topic(
    db: &DbConn,
    req: create_topic::Request,
) -> Result<create_topic::Response, AppError> {
    let topic = repo::create_topic(db, repo::topic::Model::new(req.user_id, req.title)).await?;

    Ok(create_topic::Response { topic })
}

pub mod create_topic {
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::topics::repo;

    #[derive(Validate)]
    pub struct Request {
        pub user_id: Uuid,
        #[validate(length(min = 2))]
        pub title: String,
    }

    pub struct Response {
        pub topic: repo::topic::Model,
    }
}

pub async fn get_topics(
    db: &DbConn,
    req: get_topics::Request,
) -> Result<get_topics::Response, AppError> {
    let topics = repo::get_topics_by_user_id(db, req.user_id).await?;

    Ok(get_topics::Response { topics })
}

pub mod get_topics {
    use uuid::Uuid;

    use crate::app::topics::repo;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub topics: Vec<repo::topic::Model>,
    }
}

pub async fn get_topic(
    db: &DbConn,
    req: get_topic::Request,
) -> Result<get_topic::Response, AppError> {
    let topic = repo::get_topic_by_id(db, req.topic_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if topic.user_id != req.user_id {
        return Err(AppError::NotFound);
    };

    Ok(get_topic::Response { topic })
}

pub mod get_topic {
    use uuid::Uuid;

    use crate::app::topics::repo;

    pub struct Request {
        pub topic_id: Uuid,
        pub user_id: Uuid,
    }

    pub struct Response {
        pub topic: repo::topic::Model,
    }
}
