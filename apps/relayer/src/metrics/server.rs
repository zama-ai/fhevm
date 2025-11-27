use axum::{response::IntoResponse, routing::get, Router};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;
use tracing::info;

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
    (axum::http::StatusCode::OK, "OK")
}

/// Initializes a http server for metrics endpoint and binds it to the given registry. The port in
/// endpoint can either be explicitly specified or set to :0 (in which case listener will bind to
/// free port assigned by OS). The actual socket address is returned.
pub async fn run_metrics_server(registry: Registry, endpoint: String) -> SocketAddr {
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

    // Spawn the server task
    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("metrics server failed");
    });

    actual_addr
}
