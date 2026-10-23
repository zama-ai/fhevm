mod error;
mod http;
pub mod in_flight;
mod metrics;
pub mod pads;
pub mod path;
pub mod waiter;

pub use path::PrfPath;

use alloy::primitives::Address;
use anyhow::{bail, Context};
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use in_flight::InFlight;
use pads::Admissions;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::info;
use waiter::PadWaiter;

pub struct Config {
    pub http_endpoint: SocketAddr,
    pub database_url: String,
    pub database_pool_size: u32,
    /// RFC-038 `PRF_MAX_CONCURRENT_PER_CONNECTOR`: pad bits pending for one connector before `429`.
    pub max_pending_bits_per_connector: i64,
    /// Admissions of one connector that run at once in this pod.
    pub max_adds_per_connector: usize,
    /// How often the waiting requests read their rows, in case a `pad_ready` notification is lost.
    pub pad_poll_interval: Duration,
    /// The `Retry-After` of a `429`.
    pub retry_after: Duration,
    /// The `FhevmSig` domain and audience that the proxy verifies. prf-service only recovers the
    /// connector from the forwarded header: it trusts the proxy and the NetworkPolicy.
    pub auth: AuthConfig,
}

pub struct AuthConfig {
    pub ethereum_chain_id: u64,
    pub protocol_config_address: Address,
    pub coprocessor_signer_address: Address,
}

/// What every request of one pod shares.
pub(crate) struct State {
    pub(crate) admissions: Admissions,
    pub(crate) waiter: PadWaiter,
    pub(crate) in_flight: InFlight,
    pub(crate) auth: fhevm_sig_auth::FhevmSigVerifier,
    pub(crate) retry_after: Duration,
}

/// The internal PRF output endpoint. Only the proxy calls it.
pub struct PrfService {
    server: actix_web::dev::Server,
    state: Arc<State>,
    pool: PgPool,
    cancel_token: CancellationToken,
}

impl PrfService {
    /// Binds the HTTP server. It answers requests after `start`, until `cancel_token` is cancelled.
    pub async fn new(config: Config, cancel_token: CancellationToken) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.database_pool_size)
            .connect(&config.database_url)
            .await
            .context("connect to the database")?;
        let domain = fhevm_sig_auth::domain(
            config.auth.ethereum_chain_id,
            config.auth.protocol_config_address,
        );
        let state = Arc::new(State {
            admissions: Admissions::new(
                pool.clone(),
                config.max_pending_bits_per_connector,
                config.max_adds_per_connector,
            ),
            waiter: PadWaiter::new(pool.clone(), config.pad_poll_interval),
            in_flight: InFlight::default(),
            auth: fhevm_sig_auth::FhevmSigVerifier::new(
                domain,
                config.auth.coprocessor_signer_address,
                Default::default(),
            ),
            retry_after: config.retry_after,
        });
        let server =
            http::run_server(config.http_endpoint, state.clone(), cancel_token.clone())?;
        info!("HTTP server listening at: {}", config.http_endpoint);
        Ok(Self {
            server,
            state,
            pool,
            cancel_token,
        })
    }

    pub fn health(&self) -> Health {
        Health {
            pool: self.pool.clone(),
        }
    }

    pub async fn start(self) -> anyhow::Result<()> {
        let Self {
            server,
            state,
            cancel_token,
            ..
        } = self;
        tokio::select! {
            res = server => res.context("HTTP server error")?,
            res = state.waiter.run(cancel_token.child_token()) => {
                res.context("pad_ready listener error")?;
            }
        }
        if !cancel_token.is_cancelled() {
            bail!("PRF service exited unexpectedly");
        }
        info!("PRF service stopped");
        Ok(())
    }
}

pub struct Health {
    pub pool: PgPool,
}

impl HealthCheckService for Health {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        status.set_db_connected(&self.pool).await;
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
