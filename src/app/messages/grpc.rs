use bzd_messages_api::{
    CreateMessageRequest, CreateMessageResponse, GetUserMessagesRequest, GetUserMessagesResponse,
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
    AppState { db, .. }: &AppState,
    req: GetUserMessagesRequest,
) -> Result<GetUserMessagesResponse, AppError> {
    let res = service::get_user_messages(db, req.try_into()?).await?;

    Ok(res.into())
}

mod get_user_messages {
    use bzd_messages_api::{GetUserMessagesRequest, GetUserMessagesResponse};

    use crate::app::{
        error::AppError,
        messages::service::get_user_messages::{Request, Response},
    };

    impl TryFrom<GetUserMessagesRequest> for Request {
        type Error = AppError;

        fn try_from(req: GetUserMessagesRequest) -> Result<Self, Self::Error> {
            Ok(Self {
                user_id: req.user_id().parse()?,
            })
        }
    }

    impl From<Response> for GetUserMessagesResponse {
        fn from(res: Response) -> Self {
            Self {
                message_ids: res.messages.iter().map(|it| it.message_id.into()).collect(),
            }
        }
    }
}
