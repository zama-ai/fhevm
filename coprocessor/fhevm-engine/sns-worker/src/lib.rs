mod aws_upload;
mod executor;
mod keyset;
mod squash_noise;

#[cfg(test)]
mod tests;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use aws_config::{retry::RetryConfig, timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::{config::Builder, Client};
use fhevm_engine_common::{
    healthz_server::{self},
    metrics_server,
    pg_pool::{PostgresPoolManager, ServiceError},
    telemetry::{self, OtelTracer},
    types::FhevmError,
    utils::compact_hex,
};
use futures::join;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use thiserror::Error;
use tokio::{
    spawn,
    sync::{
        mpsc::{self, Sender},
        RwLock,
    },
    task,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};

use crate::{
    aws_upload::{check_is_ready, spawn_resubmit_task, spawn_uploader},
    executor::SwitchNSquashService,
};

pub const UPLOAD_QUEUE_SIZE: usize = 20;
pub const SAFE_SER_LIMIT: u64 = 1024 * 1024 * 66;

#[derive(Serialize, Deserialize, Clone)]
pub struct KeySet {
    pub server_key: tfhe::ServerKey,
    pub client_key: Option<tfhe::ClientKey>,
}

#[derive(Clone)]
pub struct DBConfig {
    pub url: String,
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

impl std::fmt::Debug for DBConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Custom debug impl to avoid printing sensitive information
        write!(
            f,
            "db_listen_channel: {:?}, db_notify_channel: {}, db_batch_limit: {}, db_gc_batch_limit: {}, db_polling_interval: {}, db_cleanup_interval: {:?}, db_max_connections: {}, db_timeout: {:?}, lifo: {}",
            self.listen_channels, self.notify_channel, self.batch_limit, self.gc_batch_limit, self.polling_interval, self.cleanup_interval, self.max_connections, self.timeout, self.lifo
        )
    }
}

#[derive(Clone, Default, Debug)]
pub struct S3Config {
    pub bucket_ct128: String,
    pub bucket_ct64: String,
    pub max_concurrent_uploads: u32,
    pub retry_policy: S3RetryPolicy,
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

#[derive(Clone, Debug)]
pub struct Config {
    pub tenant_api_key: String,
    pub service_name: String,
    pub db: DBConfig,
    pub s3: S3Config,
    pub log_level: Level,
    pub health_checks: HealthCheckConfig,
    pub metrics_addr: Option<String>,
    pub enable_compression: bool,
    pub schedule_policy: SchedulePolicy,
    pub pg_auto_explain_with_min_duration: Option<Duration>,
}

#[derive(Debug, Clone, Copy, Default)]
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
            "db_url: {},  db_listen_channel: {:?}, db_notify_channel: {}, db_batch_limit: {}",
            self.db.url, self.db.listen_channels, self.db.notify_channel, self.db.batch_limit
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

#[derive(Clone, Debug, Default)]
pub struct BigCiphertext {
    format: Ciphertext128Format,
    bytes: Vec<u8>,
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
    pub tenant_id: i32,
    pub handle: Vec<u8>,

    /// Compressed 64-bit ciphertext
    ///
    /// Shared between the execute worker and the uploader
    ///
    /// The maximum size can be 8.1 KiB (type FheBytes256)
    pub ct64_compressed: Arc<Vec<u8>>,

    /// The computed 128-bit ciphertext
    pub(crate) ct128: Arc<BigCiphertext>,

    pub otel: OtelTracer,
    pub transaction_id: Option<Vec<u8>>,
}

impl HandleItem {
    /// Enqueues the upload task into the database
    ///
    /// If inserted into the `ciphertext_digest` table means that the both (ct64 and ct128)
    /// ciphertexts are ready to be uploaded to S3.
    pub(crate) async fn enqueue_upload_task(
        &self,
        db_txn: &mut Transaction<'_, Postgres>,
    ) -> Result<(), ExecutionError> {
        sqlx::query!(
            "INSERT INTO ciphertext_digest (tenant_id, handle, transaction_id)
            VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            self.tenant_id,
            self.handle,
            self.transaction_id
        )
        .execute(db_txn.as_mut())
        .await?;

        Ok(())
    }

    pub(crate) async fn update_ct128_uploaded(
        &self,
        trx: &mut Transaction<'_, Postgres>,
        digest: Vec<u8>,
    ) -> Result<(), ExecutionError> {
        let format: i16 = self.ct128.format().into();

        sqlx::query!(
            "UPDATE ciphertext_digest
            SET ciphertext128 = $1, ciphertext128_format = $2
            WHERE handle = $3",
            digest,
            format,
            self.handle,
        )
        .execute(trx.as_mut())
        .await?;

        info!(
            "Mark ct128 as uploaded, handle: {}, digest: {}, format: {:?}",
            compact_hex(&self.handle),
            compact_hex(&digest),
            format,
        );

        Ok(())
    }

