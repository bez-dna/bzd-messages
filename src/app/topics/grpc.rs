use bzd_messages_api::{
    CreateTopicRequest, CreateTopicResponse, CreateTopicUserRequest, CreateTopicUserResponse,
    GetTopicRequest, GetTopicResponse, GetTopicsRequest, GetTopicsResponse, GetTopicsUsersRequest,
    GetTopicsUsersResponse, topics_service_server::TopicsService,
};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, state::AppState, topics::service};

pub struct GrpcTopicsService {
    pub state: AppState,
}

impl GrpcTopicsService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TopicsService for GrpcTopicsService {
    async fn create_topic(
        &self,
        req: Request<CreateTopicRequest>,
    ) -> Result<Response<CreateTopicResponse>, Status> {
        let res = create_topic(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_topics(
        &self,
        req: Request<GetTopicsRequest>,
    ) -> Result<Response<GetTopicsResponse>, Status> {
        let res = get_topics(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_topic(
        &self,
        req: Request<GetTopicRequest>,
    ) -> Result<Response<GetTopicResponse>, Status> {
        let res = get_topic(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_topics_users(
        &self,
        req: Request<GetTopicsUsersRequest>,
    ) -> Result<Response<GetTopicsUsersResponse>, Status> {
        let res = get_topics_users(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn create_topic_user(
        &self,
        req: Request<CreateTopicUserRequest>,
    ) -> Result<Response<CreateTopicUserResponse>, Status> {
        let res = create_topic_user(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

async fn create_topic(
    AppState { db, .. }: &AppState,
    req: CreateTopicRequest,
) -> Result<CreateTopicResponse, AppError> {
    let res = service::create_topic(db, req.try_into()?).await?;

    Ok(res.into())
}

mod create_topic {
    use bzd_messages_api::{CreateTopicRequest, CreateTopicResponse};
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::{
        error::AppError,
        topics::service::create_topic::{Request, Response},
    };

    impl TryFrom<CreateTopicRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateTopicRequest) -> Result<Self, Self::Error> {
            let data = Self {
                user_id: Uuid::parse_str(req.user_id())?,
                title: req.title().into(),
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl From<Response> for CreateTopicResponse {
        fn from(res: Response) -> Self {
            Self {
                topic_id: Some(res.topic.topic_id.into()),
            }
        }
    }
}

async fn get_topics(
    AppState { db, .. }: &AppState,
    req: GetTopicsRequest,
) -> Result<GetTopicsResponse, AppError> {
    let res = service::get_topics(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_topics {
    use bzd_messages_api::{GetTopicsRequest, GetTopicsResponse, get_topics_response::Topic};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        topics::{
            repo,
            service::get_topics::{Request, Response},
        },
    };

    impl TryFrom<GetTopicsRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetTopicsRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: Uuid::parse_str(req.user_id())?,
            })
        }
    }

    impl From<Response> for GetTopicsResponse {
        fn from(res: Response) -> Self {
            Self {
                topics: res.topics.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<repo::topic::Model> for Topic {
        fn from(topic: repo::topic::Model) -> Self {
            Self {
                topic_id: Some(topic.topic_id.into()),
                title: Some(topic.title.into()),
                user_id: Some(topic.user_id.into()),
            }
        }
    }
}

async fn get_topic(
    AppState { db, .. }: &AppState,
    req: GetTopicRequest,
) -> Result<GetTopicResponse, AppError> {
    let res = service::get_topic(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_topic {
    use bzd_messages_api::{GetTopicRequest, GetTopicResponse, get_topic_response::Topic};
    use uuid::Uuid;

    use crate::app::{error::AppError, topics::service::get_topic};

    impl TryFrom<GetTopicRequest> for get_topic::Request {
        type Error = AppError;

        fn try_from(req: GetTopicRequest) -> Result<Self, Self::Error> {
            let data = Self {
                topic_id: Uuid::parse_str(req.topic_id())?,
                user_id: Uuid::parse_str(req.user_id())?,
            };

            Ok(data)
        }
    }

    impl From<get_topic::Response> for GetTopicResponse {
        fn from(res: get_topic::Response) -> Self {
            Self {
                topic: Some(Topic {
                    topic_id: Some(res.topic.topic_id.into()),
                    title: Some(res.topic.topic_id.into()),
                    user_id: Some(res.topic.user_id.into()),
                }),
            }
        }
    }
}

async fn get_topics_users(
    AppState { db, .. }: &AppState,
    req: GetTopicsUsersRequest,
) -> Result<GetTopicsUsersResponse, AppError> {
    let res = service::get_topics_users(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_topics_users {
    use bzd_messages_api::{
        GetTopicsUsersRequest, GetTopicsUsersResponse, get_topics_users_response,
    };
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        topics::{
            repo,
            service::get_topics_users::{Request, Response},
        },
    };

    impl TryFrom<GetTopicsUsersRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetTopicsUsersRequest) -> Result<Self, Self::Error> {
            let topic_ids = req
                .topic_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self {
                topic_ids,
                user_id: req.user_id().parse()?,
            })
        }
    }

    impl From<Response> for GetTopicsUsersResponse {
        fn from(res: Response) -> Self {
            Self {
                topics_users: res.topics_users.iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&repo::topic_user::Model> for get_topics_users_response::TopicUser {
        fn from(topic_user: &repo::topic_user::Model) -> Self {
            Self {
                topic_user_id: Some(topic_user.topic_user_id.into()),
            }
        }
    }
}

async fn create_topic_user(
    AppState { db, .. }: &AppState,
    req: CreateTopicUserRequest,
) -> Result<CreateTopicUserResponse, AppError> {
    let res = service::create_topic_user(db, req.try_into()?).await?;

    Ok(res.into())
}

mod create_topic_user {
    use bzd_messages_api::{CreateTopicUserRequest, CreateTopicUserResponse};

    use crate::app::{
        error::AppError,
        topics::service::create_topic_user::{Request, Response},
    };

    impl TryFrom<CreateTopicUserRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateTopicUserRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                topic_id: req.topic_id().parse()?,
                user_id: req.user_id().parse()?,
            })
        }
    }

    impl From<Response> for CreateTopicUserResponse {
        fn from(res: Response) -> Self {
            Self {
                topic_user_id: Some(res.topic_user.topic_user_id.into()),
            }
        }
    }
}
