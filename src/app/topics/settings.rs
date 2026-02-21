use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TopicsSettings {
    pub events: EventsSettings,
    pub emojis: EmojisSettings,
}

#[derive(Deserialize, Clone)]
pub struct EmojisSettings {
    pub list: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub topics_users: EventsTopicsUsersSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsTopicsUsersSettings {
    pub subject: String,
}
