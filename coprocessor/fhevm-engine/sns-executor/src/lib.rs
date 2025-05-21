mod aws_upload;
mod executor;
mod keyset;
mod squash_noise;

#[cfg(test)]
mod tests;

use std::time::Duration;

use fhevm_engine_common::{telemetry::OtelTracer, types::FhevmError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc::{self, Sender};
use tokio_util::sync::CancellationToken;
use tracing::info;

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
    pub polling_interval: u32,
    pub max_connections: u32,
}

#[derive(Clone, Default, Debug)]
pub struct S3Config {
    pub bucket_ct128: String,
    pub bucket_ct64: String,
    pub max_concurrent_uploads: u32,
    pub retry_policy: S3RetryPolicy,
}

#[derive(Clone, Debug)]
pub struct S3RetryPolicy {
    pub max_retries_per_upload: u32,
    pub max_backoff: Duration,
    pub max_retries_timeout: Duration,
    pub recheck_duration: Duration,
    pub regular_recheck_duration: Duration,
}

impl Default for S3RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries_per_upload: 60,
            max_backoff: Duration::from_secs(10),
            max_retries_timeout: Duration::from_secs(2 * 60),
            recheck_duration: Duration::from_secs(1),
            regular_recheck_duration: Duration::from_secs(60),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub tenant_api_key: String,
    pub service_name: String,
    pub db: DBConfig,
    pub s3: S3Config,
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

#[derive(Clone)]
pub struct HandleItem {
    pub tenant_id: i32,
    pub handle: Vec<u8>,
    pub ct64_compressed: Option<Vec<u8>>,
    pub ct128_uncompressed: Option<Vec<u8>>,
    pub otel: OtelTracer,
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

    #[error("Bucket S3 upload: {0}")]
    BucketNotExist(String),

    #[error("S3 Transient error: {0}")]
    S3TransientError(String),
}

/// Runs the SnS worker loop
pub async fn compute_128bit_ct(
    conf: &Config,
    tx: Sender<HandleItem>,
    token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "sns", "Worker started with {}", conf);

    executor::run_loop(conf, &tx, token).await?;

    info!(target: "sns", "Worker stopped");
    Ok(())
}

/// Runs the uploader loop
pub async fn process_s3_uploads(
    conf: &Config,
    rx: mpsc::Receiver<HandleItem>,
    tx: Sender<HandleItem>,
    token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "sns", "Uploader started with {:?}", conf.s3);

    aws_upload::process_s3_uploads(conf, rx, tx, token).await?;

    info!(target: "sns", "Uploader stopped");
    Ok(())
}
