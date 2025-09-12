use sea_orm::DbConn;

use crate::app::{error::AppError, messages::repo};

pub async fn create_message(
    _db: &DbConn,
    req: create_message::Request,
) -> Result<create_message::Response, AppError> {
    let message = repo::message::Model::new(req.user_id, req.text, req.code);

    Ok(create_message::Response { message })
}

pub mod create_message {
    use uuid::Uuid;
    use validator::Validate;

    use crate::app::messages::repo;

    #[derive(Validate)]
    pub struct Request {
        pub user_id: Uuid,
        #[validate(length(min = 2))]
        pub text: String,
        #[validate(length(min = 2))]
        pub code: String,
    }

    pub struct Response {
        pub message: repo::message::Model,
    }
}
