use uuid::Uuid;

use crate::app::error::AppError;

pub struct CurrentUser {
    pub user_id: Option<Uuid>,
}

impl CurrentUser {
    pub fn new(current_user_id: &Option<String>) -> Result<Self, AppError> {
        let user_id = if let Some(user_id) = current_user_id {
            Some(user_id.to_string().parse()?)
        } else {
            None
        };

        Ok(Self { user_id })
    }

    pub fn has_access(&self, check_user_id: Uuid) -> bool {
        self.user_id.is_some_and(|it| it == check_user_id)
    }
}
