use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Clone, Copy, Debug, Serialize)]
struct HealthResponse {
    ok: bool,
}

pub async fn run_health_server(port: u16) -> anyhow::Result<()> {
    let app = Router::new().route(
        "/healthz",
        get(|| async { Json(HealthResponse { ok: true }) }),
    );
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
