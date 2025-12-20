use bzd_messages_api::messages_service_server::MessagesServiceServer;

use crate::app::{messages::grpc::GrpcMessagesService, state::AppState};

mod events;
mod grpc;
pub mod repo;
mod service;
pub mod settings;

pub fn messages_service(state: AppState) -> MessagesServiceServer<GrpcMessagesService> {
    MessagesServiceServer::new(GrpcMessagesService::new(state))
}
