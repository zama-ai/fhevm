mod aws_upload;
mod executor;
mod keyset;
mod s3_migration;
mod s3_migration_dry_run;
mod squash_noise;

pub mod metrics;
pub use crate::s3_migration::{S3MigrationMode, DEFAULT_S3_MIGRATION_MAX_RETRIES};

#[cfg(test)]
mod tests;

use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicBool, AtomicI64, Ordering},
        Arc,
    },
    time::Duration,
};

use alloy::signers::{
    aws::{aws_sdk_kms, AwsSigner},
    local::PrivateKeySigner,
};
use anyhow::Context;
use aws_config::{retry::RetryConfig, timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::{config::Builder, Client};
use fhevm_engine_common::{
    chain_id::ChainId,
    db_keys::DbKeyId,
    drift_revert,
    gcs_activation::{run_gcs_activation_watcher, GCS_NOT_ACTIVATED},
    healthz_server::{self},
    metrics_server,
    pg_pool::{PostgresPoolManager, ServiceError},
    types::{CoproSigner, FhevmError, SignerType},
    utils::{to_hex, DatabaseURL},
    versioning::{run_stack_version_listener, StackMode},
};
use sqlx::{Postgres, Transaction};
use thiserror::Error;
use tokio::{
    spawn,
    sync::{
        mpsc::{self, Sender},
        RwLock,
    },
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn, Level};

use crate::{
    aws_upload::{check_is_ready, spawn_resubmit_task, spawn_uploader},
    executor::SwitchNSquashService,
    metrics::spawn_gauge_update_routine,
    s3_migration::{run_startup_migrations, S3MigrationConfig},
    s3_migration_dry_run::run_startup_migration_dry_run,
};

pub const UPLOAD_QUEUE_SIZE: usize = 20;
pub const SAFE_SER_LIMIT: u64 = 1024 * 1024 * 66;

pub(crate) const CLEAN_OLD_S3_FORMAT_VERSION: i16 = 0;
pub(crate) const S3_FORMAT_VERSION_V0: i16 = 0;
pub(crate) const S3_FORMAT_VERSION_V1: i16 = 1;
pub(crate) const CURRENT_S3_FORMAT_VERSION: i16 = S3_FORMAT_VERSION_V1;
pub(crate) const S3_FORMAT_VERSION_LEGACY: i16 = S3_FORMAT_VERSION_V0;
pub type InternalEvents = Option<tokio::sync::mpsc::Sender<&'static str>>;

#[cfg(feature = "gpu")]
type ServerKey = tfhe::CudaServerKey;
#[cfg(not(feature = "gpu"))]
type ServerKey = tfhe::ServerKey;

#[derive(Clone)]
pub struct KeySet {
    pub key_id_gw: DbKeyId,
    pub sequence_number: i64,
    /// Optional ClientKey for decrypting on testing
    pub client_key: Option<tfhe::ClientKey>,
    pub server_key: ServerKey,
}

#[derive(Clone)]
pub struct DBConfig {
    pub url: DatabaseURL,
    pub listen_channels: Vec<String>,
    pub notify_channel: String,
    pub batch_limit: u32,
    pub gc_batch_limit: u32,
    pub polling_interval: u32,
    pub cleanup_interval: Duration,
    pub max_connections: u32,
    pub timeout: Duration,

    /// Enable LIFO (Last In, First Out) for processing tasks
    /// This is useful for prioritizing the most recent tasks
    pub lifo: bool,
}

#[derive(Clone, Default, Debug)]
pub struct SNSMetricsConfig {
    pub addr: Option<String>,
    pub gauge_update_interval_secs: Option<u32>,
}

#[derive(Clone, Default, Debug)]
pub struct S3Config {
    pub bucket_ct128: String,
    pub bucket_ct64: String,
    pub max_concurrent_uploads: u32,
    pub retry_policy: S3RetryPolicy,
    pub verify_sha256_checksum: bool,
}

#[derive(Clone, Debug, Default)]
pub struct S3RetryPolicy {
    pub max_retries_per_upload: u32,
    pub max_backoff: Duration,
    pub max_retries_timeout: Duration,
    pub recheck_duration: Duration,
    pub regular_recheck_duration: Duration,
}

#[derive(Clone, Debug)]
pub struct HealthCheckConfig {
    pub liveness_threshold: Duration,
    pub port: u16,
}

#[derive(Clone)]
pub struct Config {
    pub service_name: String,
    pub db: DBConfig,
    pub s3: S3Config,
    pub log_level: Level,
    pub health_checks: HealthCheckConfig,
    pub metrics: SNSMetricsConfig,
    pub enable_compression: bool,
    pub schedule_policy: SchedulePolicy,
    pub pg_auto_explain_with_min_duration: Option<Duration>,
    /// When true, the sns-worker runs in GCS mode. It connects with
    /// `search_path = gcs,public` so writes (`ciphertexts128`,
    /// `pbs_computations`) land in the `gcs` schema, and it pauses until
    /// `event_upgrade_activated` is received before processing any work.
    pub gcs_mode: bool,
    pub signer_type: SignerType,
    pub private_key: Option<String>,
    pub s3_migration: S3MigrationMode,
    pub s3_migration_sleep_duration: Duration,
    pub s3_migration_max_retries: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SchedulePolicy {
    Sequential,
    #[default]
    RayonParallel,
}

impl From<String> for SchedulePolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "sequential" => SchedulePolicy::Sequential,
            "rayon_parallel" => SchedulePolicy::RayonParallel,
            _ => SchedulePolicy::default(),
        }
    }
}

