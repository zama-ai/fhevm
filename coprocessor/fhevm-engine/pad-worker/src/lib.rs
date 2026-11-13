//! The pad-worker: computes the PRF outputs of one PRF instance.

pub mod evaluator;
pub mod metrics;
pub mod rows;
pub mod upload;

use alloy::primitives::keccak256;
use anyhow::Context;
use ciphertext_attestation::{
    prf::Version as AttestationVersion, PrfOutputAttestation, PrfOutputAttestationPayload,
};
use coprocessor_api::MAX_PAD_LENGTH;
use evaluator::PadEvaluator;
use fhevm_engine_common::{
    db_keys::DbKeyId,
    healthz_server::{HealthCheckService, HealthStatus, Version},
    pg_pool::is_fatal_connection_error,
    types::CoproSigner,
};
use rows::{Claim, Output};
use sqlx::{postgres::PgListener, PgPool};
use std::{
    collections::HashSet,
    num::NonZeroUsize,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore},
    task::JoinSet,
    time::{sleep, timeout},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use upload::{s3_key, Uploader};

/// The same limit as the database check of `HealthStatus`.
const DB_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
/// Maximum rows for each resubmit read.
const RESUBMIT_BATCH: i64 = 16;
/// Channel of the `pads_work_available` trigger.
pub const PAD_WORK_AVAILABLE: &str = "pad_work_available";

#[derive(Debug, Clone)]
pub struct Config {
    /// The PRF instance of this pad-worker.
    pub prf_instance: u16,
    /// KMSGeneration key ID of the instance, in 32 big-endian bytes as in `keys.key_id`.
    pub key_id: DbKeyId,
    /// Evaluations at the same time (`PRF_MAX_CONCURRENT`).
    pub max_concurrent: NonZeroUsize,
    /// A claim older than this is taken again (`PAD_CLAIM_TIMEOUT`).
    pub claim_timeout: Duration,
    /// Interval of the read that finds a lost `pad_work_available` (`PAD_POLL_INTERVAL`).
    pub poll_interval: Duration,
    /// Pending rows older than this are not claimed (`PAD_PENDING_MAX_AGE`).
    pub pending_max_age: Duration,
    /// Delete uploaded or failed rows older than this. `None`: keep all (`PRF_OUTPUT_MAX_AGE`).
    pub output_max_age: Option<Duration>,
    /// Interval of the GC and the row age metrics (`PAD_GC_INTERVAL`).
    pub gc_interval: Duration,
    /// The GC clears the ct64 of rows older than this (`PAD_CT64_MAX_AGE`).
    pub ct64_max_age: Duration,
    /// On a stop, the time for the claimed rows to complete.
    pub drain_timeout: Duration,
}

/// Why a claim has no output.
enum OutputError {
    /// The evaluation is deterministic, so a new try fails the same way.
    Evaluation(String),
    /// A new try can pass.
    Signing(String),
}

pub struct PadWorker {
    config: Config,
    pool: PgPool,
    evaluator: Arc<dyn PadEvaluator>,
    signer: CoproSigner,
    uploader: Option<Uploader>,
    cancel_token: CancellationToken,
    /// Set once the pad-worker listens for rows: until then, a rollout keeps the old pod.
    ready: Arc<AtomicBool>,
    /// The claims of this pod whose evaluation runs.
    running: Mutex<HashSet<Claim>>,
}

/// Adds a claim to `running` until its evaluation ends.
struct Running<'a>(&'a Mutex<HashSet<Claim>>, Claim);

impl<'a> Running<'a> {
    fn new(running: &'a Mutex<HashSet<Claim>>, claim: Claim) -> Self {
        running.lock().expect("running claims").insert(claim);
        Self(running, claim)
    }
}

