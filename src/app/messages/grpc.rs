use bzd_messages_api::messages::{
    CreateMessageRequest, CreateMessageResponse, CreateMessageTopicRequest,
    CreateMessageTopicResponse, DeleteMessageTopicRequest, DeleteMessageTopicResponse,
    GetMessageMessagesRequest, GetMessageMessagesResponse, GetMessageRequest, GetMessageResponse,
    GetMessagesRequest, GetMessagesResponse, GetMessagesUsersRequest, GetMessagesUsersResponse,
    GetStreamsRequest, GetStreamsResponse, GetUserMessagesRequest, GetUserMessagesResponse,
    GetUserMessagesTopicsRequest, GetUserMessagesTopicsResponse,
    messages_service_server::MessagesService,
};
use tonic::{Request, Response, Status};

use crate::app::messages::state::MessagesState;

pub struct GrpcMessagesService {
    pub state: MessagesState,
}

impl GrpcMessagesService {
    pub fn new(state: MessagesState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl MessagesService for GrpcMessagesService {
    async fn create_message(
        &self,
        req: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let res = create_message::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_messages(
        &self,
        req: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let res = get_messages::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_message(
        &self,
        req: Request<GetMessageRequest>,
    ) -> Result<Response<GetMessageResponse>, Status> {
        let res = get_message::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_message_messages(
        &self,
        req: Request<GetMessageMessagesRequest>,
    ) -> Result<Response<GetMessageMessagesResponse>, Status> {
        let res = get_message_messages::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_user_messages(
        &self,
        req: Request<GetUserMessagesRequest>,
    ) -> Result<Response<GetUserMessagesResponse>, Status> {
        let res = get_user_messages::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_streams(
        &self,
        req: Request<GetStreamsRequest>,
    ) -> Result<Response<GetStreamsResponse>, Status> {
        let res = get_streams::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_messages_users(
        &self,
        req: Request<GetMessagesUsersRequest>,
    ) -> Result<Response<GetMessagesUsersResponse>, Status> {
        let res = get_messages_users::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_user_messages_topics(
        &self,
        req: Request<GetUserMessagesTopicsRequest>,
    ) -> Result<Response<GetUserMessagesTopicsResponse>, Status> {
        let res = get_user_messages_topics::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn create_message_topic(
        &self,
        req: Request<CreateMessageTopicRequest>,
    ) -> Result<Response<CreateMessageTopicResponse>, Status> {
        let res = create_message_topic::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn delete_message_topic(
        &self,
        req: Request<DeleteMessageTopicRequest>,
    ) -> Result<Response<DeleteMessageTopicResponse>, Status> {
        delete_message_topic::handler(&self.state, req.into_inner()).await?;

        Ok(Response::new(DeleteMessageTopicResponse::default()))
    }
}

mod create_message {
    use bzd_messages_api::messages::{CreateMessageRequest, CreateMessageResponse};
    use uuid::Uuid;
    use validator::Validate as _;

    use crate::app::{
        current_user::CurrentUser,
        error::AppError,
        messages::{
            service::{
                self,
                create_message::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState {
            db, mess, settings, ..
        }: &MessagesState,
        req: CreateMessageRequest,
    ) -> Result<CreateMessageResponse, AppError> {
        let res = service::create_message(&db.conn, &mess.js, &settings, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<CreateMessageRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateMessageRequest) -> Result<Self, Self::Error> {
            let data = Self {
                current_user: CurrentUser::new(&req.current_user_id)?,
                text: req.text().into(),
                code: req.code().parse()?,
                message_id: req.message_id.as_deref().map(Uuid::parse_str).transpose()?,
            };

            data.validate()?;

            Ok(data)
        }
    }

    impl From<Response> for CreateMessageResponse {
        fn from(res: Response) -> Self {
            Self {
                message_id: Some(res.message.message_id.into()),
            }
        }
    }
}

mod get_messages {
    use bzd_messages_api::messages::{
        GetMessagesRequest, GetMessagesResponse, get_messages_response,
    };
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        grpc::ToProtoTimestamp,
        messages::{
            repo::message,
            service::{
                self,
                get_messages::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, .. }: &MessagesState,
        req: GetMessagesRequest,
    ) -> Result<GetMessagesResponse, AppError> {
        let res = service::get_messages(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetMessagesRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetMessagesRequest) -> Result<Self, Self::Error> {
            let message_ids = req
                .message_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self { message_ids })
        }
    }

    impl From<Response> for GetMessagesResponse {
        fn from(res: Response) -> Self {
            Self {
                messages: res.messages.iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&message::Model> for get_messages_response::Message {
        fn from(message: &message::Model) -> Self {
            Self {
                message_id: Some(message.message_id.into()),
                text: message.text.clone().into(),
                user_id: Some(message.user_id.into()),
                code: message.code.clone().into(),
                order: Some(message.created_at.and_utc().timestamp_micros()),
                created_at: message.created_at.to_option_proto(),
                updated_at: message.updated_at.to_option_proto(),
            }
        }
    }
}

mod get_message {
    use bzd_messages_api::messages::{
        GetMessageRequest, GetMessageResponse,
        get_message_response::{self},
    };

    use crate::app::{
        error::AppError,
        grpc::ToProtoTimestamp,
        messages::{
            service::{
                self,
                get_message::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, .. }: &MessagesState,
        req: GetMessageRequest,
    ) -> Result<GetMessageResponse, AppError> {
        let res = service::get_message(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetMessageRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetMessageRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                message_id: req.message_id().parse()?,
            })
        }
    }

    impl From<Response> for GetMessageResponse {
        fn from(res: Response) -> Self {
            let message = res.message;

            Self {
                message: Some(get_message_response::Message {
                    message_id: Some(message.message_id.into()),
                    text: message.text.clone().into(),
                    user_id: Some(message.user_id.into()),
                    code: message.code.clone().into(),
                    order: Some(message.created_at.and_utc().timestamp_micros()),
                    created_at: message.created_at.to_option_proto(),
                    updated_at: message.updated_at.to_option_proto(),
                }),
            }
        }
    }
}

mod get_message_messages {
    use bzd_messages_api::messages::{GetMessageMessagesRequest, GetMessageMessagesResponse};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            service::{
                self,
                get_message_messages::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, settings, .. }: &MessagesState,
        req: GetMessageMessagesRequest,
    ) -> Result<GetMessageMessagesResponse, AppError> {
        let res = service::get_message_messages(&db.conn, req.try_into()?, &settings).await?;

        Ok(res.into())
    }

    impl TryFrom<GetMessageMessagesRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetMessageMessagesRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                message_id: req.message_id().parse()?,
                cursor_message_id: req
                    .cursor_message_id
                    .as_deref()
                    .map(Uuid::parse_str)
                    .transpose()?,
            })
        }
    }

    impl From<Response> for GetMessageMessagesResponse {
        fn from(res: Response) -> Self {
            Self {
                message_ids: res.messages.iter().map(|it| it.message_id.into()).collect(),
                cursor_message_id: res.cursor_message.map(|it| it.message_id.into()),
            }
        }
    }
}

mod get_user_messages {
    use bzd_messages_api::messages::{GetUserMessagesRequest, GetUserMessagesResponse};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            service::{
                self,
                get_user_messages::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, settings, .. }: &MessagesState,
        req: GetUserMessagesRequest,
    ) -> Result<GetUserMessagesResponse, AppError> {
        let res = service::get_user_messages(&db.conn, req.try_into()?, &settings).await?;

        Ok(res.into())
    }

    impl TryFrom<GetUserMessagesRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetUserMessagesRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: req.user_id().parse()?,
                cursor_message_id: req
                    .cursor_message_id
                    .as_deref()
                    .map(Uuid::parse_str)
                    .transpose()?,
            })
        }
    }

    impl From<Response> for GetUserMessagesResponse {
        fn from(res: Response) -> Self {
            Self {
                message_ids: res
                    .messages_users
                    .iter()
                    .map(|it| it.message_id.into())
                    .collect(),
                cursor_message_id: res.cursor_message_user.map(|it| it.message_id.into()),
            }
        }
    }
}

mod get_streams {
    use bzd_messages_api::messages::{
        GetStreamsRequest, GetStreamsResponse, get_streams_response::Stream,
    };
    use prost_types::Timestamp;
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            service::{
                self,
                get_streams::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, .. }: &MessagesState,
        req: GetStreamsRequest,
    ) -> Result<GetStreamsResponse, AppError> {
        let res = service::get_streams(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetStreamsRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetStreamsRequest) -> Result<Self, Self::Error> {
            let message_ids = req
                .message_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self { message_ids })
        }
    }

    impl From<Response> for GetStreamsResponse {
        fn from(res: Response) -> Self {
            Self {
                streams: res
                    .streams
                    .iter()
                    .map(|(stream, messages_users)| Stream {
                        stream_id: Some(stream.stream_id.into()),
                        message_id: Some(stream.message_id.into()),
                        text: stream.text.clone().into(),
                        user_ids: messages_users.iter().map(|it| it.user_id.into()).collect(),
                        messages_count: Some(stream.messages_count),
                        created_at: Some(Timestamp {
                            seconds: stream.created_at.and_utc().timestamp(),
                            nanos: 0,
                        }),
                        updated_at: Some(Timestamp {
                            seconds: stream.updated_at.and_utc().timestamp(),
                            nanos: 0,
                        }),
                    })
                    .collect(),
            }
        }
    }
}

mod get_messages_users {
    use bzd_messages_api::messages::{
        GetMessagesUsersRequest, GetMessagesUsersResponse, get_messages_users_response,
    };
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            repo::MessageUserModel,
            service::{
                self,
                get_messages_users::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, .. }: &MessagesState,
        req: GetMessagesUsersRequest,
    ) -> Result<GetMessagesUsersResponse, AppError> {
        let res = service::get_messages_users(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetMessagesUsersRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetMessagesUsersRequest) -> Result<Self, Self::Error> {
            let message_ids = req
                .message_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self { message_ids })
        }
    }

