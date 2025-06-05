use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

async fn metrics() -> impl IntoResponse {
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let metrics_data = encoder
        .encode_to_string(&metric_families)
        .expect("can't encode metrics");

    // Return metrics with content-type header
    ([(header::CONTENT_TYPE, "text/plain")], metrics_data)
}

async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn run_metrics_server(
    args: crate::daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("metrics server listening at {}", args.metrics_addr);

    // Create router with our routes
    let app = Router::new()
        .route("/metrics", get(metrics))
        .route("/health", get(healthcheck));

    // Parse address string to SocketAddr
    let addr: SocketAddr = args.metrics_addr.parse()?;

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
