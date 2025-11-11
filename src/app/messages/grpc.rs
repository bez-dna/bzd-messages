use bzd_messages_api::{
    CreateMessageRequest, CreateMessageResponse, GetMessageRequest, GetMessageResponse,
    GetMessagesRequest, GetMessagesResponse, GetUserMessagesRequest, GetUserMessagesResponse,
    messages_service_server::MessagesService,
};
use tonic::{Request, Response, Status};

use crate::app::{error::AppError, messages::service, state::AppState};

pub struct GrpcMessagesService {
    pub state: AppState,
}

impl GrpcMessagesService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl MessagesService for GrpcMessagesService {
    async fn create_message(
        &self,
        req: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let res = create_message(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_user_messages(
        &self,
        req: Request<GetUserMessagesRequest>,
    ) -> Result<Response<GetUserMessagesResponse>, Status> {
        let res = get_user_messages(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_messages(
        &self,
        req: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let res = get_messages(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }

    async fn get_message(
        &self,
        req: Request<GetMessageRequest>,
    ) -> Result<Response<GetMessageResponse>, Status> {
        let res = get_message(&self.state, req.into_inner()).await?;

        Ok(Response::new(res))
    }
}

async fn create_message(
    AppState { db, .. }: &AppState,
    req: CreateMessageRequest,
) -> Result<CreateMessageResponse, AppError> {
    let res = service::create_message(db, req.try_into()?).await?;

    Ok(res.into())
}

mod create_message {
    use bzd_messages_api::{
        CreateMessageRequest, CreateMessageResponse, create_message_request::Tp,
    };
    use uuid::Uuid;
    use validator::Validate as _;

    use crate::app::{
        error::AppError,
        messages::service::create_message::{Request, Response, Type},
    };

    impl TryFrom<CreateMessageRequest> for Request {
        type Error = AppError;

        fn try_from(req: CreateMessageRequest) -> Result<Self, Self::Error> {
            let data = Self {
                user_id: Uuid::parse_str(req.user_id())?,
                text: req.text().into(),
                code: req.code().into(),
                tp: match req.tp.ok_or(AppError::Other)? {
                    Tp::Starting(starting) => Type::TopicIds(
                        starting
                            .topic_ids
                            .iter()
                            .map(|it| Uuid::parse_str(&it))
                            .collect::<Result<Vec<Uuid>, uuid::Error>>()?,
                    ),
                    Tp::Regular(regular) => Type::MessageId(Uuid::parse_str(regular.message_id())?),
                },
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

async fn get_user_messages(
    AppState { db, settings, .. }: &AppState,
    req: GetUserMessagesRequest,
) -> Result<GetUserMessagesResponse, AppError> {
    let res = service::get_user_messages(db, req.try_into()?, &settings.messages).await?;

    Ok(res.into())
}

mod get_user_messages {
    use bzd_messages_api::{GetUserMessagesRequest, GetUserMessagesResponse};
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::service::get_user_messages::{Request, Response},
    };

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
                message_ids: res.messages.iter().map(|it| it.message_id.into()).collect(),
                cursor_message_id: res.cursor_message.map(|it| it.message_id.into()),
            }
        }
    }
}

async fn get_messages(
    AppState { db, .. }: &AppState,
    req: GetMessagesRequest,
) -> Result<GetMessagesResponse, AppError> {
    let res = service::get_messages(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_messages {
    use bzd_messages_api::{GetMessagesRequest, GetMessagesResponse, get_messages_response};
    use prost_types::Timestamp;
    use uuid::Uuid;

    use crate::app::{
        error::AppError,
        messages::{
            repo::message,
            service::get_messages::{Request, Response},
        },
    };

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
                created_at: Some(Timestamp {
                    seconds: message.created_at.and_utc().timestamp(),
                    nanos: 0,
                }),
                updated_at: Some(Timestamp {
                    seconds: message.updated_at.and_utc().timestamp(),
                    nanos: 0,
                }),
            }
        }
    }
}

async fn get_message(
    AppState { db, .. }: &AppState,
    req: GetMessageRequest,
) -> Result<GetMessageResponse, AppError> {
    let res = service::get_message(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_message {
    use bzd_messages_api::{GetMessageRequest, GetMessageResponse, get_message_response};
    use prost_types::Timestamp;

    use crate::app::{
        error::AppError,
        messages::service::get_message::{Request, Response},
    };

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
                    created_at: Some(Timestamp {
                        seconds: message.created_at.and_utc().timestamp(),
                        nanos: 0,
                    }),
                    updated_at: Some(Timestamp {
                        seconds: message.updated_at.and_utc().timestamp(),
                        nanos: 0,
                    }),
                }),
            }
        }
    }
}
