//! Kubernetes probes, on the same port as the API.

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;

use crate::App;

#[derive(Debug, Serialize)]
pub struct Status {
    pub status: &'static str,
}

/// `GET /liveness`: the process runs. Always 200.
pub async fn liveness() -> Json<Status> {
    Json(Status { status: "alive" })
}

/// `GET /healthz`: readiness. 200 while serving, 503 once shutdown started so Kubernetes stops routing to
/// this pod while the in-flight requests drain.
pub async fn healthz(app: State<App>) -> (StatusCode, Json<Status>) {
    if app.shutdown.is_cancelled() {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(Status {
                status: "shutting_down",
            }),
        )
    } else {
        (StatusCode::OK, Json(Status { status: "ready" }))
    }
}
