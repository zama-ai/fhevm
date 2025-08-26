use axum::{handler::get, response::IntoResponse, Router};
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

pub async fn run_metrics_server(registry: Registry, endpoint: String) {
    let addr: SocketAddr = endpoint.parse().expect("Invalid metrics endpoint address");
    info!("metrics server listening at http://{}", addr);

    let app = Router::new()
        .route(
            "/metrics",
            get({
                let registry = registry.clone();
                move || metrics_handler(registry.clone())
            }),
        )
        .route("/health", get(health_handler));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("metrics server failed");
}
