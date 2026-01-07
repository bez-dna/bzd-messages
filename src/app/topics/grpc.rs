use bzd_messages_api::topics::{
    CreateTopicRequest, CreateTopicResponse, CreateTopicUserRequest, CreateTopicUserResponse,
    DeleteTopicUserRequest, DeleteTopicUserResponse, GetTopicRequest, GetTopicResponse,
    GetTopicsRequest, GetTopicsResponse, GetTopicsUsersRequest, GetTopicsUsersResponse,
    GetUserTopicsRequest, GetUserTopicsResponse, topics_service_server::TopicsService,
};
use tonic::{Request, Response, Status};

use crate::app::topics::state::TopicsState;

pub struct GrpcTopicsService {
    pub state: TopicsState,
}

impl GrpcTopicsService {
    pub fn new(state: TopicsState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TopicsService for GrpcTopicsService {
    async fn create_topic(
        &self,
        req: Request<CreateTopicRequest>,
    ) -> Result<Response<CreateTopicResponse>, Status> {
        let res = create_topic::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_topics(
        &self,
        req: Request<GetTopicsRequest>,
    ) -> Result<Response<GetTopicsResponse>, Status> {
        let res = get_topics::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_topic(
        &self,
        req: Request<GetTopicRequest>,
    ) -> Result<Response<GetTopicResponse>, Status> {
        let res = get_topic::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_user_topics(
        &self,
        req: Request<GetUserTopicsRequest>,
    ) -> Result<Response<GetUserTopicsResponse>, Status> {
        let res = get_user_topics::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_topics_users(
        &self,
        req: Request<GetTopicsUsersRequest>,
    ) -> Result<Response<GetTopicsUsersResponse>, Status> {
        let res = get_topics_users::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn create_topic_user(
        &self,
        req: Request<CreateTopicUserRequest>,
    ) -> Result<Response<CreateTopicUserResponse>, Status> {
        let res = create_topic_user::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn delete_topic_user(
        &self,
        req: Request<DeleteTopicUserRequest>,
    ) -> Result<Response<DeleteTopicUserResponse>, Status> {
        delete_topic_user::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(DeleteTopicUserResponse::default()))
    }
}

mod create_topic {
    use bzd_messages_api::topics::{CreateTopicRequest, CreateTopicResponse};
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::{
        error::AppError,
        topics::{
            service::{
                self,
                create_topic::{Request, Response},
            },
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { db, .. }: &TopicsState,
        req: CreateTopicRequest,
    ) -> Result<CreateTopicResponse, AppError> {
        let res = service::create_topic(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

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

mod get_topics {
    use bzd_messages_api::topics::{GetTopicsRequest, GetTopicsResponse, Topic};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        topics::{
            repo::TopicModel,
            service::{
                self,
                get_topics::{Request, Response},
            },
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { db, .. }: &TopicsState,
        req: GetTopicsRequest,
    ) -> Result<GetTopicsResponse, AppError> {
        let res = service::get_topics(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetTopicsRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetTopicsRequest) -> Result<Self, Self::Error> {
            let topic_ids = req
                .topic_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self { topic_ids })
        }
    }

    impl From<Response> for GetTopicsResponse {
        fn from(res: Response) -> Self {
            Self {
                topics: res.topics.into_iter().map(Into::into).collect(),
            }
        }
    }

    impl From<TopicModel> for Topic {
        fn from(topic: TopicModel) -> Self {
            Self {
                topic_id: Some(topic.topic_id.into()),
                title: topic.title.into(),
                user_id: Some(topic.user_id.into()),
            }
        }
    }
}

mod get_topic {
    use bzd_messages_api::topics::{GetTopicRequest, GetTopicResponse, Topic};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        topics::{
            service::{self, get_topic},
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { db, .. }: &TopicsState,
        req: GetTopicRequest,
    ) -> Result<GetTopicResponse, AppError> {
        let res = service::get_topic(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetTopicRequest> for get_topic::Request {
        type Error = AppError;

        fn try_from(req: GetTopicRequest) -> Result<Self, Self::Error> {
            let data = Self {
                topic_id: Uuid::parse_str(req.topic_id())?,
            };

            Ok(data)
        }
    }

    impl From<get_topic::Response> for GetTopicResponse {
        fn from(res: get_topic::Response) -> Self {
            Self {
                topic: Some(Topic {
                    topic_id: Some(res.topic.topic_id.into()),
                    title: res.topic.title.into(),
                    user_id: Some(res.topic.user_id.into()),
                }),
            }
        }
    }
}

mod get_user_topics {
    use bzd_messages_api::topics::{GetUserTopicsRequest, GetUserTopicsResponse};

    use crate::app::{
        error::AppError,
        topics::{
            service::{
                self,
                get_user_topics::{Request, Response},
            },
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { db, .. }: &TopicsState,
        req: GetUserTopicsRequest,
    ) -> Result<GetUserTopicsResponse, AppError> {
        let res = service::get_user_topics(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetUserTopicsRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetUserTopicsRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: req.user_id().parse()?,
            })
        }
    }

    impl From<Response> for GetUserTopicsResponse {
        fn from(res: Response) -> Self {
            Self {
                topic_ids: res.topics.iter().map(|it| it.topic_id.into()).collect(),
            }
        }
    }
}

mod get_topics_users {
    use bzd_messages_api::topics::{
        GetTopicsUsersRequest, GetTopicsUsersResponse, get_topics_users_response,
    };
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        topics::{
            repo::TopicUserModel,
            service::{
                self,
                get_topics_users::{Request, Response},
            },
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { db, .. }: &TopicsState,
        req: GetTopicsUsersRequest,
    ) -> Result<GetTopicsUsersResponse, AppError> {
        let res = service::get_topics_users(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

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
                user_id: req
                    .current_user_id
                    .as_deref()
                    .map(Uuid::parse_str)
                    .transpose()?,
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

    impl From<&TopicUserModel> for get_topics_users_response::TopicUser {
        fn from(topic_user: &TopicUserModel) -> Self {
            Self {
                topic_user_id: Some(topic_user.topic_user_id.into()),
                topic_id: Some(topic_user.topic_id.into()),
                user_id: Some(topic_user.user_id.into()),
            }
        }
    }
}

mod create_topic_user {
    use bzd_messages_api::topics::{CreateTopicUserRequest, CreateTopicUserResponse};

    use crate::app::{
        current_user::CurrentUser,
        error::AppError,
        topics::{
            service::{
                self,
                create_topic_user::{Request, Response},
            },
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState {
            db, mess, settings, ..
        }: &TopicsState,
        req: CreateTopicUserRequest,
    ) -> Result<CreateTopicUserResponse, AppError> {
        let res =
            service::create_topic_user(&db.conn, &mess.js, &settings, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<CreateTopicUserRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateTopicUserRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                current_user: CurrentUser::new(&req.current_user_id)?,
                topic_id: req.topic_id().parse()?,
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

mod delete_topic_user {
    use bzd_messages_api::topics::DeleteTopicUserRequest;

    use crate::app::{
        current_user::CurrentUser,
        error::AppError,
        topics::{
            service::{self, delete_topic_user::Request},
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState {
            db, mess, settings, ..
        }: &TopicsState,
        req: DeleteTopicUserRequest,
    ) -> Result<(), AppError> {
        service::delete_topic_user(&db.conn, &mess.js, &settings, req.try_into()?).await?;

        Ok(())
    }

    impl TryFrom<DeleteTopicUserRequest> for Request {
        type Error = AppError;

        fn try_from(req: DeleteTopicUserRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                current_user: CurrentUser::new(&req.current_user_id)?,
                topic_user_id: req.topic_user_id().parse()?,
            })
        }
    }
}
