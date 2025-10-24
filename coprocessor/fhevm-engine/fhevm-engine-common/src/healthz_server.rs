use alloy_provider::Provider;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, Router},
};
use serde::Serialize;
use sqlx::PgPool;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, time::timeout};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::types::BlockchainProvider;

#[derive(Serialize)]
struct HealthResponse {
    status_code: String,
    status: String,
    dependencies: HashMap<&'static str, &'static str>,
    details: String,
}

impl From<HealthStatus> for HealthResponse {
    fn from(status: HealthStatus) -> Self {
        let details = status.error_details();
        let is_dependency = |key| {
            status
                .is_dependency_check
                .get(key)
                .copied()
                .unwrap_or(false)
        };
        let dependencies: HashMap<&'static str, &'static str> = status
            .checks
            .iter()
            .filter_map(|(&key, &value)| {
                if is_dependency(key) {
                    if value {
                        Some((key, "ok"))
                    } else {
                        Some((key, "fail"))
                    }
                } else {
                    None
                }
            })
            .collect();

        Self {
            status_code: if status.is_healthy() { "200" } else { "503" }.to_string(),
            status: if status.is_healthy() {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            dependencies,
            details,
        }
    }
}

#[derive(Serialize)]
pub struct Version {
    pub name: &'static str,
    pub version: &'static str,
    pub build: &'static str,
}

pub trait HealthCheckService: Send + Sync {
    fn health_check(&self) -> impl std::future::Future<Output = HealthStatus> + Send;
    fn is_alive(&self) -> impl std::future::Future<Output = bool> + Send;
    fn get_version(&self) -> Version;
}

/// Default implementation for the version information.
/// Rely on BUILD_ID environment variable at compile time
pub fn default_get_version() -> Version {
    Version {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        build: option_env!("BUILD_ID").unwrap_or("unknown"),
    }
}

pub struct HttpServer<S: HealthCheckService + Send + Sync + 'static> {
    service: Arc<S>,
    port: u16,
    cancel_token: CancellationToken,
}

impl<S: HealthCheckService + Send + Sync + 'static> HttpServer<S> {
    pub fn new(service: Arc<S>, port: u16, cancel_token: CancellationToken) -> Self {
        Self {
            service,
            port,
            cancel_token,
        }
    }
    pub async fn start(&self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/healthz", get(Self::health_handler))
            .route("/liveness", get(Self::liveness_handler))
            .route("/version", get(Self::version_handler))
            .with_state(self.service.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("Starting HTTP server on {}", addr);

        let shutdown = {
            let cancel_token = self.cancel_token.clone();
            async move {
                cancel_token.cancelled().await;
            }
        };

        let listener = TcpListener::bind(addr).await?;
        let server =
            axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown);

        if let Err(err) = server.await {
            error!("HTTP server error: {}", err);
            return Err(anyhow::anyhow!("HTTP server error: {}", err));
        }

        Ok(())
    }

    async fn health_handler(State(service): State<Arc<S>>) -> impl IntoResponse {
        let status = service.health_check().await;
        let http_status = if status.is_healthy() {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        (http_status, Json(HealthResponse::from(status)))
    }

    async fn liveness_handler(State(service): State<Arc<S>>) -> impl IntoResponse {
        if service.is_alive().await {
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status_code": "200",
                    "status": "alive"
                })),
            )
        } else {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "status_code": "503",
                    "status": "not_responding"
                })),
            )
        }
    }

    async fn version_handler(State(service): State<Arc<S>>) -> impl IntoResponse {
        let version = service.get_version();
        (StatusCode::OK, Json(serde_json::json!(version)))
    }
}

#[derive(Clone, Default)]
pub struct HealthStatus {
    // both dependencies and internal checks
    checks: HashMap<&'static str, bool>,
    // indicates if the check is added in dependencies JSON "dependencies" field
    is_dependency_check: HashMap<&'static str, bool>,
    error_details: Vec<String>,
}

impl HealthStatus {
    /// Checks DB availability by reusing the service internal DB connection pool
    ///
    /// query has its internal timeout
    pub async fn set_db_connected(&mut self, pool: &PgPool) {
        let reach = sqlx::query("SELECT 1").execute(pool);
        let reach_or_timeout = timeout(Duration::from_secs(5), reach).await;
        let is_connected = match reach_or_timeout {
            Ok(Ok(_)) => true,
            Ok(Err(_)) => {
                self.push_error_details("Database query error");
                false
            }
            Err(_) => {
                self.push_error_details("Database timeout");
                false
            }
        };
        self.checks.insert("database", is_connected);
        self.is_dependency_check.insert("database", true);
    }

    /// Checks if the blockchain is connected by executing a simple query
    pub async fn set_blockchain_connected(&mut self, provider: &BlockchainProvider) {
        // With a timeout because the provider can block an unlimited amount of time
        let reach = provider.get_block_number();
        let reach_or_timeout = timeout(Duration::from_secs(5), reach).await;
        let is_connected = match reach_or_timeout {
            Ok(Ok(_)) => true,
            Ok(Err(_)) => {
                self.push_error_details("Blockchain error.");
                false
            }
            Err(_) => {
                self.push_error_details("Blockchain timeout");
                false
            }
        };
        self.checks.insert("blockchain", is_connected);
        self.is_dependency_check.insert("blockchain", true);
    }

    pub fn set_custom_check(&mut self, check: &'static str, value: bool, is_dependency: bool) {
        self.checks.insert(check, value);
        self.is_dependency_check.insert(check, is_dependency);
    }

    pub fn add_error_details(&mut self, details: String) {
        self.error_details.push(details);
    }

    pub fn is_healthy(&self) -> bool {
        self.checks.iter().all(|(_, s)| *s)
    }

    fn push_error_details(&mut self, details: &str) {
        self.error_details.push(details.to_string());
    }

    pub fn error_details(&self) -> String {
        self.error_details
            .iter()
            .filter(|s| !s.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join("; ")
    }
}
