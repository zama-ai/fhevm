use crate::orchestrator::Orchestrator;
use axum::{response::IntoResponse, routing::get, Router};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

async fn wait_for_ready(addr: SocketAddr) -> anyhow::Result<()> {
    const MAX_RETRIES: u32 = 10;
    let url = format!("http://{}/health", addr);
    for _ in 0..MAX_RETRIES {
        if reqwest::get(&url)
            .await
            .is_ok_and(|r| r.status().is_success())
        {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    Err(anyhow::anyhow!("Metrics server failed to start"))
}

async fn metrics_handler(registry: Registry) -> impl axum::response::IntoResponse {
    let metric_families = registry.gather();
    let encoder = TextEncoder::new();
    match encoder.encode_to_string(&metric_families) {
        Ok(metrics) => (axum::http::StatusCode::OK, metrics).into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to encode metrics",
        )
            .into_response(),
    }
}

async fn health_handler() -> impl axum::response::IntoResponse {
    axum::http::StatusCode::OK
}

/// Initializes a http server for metrics endpoint and binds it to the given registry. The port in
/// endpoint can either be explicitly specified or set to :0 (in which case listener will bind to
/// free port assigned by OS). The actual socket address is returned.
pub async fn run_metrics_server(
    registry: Registry,
    endpoint: String,
    orchestrator: Arc<Orchestrator>,
) -> SocketAddr {
    let addr: SocketAddr = endpoint.parse().expect("Invalid metrics endpoint address");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let actual_addr = listener.local_addr().unwrap();

    info!("metrics server listening at http://{}", actual_addr);

    let app = Router::new()
        .route(
            "/metrics",
            get({
                let registry = registry.clone();
                move || metrics_handler(registry.clone())
            }),
        )
        .route("/health", get(health_handler));

    // Use orchestrator's task manager instead of raw tokio::spawn
    let addr_for_readiness = actual_addr;
    orchestrator
        .spawn_task_and_wait_ready(
            "metrics_server_axum",
            async move {
                axum::serve(listener, app)
                    .await
                    .expect("metrics server failed");
            },
            async move {
                // Wait for metrics server to be ready with actual health check
                wait_for_ready(addr_for_readiness).await
            },
        )
        .await
        .unwrap();

    actual_addr
}
