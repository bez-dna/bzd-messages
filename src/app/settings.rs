use bzd_lib::settings::DBSettings;
use bzd_lib::settings::NATSSettings;
use bzd_lib::settings::Settings;

use bzd_lib::settings::HttpSettings;
use serde::Deserialize;

// use crate::app::messages;
use crate::app::topics;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub http: HttpSettings,
    pub db: DBSettings,
    pub nats: NATSSettings,
    // pub messages: messages::settings::MessagesSettings,
    pub topics: topics::settings::TopicsSettings,
}

impl Settings<AppSettings> for AppSettings {}
