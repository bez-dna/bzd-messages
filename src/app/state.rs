use std::sync::Arc;

use bzd_lib::error::Error;
use sea_orm::{ConnectOptions, Database, DbConn};

use crate::app::settings::AppSettings;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub db: Arc<DbConn>,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let opt = ConnectOptions::new(&settings.db.endpoint);
        let db = Arc::new(Database::connect(opt).await?);

        Ok(Self { settings, db })
    }
}
