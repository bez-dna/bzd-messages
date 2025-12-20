use std::sync::Arc;

use async_nats::jetstream::Context;
use bzd_lib::error::Error;
use sea_orm::{ConnectOptions, Database, DbConn};

use crate::app::settings::AppSettings;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub db: Arc<DbConn>,
    pub js: Arc<Context>,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let opt = ConnectOptions::new(&settings.db.endpoint);
        let db = Arc::new(Database::connect(opt).await?);

        let nats = async_nats::connect(&settings.nats.endpoint).await?;
        let js = Arc::new(async_nats::jetstream::new(nats));

        Ok(Self { settings, db, js })
    }
}
