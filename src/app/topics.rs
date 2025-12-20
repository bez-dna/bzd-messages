use bzd_messages_api::topics_service_server::TopicsServiceServer;

use crate::app::{state::AppState, topics::grpc::GrpcTopicsService};

mod events;
mod grpc;
pub mod repo;
mod service;
pub mod settings;

pub fn topics_service(state: AppState) -> TopicsServiceServer<GrpcTopicsService> {
    TopicsServiceServer::new(GrpcTopicsService::new(state))
}
