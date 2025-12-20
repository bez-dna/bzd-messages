use bzd_lib::settings::DBSettings;
use bzd_lib::settings::NATSSettings;
use bzd_lib::settings::Settings;

use bzd_lib::settings::HttpSettings;
use serde::Deserialize;

use crate::app::messages;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub http: HttpSettings,
    pub db: DBSettings,
    pub nats: NATSSettings,
    pub messages: messages::settings::Settings,
}

impl Settings<AppSettings> for AppSettings {}