/// Implement Display for Config
impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "db_url: {},  db_listen_channel: {:?}, db_notify_channel: {}, db_batch_limit: {}, gcs_mode: {}",
            self.db.url,
            self.db.listen_channels,
            self.db.notify_channel,
            self.db.batch_limit,
            self.gcs_mode
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i16)]
pub enum Ciphertext128Format {
    #[default]
    Unknown = 0,
    UncompressedOnCpu = 10,
    CompressedOnCpu = 11,
    UncompressedOnGpu = 20,
    CompressedOnGpu = 21,
}

impl Ciphertext128Format {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            10 => Some(Self::UncompressedOnCpu),
            11 => Some(Self::CompressedOnCpu),
            20 => Some(Self::UncompressedOnGpu),
            21 => Some(Self::CompressedOnGpu),
            _ => None,
        }
    }
}

impl From<Ciphertext128Format> for i16 {
    fn from(format: Ciphertext128Format) -> Self {
        format as i16
    }
}

#[derive(Clone, Default)]
pub struct BigCiphertext {
    format: Ciphertext128Format,
    bytes: Vec<u8>,
}

impl std::fmt::Debug for BigCiphertext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BigCiphertext")
            .field("format", &self.format)
            .field("bytes_len", &self.bytes.len())
            .finish()
    }
}

impl BigCiphertext {
    pub fn new_with_format_id(bytes: Vec<u8>, format_id: i16) -> Option<Self> {
        let format = Ciphertext128Format::from_i16(format_id)?;
        Some(Self { format, bytes })
    }

    pub fn new(bytes: Vec<u8>, format: Ciphertext128Format) -> Self {
        Self { format, bytes }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn format(&self) -> Ciphertext128Format {
        self.format
    }
}

impl std::fmt::Display for Ciphertext128Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ciphertext128Format::Unknown => write!(f, "unknown"),
            Ciphertext128Format::UncompressedOnCpu => write!(f, "uncompressed_on_cpu"),
            Ciphertext128Format::CompressedOnCpu => write!(f, "compressed_on_cpu"),
            Ciphertext128Format::UncompressedOnGpu => write!(f, "uncompressed_on_gpu"),
            Ciphertext128Format::CompressedOnGpu => write!(f, "compressed_on_gpu"),
        }
    }
}

#[derive(Clone)]
pub struct HandleItem {
    pub host_chain_id: ChainId,
    pub key_id_gw: DbKeyId,
    pub handle: Vec<u8>,

    /// Compressed 64-bit ciphertext
    ///
    /// Shared between the execute worker and the uploader
    ///
    /// The maximum size can be 8.1 KiB (type FheBytes256)
    pub ct64_compressed: Arc<Vec<u8>>,

    /// The computed 128-bit ciphertext
    pub(crate) ct128: Arc<BigCiphertext>,

