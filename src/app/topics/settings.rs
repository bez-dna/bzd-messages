use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct TopicsSettings {
    pub events: EventsSettings,
}

#[derive(Deserialize, Clone)]
pub struct EventsSettings {
    pub topic_user: TopicUserSettings,
}

#[derive(Deserialize, Clone)]
pub struct TopicUserSettings {
    pub subject: String,
}
