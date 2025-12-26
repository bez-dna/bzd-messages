use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation => Self::invalid_argument(error.to_string()),
            // AppError::NotFound => Self::not_found(error.to_string()),
            _ => Self::internal(error.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    // Ok
    #[error("VALIDATION")]
    Validation,

    #[error("ENCODE")]
    Encode(#[from] prost::EncodeError),
    #[error("PUBLISH")]
    Publish(#[from] async_nats::jetstream::context::PublishError),
    #[error("DB")]
    Db(#[from] sea_orm::DbErr),
    #[error("UUID")]
    Uuid(#[from] uuid::Error),
    #[error("NOT_FOUND")]
    NotFound,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("OTHER")]
    Other,

    // Ok
    #[error("UNREACHABLE")]
    Unreachable,
}

impl From<validator::ValidationErrors> for AppError {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::Validation
    }
}
