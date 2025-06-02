use crate::{Config, ExecutionError, HandleItem, S3Config, UploadJob};
use aws_config::retry::RetryConfig;
use aws_config::timeout::TimeoutConfig;
use aws_config::BehaviorVersion;
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_bucket::HeadBucketError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytesize::ByteSize;
use fhevm_engine_common::telemetry::{self};
use fhevm_engine_common::utils::compact_hex;
use futures::future::join_all;
use sha3::{Digest, Keccak256};

use opentelemetry::global::BoxedSpan;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres, Transaction};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::select;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

// TODO: Use a config TOML to set these values
pub const EVENT_CIPHERTEXTS_UPLOADED: &str = "event_ciphertexts_uploaded";

/// Process the S3 uploads
pub(crate) async fn process_s3_uploads(
    conf: &Config,
    mut jobs: mpsc::Receiver<UploadJob>,
    jobs_tx: mpsc::Sender<UploadJob>,
    token: CancellationToken,
) -> Result<(), ExecutionError> {
    // Client construction is expensive due to connection thread pool initialization, and should
    // be done once at application start-up.
    let (client, is_ready) = create_s3_client(conf).await;
    let is_ready = Arc::new(AtomicBool::new(is_ready));

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(conf.db.max_connections)
            .connect(&conf.db.url)
            .await?,
    );

    // Spawn the resubmits_loop as a helper task
    tokio::spawn({
        let client = client.clone();
        let is_ready = is_ready.clone();
        let conf = conf.clone();
        let token = token.clone();
        let pool = pool.clone();
        async move {
            do_resubmits_loop(client, pool, &conf, jobs_tx, token, is_ready)
                .await
                .unwrap_or_else(|err| {
                    error!("Failed to spawn do_resubmits_loop: {}", err);
                });
        }
    });

    let conf = &conf.s3;
    let max_concurrent_uploads = conf.max_concurrent_uploads as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_uploads));
    let mut upload_jobs: Vec<JoinHandle<()>> = Vec::new();

    loop {
        select! {
            job = jobs.recv() => {
                let job = match job {
                    Some(job) => job,
                    None => return Ok(()),
                };

                if !is_ready.load(Ordering::Acquire) {
                    // If the S3 setup is not ready, we need to wait for its ready status
                    // before we can continue spawning uploading job
                    info!("Upload task skipped, S3 connection still not ready");
                    continue;
                }

                let mut trx = pool.begin().await?;

                let item = match job {
                    UploadJob::Normal(item) => item,
                    UploadJob::DatabaseLock(item) => {
                        if let Err(err) = sqlx::query!(
                            "SELECT * FROM ciphertext_digest
                                    WHERE handle = $2 AND tenant_id = $1 AND
                                    (ciphertext128 IS NULL OR ciphertext IS NULL)
                                    FOR UPDATE SKIP LOCKED",
                            item.tenant_id,
                            item.handle,
                        )
                        .fetch_one(trx.as_mut())
                        .await
                        {
                            warn!(
                                "Failed to lock pending uploads {}, handle: {}",
                                err,
                                compact_hex(&item.handle)
                            );
                            trx.rollback().await?;
                            continue;
                        }
                        item
                    },
                 };


                debug!("Received task, handle: {}", hex::encode(&item.handle));

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
                let conf = conf.clone();
                let ready_flag = is_ready.clone();

                // Spawn a new task to upload the ciphertexts
                let h = tokio::spawn(async move {
                    let s = item.otel.child_span("upload_s3");
                        if let Err(err) = upload_ciphertexts(trx, item, &client, &conf).await {
                            if let ExecutionError::S3TransientError(_) = err {
                                ready_flag.store(false, Ordering::Release);
                                info!("S3 setup is not ready, due to transient error: {}", err);
                            }else {
                                error!("Failed to upload ciphertexts: {}", err);
                            }

                            telemetry::end_span_with_err(s, err.to_string());

                        } else {
                            telemetry::end_span(s);
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
            }
        }
    }
}

enum UploadResult {
    CtType128((Vec<u8>, BoxedSpan)),
    CtType64((Vec<u8>, BoxedSpan)),
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
    mut trx: Transaction<'_, Postgres>,
    task: HandleItem,
    client: &Client,
    conf: &S3Config,
) -> Result<(), ExecutionError> {
    let handle_as_hex: String = compact_hex(&task.handle);
    info!("Received task, handle: {}", handle_as_hex);

    let mut jobs = vec![];

    if !task.ct128_uncompressed.is_empty() {
        let ct128_bytes = task.ct128_uncompressed.as_ref();
        let ct128_digest = compute_digest(ct128_bytes);
        info!(
            "Uploading ct128, handle: {}, len: {}, tenant: {}",
            handle_as_hex,
            ByteSize::b(ct128_bytes.len() as u64),
            task.tenant_id,
        );

        let key = hex::encode(&ct128_digest);

        let mut s = task.otel.child_span("ct128_check_s3");
        let exists = check_object_exists(client, &conf.bucket_ct128, &key).await?;
        telemetry::attribute(&mut s, "exists", exists.to_string());
        telemetry::end_span(s);

        if !exists {
            let mut span: BoxedSpan = task.otel.child_span("ct128_upload_s3");
            telemetry::attribute(&mut span, "len", ct128_bytes.len().to_string());

            jobs.push((
                client
                    .put_object()
                    .bucket(conf.bucket_ct128.clone())
                    .key(key)
                    .body(ByteStream::from(ct128_bytes.clone()))
                    .send(),
                UploadResult::CtType128((ct128_digest.clone(), span)),
            ));
        } else {
            info!(
                "ct128 already exists in S3, handle: {}, digest: {}",
                handle_as_hex,
                hex::encode(&ct128_digest)
            );

            // In case of a sns-worker failure after uploading to S3,
            // the state between both storages may become inconsistent
            task.update_ct128_uploaded(&mut trx, ct128_digest).await?;
        }
    }

    if !task.ct64_compressed.is_empty() {
        let ct64_compressed = task.ct64_compressed.as_ref();
        info!(
            "Uploading ct64, handle: {}, len: {}, tenant: {}",
            handle_as_hex,
            ByteSize::b(ct64_compressed.len() as u64),
            task.tenant_id,
        );

        let ct64_digest = compute_digest(ct64_compressed);

        let key = hex::encode(&ct64_digest);

        let mut s = task.otel.child_span("ct64_check_s3");
        let exists = check_object_exists(client, &conf.bucket_ct128, &key).await?;
        telemetry::attribute(&mut s, "exists", exists.to_string());
        telemetry::end_span(s);

        if !exists {
            let mut span = task.otel.child_span("ct64_upload_s3");
            telemetry::attribute(&mut span, "len", ct64_compressed.len().to_string());

            jobs.push((
                client
                    .put_object()
                    .bucket(conf.bucket_ct64.clone())
                    .key(key)
                    .body(ByteStream::from(ct64_compressed.clone()))
                    .send(),
                UploadResult::CtType64((ct64_digest.clone(), span)),
            ));
        } else {
            info!(
                "ct64 already exists in S3, handle: {}, digest: {}",
                handle_as_hex,
                hex::encode(&ct64_digest)
            );

            // In case of a sns-worker failure after uploading to S3,
            // the state between both storages may become inconsistent
            task.update_ct64_uploaded(&mut trx, ct64_digest).await?;
        }
    }

    // Execute all uploads and collect results with their IDs
    let results: Vec<(Result<_, _>, UploadResult, SystemTime)> = join_all(
        jobs.into_iter()
            .map(|(fut, upload)| async move { (fut.await, upload, SystemTime::now()) }),
    )
    .await;

    let mut transient_error: Option<ExecutionError> = None;

    for (ct_variant, result, finish_time) in results {
        match result {
            UploadResult::CtType128((digest, span)) => {
                if let Err(err) = ct_variant {
                    error!(
                        "Failed to upload ct128, handle: {}, err: {}",
                        handle_as_hex, err
                    );

                    telemetry::end_span_with_err(span, err.to_string());
                    transient_error = Some(ExecutionError::S3TransientError(err.to_string()));
                } else {
                    task.update_ct128_uploaded(&mut trx, digest).await?;
                    telemetry::end_span_with_timestamp(span, finish_time);
                }
            }
            UploadResult::CtType64((digest, span)) => {
                if let Err(err) = ct_variant {
                    error!(
                        "Failed to upload ct64, handle: {}, err: {}",
                        handle_as_hex, err
                    );

                    telemetry::end_span_with_err(span, err.to_string());
                    transient_error = Some(ExecutionError::S3TransientError(err.to_string()));
                } else {
                    task.update_ct64_uploaded(&mut trx, digest).await?;
                    telemetry::end_span_with_timestamp(span, finish_time);
                }
            }
        }
    }

    sqlx::query("SELECT pg_notify($1, '')")
        .bind(EVENT_CIPHERTEXTS_UPLOADED)
        .execute(trx.as_mut())
        .await?;

    trx.commit().await?;

    transient_error.map_or(Ok(()), Err)
}

fn compute_digest(ct: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    hasher.finalize().to_vec()
}

/// Fetches incomplete upload tasks from the database.
async fn fetch_pending_uploads(
    db_pool: &Pool<Postgres>,
    limit: i64,
) -> Result<Vec<UploadJob>, ExecutionError> {
    let rows = sqlx::query!(
        "SELECT tenant_id, handle, ciphertext, ciphertext128
        FROM ciphertext_digest 
        WHERE ciphertext IS NULL OR ciphertext128 IS NULL
        FOR UPDATE SKIP LOCKED
        LIMIT $1;",
        limit
    )
    .fetch_all(db_pool)
    .await?;

    let mut jobs = Vec::new();

    for row in rows {
        let mut ct64_compressed = Arc::new(Vec::new());
        let mut ct128_uncompressed = Arc::new(Vec::new());
        let ciphertext_digest = row.ciphertext;
        let ciphertext128_digest = row.ciphertext128;
        let handle = row.handle;

        // Fetch missing ciphertext
        if ciphertext_digest.is_none() {
            if let Ok(row) = sqlx::query!(
                "SELECT ciphertext FROM ciphertexts WHERE tenant_id = $1 AND handle = $2;",
                row.tenant_id,
                handle
            )
            .fetch_optional(db_pool)
            .await
            {
                if let Some(record) = row {
                    ct64_compressed = Arc::new(record.ciphertext);
                } else {
                    error!("Missing ciphertext, handle: {}", hex::encode(&handle));
                }
            }
        }

        // Fetch missing ciphertext128
        if ciphertext128_digest.is_none() {
            if let Ok(row) = sqlx::query!(
                "SELECT ciphertext128 FROM ciphertexts WHERE tenant_id = $1 AND handle = $2;",
                row.tenant_id,
                handle
            )
            .fetch_optional(db_pool)
            .await
            {
                if let Some(record) = row {
                    match record.ciphertext128 {
                        Some(ct) if !ct.is_empty() => {
                            ct128_uncompressed = Arc::new(ct);
                        }
                        _ => {
                            warn!("Fetched empty ct128, handle: {}", hex::encode(&handle));
                        }
                    }
                } else {
                    error!("Missing ciphertext128, handle: {}", hex::encode(&handle));
                }
            }
        }

        if !ct64_compressed.is_empty() || !ct128_uncompressed.is_empty() {
            let item = HandleItem {
                tenant_id: row.tenant_id,
                handle: handle.clone(),
                ct64_compressed,
                ct128_uncompressed,
                otel: telemetry::tracer_with_handle("recovery_task", handle),
            };

            // Instruct the uploader to acquire DB lock when processing the item
            jobs.push(UploadJob::DatabaseLock(item));
        }
    }

    Ok(jobs)
}

/// Resubmit for uploading ciphertexts.
/// If a handle has a missing digest in ciphertext_digest table then
/// retry uploading the actual ciphertext.
async fn do_resubmits_loop(
    client: Arc<aws_sdk_s3::Client>,
    pool: Arc<Pool<Postgres>>,
    conf: &Config,
    tasks: mpsc::Sender<UploadJob>,
    token: CancellationToken,
    is_ready: Arc<AtomicBool>,
) -> Result<(), ExecutionError> {
    // Retry to resubmit all upload tasks at the start-up
    try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone())
        .await
        .unwrap_or_else(|err| {
            error!("Failed to resubmit tasks: {}", err);
        });

    let retry_conf = &conf.s3.retry_policy;

    loop {
        select! {
            _ = token.cancelled() => {
                return Ok(())
            },
            // Recheck S3 ready status
            _ = tokio::time::sleep(retry_conf.recheck_duration) => {
                if !is_ready.load(Ordering::Acquire) {
                    info!("Recheck S3 setup ...");
                    let (is_ready_res, _) = check_is_ready(&client, conf).await;
                    if is_ready_res {
                        info!("Reconnected to S3, buckets exist");
                        is_ready.store(true, Ordering::Release);
                        try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone()).await
                            .unwrap_or_else(|err| {
                                error!("Failed to resubmit tasks: {}", err);
                            });
                    }
                }
            }

            // A regular resubmit to ensure there no left over tasks
            _ = tokio::time::sleep(retry_conf.regular_recheck_duration) => {
                try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone()).await
                    .unwrap_or_else(|err| {
                        error!("Failed to resubmit tasks: {}", err);
                    });
            }
        }
    }
}

