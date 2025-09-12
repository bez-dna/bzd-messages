use axum::Router;
use bzd_lib::error::Error;
use bzd_lib::settings::Settings as _;
use tonic::service::Routes;
use tracing::info;

use crate::app::settings::AppSettings;
use crate::app::state::AppState;

mod error;
mod settings;
mod state;
mod topics;

pub async fn run() -> Result<(), Error> {
    let settings = AppSettings::new()?;
    let state = AppState::new(settings).await?;

    http_and_grpc(&state).await?;

    Ok(())
}

async fn http_and_grpc(state: &AppState) -> Result<(), Error> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(bzd_messages_api::MESSAGES_FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(bzd_messages_api::TOPICS_FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let (_, health_service) = tonic_health::server::health_reporter();

    let router = Router::new().with_state(());
    let routes = Routes::from(router);
    let router = routes
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(topics::topics_service(state.clone()))
        .into_axum_router();

    let listener = tokio::net::TcpListener::bind(&state.settings.http.endpoint).await?;

    info!("app: started on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
