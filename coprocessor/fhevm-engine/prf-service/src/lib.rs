mod error;
mod http;
mod metrics;
mod pad_source;
mod path;

use anyhow::{bail, Context};
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use std::net::SocketAddr;
use tokio_util::sync::CancellationToken;
use tracing::info;

pub struct Config {
    pub http_endpoint: SocketAddr,
}

/// The internal PRF output endpoint. Only the proxy calls it.
pub struct PrfService {
    server: actix_web::dev::Server,
    cancel_token: CancellationToken,
}

impl PrfService {
    /// Binds the HTTP server. It answers requests after `start`, until `cancel_token` is cancelled.
    pub fn new(config: Config, cancel_token: CancellationToken) -> anyhow::Result<Self> {
        let server = http::run_server(config.http_endpoint, cancel_token.clone())?;
        info!("HTTP server listening at: {}", config.http_endpoint);
        Ok(Self {
            server,
            cancel_token,
        })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        self.server.await.context("HTTP server error")?;
        if !self.cancel_token.is_cancelled() {
            bail!("HTTP server exited unexpectedly");
        }
        info!("PRF service stopped");
        Ok(())
    }
}

/// The HTTP server is bound before the health server starts, so the service is always ready.
pub struct Health;

impl HealthCheckService for Health {
    async fn health_check(&self) -> HealthStatus {
        HealthStatus::default()
    }

    async fn is_alive(&self) -> bool {
        true
    }

    fn get_version(&self) -> Version {
        Version {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            build: option_env!("BUILD_ID").unwrap_or("unknown"),
        }
    }
}
