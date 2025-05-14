use crate::{Config, ExecutionError, HandleItem, S3Config};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use bytesize::ByteSize;
use fhevm_engine_common::telemetry::{self};
use fhevm_engine_common::utils::compact_hex;
use sha3::{Digest, Keccak256};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use tokio::{join, select};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

// TODO: Use a config TOML to set these values
pub const EVENT_CIPHERTEXTS_UPLOADED: &str = "event_ciphertexts_uploaded";
pub const UPLOAD_TIMEOUT_DURATION: Duration = Duration::from_secs(10);

/// Process the S3 uploads
pub(crate) async fn process_s3_uploads(
    conf: &Config,
    mut tasks: mpsc::Receiver<HandleItem>,
    token: CancellationToken,
) -> Result<(), ExecutionError> {
    // Client construction is expensive due to connection thread pool initialization, and should
    // be done once at application start-up.
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Arc::new(aws_sdk_s3::Client::new(&sdk_config));

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(conf.db.max_connections)
            .connect(&conf.db.url)
            .await?,
    );

    let conf = &conf.s3;
    let max_concurrent_uploads = conf.max_concurrent_uploads as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_uploads));
    let mut upload_jobs: Vec<JoinHandle<()>> = Vec::new();

    loop {
        select! {
            task = tasks.recv() => {
                let task = match task {
                    Some(task) => task,
                    None => return Ok(()),
                };

                // Cleanup completed tasks
                upload_jobs.retain(|h| !h.is_finished());

                // Check if we have reached the max concurrent uploads
                if upload_jobs.len() >= max_concurrent_uploads {
                    warn!({target = "worker", action = "review"},
                        "Max concurrent uploads reached: {}, waiting for a slot ...",
                        max_concurrent_uploads
                    );
                } else {
                    debug!(
                        "Available upload slots: {}",
                        max_concurrent_uploads - upload_jobs.len(),
                    );
                }

                // Acquire a permit for an upload
                let permit = semaphore.clone().acquire_owned().await.expect("Failed to acquire semaphore permit");
                let client = client.clone();
                let pool = pool.clone();
                let conf = conf.clone();

                // Spawn a new task to upload the ciphertexts
                let h = tokio::spawn(async move {
                        if let Err(err) = upload_ciphertexts(task, &client, &pool, &conf).await {
                            error!("Failed to upload ciphertexts: {}", err);
                            // TODO: Implement retry-mechanism.
                        }
                        drop(permit);
                });

                upload_jobs.push(h);
            },
            _ = token.cancelled() => {
                // Cleanup completed tasks
                upload_jobs.retain(|h| !h.is_finished());

                info!("Waiting for all uploads to finish ..");
                for handle in upload_jobs {
                    if let Err(err) = handle.await {
                        error!("Failed to join upload task: {}", err);
                    }
                }

                return Ok(())
            },
        }
    }
}

/// Uploads both 128-bit bootstrapped ciphertext and regular ciphertext to S3
/// buckets. If successful, it stores their digests in the database.
///
/// Guarantees:
/// - If the upload of the 128-bit ciphertext fails, the function will not store
///   its digest in the database.
/// - If the upload of the regular ciphertext fails, the function will not store
///   its digest in the database.
async fn upload_ciphertexts(
    task: HandleItem,
    client: &Client,
    pool: &PgPool,
    conf: &S3Config,
) -> Result<(), ExecutionError> {
    let handle_as_hex: String = compact_hex(&task.handle);
    info!("Received uploading task, handle: {}", handle_as_hex);

    let ct128_bytes = match task.ct128_uncompressed {
        Some(ct) => ct,
        None => {
            return Err(ExecutionError::MissingCiphertext128(handle_as_hex));
        }
    };

    let s = task.otel.child_span("compute_digest");
    let ct128_digest = compute_digest(&ct128_bytes);
    let ct64_digest = compute_digest(&task.ct64_compressed);
    telemetry::end_span(s);

    info!(
        "Start uploading task, handle: {}, tenant_id: {}, ct128_len: {}, ct64_compressed_len: {}",
        handle_as_hex,
        task.tenant_id,
        ByteSize::b(ct128_bytes.len() as u64),
        ByteSize::b(task.ct64_compressed.len() as u64)
    );

    let s = task.otel.child_span("s3_upload");
    let (up1, up2) = join!(
        tokio::time::timeout(
            3 * UPLOAD_TIMEOUT_DURATION,
            client
                .put_object()
                .bucket(conf.bucket_ct128.clone())
                .key(hex::encode(&ct128_digest))
                .body(ct128_bytes.into())
                .send(),
        ),
        tokio::time::timeout(
            UPLOAD_TIMEOUT_DURATION,
            client
                .put_object()
                .bucket(conf.bucket_ct64.clone())
                .key(hex::encode(&ct64_digest))
                .body(task.ct64_compressed.into())
                .send(),
        )
    );

    let upload_finish_time = SystemTime::now();

    let mut trx = pool.begin().await?;

    sqlx::query!(
        "INSERT INTO ciphertext_digest (tenant_id, handle)
        VALUES ($1, $2)",
        task.tenant_id,
        task.handle,
    )
    .execute(trx.as_mut())
    .await?;

    let mut ct128_uploaded = false;
    // Insert digest for ct128 only if ct128 upload was successful
    match &up1 {
        Ok(Ok(_)) => {
            ct128_uploaded = true;
            sqlx::query!(
                "UPDATE ciphertext_digest
                 SET ciphertext128 = $1
                 WHERE handle = $2",
                ct128_digest,
                task.handle
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
                task.handle
            )
            .execute(trx.as_mut())
            .await?;
        }
        Ok(Err(err)) => {
            error!(
                "Failed to upload ct128, handle: {}, err: {}",
                handle_as_hex, err
            );
        }
        Err(err) => {
            error!(
                "Failed to upload ct128, handle: {}, err: {}",
                handle_as_hex, err
            );
        }
    };

    // Insert digest for ct64 only if ct64 upload was successful
    let mut ct64_uploaded = false;
    match &up2 {
        Ok(Ok(_)) => {
            ct64_uploaded = true;
            sqlx::query!(
                "UPDATE ciphertext_digest
                 SET ciphertext = $1
                 WHERE handle = $2",
                ct64_digest,
                task.handle
            )
            .execute(trx.as_mut())
            .await?;
        }
        Ok(Err(err)) => {
            error!(
                "Failed to upload ct64, handle: {}, err: {}",
                handle_as_hex, err
            );
        }
        Err(err) => {
            error!(
                "Failed to upload ct64, handle: {}, err: {}",
                handle_as_hex, err
            );
        }
    }

    // If both uploads are successful, notify the Transaction Sender
    if ct128_uploaded && ct64_uploaded {
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(EVENT_CIPHERTEXTS_UPLOADED)
            .execute(trx.as_mut())
            .await?;

        info!(
            "Uploaded to S3, handle = {}, ct64_digest = {}, ct128_digest = {}",
            handle_as_hex,
            compact_hex(&ct64_digest),
            compact_hex(&ct128_digest)
        );
        telemetry::end_span_with_timestamp(s, upload_finish_time);
    } else {
        telemetry::end_span_with_err(
            s,
            format!(
                "Upload failed ct128: {}, ct64: {}",
                ct128_uploaded, ct64_uploaded
            ),
        );
    }

    trx.commit().await?;

    Ok(())
}

fn compute_digest(ct: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    hasher.finalize().to_vec()
}
