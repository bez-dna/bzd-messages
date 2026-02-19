use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TopicsSettings {
    pub events: EventsSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub topics_users: EventsTopicsUsersSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsTopicsUsersSettings {
    pub subject: String,
}
