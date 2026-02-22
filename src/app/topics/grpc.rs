use bzd_messages_api::topics::{
    CreateTopicRequest, CreateTopicResponse, CreateTopicUserRequest, CreateTopicUserResponse,
    DeleteTopicUserRequest, DeleteTopicUserResponse, GetEmojisRequest, GetEmojisResponse,
    GetTopicRequest, GetTopicResponse, GetTopicsRequest, GetTopicsResponse, GetUserTopicsRequest,
    GetUserTopicsResponse, GetUserTopicsUsersRequest, GetUserTopicsUsersResponse,
    topics_service_server::TopicsService,
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

    async fn get_user_topics_users(
        &self,
        req: Request<GetUserTopicsUsersRequest>,
    ) -> Result<Response<GetUserTopicsUsersResponse>, Status> {
        let res = get_user_topics_users::handler(&self.state, req.into_inner()).await?;

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

    async fn get_emojis(
        &self,
        _: Request<GetEmojisRequest>,
    ) -> Result<Response<GetEmojisResponse>, Status> {
        let res = get_emojis::handler(&self.state).await?;

        Ok(Response::new(res))
    }
}

mod create_topic {
    use bzd_messages_api::topics::{CreateTopicRequest, CreateTopicResponse};
    use validator::Validate;

    use crate::app::{
        current_user::CurrentUser,
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
                current_user: CurrentUser::new(&req.current_user_id)?,
                title: emojis::get(req.title()).ok_or(AppError::Validation)?,
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

mod get_user_topics_users {
    use bzd_messages_api::topics::{
        GetUserTopicsUsersRequest, GetUserTopicsUsersResponse, get_user_topics_users_response,
    };
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        topics::{
            repo::TopicUserModel,
            service::{
                self,
                get_user_topics_users::{Request, Response},
            },
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { db, .. }: &TopicsState,
        req: GetUserTopicsUsersRequest,
    ) -> Result<GetUserTopicsUsersResponse, AppError> {
        let res = service::get_user_topics_users(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetUserTopicsUsersRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetUserTopicsUsersRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: req
                    .current_user_id
                    .as_deref()
                    .map(Uuid::parse_str)
                    .transpose()?,
            })
        }
    }

    impl From<Response> for GetUserTopicsUsersResponse {
        fn from(res: Response) -> Self {
            Self {
                topics_users: res.topics_users.iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&TopicUserModel> for get_user_topics_users_response::TopicUser {
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

mod get_emojis {
    use bzd_messages_api::topics::{GetEmojisResponse, get_emojis_response::Emoji};

    use crate::app::{
        error::AppError,
        topics::{
            service::{self, get_emojis::Response},
            state::TopicsState,
        },
    };

    pub async fn handler(
        TopicsState { settings, .. }: &TopicsState,
    ) -> Result<GetEmojisResponse, AppError> {
        let res = service::get_emojis(&settings)?;

        Ok(res.into())
    }

    impl From<Response> for GetEmojisResponse {
        fn from(res: Response) -> Self {
            Self {
                emojis: res
                    .emojis
                    .into_iter()
                    .map(|emoji| Emoji {
                        title: Some(emoji.to_string()),
                        code: emoji.shortcode().map(|it| it.into()),
                    })
                    .collect(),
            }
        }
    }
}