    pub(crate) ct64_digest: Option<Vec<u8>>,
    pub(crate) ct128_digest: Option<Vec<u8>>,
    pub(crate) s3_format_version: Option<i16>,

    pub span: tracing::Span,
    pub transaction_id: Option<Vec<u8>>,
}

impl std::fmt::Debug for HandleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let handle = to_hex(&self.handle);
        let key_id_gw = to_hex(&self.key_id_gw);
        let ct64_digest = self.ct64_digest.as_deref().map(to_hex);
        let ct128_digest = self.ct128_digest.as_deref().map(to_hex);
        let transaction_id = self.transaction_id.as_deref().map(to_hex);

        f.debug_struct("HandleItem")
            .field("host_chain_id", &self.host_chain_id.as_i64())
            .field("key_id_gw", &key_id_gw)
            .field("handle", &handle)
            .field("ct64_compressed_len", &self.ct64_compressed.len())
            .field("ct128", &self.ct128) // only superficial debug print
            .field("ct64_digest", &ct64_digest)
            .field("ct128_digest", &ct128_digest)
            .field("s3_format_version", &self.s3_format_version)
            .field("transaction_id", &transaction_id)
            .finish()
    }
}

impl HandleItem {
    /// Enqueues the upload task into the database
    ///
    /// If inserted into the `ciphertext_digest` table means that the both (ct64 and ct128)
    /// ciphertexts are ready to be uploaded to S3.
    ///
    /// Returns `false` (and inserts nothing) when the handle's provenance is
    /// gone: reorg cleanup deletes the `pbs_computations` row of handles that
    /// lived solely on an orphaned fork, and bridge retraction deletes the
    /// copied `ciphertexts` row of a retracted bridged handle — both also
    /// delete the digest row, so an unguarded insert here would resurrect it
    /// and drive a phantom `addCiphertextMaterial` publication. Both witness
    /// rows are locked FOR KEY SHARE, and both deleters remove their witness
    /// BEFORE the digest row, so whichever transaction commits first, the
    /// other observes it: the deleter's digest DELETE (a later statement)
    /// removes a just-committed insert, and a later witness read sees the
    /// deletion and skips. The mirror triggers' advisory stripe locks make a
    /// deadlock between this transaction and a concurrent cleanup possible
    /// instead of a clean block; that is safe — the victim rolls back whole
    /// (an aborted finalization pass re-runs from scratch, an aborted sns
    /// batch is re-fetched) and the retry converges. What can never happen
    /// is a silent resurrection.
    pub(crate) async fn enqueue_upload_task(
        &self,
        db_txn: &mut Transaction<'_, Postgres>,
    ) -> Result<bool, ExecutionError> {
        let provenance_alive = sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM pbs_computations
             WHERE handle = $1 AND host_chain_id = $2
             FOR KEY SHARE",
        )
        .bind(&self.handle)
        .bind(self.host_chain_id.as_i64())
        .fetch_optional(db_txn.as_mut())
        .await?
        .is_some();

