mod error;
mod http;
pub mod in_flight;
mod instance;
mod metrics;
pub mod pads;
pub mod waiter;

pub use http::SIGNER_HEADER;

use actix_web::web;
use anyhow::{bail, Context};
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use in_flight::InFlight;
use pads::Admissions;
use sqlx::PgPool;
use std::{net::SocketAddr, num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use tracing::info;
use waiter::PadWaiter;

/// The same limit as the database check of `HealthStatus`.
const DB_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Config {
    pub http_endpoint: SocketAddr,
    /// Pending pad bits of each connector (`PRF_MAX_CONCURRENT_PER_CONNECTOR`).
    pub max_pending_bits: i64,
    /// Requests in progress of each connector in this pod (`PRF_MAX_IN_FLIGHT_PER_CONNECTOR`).
    pub max_in_flight_per_connector: NonZeroUsize,
    /// Requests of each connector that add a row at the same time, in this pod.
    pub max_adds_per_connector: NonZeroUsize,
    /// Interval of the read that finds a lost `pad_ready`.
    pub pad_poll_interval: Duration,
}

/// The internal PRF output endpoint. Only the proxy calls it.
pub struct PrfService {
    server: actix_web::dev::Server,
    pool: PgPool,
    waiter: Arc<PadWaiter>,
    cancel_token: CancellationToken,
}

impl PrfService {
    /// Binds the HTTP server. It answers requests after `start`, until `cancel_token` is cancelled.
    /// After `draining`, each response closes its connection, so the proxy moves to another pod.
    pub fn new(
        config: Config,
        pool: PgPool,
        cancel_token: CancellationToken,
        draining: CancellationToken,
    ) -> anyhow::Result<Self> {
        let waiter = Arc::new(PadWaiter::new(pool.clone(), config.pad_poll_interval));
        let state = web::Data::new(http::AppState {
            pool: pool.clone(),
            draining,
            in_flight: InFlight::new(config.max_in_flight_per_connector),
            admissions: Admissions::new(
                pool.clone(),
                config.max_pending_bits,
                config.max_adds_per_connector,
            ),
            waiter: waiter.clone(),
        });
        let server = http::run_server(config.http_endpoint, state, cancel_token.clone())?;
        info!("HTTP server listening at: {}", config.http_endpoint);
        Ok(Self {
            server,
            pool,
            waiter,
            cancel_token,
        })
    }

    /// Readiness: the database.
    pub fn health(&self) -> Health {
        Health(self.pool.clone())
    }

    /// Serves until `cancel_token`. Returns an error if the server stops.
    pub async fn start(self) -> anyhow::Result<()> {
        let server = self.server;
        tokio::pin!(server);
        tokio::select! {
            res = &mut server => res.context("HTTP server error")?,
            () = self.waiter.run(self.cancel_token.clone()) => {
                server.await.context("HTTP server error")?;
            }
        }
        if !self.cancel_token.is_cancelled() {
            bail!("HTTP server exited unexpectedly");
        }
        info!("PRF service stopped");
        Ok(())
    }
}

/// Readiness: the database, and the rights of the service on `pads`.
pub struct Health(pub PgPool);

impl HealthCheckService for Health {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        status.set_db_connected(&self.0).await;
        // Without these rights the role connects, but every request gets `503`. INSERT is on columns.
        let rights = sqlx::query_scalar::<_, bool>(
            "SELECT has_table_privilege('pads', 'SELECT') AND has_any_column_privilege('pads', 'INSERT')",
        )
        .fetch_one(&self.0);
        let has_rights = matches!(timeout(DB_CHECK_TIMEOUT, rights).await, Ok(Ok(true)));
        if !has_rights {
            status.add_error_details("No SELECT and INSERT rights on pads".to_string());
        }
        status.set_custom_check("pads", has_rights, true);
        status
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