    pub(crate) async fn update_ct64_uploaded(
        &self,
        trx: &mut Transaction<'_, Postgres>,
        digest: Vec<u8>,
    ) -> Result<(), ExecutionError> {
        sqlx::query!(
            "UPDATE ciphertext_digest
             SET ciphertext = $1
             WHERE handle = $2",
            digest,
            self.handle
        )
        .execute(trx.as_mut())
        .await?;

        info!(
            "Mark ct64 as uploaded, handle: {}, digest: {}",
            compact_hex(&self.handle),
            compact_hex(&digest)
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

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

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

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Bucket not found {0}")]
    BucketNotFound(String),

    #[error("S3 Transient error: {0}")]
    S3TransientError(String),

    #[error("Internal send error: {0}")]
    InternalSendError(String),
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

/// Runs the SnS worker loop
pub async fn run_computation_loop(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    tx: Sender<UploadJob>,
    token: CancellationToken,
    client: Arc<Client>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = conf.health_checks.port;

    // Start metrics server
    metrics_server::spawn(conf.metrics_addr.clone(), token.child_token());

    let service = Arc::new(
        SwitchNSquashService::create(pool_mngr, conf, tx, token.child_token(), client).await?,
    );

    // Start health check server
    let healthz = healthz_server::HttpServer::new(service.clone(), port, token.child_token());
    task::spawn(async move {
        if let Err(err) = healthz.start().await {
            error!(
                task = "health_check",
                error = %err,
                "Error while running server"
            );
        }
        anyhow::Ok(())
    });

    // Run the main service loop
    service.run(pool_mngr).await;
    token.cancel();

    info!("Worker stopped");
    Ok(())
}

/// Runs the uploader loop
pub async fn run_uploader_loop(
    pool_mngr: &PostgresPoolManager,
    conf: &Config,
    rx: Arc<RwLock<mpsc::Receiver<UploadJob>>>,
    tx: Sender<UploadJob>,
    client: Arc<Client>,
    is_ready: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (is_ready_res, _) = check_is_ready(&client, conf).await;
    is_ready.store(is_ready_res, Ordering::Release);

    let handle_resubmit = spawn_resubmit_task(
        pool_mngr,
        conf.clone(),
        tx.clone(),
        client.clone(),
        is_ready.clone(),
    )
    .await?;

    let handle_uploader = spawn_uploader(pool_mngr, conf.clone(), rx, client, is_ready).await?;
    let _res = join!(handle_resubmit, handle_uploader);

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
    let (is_ready, is_connected) = check_is_ready(&client, conf).await;
    if is_connected {
        info!(is_ready = is_ready, "Connected to S3");
    }

    (client, is_ready)
}

/// Run all SNS worker components.
pub async fn run_all(
    config: Config,
    parent_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Queue of tasks to upload ciphertexts is 10 times the number of concurrent uploads
    // to avoid blocking the worker
    // and to allow for some burst of uploads
    let (uploads_tx, uploads_rx) =
        mpsc::channel::<UploadJob>(10 * config.s3.max_concurrent_uploads as usize);

    let rayon_threads = rayon::current_num_threads();
    info!(config = ?config, rayon_threads, "Starting SNS worker");

    if !config.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&config.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let conf = config.clone();
    let token = parent_token.child_token();
    let tx = uploads_tx.clone();
    // Initialize the S3 uploader
    let (client, is_ready) = create_s3_client(&conf).await;
    let is_ready = Arc::new(AtomicBool::new(is_ready));
    let s3 = client.clone();
    let jobs_rx: Arc<RwLock<mpsc::Receiver<UploadJob>>> = Arc::new(RwLock::new(uploads_rx));

    let Some(pool_mngr) = PostgresPoolManager::connect_pool(
        token.child_token(),
        conf.db.url.as_str(),
        conf.db.timeout,
        conf.db.max_connections,
        Duration::from_secs(2),
        conf.pg_auto_explain_with_min_duration,
    )
    .await
    else {
        error!("Service was cancelled during Postgres pool initialization");
        return Ok(());
    };

    let pg_mngr = pool_mngr.clone();

    // Spawns a task to handle S3 uploads
    spawn(async move {
        if let Err(err) = run_uploader_loop(&pg_mngr, &conf, jobs_rx, tx, s3, is_ready).await {
            error!(error = %err, "Failed to run the upload-worker");
        }
    });

    // Run the main computation loop
    // This will handle the PBS computations
    let conf = config.clone();
    let token = parent_token.child_token();

    if let Err(err) = run_computation_loop(&pool_mngr, conf, uploads_tx, token, client).await {
        error!(error = %err, "SnS worker failed");
    }

    Ok(())
}
