use crate::orchestrator::Orchestrator;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LivenessResponse {
    #[schema(example = "alive")]
    status: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "healthy")]
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionResponse {
    #[schema(example = "fhevm-relayer")]
    name: String,
    #[schema(example = "0.9.0")]
    version: String,
    #[schema(example = "a1b2c3d4-clean")]
    build: String,
}

/// Liveness probe
#[utoipa::path(
    get,
    path = "/liveness",
    responses(
        (status = 200, description = "Service is alive.", body = LivenessResponse),
    ),
    tag = "Health",
    security(())
)]
pub async fn liveness_handler() -> impl IntoResponse {
    // Simple check - if we can respond, we're alive
    (
        StatusCode::OK,
        Json(LivenessResponse {
            status: "alive".to_string(),
        }),
    )
        .into_response()
}

/// Health check with dependency status
#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Service is healthy.", body = HealthResponse),
        (status = 503, description = "Service is unhealthy.", body = HealthResponse),
    ),
    tag = "Health",
    security(())
)]
pub async fn health_handler(orchestrator: Arc<Orchestrator>) -> impl IntoResponse {
    let (is_healthy, dependencies) = orchestrator.check_all_health().await;

    let status = if is_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let response = if dependencies.is_empty() {
        HealthResponse {
            status: if is_healthy { "healthy" } else { "unhealthy" }.to_string(),
            dependencies: None,
        }
    } else {
        HealthResponse {
            status: if is_healthy { "healthy" } else { "unhealthy" }.to_string(),
            dependencies: Some(dependencies),
        }
    };

    (status, Json(response)).into_response()
}

// Include the generated-file as a separate module
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Build version and git info
#[utoipa::path(
    get,
    path = "/version",
    responses(
        (status = 200, description = "Version information.", body = VersionResponse),
    ),
    tag = "Health",
    security(())
)]
pub async fn version_handler() -> impl IntoResponse {
    // These values should be injected at build time via environment variables
    let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "relayer-service".to_string());
    let build = format!(
        "{}-{}",
        built_info::GIT_COMMIT_HASH.unwrap_or("no_git_hash"),
        if let Some(is_dirty) = built_info::GIT_DIRTY {
            if is_dirty {
                "dirty"
            } else {
                "clean"
            }
        } else {
            "no_git_dirty"
        }
    );
    let version = built_info::PKG_VERSION.to_string();

    (
        StatusCode::OK,
        Json(VersionResponse {
            name: app_name,
            version,
            build,
        }),
    )
        .into_response()
}
