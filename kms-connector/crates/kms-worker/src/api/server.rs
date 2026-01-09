use actix_web::{HttpResponse, web::Data};
use connector_utils::monitoring::{
    health::Healthcheck,
    otlp::metrics_responder,
    server::{GIT_COMMIT_HASH, LivenessResponse, VersionResponse},
};
use std::net::SocketAddr;
use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{api::{ApiState, handlers}, monitoring::health::State};

/// Number of workers for the API server.
const API_SERVER_WORKERS: usize = 1;

/// Starts the HTTP server exposing the healthchecks, metrics, and V2 API endpoints.
pub fn start_api_server<P>(
    endpoint: SocketAddr,
    state: State<P>,
    api_state: ApiState,
    cancel_token: CancellationToken,
) -> JoinHandle<()>
where
    P: alloy::providers::Provider + Clone + Send + Sync + 'static,
{
    tokio::spawn(async move {
        let api_server = match actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .app_data(Data::new(state.clone()))
                .app_data(Data::new(api_state.clone()))
                .route("/metrics", actix_web::web::to(metrics_responder))
                .route("/healthz", actix_web::web::to(healthcheck_responder::<P>))
                .route("/liveness", actix_web::web::to(liveness_responder))
                .route("/version", actix_web::web::to(version_responder::<P>))
                .route("/v1/share/{request_id}", actix_web::web::get().to(handlers::get_share_handler))
                .route("/v1/health", actix_web::web::get().to(handlers::health_handler))
        })
        .bind(&endpoint)
        {
            Ok(server) => server,
            Err(e) => return error!("Failed to bind API server to {endpoint}: {e}"),
        };
        info!("API server listening at: {endpoint}");

        select! {
            res = api_server.workers(API_SERVER_WORKERS).run() => if let Err(e) = res {
                error!("API server stopped on error: {e}");
            },
            _ = cancel_token.cancelled() => info!("API server successfully stopped")
        }
    })
}

async fn healthcheck_responder<P: alloy::providers::Provider>(state: Data<State<P>>) -> impl actix_web::Responder {
    state.healthcheck().await
}

async fn liveness_responder() -> impl actix_web::Responder {
    HttpResponse::Ok().json(LivenessResponse {
        status_code: "200".to_string(),
        status: "alive".to_string(),
    })
}

async fn version_responder<P: alloy::providers::Provider>() -> impl actix_web::Responder {
    HttpResponse::Ok().json(VersionResponse {
        name: State::<P>::service_name().to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        build: GIT_COMMIT_HASH.to_string(),
    })
}
