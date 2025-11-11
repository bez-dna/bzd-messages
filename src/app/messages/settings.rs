use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagesSettings {
    pub limit: i64,
}
