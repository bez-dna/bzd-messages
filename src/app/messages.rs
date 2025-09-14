use bzd_messages_api::messages_service_server::MessagesServiceServer;

use crate::app::{messages::grpc::GrpcMessagesService, state::AppState};

mod grpc;
pub mod repo;
mod service;

pub fn messages_service(state: AppState) -> MessagesServiceServer<GrpcMessagesService> {
    MessagesServiceServer::new(GrpcMessagesService::new(state))
}
