use uuid::Uuid;

use crate::app::error::AppError;

#[derive(Clone, Copy)]
pub struct CurrentUser {
    pub user_id: Uuid,
}

impl CurrentUser {
    pub fn new(current_user_id: &Option<String>) -> Result<Option<Self>, AppError> {
        current_user_id
            .as_deref()
            .map(|it| {
                Ok(Self {
                    user_id: it.to_string().parse()?,
                })
            })
            .transpose()
    }

    pub fn check_access(&self, user_id: Uuid) -> Result<(), AppError> {
        (self.user_id == user_id)
            .then_some(())
            .ok_or(AppError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use bzd_lib::error::Error;
    use uuid::Uuid;

    use crate::app::current_user::CurrentUser;

    #[test]
    fn successfully_parse_token() -> Result<(), Error> {
        let user_id = Uuid::now_v7();

        let current_user = CurrentUser::new(&Some(user_id.to_string()));
        assert!(current_user.is_ok());

        let current_user = current_user?;
        assert_eq!(Some(user_id), current_user.map(|it| it.user_id));

        Ok(())
    }

    #[test]
    fn successfully_parse_anon_token() -> Result<(), Error> {
        let current_user = CurrentUser::new(&None);
        assert!(current_user.is_ok());

        let current_user = current_user?;
        assert!(current_user.is_none());

        Ok(())
    }

    #[test]
    fn failed_parse_token() -> Result<(), Error> {
        let user_id = "WRONG_UUID".to_string();

        let current_user = CurrentUser::new(&Some(user_id.clone()));
        assert!(current_user.is_err());

        Ok(())
    }
}
