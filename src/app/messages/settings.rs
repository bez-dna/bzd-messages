use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagesSettings {
    pub message_messages_limit: i64,

    pub events: EventsSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub message: MessageSettings,
}

#[derive(Deserialize, Clone)]
pub struct MessageSettings {
    pub subject: String,
}
