pub const MESSAGES_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("messages_descriptor");

tonic::include_proto!("bzd.messages.messages");

pub const TOPICS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("topics_descriptor");

tonic::include_proto!("bzd.messages.topics");

pub mod events {
    pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("events_descriptor");

    tonic::include_proto!("bzd.messages.events");

    pub mod message {
        use strum_macros::{Display, EnumString};

        #[derive(PartialEq, Debug, EnumString, Display, Clone)]
        #[strum(ascii_case_insensitive)]
        pub enum Type {
            #[strum(serialize = "app.bezdna.message.created")]
            Created,
            #[strum(serialize = "app.bezdna.message.updated")]
            Updated,
            #[strum(serialize = "app.bezdna.message.deleted")]
            Deleted,
        }

        #[cfg(test)]
        mod tests {
            use std::str::FromStr;

            use bzd_lib::error::Error;

            use crate::events::message::Type;

            #[test]
            fn test_parse_and_serialize() -> Result<(), Error> {
                let tp = Type::from_str("app.bezdna.message.updated")?;
                assert_eq!(Type::Updated, tp);

                assert_eq!("app.bezdna.message.updated", tp.to_string());

                Ok(())
            }
        }
    }

    pub mod topic_user {
        use strum_macros::{Display, EnumString};

        #[derive(PartialEq, Debug, EnumString, Display, Clone)]
        #[strum(ascii_case_insensitive)]
        pub enum Type {
            #[strum(serialize = "app.bezdna.topic-user.created")]
            Created,
            #[strum(serialize = "app.bezdna.topic-user.updated")]
            Updated,
            #[strum(serialize = "app.bezdna.topic-user.deleted")]
            Deleted,
        }
    }
}
