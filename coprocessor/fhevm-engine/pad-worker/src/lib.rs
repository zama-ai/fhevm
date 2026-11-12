//! The pad-worker: one lane computes the PRF outputs of one PRF instance.

pub mod evaluator;
pub mod metrics;
pub mod rows;
pub mod upload;

use alloy::primitives::keccak256;
use anyhow::{anyhow, bail, Context};
use ciphertext_attestation::{
    prf::Version as AttestationVersion, PrfOutputAttestation, PrfOutputAttestationPayload,
};
use coprocessor_api::MAX_PAD_LENGTH;
use evaluator::PadEvaluator;
use fhevm_engine_common::{
    db_keys::DbKeyId,
    healthz_server::{HealthCheckService, HealthStatus, Version},
    pg_pool::is_fatal_connection_error,
    prf_reference::{reference_digest, reference_outputs},
    types::CoproSigner,
};
use rows::{Claim, Output};
use sqlx::{postgres::PgListener, PgPool};
use std::{
    num::NonZeroUsize,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
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
use upload::{reference_key, s3_key, Uploader};

/// The same limit as the database check of `HealthStatus`.
const DB_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
/// Maximum rows for each resubmit read.
const RESUBMIT_BATCH: i64 = 16;
/// On a stop, the time for the claimed rows to complete. Below the 30 s grace period.
const DRAIN_TIMEOUT: Duration = Duration::from_secs(20);
/// Channel of the `pads_work_available` trigger.
pub const PAD_WORK_AVAILABLE: &str = "pad_work_available";

#[derive(Debug, Clone)]
pub struct Config {
    /// The PRF instance of this lane.
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
    /// Interval of the reference digest read until the lane is ready (`PAD_START_CHECK_INTERVAL`).
    pub start_check_interval: Duration,
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
    /// Set after the start check: until then, a rollout keeps the old pod.
    ready: Arc<AtomicBool>,
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

    /// Registers the lane, waits for the start check, then claims rows until `cancel_token`. Fails
    /// on a lost database connection or on bytes that are not the agreed ones.
    pub async fn run(self) -> anyhow::Result<()> {
        register_lane(&self.pool, self.config.prf_instance).await?;
        info!(prf_instance = self.config.prf_instance, "Lane registered");
        if !self.start_check().await? {
            return Ok(());
        }
        self.ready.store(true, Ordering::Relaxed);
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen(PAD_WORK_AVAILABLE).await?;

        let worker = Arc::new(self);
        let slots = Arc::new(Semaphore::new(worker.config.max_concurrent.get()));
        let mut tasks = JoinSet::new();
        tasks.spawn(worker.clone().gc());
        if let Some(uploader) = worker.uploader.clone() {
            tasks.spawn(worker.clone().resubmit(uploader));
        }
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
        // Let the claimed rows complete. A row still running after the limit keeps its claim.
        let drained = timeout(DRAIN_TIMEOUT, async {
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
        tasks.shutdown().await;
        info!("pad-worker stopped");
        Ok(())
    }

    /// Waits until the operators agree on a reference digest. Returns true when the lane is ready,
    /// false on `cancel_token`. Fails if the agreed digest is another.
    async fn start_check(&self) -> anyhow::Result<bool> {
        let prf_instance = self.config.prf_instance;
        let mut outputs = Vec::new();
        for (label, pad_length) in reference_outputs() {
            let claim = Claim {
                prf_instance,
                label,
                pad_length,
            };
            match self.compute(claim).await {
                Ok((output, _)) => outputs.push(output),
                Err(OutputError::Evaluation(e) | OutputError::Signing(e)) => {
                    return Err(anyhow!("reference output {claim:?}: {e}"));
                }
            }
        }
        let digest = reference_digest(outputs.iter().map(|o| {
            (
                o.attestation.ciphertext_digest,
                o.attestation.sns_ciphertext_digest,
            )
        }));
        let mut uploaded = false;
        loop {
            let agreement = rows::lane_agreement(&self.pool, prf_instance).await?;
            match agreement.reference_digest {
                Some(agreed) if agreed == digest => {
                    rows::set_ready(&self.pool, prf_instance).await?;
                    info!(%digest, "Reference digest agreed, lane ready");
                    return Ok(true);
                }
                Some(agreed) => bail!(
                    "reference digest {digest} is not the agreed {agreed}: this image changes the \
                     bytes of instance {prf_instance}, so it needs a new prfId"
                ),
                // A new instance: the consensus-detector compares the reference objects.
                None => {
                    if !uploaded {
                        uploaded = self.upload_references(&outputs).await;
                    }
                    if agreement.mismatch {
                        error!(%digest, "Operators have other reference digests, lane not ready");
                    } else {
                        info!(%digest, "Waiting for the agreed reference digest");
                    }
                }
            }
            tokio::select! {
                () = self.cancel_token.cancelled() => return Ok(false),
                () = sleep(self.config.start_check_interval) => {}
            }
        }
    }

    /// Returns true if all the reference objects are in S3, or if there is no uploader.
    async fn upload_references(&self, outputs: &[Output]) -> bool {
        let Some(uploader) = &self.uploader else {
            return true;
        };
        for output in outputs {
            let _slot = uploader.slot(true).await;
            if let Err(e) = uploader.upload(&reference_key(&output.claim), output).await {
                warn!(claim = ?output.claim, error = %e, "Reference upload failed");
                return false;
            }
        }
        true
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
        let (output, ct64) = match self.compute(claim).await {
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
        metrics::set_oldest_rows(&rows::oldest_rows(&self.pool).await?);
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

/// Readiness: the start check, the database and the rights on `pads` and `prf_lanes`.
pub struct Health {
    pool: PgPool,
    ready: Arc<AtomicBool>,
}

impl HealthCheckService for Health {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        let ready = self.ready.load(Ordering::Relaxed);
        if !ready {
            status.add_error_details("The lane has not passed its start check".to_string());
        }
        status.set_custom_check("lane_ready", ready, false);
        status.set_db_connected(&self.pool).await;
        let rights = sqlx::query_scalar::<_, bool>(
            "SELECT has_column_privilege('pads', 'signature', 'UPDATE')
                AND has_column_privilege('pads', 'ciphertext64', 'UPDATE')
                AND has_column_privilege('prf_lanes', 'heartbeat_at', 'UPDATE')",
        )
        .fetch_one(&self.pool);
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
