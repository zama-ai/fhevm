use crate::{
    BigCiphertext, Ciphertext128Format, Config, ExecutionError, HandleItem, S3Config, UploadJob,
};
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
use std::time::SystemTime;
use tokio::select;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

// TODO: Use a config TOML to set these values
pub const EVENT_CIPHERTEXTS_UPLOADED: &str = "event_ciphertexts_uploaded";

// Default batch size for fetching pending uploads
// There might be pending uploads in the database
// with sizes of 32MiB so the batch size is set to 10
const DEFAULT_BATCH_SIZE: usize = 10;

/// Process the S3 uploads
pub(crate) async fn process_s3_uploads(
    conf: &Config,
    mut jobs: mpsc::Receiver<UploadJob>,
    jobs_tx: mpsc::Sender<UploadJob>,
    token: CancellationToken,
    client: Arc<aws_sdk_s3::Client>,
    is_ready: Arc<AtomicBool>,
) -> Result<(), ExecutionError> {
    let (is_ready_res, _) = check_is_ready(&client, conf).await;
    is_ready.store(is_ready_res, Ordering::Release);

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(conf.db.max_connections)
            .acquire_timeout(conf.db.timeout)
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
                    error!(error = %err, "Failed to spawn do_resubmits_loop");
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
                    UploadJob::Normal(item) =>
                    {
                        item.enqueue_upload_task(&mut trx).await?;
                        item
                    },
                    UploadJob::DatabaseLock(item) => {
                        if let Err(err) = sqlx::query!(
                            "SELECT * FROM ciphertext_digest
                                    WHERE handle = $2 AND tenant_id = $1 AND
                                    (ciphertext128 IS NULL OR ciphertext IS NULL)
                                    FOR UPDATE SKIP LOCKED",
                                    item.tenant_id,
                                    item.handle
                        )
                        .fetch_one(trx.as_mut())
                        .await
                        {
                            warn!(
                                error = %err,
                                handle = compact_hex(&item.handle),
                                "Failed to lock pending uploads",
                            );
                            trx.rollback().await?;
                            continue;
                        }
                        item
                    },
                 };


                debug!(handle = hex::encode(&item.handle), "Received task, handle");

                // Cleanup completed tasks
                upload_jobs.retain(|h| !h.is_finished());

                // Check if we have reached the max concurrent uploads
                if upload_jobs.len() >= max_concurrent_uploads {
                    warn!({target = "worker", action = "review", max_concurrent_uploads = max_concurrent_uploads},
                        "Max concurrent uploads reached, waiting for a slot ...",
                    );
                } else {
                    debug!(
                        available_upload_slots = max_concurrent_uploads - upload_jobs.len(),
                        "Available upload slots"
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
                                info!(error = %err, "S3 setup is not ready, due to transient error");
                            } else {
                                error!(error = %err, "Failed to upload ciphertexts");
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

                info!("Waiting for all uploads to finish...");
                for handle in upload_jobs {
                    if let Err(err) = handle.await {
                        error!(error = %err, "Failed to join upload task");
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
    info!(handle = handle_as_hex, "Received task");

    let mut jobs = vec![];

    if !task.ct128.is_empty() && task.ct128.format() != Ciphertext128Format::Unknown {
        let ct128_bytes = task.ct128.bytes();
        let ct128_digest = compute_digest(ct128_bytes);
        info!(
            handle = handle_as_hex,
            len = ?ByteSize::b(ct128_bytes.len() as u64),
            tenant_id = task.tenant_id,
            "Uploading ct128"
        );

        let format_as_str = task.ct128.format().to_string();

        let key = if cfg!(feature = "test_s3_use_handle_as_key") {
            hex::encode(&task.handle)
        } else {
            // Use the digest as the key for the ct128 object
            // This is the production behavior
            hex::encode(&ct128_digest)
        };

        let mut s = task.otel.child_span("ct128_check_s3");
        let exists = check_object_exists(client, &conf.bucket_ct128, &key).await?;
        telemetry::attribute(&mut s, "exists", exists.to_string());
        telemetry::end_span(s);

        if !exists {
            let mut span: BoxedSpan = task.otel.child_span("ct128_upload_s3");
            telemetry::attribute(&mut span, "len", ct128_bytes.len().to_string());
            telemetry::attribute(&mut span, "format", format_as_str.to_owned());

            jobs.push((
                client
                    .put_object()
                    .bucket(conf.bucket_ct128.clone())
                    .metadata("Ct-Format", format_as_str)
                    .key(key)
                    .body(ByteStream::from(ct128_bytes.to_vec()))
                    .send(),
                UploadResult::CtType128((ct128_digest.clone(), span)),
            ));
        } else {
            info!(
                handle = handle_as_hex,
                ct128_digest = hex::encode(&ct128_digest),
                "ct128 already exists in S3",
            );

            // In case of a sns-worker failure after uploading to S3,
            // the state between both storages may become inconsistent
            task.update_ct128_uploaded(&mut trx, ct128_digest).await?;
        }
    }

    if !task.ct64_compressed.is_empty() {
        let ct64_compressed = task.ct64_compressed.as_ref();
        info!(
            handle = handle_as_hex,
            len = ?ByteSize::b(ct64_compressed.len() as u64),
            tenant_id = task.tenant_id,
            "Uploading ct64",
        );

        let ct64_digest = compute_digest(ct64_compressed);

        let key = if cfg!(feature = "test_s3_use_handle_as_key") {
            hex::encode(&task.handle)
        } else {
            // Use the digest as the key for the ct64 object
            // This is the production behavior
            hex::encode(&ct64_digest)
        };

        let mut s = task.otel.child_span("ct64_check_s3");
        let exists = check_object_exists(client, &conf.bucket_ct64, &key).await?;
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
                handle = handle_as_hex,
                ct64_digest = hex::encode(&ct64_digest),
                "ct64 already exists in S3",
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
                        error = %err,
                        handle = handle_as_hex,
                        "Failed to upload ct128",
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
                        error = %err,
                        handle = handle_as_hex,
                        "Failed to upload ct64"
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

pub fn compute_digest(ct: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    hasher.finalize().to_vec()
}

/// Fetches incomplete upload tasks from the database.
///
/// An incomplete upload task is defined as a task that has either
/// `ciphertext` or `ciphertext128` as NULL in the `ciphertext_digest` table.
async fn fetch_pending_uploads(
    db_pool: &Pool<Postgres>,
    limit: i64,
) -> Result<Vec<UploadJob>, ExecutionError> {
    let rows = sqlx::query!(
        "SELECT tenant_id, handle, ciphertext, ciphertext128, ciphertext128_format 
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
        let mut ct128 = Vec::new();
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
                    error!(handle = hex::encode(&handle), "Missing ciphertext");
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
                            ct128 = ct;
                        }
                        _ => {
                            warn!(handle = hex::encode(&handle), "Fetched empty ct128");
                        }
                    }
                } else {
                    error!(handle = hex::encode(&handle), "Missing ciphertext128");
                }
            }
        }

        let is_ct128_empty = ct128.is_empty();

        let ct128 = if !is_ct128_empty {
            match BigCiphertext::new_with_format_id(ct128, row.ciphertext128_format) {
                Some(ct) => ct,
                None => {
                    error!(
                        handle = compact_hex(&handle),
                        format_id = row.ciphertext128_format,
                        "Failed to create a BigCiphertext from DB data",
                    );
                    continue;
                }
            }
        } else {
            // Already uploaded
            BigCiphertext::default()
        };

        if !ct64_compressed.is_empty() || !is_ct128_empty {
            let item = HandleItem {
                tenant_id: row.tenant_id,
                handle: handle.clone(),
                ct64_compressed,
                ct128: Arc::new(ct128),
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
    try_resubmit(
        &pool,
        is_ready.clone(),
        tasks.clone(),
        token.clone(),
        DEFAULT_BATCH_SIZE,
    )
    .await
    .unwrap_or_else(|err| {
        error!(error = %err, "Failed to resubmit tasks");
    });

    let retry_conf = &conf.s3.retry_policy;

    let mut recheck_ticker = interval(retry_conf.recheck_duration);
    let mut resubmit_ticker = interval(retry_conf.regular_recheck_duration);

    loop {
        select! {
            _ = token.cancelled() => {
                return Ok(())
            },
            // Recheck S3 ready status
            _ = recheck_ticker.tick() => {
                if !is_ready.load(Ordering::Acquire) {
                    info!("Recheck S3 setup ...");
                    let (is_ready_res, _) = check_is_ready(&client, conf).await;
                    if is_ready_res {
                        info!("Reconnected to S3, buckets exist");
                        is_ready.store(true, Ordering::Release);
                        try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone(), DEFAULT_BATCH_SIZE).await
                            .unwrap_or_else(|err| {
                                error!(error = %err, "Failed to resubmit tasks");
                            });
                    }
                }
            }
            // A regular resubmit to ensure there no remaining tasks
            _ = resubmit_ticker.tick() => {
                info!("Retry resubmit ...");
                try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone(), DEFAULT_BATCH_SIZE).await
                    .unwrap_or_else(|err| {
                        error!(error = %err, "Failed to resubmit tasks");
                });
            }
        }
    }
}

/// Attempts to resubmit all pending uploads from the database.
///
/// If the S3 setup is not ready, it will skip resubmitting.
///
/// This function will keep fetching pending uploads in batches until there are no more
async fn try_resubmit(
    pool: &PgPool,
    is_ready: Arc<AtomicBool>,
    tasks: mpsc::Sender<UploadJob>,
    token: CancellationToken,
    batch_size: usize,
) -> Result<(), ExecutionError> {
    loop {
        if !is_ready.load(Ordering::SeqCst) {
            info!("S3 setup is not ready, skipping resubmit");
            return Ok(());
        }

        match fetch_pending_uploads(pool, batch_size as i64).await {
            Ok(jobs) => {
                info!(
                    target: "worker",
                    action = "retry_s3_uploads",
                    pending_uploads = jobs.len(),
                    "Fetched pending uploads from the database"
                );
                let jobs_count = jobs.len();
                // Resubmit for uploading ciphertexts
                for task in jobs {
                    select! {
                        _ = tasks.send(task.clone()) => {
                            info!(handle = compact_hex(task.handle()), "resubmitted");
                        },
                        _ = token.cancelled() => {
                            return Ok(());
                        }
                    }
                }

                if jobs_count < batch_size {
                    info!(
                        target: "worker",
                        action = "retry_s3_uploads",
                        "No (more) pending uploads to resubmit"
                    );
                    return Ok(());
                }
            }
            Err(err) => {
                error!(error = %err, "Failed to fetch pending uploads");
                return Err(err);
            }
        }
    }
}

/// Checks if the S3 client is ready by verifying the existence of both
/// the ct64 and ct128 buckets.
///
/// Returns is_ready and is_connected status.
pub(crate) async fn check_is_ready(client: &Client, conf: &Config) -> (bool, bool) {
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
            error!(error = %err, "Failed to check object existence");
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
                error!(error = %err, "Failed to check bucket existence");
                Err(err)
            }
        };

    match res {
        Ok(true) => {
            info!(bucket = bucket, "Bucket exists");
            (true, true)
        }
        Ok(false) => {
            error!({ action = "review", bucket = bucket }, "Bucket does not exist");
            (false, true)
        }
        Err(err) => {
            error!(
                { action = "review", error = %err, },
                "Failed to check bucket existence"
            );
            (false, false)
        }
    }
}
