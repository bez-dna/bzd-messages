use crate::app::{db::DbState, mess::MessState, topics::settings::TopicsSettings};

#[derive(Clone)]
pub struct TopicsState {
    pub settings: TopicsSettings,
    pub db: DbState,
    pub mess: MessState,
}
