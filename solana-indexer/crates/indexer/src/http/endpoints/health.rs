//! Health/version endpoints, mirroring relayer/src/http/endpoints/health.rs.
//! `/healthz` pings the DB; `/version` returns the build-embedded semver + git sha.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::http::AppState;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Serialize, ToSchema)]
pub struct VersionResponse {
    pub version: String,
    pub git_sha: String,
}

/// Health/version routes, with `AppState` supplied at merge time.
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/liveness", get(liveness))
        .route("/healthz", get(healthz))
        .route("/version", get(version))
}

/// Liveness probe — returns as soon as the server can respond.
#[utoipa::path(get, path = "/liveness", responses((status = 200, description = "alive")))]
pub async fn liveness() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

/// Readiness probe — 200 after a DB ping, 503 on failure.
#[utoipa::path(
    get,
    path = "/healthz",
    responses((status = 200, description = "healthy"), (status = 503, description = "unhealthy"))
)]
pub async fn healthz(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(state.repo.pool())
        .await
    {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "db unavailable"),
    }
}

/// Build version + git sha embedded at compile time.
#[utoipa::path(get, path = "/version", responses((status = 200, body = VersionResponse)))]
pub async fn version() -> impl IntoResponse {
    let git_sha = format!(
        "{}{}",
        built_info::GIT_COMMIT_HASH.unwrap_or("unknown"),
        match built_info::GIT_DIRTY {
            Some(true) => "-dirty",
            Some(false) => "",
            None => "",
        }
    );
    (
        StatusCode::OK,
        Json(VersionResponse {
            version: built_info::PKG_VERSION.to_string(),
            git_sha,
        }),
    )
}
