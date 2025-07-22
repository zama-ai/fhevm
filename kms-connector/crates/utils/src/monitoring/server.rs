use crate::monitoring::{health::Healthcheck, otlp::metrics_responder};
use actix_web::{HttpResponse, web::Data};
use std::{collections::HashMap, net::SocketAddr};
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// Number of workers for the monitoring server.
const MONITORING_SERVER_WORKER: usize = 1;

pub fn default_monitoring_endpoint() -> String {
    "0.0.0.0:9100".to_string()
}

/// Starts the HTTP server exposing the healthchecks and metrics collection endpoints.
pub fn start_monitoring_server<S>(endpoint: SocketAddr, state: S, cancel_token: CancellationToken)
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
    });
}

/// Performs the healthcheck verification using the service's `State`.
async fn healthcheck_responder<S: Healthcheck>(state: Data<S>) -> impl actix_web::Responder {
    state.healthcheck().await
}

/// Responder used to check if the monitoring server is still up and running.
async fn liveness_responder() -> impl actix_web::Responder {
    let mut body = HashMap::new();
    body.insert("status_code", "200");
    body.insert("status", "alive");
    HttpResponse::Ok().json(body)
}
