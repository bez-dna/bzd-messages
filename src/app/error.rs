use thiserror::Error;
use tonic::Status;

impl From<AppError> for Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::Validation(_) => Self::invalid_argument(error.to_string()),
            // AppError::NotFound => Self::not_found(error.to_string()),
            _ => Self::internal(error.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("VALIDATION")]
    Validation(#[from] validator::ValidationErrors),
    #[error("DB")]
    Db(#[from] sea_orm::DbErr),
    #[error("UUID")]
    Uuid(#[from] uuid::Error),
    #[error("NOT_FOUND")]
    NotFound,
    #[error("OTHER")]
    Other,
    #[error("UNREACHABLE")]
    Unreachable,
}