        let ciphertext_alive = sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM ciphertexts
             WHERE handle = $1
             LIMIT 1
             FOR KEY SHARE",
        )
        .bind(&self.handle)
        .fetch_optional(db_txn.as_mut())
        .await?
        .is_some();

        if !provenance_alive || !ciphertext_alive {
            warn!(
                handle = %to_hex(&self.handle),
                host_chain_id = self.host_chain_id.as_i64(),
                provenance_alive,
                ciphertext_alive,
                "Skipping upload enqueue: provenance gone (reorg cleanup or bridge retraction)"
            );
            return Ok(false);
        }

        if self.ct128.is_empty() {
            sqlx::query(
                "INSERT INTO ciphertext_digest
                    (host_chain_id, key_id_gw, handle, transaction_id)
                VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            )
            .bind(self.host_chain_id.as_i64())
            .bind(&self.key_id_gw)
            .bind(&self.handle)
            .bind(&self.transaction_id)
            .execute(db_txn.as_mut())
            .await?;
        } else if self.ct128.format() == Ciphertext128Format::Unknown {
            return Err(ExecutionError::InvalidCiphertext128Format(format!(
                "non-empty ct128 has unknown format, host_chain_id: {}, handle: {}",
                self.host_chain_id.as_i64(),
                to_hex(&self.handle),
            )));
        } else {
            let ct128_format: i16 = self.ct128.format().into();
            sqlx::query(
                "INSERT INTO ciphertext_digest (
                    host_chain_id, key_id_gw, handle, transaction_id, ciphertext128_format
                )
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (handle) DO UPDATE
                SET ciphertext128_format = EXCLUDED.ciphertext128_format
                WHERE ciphertext_digest.ciphertext128 IS NULL",
            )
            .bind(self.host_chain_id.as_i64())
            .bind(&self.key_id_gw)
            .bind(&self.handle)
            .bind(&self.transaction_id)
            .bind(ct128_format)
            .execute(db_txn.as_mut())
            .await?;
        }

        Ok(true)
    }

    pub(crate) async fn mark_ciphertexts_uploaded(
        &self,
        trx: &mut Transaction<'_, Postgres>,
        ct64_digest: Vec<u8>,
        ct128_digest: Vec<u8>,
        s3_format_version: i16,
    ) -> Result<(), ExecutionError> {
        let format: i16 = self.ct128.format().into();

        let result = sqlx::query!(
            "UPDATE ciphertext_digest
            SET ciphertext = $1,
                ciphertext128 = $2,
                ciphertext128_format = $3,
                s3_format_version = $4
            WHERE handle = $5",
            &ct64_digest,
            &ct128_digest,
            format,
            s3_format_version,
            &self.handle,
        )
        .execute(trx.as_mut())
        .await?;

        if result.rows_affected() == 0 {
            // The digest row was deleted by reorg cleanup while the upload
            // was in flight: the handle lived solely on an orphaned fork and
            // its publication is cancelled. The uploaded S3 objects are
            // unreferenced garbage, not a correctness problem.
            warn!(
                handle = %to_hex(&self.handle),
                "ciphertext_digest row gone (reorg cleanup); publication cancelled"
            );
            return Ok(());
        }
        if result.rows_affected() != 1 {
            return Err(ExecutionError::InternalError(format!(
                "expected to mark exactly one ciphertext_digest row as uploaded for handle {}, updated {}",
                to_hex(&self.handle),
                result.rows_affected(),
            )));
        }

        info!(
            "Mark ciphertexts as uploaded, handle: {}, ct64_digest: {}, ct128_digest: {}, format: {:?}, s3_format_version: {}",
            to_hex(&self.handle),
            to_hex(&ct64_digest),
            to_hex(&ct128_digest),
            self.ct128.format(),
            s3_format_version,
        );

        Ok(())
    }
}

