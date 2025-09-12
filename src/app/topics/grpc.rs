use bzd_messages_api::{
    CreateTopicRequest, CreateTopicResponse, GetTopicsRequest, GetTopicsResponse,
    topics_service_server::TopicsService,
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
        fn from(_: Response) -> Self {
            Self::default()
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
