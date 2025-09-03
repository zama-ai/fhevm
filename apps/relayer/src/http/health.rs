use alloy::providers::{Provider, ProviderBuilder};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// Response structures for reliability endpoints
#[derive(Serialize, Deserialize)]
pub struct LivenessResponse {
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct VersionResponse {
    name: String,
    version: String,
    build: String,
}

// Health check trait for dependency checking
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    async fn check(&self) -> anyhow::Result<()>;
}

// FIXME: https://github.com/zama-ai/console/issues/555
// struct CacheHealthCheck {
//     cache: Arc<dyn KVStore>,
// }
//
// #[async_trait::async_trait]
// impl HealthCheck for CacheHealthCheck {
//     async fn check(&self) -> anyhow::Result<()> {
//         // TODO: add health-check method to KVSTore trait
//         Ok(())
//     }
// }

pub struct BlockchainHealthCheck {
    rpc_url: String,
}

#[async_trait::async_trait]
impl HealthCheck for BlockchainHealthCheck {
    async fn check(&self) -> anyhow::Result<()> {
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await?;
        provider.get_block_number().await?;
        Ok(())
    }
}

// Health checker to manage all dependency checks
pub struct HealthChecker {
    checks: HashMap<String, Arc<dyn HealthCheck>>,
}

impl HealthChecker {
    pub fn new(gateway_rpc_url: String, host_rpc_url: String) -> Self {
        let mut checks: HashMap<String, Arc<dyn HealthCheck>> = HashMap::new();

        // checks.insert(
        //     "cache".to_string(),
        //     Arc::new(CacheHealthCheck { /* ... */ }),
        // );

        checks.insert(
            "gateway".to_string(),
            Arc::new(BlockchainHealthCheck {
                rpc_url: gateway_rpc_url,
            }),
        );
        checks.insert(
            "host".to_string(),
            Arc::new(BlockchainHealthCheck {
                rpc_url: host_rpc_url,
            }),
        );
        Self { checks }
    }

    pub async fn check_all(&self) -> (bool, HashMap<String, String>) {
        let mut results = HashMap::new();
        let mut all_healthy = true;

        for (name, check) in &self.checks {
            match check.check().await {
                Ok(_) => {
                    results.insert(name.clone(), "ok".to_string());
                }
                Err(_) => {
                    results.insert(name.clone(), "fail".to_string());
                    all_healthy = false;
                }
            }
        }

        (all_healthy, results)
    }
}

// Handler functions
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

pub async fn health_handler(health_checker: Arc<HealthChecker>) -> impl IntoResponse {
    let (is_healthy, dependencies) = health_checker.check_all().await;

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