async fn try_resubmit(
    pool: &PgPool,
    is_ready: Arc<AtomicBool>,
    tasks: mpsc::Sender<UploadJob>,
    token: CancellationToken,
) -> Result<(), ExecutionError> {
    if !is_ready.load(Ordering::SeqCst) {
        info!("S3 setup is not ready, skipping resubmit");
        return Ok(());
    }

    match fetch_pending_uploads(pool, 10).await {
        // TODO: const, token
        Ok(jobs) => {
            info!(
                target: "worker",
                action = "retry_s3_uploads",
                "Fetched {} pending uploads from the database",
                jobs.len()
            );
            // Resubmit for uploading ciphertexts
            for task in jobs {
                select! {
                    _ = tasks.send(task.clone()) => {
                        // TODO: debug!("Task sent to upload worker, handle: {}", hex::encode(&task.handle));
                    },
                    _ = token.cancelled() => {
                        return Ok(())
                    }
                }
            }
        }
        Err(err) => error!("Failed to fetch pending uploads: {}", err),
    };
    Ok(())
}

/// Configure and create the S3 client.
///
/// Logs errors if the connection fails or if any buckets are missing.
/// Even in the event of a failure or missing buckets, the function returns a valid
/// S3 client capable of retrying S3 operations later.
async fn create_s3_client(conf: &Config) -> (Arc<aws_sdk_s3::Client>, bool) {
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
        info!("Connected to S3, is_ready: {}", is_ready);
    }

    (client, is_ready)
}

