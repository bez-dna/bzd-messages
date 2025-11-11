use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagesSettings {
    pub user_messages_limit: i64,
    pub message_messages_limit: i64,
}
