use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagesSettings {
    // pub messages_limit: i64,
    pub limits: LimitsSettings,

    pub events: EventsSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub messages_topics: EventsMessagesTopicsSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsMessagesTopicsSettings {
    pub subject: String,
}

#[derive(Deserialize, Clone)]
pub struct LimitsSettings {
    pub user: u64,
    pub message: u64,
}
