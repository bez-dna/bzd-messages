use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagesSettings {
    // pub messages_limit: i64,
    pub limits: LimitsSettings,

    pub events: EventsSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub message: MessageSettings,
    pub message_topic: MessageTopicSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessageSettings {
    pub subject: String,
}

#[derive(Deserialize, Clone)]
pub struct MessageTopicSettings {
    pub subject: String,
}

#[derive(Deserialize, Clone)]
pub struct LimitsSettings {
    pub user: u64,
    pub message: u64,
}
