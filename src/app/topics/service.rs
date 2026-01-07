use async_nats::jetstream::Context;
use bzd_messages_api::events::topic_user::Type;
use sea_orm::DbConn;

use crate::app::{
    error::AppError,
    topics::{events, repo, settings::TopicsSettings},
};

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
    let topics = repo::get_topics_by_ids(db, req.topic_ids).await?;

    Ok(get_topics::Response { topics })
}

pub mod get_topics {
    use uuid::Uuid;

    use crate::app::topics::repo;

    pub struct Request {
        pub topic_ids: Vec<Uuid>,
    }

    pub struct Response {
        pub topics: Vec<repo::topic::Model>,
    }
}

pub async fn get_topic(
    db: &DbConn,
    req: get_topic::Request,
) -> Result<get_topic::Response, AppError> {
    let topic = repo::get_topic_by_id(db, req.topic_id).await?;

    Ok(get_topic::Response { topic })
}

pub mod get_topic {
    use uuid::Uuid;

    use crate::app::topics::repo;

    pub struct Request {
        pub topic_id: Uuid,
    }

    pub struct Response {
        pub topic: repo::topic::Model,
    }

    #[cfg(test)]
    mod tests {
        use bzd_lib::error::Error;
        use sea_orm::{DatabaseBackend, MockDatabase};

        use crate::app::{
            error::AppError,
            topics::{
                repo,
                service::{self, get_topic::Request},
            },
        };

        #[tokio::test]
        async fn test_ok_get_topic() -> Result<(), Error> {
            let topic = repo::topic::Model::stub();

            let req = Request {
                topic_id: topic.topic_id,
            };

            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![topic.clone()]])
                .into_connection();

            let res = service::get_topic(&db, req).await?;

            assert_eq!(res.topic, topic);

            Ok(())
        }

        #[tokio::test]
        async fn test_not_found_get_topic() -> Result<(), Error> {
            let topic = repo::topic::Model::stub();

            let req = Request {
                topic_id: topic.topic_id,
            };

            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([Vec::<repo::topic::Model>::new()])
                .into_connection();

            let res = service::get_topic(&db, req).await;

            assert!(res.is_err());
            assert!(matches!(res, Err(AppError::NotFound)));

            Ok(())
        }
    }
}

pub async fn get_user_topics(
    db: &DbConn,
    req: get_user_topics::Request,
) -> Result<get_user_topics::Response, AppError> {
    let topics = repo::get_topics_by_user_id(db, req.user_id).await?;

    Ok(get_user_topics::Response { topics })
}

pub mod get_user_topics {
    use uuid::Uuid;

    use crate::app::topics::repo::topic;

    pub struct Request {
        pub user_id: Uuid,
    }

    pub struct Response {
        pub topics: Vec<topic::Model>,
    }
}

pub async fn get_topics_users(
    db: &DbConn,
    req: get_topics_users::Request,
) -> Result<get_topics_users::Response, AppError> {
    let topics_users = if let Some(user_id) = req.user_id {
        repo::get_topics_users_by_ids_and_user_id(db, req.topic_ids, user_id).await?
    } else {
        vec![]
    };

    Ok(get_topics_users::Response { topics_users })
}

pub mod get_topics_users {
    use uuid::Uuid;

    use crate::app::topics::repo::topic_user;

    pub struct Request {
        pub topic_ids: Vec<Uuid>,
        pub user_id: Option<Uuid>,
    }

    pub struct Response {
        pub topics_users: Vec<topic_user::Model>,
    }
}

pub async fn create_topic_user(
    db: &DbConn,
    js: &Context,
    settings: &TopicsSettings,
    req: create_topic_user::Request,
) -> Result<create_topic_user::Response, AppError> {
    let current_user = req.current_user.ok_or(AppError::Forbidden)?;

    let topic = repo::get_topic_by_id(db, req.topic_id).await?;

    let topic_user = repo::create_topic_user(
        db,
        repo::topic_user::Model::new(current_user.user_id, topic.topic_id),
    )
    .await?;

    // TODO: нужно сделать асинк отсылку (аутбокс??)
    events::topic_user(js, &settings.events, &topic_user, Type::Created).await?;

    Ok(create_topic_user::Response { topic_user })
}

pub mod create_topic_user {
    use uuid::Uuid;

    use crate::app::{current_user::CurrentUser, topics::repo};

    pub struct Request {
        pub current_user: Option<CurrentUser>,
        pub topic_id: Uuid,
    }

    pub struct Response {
        pub topic_user: repo::topic_user::Model,
    }

    // #[cfg(test)]
    // mod tests {
    //     use bzd_lib::error::Error;

    //     use crate::app::topics::service::{self, create_topic_user::Request};

    //     #[tokio::test]
    //     async fn test_ok_create_topic_user() -> Result<(), Error> {
    //         let req = Request {
    //             current_user: todo!(),
    //             topic_id: todo!(),
    //         };

    //         // let res = service::create_topic_user(db, js, settings, req).await?;

    //         Ok(())
    //     }
    // }
}

pub async fn delete_topic_user(
    db: &DbConn,
    js: &Context,
    settings: &TopicsSettings,
    req: delete_topic_user::Request,
) -> Result<(), AppError> {
    let current_user = req.current_user.ok_or(AppError::Forbidden)?;

    let topic_user = repo::get_topic_user_by_id(db, req.topic_user_id).await?;

    current_user.check_access(topic_user.user_id)?;

    repo::delete_topic_user(db, topic_user.clone()).await?;

    events::topic_user(js, &settings.events, &topic_user, Type::Deleted).await?;

    Ok(())
}

pub mod delete_topic_user {
    use uuid::Uuid;

    use crate::app::current_user::CurrentUser;

    pub struct Request {
        pub current_user: Option<CurrentUser>,
        pub topic_user_id: Uuid,
    }
}
