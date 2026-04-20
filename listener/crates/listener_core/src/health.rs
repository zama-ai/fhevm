//! HTTP health endpoints: `/livez` and `/readyz`.
//!
//! # Design
//!
//! - **`/livez`** — stateless liveness beacon. Always returns
//!   `200 {"status":"ok"}`. The only condition that can make it fail is the
//!   process being hard-down (no TCP listener, no axum task) — in which case
//!   Kubernetes already restarts the pod via the probe's TCP error. We
//!   deliberately do NOT infer logical stalls here: a stall is not fixed by
//!   a restart, and false-positive 503s during transient broker/DB blips
//!   would cause pointless restart loops. Stall detection belongs in
//!   Prometheus-backed alerting (queue depth, cursor progress, etc.).
//!
//! - **`/readyz`** — one-shot readiness probe. Runs a single DB ping
//!   (`SELECT 1`) plus a broker connectivity check. Returns
//!   `200 {"status":"ok"}` when both respond; `503 {"status":"error","reason":"not ready"}`
//!   otherwise. K8s readiness failures only **gate traffic** — they never
//!   restart the pod — so it is safe to 503 during an upstream outage. The
//!   probe has no app-level retry: K8s `readinessProbe.periodSeconds` ×
//!   `failureThreshold` is the tolerance budget.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use broker::Broker;
use serde::Serialize;
use sqlx::PgPool;
use tracing::{info, warn};

/// JSON body returned by `/livez` and `/readyz`.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl HealthResponse {
    fn ok() -> Self {
        Self {
            status: "ok",
            reason: None,
        }
    }

    fn error(reason: &'static str) -> Self {
        Self {
            status: "error",
            reason: Some(reason.to_string()),
        }
    }
}

/// Readiness probe for upstream dependencies (DB + broker).
#[derive(Clone)]
pub struct ReadinessChecker {
    broker: Broker,
    pool: PgPool,
}

impl ReadinessChecker {
    pub fn new(broker: Broker, pool: PgPool) -> Self {
        Self { broker, pool }
    }

    async fn check_once(&self) -> Result<(), String> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| format!("database: {e}"))?;
        self.broker
            .health_check()
            .await
            .map_err(|e| format!("broker: {e}"))?;
        Ok(())
    }

    async fn check(&self) -> (StatusCode, Json<HealthResponse>) {
        match self.check_once().await {
            Ok(()) => (StatusCode::OK, Json(HealthResponse::ok())),
            Err(e) => {
                warn!(error = %e, "readyz: dependency probe failed");
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(HealthResponse::error("not ready")),
                )
            }
        }
    }
}

async fn livez_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse::ok()))
}

async fn readyz_handler(State(readyz): State<Arc<ReadinessChecker>>) -> impl IntoResponse {
    readyz.check().await
}

/// Build the axum `Router` exposing `/livez` and `/readyz`.
///
/// `/livez` is stateless; only `/readyz` needs the `ReadinessChecker`.
pub fn router(readyz: ReadinessChecker) -> Router {
    Router::new()
        .route("/livez", get(livez_handler))
        .route("/readyz", get(readyz_handler))
        .with_state(Arc::new(readyz))
}

/// Bind `addr` and spawn the HTTP server as a background tokio task.
///
/// Returns once the TCP listener is bound, so the caller can proceed to start
/// the consumer loop. Errors only on bind failure.
pub async fn serve(addr: SocketAddr, app: Router) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(addr = %addr, "HTTP endpoints listening: /livez /readyz");
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            tracing::error!(error = %e, "HTTP server exited");
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_response_ok_shape() {
        let body = serde_json::to_string(&HealthResponse::ok()).unwrap();
        assert_eq!(body, r#"{"status":"ok"}"#);
    }

    #[test]
    fn health_response_error_shape() {
        let body = serde_json::to_string(&HealthResponse::error("not ready")).unwrap();
        assert_eq!(body, r#"{"status":"error","reason":"not ready"}"#);
    }
}
