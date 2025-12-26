use crate::app::{db::DbState, mess::MessState, messages::settings::MessagesSettings};

#[derive(Clone)]
pub struct MessagesState {
    pub settings: MessagesSettings,
    pub db: DbState,
    pub mess: MessState,
}
