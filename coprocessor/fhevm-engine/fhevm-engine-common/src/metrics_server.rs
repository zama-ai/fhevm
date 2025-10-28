use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, Router},
};

use std::{io, net::SocketAddr};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

struct HttpServer {
    addr: String,
    cancel_token: CancellationToken,
}

impl HttpServer {
    pub fn new(addr: &str, cancel_token: CancellationToken) -> Self {
        Self {
            addr: addr.to_string(),
            cancel_token,
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        let app = Router::new().route("/metrics", get(Self::metrics_handler));

        let addr = self.addr.parse::<SocketAddr>().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid address {}: {}", self.addr, e),
            )
        })?;
        info!(addr = %addr, "Starting metrics server");

        let shutdown = {
            let cancel_token = self.cancel_token.clone();
            async move {
                cancel_token.cancelled().await;
            }
        };

        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown)
            .await
    }

    async fn metrics_handler() -> impl IntoResponse {
        let encoder = prometheus::TextEncoder::new();
        let metric_families = prometheus::gather();

        debug!(num_metrics = metric_families.len(), "scrape event");

        match encoder.encode_to_string(&metric_families) {
            Ok(encoded_metrics) => (StatusCode::OK, encoded_metrics),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}

/// Spawns a HTTP server that exposes Prometheus metrics at the /metrics endpoint.
pub fn spawn(addr: Option<String>, cancel_token: CancellationToken) -> Option<JoinHandle<()>> {
    if let Some(metrics_future) = metrics_future(addr, cancel_token) {
        let handle = tokio::spawn(async move {
            metrics_future.await;
        });
        return Some(handle);
    }

    None
}

pub fn metrics_future(
    addr: Option<String>,
    cancel_token: CancellationToken,
) -> Option<impl std::future::Future<Output = ()>> {
    let Some(addr) = addr else {
        info!("Metrics server disabled");
        return None;
    };

    let server = HttpServer::new(&addr, cancel_token);
    Some(async move {
        if let Err(err) = server.run().await {
            error!(target = "metrics", err = %err, "server failed");
        }
        info!(addr = %server.addr, "Shutting down metrics server");
    })
}
