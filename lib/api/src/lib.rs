pub mod messages {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("messages_descriptor");

    tonic::include_proto!("bzd.messages.messages");
}

pub mod topics {
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("topics_descriptor");

    tonic::include_proto!("bzd.messages.topics");
}

pub mod events {
    pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("events_descriptor");

    tonic::include_proto!("bzd.messages.events");

    pub mod message_topic {
        use strum_macros::{Display, EnumString};

        #[derive(PartialEq, Debug, EnumString, Display, Clone)]
        #[strum(ascii_case_insensitive)]
        pub enum Type {
            #[strum(serialize = "app.bezdna.message-topic.created")]
            Created,
            #[strum(serialize = "app.bezdna.message-topic.deleted")]
            Deleted,
        }
    }

    pub mod topic_user {
        use strum_macros::{Display, EnumString};

        #[derive(PartialEq, Debug, EnumString, Display, Clone)]
        #[strum(ascii_case_insensitive)]
        pub enum Type {
            #[strum(serialize = "app.bezdna.topic-user.created")]
            Created,
            #[strum(serialize = "app.bezdna.topic-user.deleted")]
            Deleted,
        }
    }
}
