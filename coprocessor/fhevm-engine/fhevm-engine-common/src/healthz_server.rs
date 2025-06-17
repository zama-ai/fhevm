use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, Router},
};
use serde::Serialize;
use sqlx::PgPool;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[derive(Serialize)]
struct HealthResponse {
    status_code: String,
    status: String,
    fields: HashMap<String, bool>,
    details: Option<String>,
}

impl From<HealthStatus> for HealthResponse {
    fn from(status: HealthStatus) -> Self {
        let healthy = status.fields.iter().all(|(_, s)| *s);

        Self {
            status_code: if status.is_healthy() { "200" } else { "503" }.to_string(),
            status: if healthy {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            fields: status.fields,
            details: Some(status.error_details.join("; ")),
        }
    }
}

pub trait HealthCheckService: Send + Sync {
    fn health_check(&self) -> impl std::future::Future<Output = HealthStatus> + Send;
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

    async fn liveness_handler(State(_service): State<Arc<S>>) -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status_code": "200",
                "status": "alive"
            })),
        )
    }
}

#[derive(Clone, Default)]
pub struct HealthStatus {
    pub fields: HashMap<String, bool>,
    pub error_details: Vec<String>,
}

impl HealthStatus {
    /// Checks DB availability by reusing the service internal DB connection pool
    ///
    /// query has its internal timeout
    pub async fn register_db_status(&mut self, pool: &PgPool) {
        let mut is_connected = false;
        match sqlx::query("SELECT 1").execute(pool).await {
            Ok(_) => {
                is_connected = true;
            }
            Err(e) => {
                self.error_details
                    .push(format!("Database query error: {}", e));
            }
        }
        self.fields
            .insert("database_connected".to_string(), is_connected);
    }

    pub fn is_healthy(&self) -> bool {
        self.fields.iter().all(|(_, s)| *s)
    }
}