/// Checks if the S3 client is ready by verifying the existence of both
/// the ct64 and ct128 buckets.
///
/// Returns is_ready and is_connected status.
async fn check_is_ready(client: &Client, conf: &Config) -> (bool, bool) {
    // Check if the S3 client is ready
    //
    // By checking the existence of both ct64 and ct128 buckets here,
    // we also incorporate the aws-sdk connection retry
    let (ct64_exists, _) = check_bucket_exists(client, &conf.s3.bucket_ct64).await;
    let (ct128_exists, conn) = check_bucket_exists(client, &conf.s3.bucket_ct128).await;

    ((ct64_exists && ct128_exists), conn)
}

async fn check_object_exists(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<bool, ExecutionError> {
    match client.head_object().bucket(bucket).key(key).send().await {
        Ok(_) => Ok(true),
        Err(SdkError::ServiceError(err)) if matches!(err.err(), HeadObjectError::NotFound(_)) => {
            Ok(false)
        }
        Err(err) => {
            error!("Failed to check object existence: {}", err);
            Err(ExecutionError::S3TransientError(err.to_string()))
        }
    }
}

async fn check_bucket_exists(
    client: &Client,
    bucket: &str,
) -> (bool, bool /* connection status */) {
    let res: Result<bool, SdkError<HeadBucketError, _>> =
        match client.head_bucket().bucket(bucket).send().await {
            Ok(_) => Ok(true),
            Err(SdkError::ServiceError(err))
                if matches!(err.err(), HeadBucketError::NotFound(_)) =>
            {
                Ok(false)
            }
            Err(err) => {
                error!("Failed to check bucket existence: {}", err);
                Err(err)
            }
        };

    match res {
        Ok(true) => {
            info!("Bucket {} exists", bucket);
            (true, true)
        }
        Ok(false) => {
            error!({ action = "review" }, "Bucket {} does not exist", bucket);
            (false, true)
        }
        Err(err) => {
            error!(
                { action = "review" },
                "Failed to check bucket existence: {:?}", err
            );
            (false, false)
        }
    }
}
