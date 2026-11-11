//! The pad-worker: one lane computes the PRF outputs of one PRF instance (RFC-038).

pub mod evaluator;
pub mod rows;

use alloy::primitives::keccak256;
use anyhow::Context;
use ciphertext_attestation::{
    prf::{consensus::MAX_PAD_LENGTH, Version as AttestationVersion},
    PrfOutputAttestation, PrfOutputAttestationPayload,
};
use evaluator::PadEvaluator;
use fhevm_engine_common::{
    healthz_server::{HealthCheckService, HealthStatus, Version},
    pg_pool::is_fatal_connection_error,
    types::CoproSigner,
};
use rows::Claim;
use sqlx::{postgres::PgListener, PgPool};
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore},
    task::JoinSet,
    time::timeout,
};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

/// The same limit as the database check of `HealthStatus`.
const DB_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
/// Channel of the `pads_work_available` trigger.
pub const PAD_WORK_AVAILABLE: &str = "pad_work_available";

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
    /// Interval of the read that finds a lost `pad_work_available` (`PAD_POLL_INTERVAL`).
    pub poll_interval: Duration,
    /// Pending rows older than this are not claimed (`PAD_PENDING_MAX_AGE`).
    pub pending_max_age: Duration,
}

pub struct PadWorker {
    config: Config,
    pool: PgPool,
    evaluator: Arc<dyn PadEvaluator>,
    signer: CoproSigner,
    cancel_token: CancellationToken,
}

impl PadWorker {
    pub fn new(
        config: Config,
        pool: PgPool,
        evaluator: Arc<dyn PadEvaluator>,
        signer: CoproSigner,
        cancel_token: CancellationToken,
    ) -> Self {
        Self {
            config,
            pool,
            evaluator,
            signer,
            cancel_token,
        }
    }

    pub fn health(&self) -> Health {
        Health(self.pool.clone())
    }

    /// Registers the lane, then claims and computes rows until `cancel_token`. Fails on a lost
    /// database connection.
    pub async fn run(self) -> anyhow::Result<()> {
        register_lane(&self.pool, self.config.prf_instance).await?;
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(PAD_WORK_AVAILABLE).await?;
        info!(prf_instance = self.config.prf_instance, "Lane registered");

        let worker = Arc::new(self);
        let slots = Arc::new(Semaphore::new(worker.config.max_concurrent.get()));
        let mut tasks = JoinSet::new();
        loop {
            rows::heartbeat(&worker.pool, worker.config.prf_instance)
                .await
                .or_else(non_fatal)?;
            while let Ok(slot) = slots.clone().try_acquire_owned() {
                let claimed = rows::claim(
                    &worker.pool,
                    worker.config.prf_instance,
                    worker.config.pending_max_age,
                    worker.config.claim_timeout,
                )
                .await;
                match claimed {
                    Ok(Some(claim)) => {
                        tasks.spawn(worker.clone().process(claim, slot));
                    }
                    Ok(None) => break,
                    Err(e) => {
                        non_fatal(e)?;
                        break;
                    }
                }
            }
            tokio::select! {
                () = worker.cancel_token.cancelled() => break,
                notification = listener.recv() => {
                    notification.context("listen for pad_work_available")?;
                }
                () = tokio::time::sleep(worker.config.poll_interval) => {}
                Some(done) = tasks.join_next() => done??,
            }
        }
        // An aborted row keeps its claim: another pod takes it after the timeout.
        tasks.shutdown().await;
        info!("pad-worker stopped");
        Ok(())
    }

    /// Computes, signs and writes one claimed row. Fails only on a lost database connection.
    async fn process(
        self: Arc<Self>,
        claim: Claim,
        _slot: OwnedSemaphorePermit,
    ) -> anyhow::Result<()> {
        if claim.pad_length > MAX_PAD_LENGTH {
            return rows::fail(&self.pool, &claim, "pad_length above MAX_PAD_LENGTH")
                .await
                .or_else(non_fatal);
        }
        let evaluator = self.evaluator.clone();
        let evaluated = tokio::task::spawn_blocking(move || {
            evaluator.evaluate(claim.prf_instance, claim.label, claim.pad_length)
        })
        .await;
        let outputs = match evaluated {
            Ok(Ok(outputs)) => outputs,
            // The evaluation is deterministic, so a new try fails the same way.
            Ok(Err(e)) => {
                warn!(?claim, error = %e, "PRF evaluation failed");
                return rows::fail(&self.pool, &claim, &format!("evaluation: {e}"))
                    .await
                    .or_else(non_fatal);
            }
            Err(e) => {
                warn!(?claim, error = %e, "PRF evaluation panicked");
                return rows::fail(&self.pool, &claim, "evaluation panicked")
                    .await
                    .or_else(non_fatal);
            }
        };
        let payload = PrfOutputAttestationPayload::new(
            AttestationVersion::V1,
            claim.prf_instance,
            claim.label,
            claim.pad_length,
            keccak256(&outputs.ct64),
            keccak256(&outputs.ct128),
        );
        // `sign` takes a sized `Signer`, and `CoproSigner` is a trait object.
        let signature = match self.signer.sign_hash(&payload.canonical_digest()).await {
            Ok(signature) => signature,
            Err(e) => return self.retry(&claim, &format!("signing: {e}")).await,
        };
        let attestation = PrfOutputAttestation {
            version: payload.version,
            ciphertext_digest: payload.ciphertext_digest,
            sns_ciphertext_digest: payload.sns_ciphertext_digest,
            signer: self.signer.address(),
            signature: signature.as_bytes().to_vec(),
        };
        match rows::write_output(&self.pool, &claim, &outputs.ct128, &attestation).await {
            Ok(()) => Ok(()),
            Err(e) if is_fatal_connection_error(&e) => Err(e.into()),
            Err(e) => self.retry(&claim, &format!("write: {e}")).await,
        }
    }

    async fn retry(&self, claim: &Claim, message: &str) -> anyhow::Result<()> {
        warn!(
            ?claim,
            message, "Temporary error, the row is tried again after a backoff"
        );
        rows::retry(&self.pool, claim, message, self.config.claim_timeout)
            .await
            .or_else(non_fatal)
    }
}

/// Logs a database error. Returns a lost connection, so the process stops.
fn non_fatal(e: sqlx::Error) -> anyhow::Result<()> {
    if is_fatal_connection_error(&e) {
        return Err(e.into());
    }
    warn!(error = %e, "database error");
    Ok(())
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
