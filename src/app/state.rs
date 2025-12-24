use bzd_lib::error::Error;

use crate::app::{db::DbState, mess::MessState, settings::AppSettings, topics::state::TopicsState};

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub topics: TopicsState,
    pub mess: MessState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let db = DbState::new(&settings.db).await?;

        let mess = MessState::new(&settings.nats).await?;

        let topics = TopicsState {
            settings: settings.topics.clone(),
            db: db.clone(),
            mess: mess.clone(),
        };

        Ok(Self { db, mess, topics })
    }
}