impl Drop for Running<'_> {
    fn drop(&mut self) {
        self.0.lock().expect("running claims").remove(&self.1);
    }
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
            uploader: None,
            cancel_token,
            ready: Arc::default(),
            running: Mutex::default(),
        }
    }

    /// Uploads each output to S3. Without an uploader, no row gets `s3_verified_at`.
    pub fn with_uploader(mut self, uploader: Uploader) -> Self {
        self.uploader = Some(uploader);
        self
    }

    pub fn health(&self) -> Health {
        Health {
            pool: self.pool.clone(),
            ready: self.ready.clone(),
        }
    }

    /// Claims rows until `cancel_token`. Fails on a lost database connection.
    pub async fn run(self) -> anyhow::Result<()> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(PAD_WORK_AVAILABLE).await?;
        info!(prf_instance = self.config.prf_instance, "Pad-worker ready");
        self.ready.store(true, Ordering::Relaxed);

        let worker = Arc::new(self);
        let slots = Arc::new(Semaphore::new(worker.config.max_concurrent.get()));
        let mut tasks = JoinSet::new();
        tasks.spawn(worker.clone().gc());
        if let Some(uploader) = worker.uploader.clone() {
            tasks.spawn(worker.clone().resubmit(uploader));
        }
        loop {
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
        // Let the claimed rows complete. A row still running after the limit is released below.
        let drained = timeout(worker.config.drain_timeout, async {
            while let Some(done) = tasks.join_next().await {
                match done {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => warn!(error = %e, "task failed during the stop"),
                    Err(e) => error!(error = %e, "task panicked during the stop"),
                }
            }
        })
        .await;
        if drained.is_err() {
            warn!("claimed rows did not complete before the stop");
        }
        let running: Vec<Claim> = worker
            .running
            .lock()
            .expect("running claims")
            .iter()
            .copied()
            .collect();
        tasks.shutdown().await;
        // So that the next pod claims them at once, not after `claim_timeout`.
        if !running.is_empty() {
            match rows::release(&worker.pool, worker.config.prf_instance, &running).await {
                Ok(released) => info!(released, "Released the claims of the aborted evaluations"),
                Err(e) => {
                    warn!(error = %e, "Claims not released: they expire after the claim timeout")
                }
            }
        }
        info!("pad-worker stopped");
        Ok(())
    }

    /// Evaluates and signs the output of `claim`. Also returns the ct64.
    async fn compute(&self, claim: Claim) -> Result<(Output, Vec<u8>), OutputError> {
        let evaluator = self.evaluator.clone();
        let outputs = tokio::task::spawn_blocking(move || {
            evaluator.evaluate(claim.prf_instance, claim.label, claim.pad_length)
        })
        .await
        .map_err(|_| OutputError::Evaluation("evaluation panicked".to_string()))?
        .map_err(|e| OutputError::Evaluation(format!("evaluation: {e}")))?;
        let payload = PrfOutputAttestationPayload::new(
            AttestationVersion::V1,
            claim.prf_instance,
            claim.label,
            claim.pad_length,
            keccak256(&outputs.ct64),
            keccak256(&outputs.ct128),
        );
        // `sign` takes a sized `Signer`, and `CoproSigner` is a trait object.
        let signature = self
            .signer
            .sign_hash(&payload.canonical_digest())
            .await
            .map_err(|e| OutputError::Signing(format!("signing: {e}")))?;
        let output = Output {
            claim,
            ct128: outputs.ct128,
            attestation: PrfOutputAttestation {
                version: payload.version,
                ciphertext_digest: payload.ciphertext_digest,
                sns_ciphertext_digest: payload.sns_ciphertext_digest,
                signer: self.signer.address(),
                signature: signature.as_bytes().to_vec(),
            },
        };
        Ok((output, outputs.ct64))
    }

    /// Computes, signs and writes one claimed row. Fails only on a lost database connection.
    async fn process(
        self: Arc<Self>,
        claim: Claim,
        slot: OwnedSemaphorePermit,
    ) -> anyhow::Result<()> {
        if claim.pad_length > MAX_PAD_LENGTH {
            return rows::fail(&self.pool, &claim, "pad_length above MAX_PAD_LENGTH")
                .await
                .or_else(non_fatal);
        }
        metrics::count_evaluation(claim.prf_instance);
        let running = Running::new(&self.running, claim);
        let computed = self.compute(claim).await;
        drop(running);
        let (output, ct64) = match computed {
            Ok(computed) => computed,
            Err(OutputError::Evaluation(message)) => {
                warn!(?claim, message, "PRF evaluation failed");
                return rows::fail(&self.pool, &claim, &message)
                    .await
                    .or_else(non_fatal);
            }
            Err(OutputError::Signing(message)) => return self.retry(&claim, &message).await,
        };
        match rows::write_output(
            &self.pool,
            &claim,
            &ct64,
            &output.ct128,
            &output.attestation,
        )
        .await
        {
            Ok(true) => {}
            // Another pod completed the row, or the GC deleted it: do not upload this output.
            Ok(false) => return Ok(()),
            Err(e) if is_fatal_connection_error(&e) => return Err(e.into()),
            Err(e) => return self.retry(&claim, &format!("write: {e}")).await,
        }
        drop(slot);
        self.upload(output, false).await
    }

    /// Uploads the output and sets `s3_verified_at`. If the upload fails, `resubmit` tries again.
    /// With `wait` false and no free slot, it leaves the output to `resubmit`.
    async fn upload(&self, output: Output, wait: bool) -> anyhow::Result<()> {
        let Some(uploader) = &self.uploader else {
            return Ok(());
        };
        let Some(_slot) = uploader.slot(wait).await else {
            return Ok(());
        };
        if let Err(e) = uploader.upload(&s3_key(&output.claim), &output).await {
            warn!(claim = ?output.claim, error = %e, "S3 upload failed");
            return Ok(());
        }
        rows::set_uploaded(&self.pool, &output.claim)
            .await
            .or_else(non_fatal)
    }

    /// Deletes old and expired rows, and sets the row age metrics.
    async fn gc(self: Arc<Self>) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                () = self.cancel_token.cancelled() => return Ok(()),
                () = sleep(self.config.gc_interval) => {}
            }
            self.gc_pass().await.or_else(non_fatal)?;
        }
    }

    async fn gc_pass(&self) -> sqlx::Result<()> {
        let config = &self.config;
        let old = match config.output_max_age {
            Some(max_age) => rows::delete_old_outputs(&self.pool, max_age).await?,
            None => 0,
        };
        let expired =
            rows::delete_expired_pending(&self.pool, config.pending_max_age, config.claim_timeout)
                .await?;
        let ct64 = rows::clear_old_ct64(&self.pool, config.ct64_max_age).await?;
        if old + expired + ct64 > 0 {
            info!(old, expired, ct64, "Deleted pads rows and ct64");
        }
        metrics::set_oldest_rows(&rows::oldest_rows(&self.pool, config.prf_instance).await?);
        metrics::set_stored_ct64_bytes(rows::stored_ct64_bytes(&self.pool).await?);
        Ok(())
    }

    /// Uploads the complete rows whose upload failed or stopped.
    async fn resubmit(self: Arc<Self>, uploader: Uploader) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                () = self.cancel_token.cancelled() => return Ok(()),
                () = sleep(uploader.resubmit_interval) => {}
            }
            let claimed = rows::claim_uploads(
                &self.pool,
                self.config.prf_instance,
                self.config.claim_timeout,
                RESUBMIT_BATCH,
            )
            .await;
            match claimed {
                Ok(outputs) => {
                    futures_util::future::try_join_all(
                        outputs.into_iter().map(|output| self.upload(output, true)),
                    )
                    .await?;
                }
                Err(e) => non_fatal(e)?,
            }
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

/// Readiness: the listener, the database and the rights on `pads`.
pub struct Health {
    pool: PgPool,
    ready: Arc<AtomicBool>,
}

impl HealthCheckService for Health {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        let ready = self.ready.load(Ordering::Relaxed);
        if !ready {
            status.add_error_details("The pad-worker does not listen for rows".to_string());
        }
        status.set_custom_check("listening", ready, false);
        status.set_db_connected(&self.pool).await;
        let rights = sqlx::query_scalar::<_, bool>(
            "SELECT has_column_privilege('pads', 'signature', 'UPDATE')
                AND has_column_privilege('pads', 'ciphertext64', 'UPDATE')",
        )
        .fetch_one(&self.pool);
        let has_rights = matches!(timeout(DB_CHECK_TIMEOUT, rights).await, Ok(Ok(true)));
        if !has_rights {
            status.add_error_details("No UPDATE rights on pads".to_string());
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
