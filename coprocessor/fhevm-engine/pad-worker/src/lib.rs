//! The pad-worker: one lane computes the PRF outputs of one PRF instance (RFC-038).

pub mod evaluator;

use anyhow::Context;
use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use sqlx::PgPool;
use std::{num::NonZeroUsize, time::Duration};
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use tracing::info;

/// The same limit as the database check of `HealthStatus`.
const DB_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct Config {
    /// The PRF instance of this lane.
    pub prf_instance: u16,
    /// Gateway key ID (`key_id_gw`) of the instance.
    pub key_id: Vec<u8>,
    /// Evaluations at the same time (`PRF_MAX_CONCURRENT`).
    pub max_concurrent: NonZeroUsize,
    /// A claim older than this is taken again (`PAD_CLAIM_TIMEOUT`).
    pub claim_timeout: Duration,
    /// Failed tries before a row fails for good (`PAD_MAX_ATTEMPTS`).
    pub max_attempts: u32,
    /// Interval of the read that finds a lost `pad_work_available` (`PAD_POLL_INTERVAL`).
    pub poll_interval: Duration,
    /// Pending rows older than this are not claimed (`PAD_PENDING_MAX_AGE`).
    pub pending_max_age: Duration,
}

pub struct PadWorker {
    config: Config,
    pool: PgPool,
    cancel_token: CancellationToken,
}

impl PadWorker {
    pub fn new(config: Config, pool: PgPool, cancel_token: CancellationToken) -> Self {
        Self {
            config,
            pool,
            cancel_token,
        }
    }

    pub fn health(&self) -> Health {
        Health(self.pool.clone())
    }

    /// Registers the lane, then runs until `cancel_token`.
    pub async fn run(self) -> anyhow::Result<()> {
        register_lane(&self.pool, self.config.prf_instance).await?;
        info!(prf_instance = self.config.prf_instance, "Lane registered");
        self.cancel_token.cancelled().await;
        info!("pad-worker stopped");
        Ok(())
    }
}

/// Adds the `prf_lanes` row of `prf_instance`, if it does not exist.
pub async fn register_lane(pool: &PgPool, prf_instance: u16) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO prf_lanes (prf_instance) VALUES ($1) ON CONFLICT DO NOTHING",
        i32::from(prf_instance),
    )
    .execute(pool)
    .await
    .context("register the lane")?;
    Ok(())
}

/// Readiness: the database, and the rights of the pad-worker on `pads` and `prf_lanes`.
pub struct Health(pub PgPool);

impl HealthCheckService for Health {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        status.set_db_connected(&self.0).await;
        let rights = sqlx::query_scalar::<_, bool>(
            "SELECT has_column_privilege('pads', 'signature', 'UPDATE')
                AND has_column_privilege('prf_lanes', 'heartbeat_at', 'UPDATE')",
        )
        .fetch_one(&self.0);
        let has_rights = matches!(timeout(DB_CHECK_TIMEOUT, rights).await, Ok(Ok(true)));
        if !has_rights {
            status.add_error_details("No UPDATE rights on pads and prf_lanes".to_string());
        }
        status.set_custom_check("pad_rights", has_rights, true);
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