impl From<ExecutionError> for ServiceError {
    fn from(err: ExecutionError) -> Self {
        match err {
            ExecutionError::DbError(e) => ServiceError::Database(e),

            // collapse everything else into InternalError
            other => ServiceError::InternalError(other.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Conversion error: {0}")]
    ConversionError(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("CtType error: {0}")]
    CtType(#[from] FhevmError),

    #[error("Missing 128-bit ciphertext: {0}")]
    MissingCiphertext128(String),

    #[error("Missing 64-bit ciphertext: {0}")]
    MissingCiphertext64(String),

    #[error("Recv error")]
    RecvFailure,

    #[error("Failed S3 upload: {0}")]
    FailedUpload(String),

    #[error("Upload timeout")]
    UploadTimeout,

    #[error("Squashed noise error: {0}")]
    SquashedNoiseError(#[from] tfhe::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Invalid ciphertext128 format: {0}")]
    InvalidCiphertext128Format(String),

    #[error("Bucket not found {0}")]
    BucketNotFound(String),

    #[error("S3 Transient error: {0}")]
    S3TransientError(String),

    #[error("Internal send error: {0}")]
    InternalSendError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Clone)]
pub enum UploadJob {
    /// Represents a standard upload that is dispatched immediately
    /// after a successful squash_noise computation
    Normal(HandleItem),

    /// Represents a job that requires acquiring a database lock
    /// before initiating the upload process.
    DatabaseLock(HandleItem),
}

impl UploadJob {
    pub fn handle(&self) -> &[u8] {
        match self {
            UploadJob::Normal(item) => &item.handle,
            UploadJob::DatabaseLock(item) => &item.handle,
        }
    }
}

/// Runs the uploader loop
#[allow(clippy::too_many_arguments)]
pub async fn run_uploader_loop(
    pool_mngr: &PostgresPoolManager,
    conf: &Config,
    rx: Arc<RwLock<mpsc::Receiver<UploadJob>>>,
    tx: Sender<UploadJob>,
    client: Arc<Client>,
    is_ready: Arc<AtomicBool>,
    mode: Arc<StackMode>,
    token: CancellationToken,
    signer: CoproSigner,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Park until uploads are enabled. A blue (live) worker has `gcs_mode == false`
    // and proceeds immediately; a green worker parks for the whole dry-run
    // window and only begins uploading after cutover flips the mode (on
    // `event_stack_version_upgraded`), at which point it drains the accumulated
    // `ciphertext_digest` backlog via the resubmit loop.
    while mode.gcs_mode() {
        if token.is_cancelled() {
            info!("Uploader cancelled before activation");
            return Ok(());
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    info!("uploads enabled — starting S3 uploader and resubmit loops");

    let (is_ready_res, _) = check_is_ready(&client, &conf.s3).await;
    is_ready.store(is_ready_res, Ordering::Release);

    let mut handle_resubmit = spawn_resubmit_task(
        pool_mngr,
        conf.clone(),
        tx.clone(),
        client.clone(),
        is_ready.clone(),
    )
    .await?;

    let mut handle_uploader =
        spawn_uploader(pool_mngr, conf.clone(), rx, client, is_ready, signer).await?;

    // Return when either task ends; abort the other and propagate its result.
    let res = tokio::select! {
        r = &mut handle_resubmit => r,
        r = &mut handle_uploader => r,
    };
    handle_resubmit.abort();
    handle_uploader.abort();
    res??;

    info!("Uploader stopped");
    Ok(())
}

/// Configure and create the S3 client.
///
/// Logs errors if the connection fails or if any buckets are missing.
/// Even in the event of a failure or missing buckets, the function returns a valid
/// S3 client capable of retrying S3 operations later.
pub async fn create_s3_client(conf: &Config) -> (Arc<aws_sdk_s3::Client>, bool) {
    let s3config = &conf.s3;

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let timeout_config = TimeoutConfig::builder()
        .connect_timeout(Duration::from_secs(10))
        .operation_attempt_timeout(s3config.retry_policy.max_retries_timeout)
        .build();

    let retry_config = RetryConfig::standard()
        .with_max_attempts(s3config.retry_policy.max_retries_per_upload)
        .with_max_backoff(s3config.retry_policy.max_backoff);

    let config = Builder::from(&sdk_config)
        .timeout_config(timeout_config)
        .retry_config(retry_config)
        .build();

    let client = Arc::new(Client::from_conf(config));
    let (is_ready, is_connected) = check_is_ready(&client, &conf.s3).await;
    if is_connected {
        info!(is_ready = is_ready, "Connected to S3");
    }

    (client, is_ready)
}

/// Run all SNS worker components.
pub async fn run_all(
    config: Config,
    parent_token: CancellationToken,
    events_tx: InternalEvents,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Queue of tasks to upload ciphertexts is 10 times the number of concurrent uploads
    // to avoid blocking the worker
    // and to allow for some burst of uploads
    let (uploads_tx, uploads_rx) =
        mpsc::channel::<UploadJob>(10 * config.s3.max_concurrent_uploads as usize);

    let rayon_threads = rayon::current_num_threads();
    let gpu_enabled = fhevm_engine_common::utils::log_backend();
    info!(gpu_enabled, rayon_threads, config = %config, "Starting SNS worker");

    let conf = config.clone();
    let token = parent_token.child_token();
    let tx = uploads_tx.clone();
    // Initialize the S3 uploader
    let (client, is_ready_bool) = create_s3_client(&conf).await;
    let is_ready = Arc::new(AtomicBool::new(is_ready_bool));
    let s3 = client.clone();
    let jobs_rx: Arc<RwLock<mpsc::Receiver<UploadJob>>> = Arc::new(RwLock::new(uploads_rx));

    // In --gcs-mode the pool is pinned to `search_path = gcs,public` so
    // unqualified writes (`ciphertexts128`, `pbs_computations`) land in the
    // `gcs` schema; shared read-only tables (keys, crs, host_chains,
    // upgrade_state, …) still resolve from `public` via fallback.
    let Some(pool_mngr) = PostgresPoolManager::connect_pool_with_gcs_mode(
        token.child_token(),
        conf.db.url.as_str(),
        conf.db.timeout,
        conf.db.max_connections,
        Duration::from_secs(2),
        conf.pg_auto_explain_with_min_duration,
        conf.gcs_mode,
    )
    .await
    else {
        error!("Service was cancelled during Postgres pool initialization");
        return Ok(());
    };

    let pg_mngr = pool_mngr.clone();

    // Shared blue-green stack mode, seeded from the startup-resolved gcs_mode.
    // The version-upgrade listener flips it out of GCS mode when the cutover
    // commits (on `event_stack_version_upgraded`); that transition is what
    // re-enables S3 uploads + GC for the now-live green worker. A blue (live)
    // worker starts with `gcs_mode == false`, so uploads run immediately.
    let stack_mode = StackMode::new(conf.gcs_mode);
    {
        let listener_pool = pool_mngr.pool();
        let listener_mode = stack_mode.clone();
        let listener_token = token.child_token();
        spawn(async move {
            if let Err(err) =
                run_stack_version_listener(listener_pool, listener_mode, listener_token).await
            {
                error!(error = %err, "stack-version listener exited with error");
            }
        });
    }

    // GCS gating: spawn the activation watcher and pause until it observes
    // `upgrade_state.start_block` for stack_role='GCS'. In BCS mode this is
    // a no-op — the loop falls through immediately.
    let start_block_state = Arc::new(AtomicI64::new(GCS_NOT_ACTIVATED));
    if conf.gcs_mode {
        let watcher_pool = pool_mngr.pool();
        let watcher_state = start_block_state.clone();
        spawn(async move {
            loop {
                if let Err(err) = run_gcs_activation_watcher(&watcher_pool, &watcher_state).await {
                    error!(error = %err, "GCS activation watcher errored; restarting in 5s");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        info!("sns-worker in --gcs-mode (paused, not yet activated). Waiting for event_upgrade_activated.");
        loop {
            if token.is_cancelled() {
                return Ok(());
            }
            if start_block_state.load(Ordering::SeqCst) != GCS_NOT_ACTIVATED {
                break;
            }
            tokio::select! {
                _ = token.cancelled() => return Ok(()),
                _ = tokio::time::sleep(Duration::from_secs(5)) => {}
            }
        }
        info!("sns-worker observed GCS activation; resuming.");
    }

    // Start metrics server
    metrics_server::spawn(conf.metrics.addr.clone(), token.child_token());

    // Start gauge update routine.
    if let Some(interval_secs) = conf.metrics.gauge_update_interval_secs {
        info!(
            interval_secs = interval_secs,
            "Starting gauge update routine"
        );
        spawn_gauge_update_routine(Duration::from_secs(interval_secs.into()), pg_mngr.pool());
    }
    let signer: CoproSigner = match conf.signer_type {
        SignerType::PrivateKey => {
            let Some(private_key) = conf.private_key.clone() else {
                error!("Private key is required for PrivateKey signer");
                return Err(
                    anyhow::anyhow!("Private key is required for PrivateKey signer").into(),
                );
            };
            let signer = PrivateKeySigner::from_str(private_key.trim())?;
            Arc::new(signer)
        }
        SignerType::AwsKms => {
            let key_id = std::env::var("AWS_KEY_ID")
                .context("AWS_KEY_ID environment variable is required for AwsKms signer")?;
            let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
            let aws_kms_client = aws_sdk_kms::Client::new(&aws_conf);
            Arc::new(AwsSigner::new(aws_kms_client, key_id, None).await?)
        }
    };

    // Build the service.
    // create() is a pure struct constructor — no DB or
    // S3 calls — so it's safe to run before drift_revert::init.
    let service = Arc::new(
        SwitchNSquashService::create(
            &pool_mngr,
            conf.clone(),
            uploads_tx,
            token.child_token(),
            client.clone(),
            events_tx.clone(),
            stack_mode.clone(),
        )
        .await?,
    );

    // Start health check BEFORE drift_revert::init so the orchestrator sees us as alive.
    let healthz = healthz_server::HttpServer::new(
        service.clone(),
        conf.health_checks.port,
        token.child_token(),
    );
    spawn(async move {
        if let Err(err) = healthz.start().await {
            error!(
                task = "health_check",
                error = %err,
                "Error while running server"
            );
        }
        anyhow::Ok(())
    });

    // Drift-revert: must run before any DB-using task so the uploader and
    // service loop don't read or write rows the revert SQL is about to delete.
    drift_revert::init(
        pool_mngr.pool().clone(),
        token.clone(),
        None,
        drift_revert::WatcherTimeouts::default(),
    )
    .await?;

    let migration_config = S3MigrationConfig {
        batch_size: 16,
        signer: signer.clone(),
        s3: conf.s3.clone(),
        mode: conf.s3_migration,
        sleep_duration: conf.s3_migration_sleep_duration,
        max_retries: conf.s3_migration_max_retries,
    };

    let not_ready_error = Err(ExecutionError::BucketNotFound(conf.s3.bucket_ct128.clone()).into());
    let mut concurrent_migration = None;
    match migration_config.mode {
        S3MigrationMode::No => {
            info!("S3 migration is disabled");
        }
        S3MigrationMode::Before | S3MigrationMode::BeforeAndQuit | S3MigrationMode::DryRun => {
            info!("S3 migration is enabled: {}", conf.s3_migration);
            if !is_ready_bool {
                error!("S3 is not ready, migration cannot be done");
                return not_ready_error;
            };
            let db_pool = pool_mngr.pool();
            if matches!(migration_config.mode, S3MigrationMode::DryRun) {
                run_startup_migration_dry_run(&migration_config, &db_pool, &client).await?;
            } else {
                run_startup_migrations(&migration_config, &token, &db_pool, &client).await?;
            }
        }
        S3MigrationMode::Concurrent => {
            let token = token.clone();
            let db_pool = pool_mngr.pool();
            let client = client.clone();
            let task = spawn(async move {
                info!("S3 migration is enabled: {}", conf.s3_migration);
                if !is_ready_bool {
                    error!("S3 is not ready but will start when ready");
                };
                if let Err(err) =
                    run_startup_migrations(&migration_config, &token, &db_pool, &client).await
                {
                    error!(
                        error = %err,
                        "Failed to run concurrent S3 format migration"
                    );
                }
            });
            concurrent_migration = Some(task)
        }
    }

    if matches!(
        conf.s3_migration,
        S3MigrationMode::BeforeAndQuit | S3MigrationMode::DryRun
    ) {
        info!("SNS worker stopped after S3 migration-only run");
        token.cancel();
        return Ok(());
    }

    // Spawns a task to handle S3 uploads. In GCS mode the loop parks until the
    // cutover flips `stack_mode` out of GCS mode, so nothing is uploaded during
    // the dry-run window. Keep the uploader's handle so its failure exits the
    // process too.
    let uploader_mode = stack_mode.clone();
    let uploader_token = token.child_token();
    let uploader = spawn(async move {
        run_uploader_loop(
            &pg_mngr,
            &conf,
            jobs_rx,
            tx,
            s3,
            is_ready,
            uploader_mode,
            uploader_token,
            signer,
        )
        .await
    });

    // Exit if either the service loop or the uploader fails. Propagate the
    // uploader's own error (`Ok(inner)`), not just a JoinError — otherwise a
    // persistent upload failure would return `Ok(())` here and the process
    // would exit 0, skipping the fatal log, telemetry::flush(), and exit(1)
    // that alerting keys on.
    let result: Result<(), Box<dyn std::error::Error + Send + Sync>> = tokio::select! {
        res = service.run(&pool_mngr) => res.map_err(Into::into),
        res = uploader => match res {
            Ok(inner) => inner,
            Err(join_err) => Err(join_err.into()),
        },
    };
    token.cancel();

    if let Err(err) = result {
        error!(error = %err, "SNS worker exited with a fatal error");
        return Err(err);
    }

    if let Some(migration_task) = concurrent_migration {
        if let Err(join_err) = migration_task.await {
            error!(error = %join_err, "SNS worker, S3 migration exited with a fatal error");
            return Err(join_err.into());
        }
    }

    info!("Worker stopped");
    Ok(())
}
