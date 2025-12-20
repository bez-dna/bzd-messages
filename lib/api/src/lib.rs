pub const MESSAGES_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("messages_descriptor");

tonic::include_proto!("bzd.messages.messages");

pub const TOPICS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("topics_descriptor");

tonic::include_proto!("bzd.messages.topics");

pub mod events {
    pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("events_descriptor");

    tonic::include_proto!("bzd.messages.events");
}
