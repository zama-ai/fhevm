//! Prometheus metrics server on its own port (`APP_METRICS__ENDPOINT`).
//! Mirrors relayer/src/metrics/server.rs.

use axum::{response::IntoResponse, routing::get, Router};
use prometheus::{Registry, TextEncoder};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

async fn metrics_handler(registry: Registry) -> impl IntoResponse {
    let families = registry.gather();
    match TextEncoder::new().encode_to_string(&families) {
        Ok(body) => (axum::http::StatusCode::OK, body).into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "failed to encode metrics",
        )
            .into_response(),
    }
}

/// Binds the metrics server and returns its `(listener, addr)` ready to serve.
pub async fn bind(endpoint: &str) -> anyhow::Result<(TcpListener, SocketAddr)> {
    let addr: SocketAddr = endpoint.parse()?;
    let listener = TcpListener::bind(addr).await?;
    let actual = listener.local_addr()?;
    info!("metrics server listening at http://{actual}");
    Ok((listener, actual))
}

/// Serves `/metrics` until the listener errors; intended to be spawned.
pub async fn serve(listener: TcpListener, registry: Registry) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/metrics", get(move || metrics_handler(registry.clone())))
        .route("/health", get(|| async { axum::http::StatusCode::OK }));
    axum::serve(listener, app).await?;
    Ok(())
}
