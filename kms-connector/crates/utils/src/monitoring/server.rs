use crate::monitoring::{health::Healthcheck, otlp::metrics_responder};
use actix_web::{HttpResponse, web::Data};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// Number of workers for the monitoring server.
const MONITORING_SERVER_WORKER: usize = 1;

pub fn default_monitoring_endpoint() -> String {
    "0.0.0.0:9100".to_string()
}

/// Starts the HTTP server exposing the healthchecks and metrics collection endpoints.
pub fn start_monitoring_server<S>(
    endpoint: SocketAddr,
    state: S,
    cancel_token: CancellationToken,
) -> JoinHandle<()>
where
    S: Healthcheck + Clone + Send + Sync + 'static,
{
    tokio::spawn(async move {
        let monitoring_server = match actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .app_data(Data::new(state.clone()))
                .route("/metrics", actix_web::web::to(metrics_responder))
                .route("/healthz", actix_web::web::to(healthcheck_responder::<S>))
                .route("/liveness", actix_web::web::to(liveness_responder))
                .route("/version", actix_web::web::to(version_responder::<S>))
        })
        .bind(&endpoint)
        {
            Ok(server) => server,
            Err(e) => return error!("Failed to bind monitoring server to {endpoint}: {e}"),
        };
        info!("Monitoring server listening at: {endpoint}");

        select! {
            res = monitoring_server.workers(MONITORING_SERVER_WORKER).run() => if let Err(e) = res {
                error!("Monitoring server stopped on error: {e}");
            },
            _ = cancel_token.cancelled() => info!("Monitoring server successfully stopped")
        }
    })
}

/// Performs the healthcheck verification using the service's `State`.
async fn healthcheck_responder<S: Healthcheck>(state: Data<S>) -> impl actix_web::Responder {
    state.healthcheck().await
}

/// Responder used to check if the monitoring server is still up and running.
async fn liveness_responder() -> impl actix_web::Responder {
    HttpResponse::Ok().json(LivenessResponse {
        status_code: "200".to_string(),
        status: "alive".to_string(),
    })
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LivenessResponse {
    pub status_code: String,
    pub status: String,
}

/// The commit hash (shor 7 chars format) used during the build of the service.
pub const GIT_COMMIT_HASH: &str = git_version::git_version!(args = ["--always", "--exclude", "*"]);

/// Responder used to provide version and build information of the service.
async fn version_responder<S: Healthcheck>() -> impl actix_web::Responder {
    HttpResponse::Ok().json(VersionResponse {
        name: S::service_name().to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        build: GIT_COMMIT_HASH.to_string(),
    })
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct VersionResponse {
    pub name: String,
    pub version: String,
    pub build: String,
}