    impl From<Response> for GetMessagesUsersResponse {
        fn from(res: Response) -> Self {
            Self {
                messages_users: res.messages_users.iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&MessageUserModel> for get_messages_users_response::MessageUser {
        fn from(message_user: &MessageUserModel) -> Self {
            Self {
                message_user_id: Some(message_user.message_user_id.into()),
                message_id: Some(message_user.message_id.into()),
                user_id: Some(message_user.user_id.into()),
            }
        }
    }
}

mod get_user_messages_topics {
    use bzd_messages_api::messages::{
        GetUserMessagesTopicsRequest, GetUserMessagesTopicsResponse,
        get_user_messages_topics_response,
    };
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            repo::MessageTopicModel,
            service::{
                self,
                get_user_messages_topics::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState { db, .. }: &MessagesState,
        req: GetUserMessagesTopicsRequest,
    ) -> Result<GetUserMessagesTopicsResponse, AppError> {
        let res = service::get_user_messages_topics(&db.conn, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<GetUserMessagesTopicsRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetUserMessagesTopicsRequest) -> Result<Self, Self::Error> {
            let message_ids = req
                .message_ids
                .iter()
                .map(|it| it.parse())
                .collect::<Result<Vec<Uuid>, _>>()?;

            Ok(Self {
                user_id: req.user_id().parse()?,
                message_ids,
            })
        }
    }

    impl From<Response> for GetUserMessagesTopicsResponse {
        fn from(res: Response) -> Self {
            Self {
                messages_topics: res.messages_topics.iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&MessageTopicModel> for get_user_messages_topics_response::MessageTopic {
        fn from(message_user: &MessageTopicModel) -> Self {
            Self {
                message_topic_id: Some(message_user.message_topic_id.into()),
                message_id: Some(message_user.message_id.into()),
                topic_id: Some(message_user.topic_id.into()),
            }
        }
    }
}

mod create_message_topic {
    use bzd_messages_api::messages::{CreateMessageTopicRequest, CreateMessageTopicResponse};

    use crate::app::{
        current_user::CurrentUser,
        error::AppError,
        messages::{
            service::{
                self,
                create_message_topic::{Request, Response},
            },
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState {
            db, mess, settings, ..
        }: &MessagesState,
        req: CreateMessageTopicRequest,
    ) -> Result<CreateMessageTopicResponse, AppError> {
        let res =
            service::create_message_topic(&db.conn, &mess.js, settings, req.try_into()?).await?;

        Ok(res.into())
    }

    impl TryFrom<CreateMessageTopicRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateMessageTopicRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                current_user: CurrentUser::new(&req.current_user_id)?,
                message_id: req.message_id().parse()?,
                topic_id: req.topic_id().parse()?,
            })
        }
    }

    impl From<Response> for CreateMessageTopicResponse {
        fn from(res: Response) -> Self {
            Self {
                message_topic_id: Some(res.message_topic.message_topic_id.into()),
            }
        }
    }
}

mod delete_message_topic {
    use bzd_messages_api::messages::DeleteMessageTopicRequest;

    use crate::app::{
        current_user::CurrentUser,
        error::AppError,
        messages::{
            service::{self, delete_message_topic::Request},
            state::MessagesState,
        },
    };

    pub async fn handler(
        MessagesState {
            db, mess, settings, ..
        }: &MessagesState,
        req: DeleteMessageTopicRequest,
    ) -> Result<(), AppError> {
        service::delete_message_topic(&db.conn, &mess.js, settings, req.try_into()?).await?;

        Ok(())
    }

    impl TryFrom<DeleteMessageTopicRequest> for Request {
        type Error = AppError;

        fn try_from(req: DeleteMessageTopicRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                current_user: CurrentUser::new(&req.current_user_id)?,
                message_topic_id: req.message_topic_id().parse()?,
            })
        }
    }
}
