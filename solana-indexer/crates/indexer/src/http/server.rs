//! axum 0.8 HTTP server wiring. Merges handler-struct `.routes()` with the
//! health/version routers, mirroring relayer/src/http/server.rs.

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

use crate::http::endpoints::{
    build_proof::BuildProofHandler, health, lineage_leaf::LineageLeafHandler,
};
use crate::http::AppState;

/// Builds the full router from the shared state.
pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(BuildProofHandler.routes(state.clone()))
        .merge(LineageLeafHandler.routes(state.clone()))
        .with_state(state)
}

/// Binds the HTTP server and returns its `(listener, addr)`.
pub async fn bind(endpoint: &str) -> anyhow::Result<(TcpListener, SocketAddr)> {
    let addr: SocketAddr = endpoint.parse()?;
    let listener = TcpListener::bind(addr).await?;
    let actual = listener.local_addr()?;
    info!("HTTP server listening at http://{actual}");
    Ok((listener, actual))
}

/// Serves the router until the listener errors; intended to be spawned.
pub async fn serve(listener: TcpListener, state: Arc<AppState>) -> anyhow::Result<()> {
    axum::serve(listener, router(state)).await?;
    Ok(())
}
