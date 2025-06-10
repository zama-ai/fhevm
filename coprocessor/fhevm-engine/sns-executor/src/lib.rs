mod aws_upload;
mod executor;
mod keyset;
mod squash_noise;

#[cfg(test)]
mod tests;

use std::{sync::Arc, time::Duration};

use fhevm_engine_common::{telemetry::OtelTracer, types::FhevmError, utils::compact_hex};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use thiserror::Error;
use tokio::sync::mpsc::{self, Sender};
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

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

#[derive(Clone, Debug, Default)]
pub struct S3RetryPolicy {
    pub max_retries_per_upload: u32,
    pub max_backoff: Duration,
    pub max_retries_timeout: Duration,
    pub recheck_duration: Duration,
    pub regular_recheck_duration: Duration,
}

#[derive(Clone)]
pub struct Config {
    pub tenant_api_key: String,
    pub service_name: String,
    pub db: DBConfig,
    pub s3: S3Config,
    pub log_level: Level,
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

    /// Compressed 64-bit ciphertext
    ///
    /// Shared between the execute worker and the uploader
    ///
    /// The maximum size can be 8.1 KiB (type FheBytes256)
    pub ct64_compressed: Arc<Vec<u8>>,

    /// Uncompressed 128-bit ciphertext
    ///
    /// Shared between the execute worker and the uploader
    ///
    /// The maximum size can be 32.0 MiB (type FheBytes256)
    pub ct128_uncompressed: Arc<Vec<u8>>,
    pub otel: OtelTracer,
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
            "INSERT INTO ciphertext_digest (tenant_id, handle)
            VALUES ($1, $2) ON CONFLICT DO NOTHING",
            self.tenant_id,
            &self.handle,
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
        sqlx::query!(
            "UPDATE ciphertext_digest
            SET ciphertext128 = $1
            WHERE handle = $2",
            digest,
            self.handle
        )
        .execute(trx.as_mut())
        .await?;

        // Reset ciphertext128 as the ct128 has been successfully uploaded to S3
        // NB: For reclaiming the disk-space in DB, we rely on auto vacuuming in
        // Postgres

        sqlx::query!(
            "UPDATE ciphertexts
             SET ciphertext128 = NULL
             WHERE handle = $1",
            self.handle
        )
        .execute(trx.as_mut())
        .await?;

        info!(
            "Mark ct128 as uploaded, handle: {}, digest: {}",
            compact_hex(&self.handle),
            compact_hex(&digest)
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
    pub fn handle(&self) -> &Vec<u8> {
        match self {
            UploadJob::Normal(item) => &item.handle,
            UploadJob::DatabaseLock(item) => &item.handle,
        }
    }
}

/// Runs the SnS worker loop
pub async fn compute_128bit_ct(
    conf: &Config,
    tx: Sender<UploadJob>,
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
    rx: mpsc::Receiver<UploadJob>,
    tx: Sender<UploadJob>,
    token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "sns", "Uploader started with {:?}", conf.s3);

    aws_upload::process_s3_uploads(conf, rx, tx, token).await?;

    info!(target: "sns", "Uploader stopped");
    Ok(())
}
