use bzd_lib::settings::DBSettings;
use bzd_lib::settings::Settings;

use bzd_lib::settings::HttpSettings;
use serde::Deserialize;

use crate::app::messages::settings::MessagesSettings;

#[derive(Deserialize, Clone)]
pub struct AppSettings {
    pub http: HttpSettings,
    pub db: DBSettings,
    pub messages: MessagesSettings,
}

impl Settings<AppSettings> for AppSettings {}
