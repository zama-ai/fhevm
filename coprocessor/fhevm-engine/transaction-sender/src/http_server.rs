use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{transaction_sender::TransactionSender, HealthStatus};
use alloy::{network::Ethereum, providers::Provider};

#[derive(Serialize)]
struct HealthResponse {
    status_code: String,
    status: String,
    database_connected: bool,
    blockchain_connected: bool,
    details: Option<String>,
}

impl From<HealthStatus> for HealthResponse {
    fn from(status: HealthStatus) -> Self {
        Self {
            status_code: if status.healthy { "200" } else { "503" }.to_string(),
            status: if status.healthy {
                "healthy".to_string()
            } else {
                "unhealthy".to_string()
            },
            database_connected: status.database_connected,
            blockchain_connected: status.blockchain_connected,
            details: status.details,
        }
    }
}

pub struct HttpServer<P: Provider<Ethereum> + Clone + Send + Sync + 'static> {
    sender: Arc<TransactionSender<P>>,
    port: u16,
    cancel_token: CancellationToken,
}

impl<P: Provider<Ethereum> + Clone + Send + Sync + 'static> HttpServer<P> {
    pub fn new(
        sender: Arc<TransactionSender<P>>,
        port: u16,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            sender,
            port,
            cancel_token,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/healthz", get(health_handler))
            .route("/liveness", get(liveness_handler))
            .with_state(self.sender.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!(address = %addr, "Starting HTTP server");

        // Create a shutdown future that owns the token
        let cancel_token = self.cancel_token.clone();
        let shutdown = async move {
            cancel_token.cancelled().await;
        };

        let listener = TcpListener::bind(addr).await?;
        let server =
            axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown);

        if let Err(err) = server.await {
            error!(error = %err, "HTTP server error");
            return Err(anyhow::anyhow!("HTTP server error: {}", err));
        }

        Ok(())
    }
}

// Health handler returns appropriate HTTP status code based on health
async fn health_handler<P: Provider<Ethereum> + Clone + Send + Sync + 'static>(
    State(sender): State<Arc<TransactionSender<P>>>,
) -> impl IntoResponse {
    let status = sender.health_check().await;
    let http_status = if status.healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    // Return HTTP status code that matches the health status
    (http_status, Json(HealthResponse::from(status)))
}

async fn liveness_handler<P: Provider<Ethereum> + Clone + Send + Sync + 'static>(
    State(_sender): State<Arc<TransactionSender<P>>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status_code": "200",
            "status": "alive"
        })),
    )
}