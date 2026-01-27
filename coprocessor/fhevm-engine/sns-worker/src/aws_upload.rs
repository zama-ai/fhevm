use crate::json_sidecar::CiphertextSideCar;
use crate::metrics::{AWS_UPLOAD_FAILURE_COUNTER, AWS_UPLOAD_SUCCESS_COUNTER};
use crate::{
    BigCiphertext, Ciphertext128Format, Config, ExecutionError, HandleItem, S3Config, UploadJob,
};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_bucket::HeadBucketError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytesize::ByteSize;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::pg_pool::{PostgresPoolManager, ServiceError};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::CoproSigner;
use fhevm_engine_common::utils::to_hex;
use futures::future::join_all;
use opentelemetry::trace::{Status, TraceContextExt};
use sha3::{Digest, Keccak256};
use sqlx::{PgPool, Pool, Postgres, Transaction};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::select;
use tokio::sync::{mpsc, RwLock, Semaphore};
use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, error_span, info, warn, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

// TODO: Use a config TOML to set these values
pub const EVENT_CIPHERTEXTS_UPLOADED: &str = "event_ciphertexts_uploaded";

// Default batch size for fetching pending uploads
// There might be pending uploads in the database
// with sizes of 32MiB so the batch size is set to 10
const DEFAULT_BATCH_SIZE: usize = 10;

pub(crate) async fn spawn_resubmit_task(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    jobs_tx: mpsc::Sender<UploadJob>,
    client: Arc<aws_sdk_s3::Client>,
    is_ready: Arc<AtomicBool>,
) -> Result<JoinHandle<()>, ExecutionError> {
    let op = move |pool, token| {
        let client = client.clone();
        let is_ready = is_ready.clone();
        let conf = conf.clone();
        let jobs_tx = jobs_tx.clone();

        async move {
            do_resubmits_loop(client, pool, conf, jobs_tx, token, is_ready)
                .await
                .map_err(ServiceError::from)
        }
    };

    // Spawn the resubmits_loop as a helper task
    Result::Ok(pool_mngr.spawn_with_db_retry(op, "s3_resubmit").await)
}

pub(crate) async fn spawn_uploader(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    rx: Arc<RwLock<mpsc::Receiver<UploadJob>>>,
    client: Arc<aws_sdk_s3::Client>,
    is_ready: Arc<AtomicBool>,
    signer: CoproSigner,
) -> Result<JoinHandle<()>, ExecutionError> {
    let op = move |pool, token| {
        let client = client.clone();
        let is_ready = is_ready.clone();
        let conf = conf.s3.clone();
        let rx = rx.clone();
        let signer = signer.clone();

        async move {
            run_uploader_loop(rx, token, client, is_ready, pool, conf, signer)
                .await
                .map_err(ServiceError::from)
        }
    };

    // Spawn the uploader loop
    Result::Ok(pool_mngr.spawn_with_db_retry(op, "s3").await)
}

async fn run_uploader_loop(
    jobs_rx: Arc<RwLock<mpsc::Receiver<UploadJob>>>,
    token: CancellationToken,
    client: Arc<Client>,
    is_ready: Arc<AtomicBool>,
    pool: Pool<Postgres>,
    conf: S3Config,
    signer: CoproSigner,
) -> Result<(), ExecutionError> {
    let mut ongoing_upload_tasks: Vec<JoinHandle<()>> = Vec::new();
    let max_concurrent_uploads = conf.max_concurrent_uploads as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_uploads));
    let mut jobs_rx = jobs_rx.write().await;
    loop {
        select! {
            job = jobs_rx.recv() => {
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
                                    WHERE handle = $1 AND
                                    (ciphertext128 IS NULL OR ciphertext IS NULL)
                                    FOR UPDATE SKIP LOCKED",
                                    item.handle
                        )
                        .fetch_one(trx.as_mut())
                        .await
                        {
                            warn!(
                                error = %err,
                                handle = to_hex(&item.handle),
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
                ongoing_upload_tasks.retain(|h| !h.is_finished());
                // Check if we have reached the max concurrent uploads
                if ongoing_upload_tasks.len() >= max_concurrent_uploads {
                    warn!({target = "worker", action = "review", max_concurrent_uploads = max_concurrent_uploads},
                        "Max concurrent uploads reached, waiting for a slot ...",
                    );
                } else {
                    debug!(
                        available_upload_slots = max_concurrent_uploads - ongoing_upload_tasks.len(),
                        "Available upload slots"
                    );
                }

                // Acquire a permit for an upload
                let permit = semaphore.clone().acquire_owned().await.expect("Failed to acquire semaphore permit");
                let client = client.clone();
                let conf = conf.clone();
                let ready_flag = is_ready.clone();
                let signer = signer.clone();

                // Spawn a new task to upload the ciphertexts
                let h = tokio::spawn(async move {
                    // Cross-boundary: spawned task; restore the OTel context
                    // that was captured when the upload item was created.
                    let upload_span = error_span!("upload_s3");
                    upload_span.set_parent(item.span.context());
                    match upload_ciphertexts(trx, item, &client, &conf, signer)
                        .instrument(upload_span.clone())
                        .await
                    {
                        Ok(()) => {
                            AWS_UPLOAD_SUCCESS_COUNTER.inc();
                        }
                        Err(err) => {
                            if let Some(exec_err) = err.downcast_ref::<ExecutionError>() {
                                if let ExecutionError::S3TransientError(_) = exec_err {
                                    ready_flag.store(false, Ordering::Release);
                                    info!(error = %err, "S3 setup is not ready, due to transient error");
                                } else {
                                    error!(error = %err, "Failed to upload ciphertexts");
                                }
                            } else {
                                error!(error = %err, "Failed to upload ciphertexts");
                            }
                            upload_span
                                .context()
                                .span()
                                .set_status(Status::error(err.to_string()));
                            AWS_UPLOAD_FAILURE_COUNTER.inc();
                        }
                    }
                    drop(upload_span);
                    drop(permit);
                });

                ongoing_upload_tasks.push(h);
            },
            _ = token.cancelled() => {
                // Cleanup completed tasks
                ongoing_upload_tasks.retain(|h| !h.is_finished());

                info!("Waiting for all uploads to finish...");
                for handle in ongoing_upload_tasks {
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
    CtType128(Vec<u8>),
    CtType64(Vec<u8>),
}

#[allow(clippy::too_many_arguments)]
async fn upload_ct(
    span: Span,
    client: Client,
    bucket: String,
    sidecar: CiphertextSideCar,
    ct_bytes: Vec<u8>,
    extra_digest_as_key: bool,
    result: UploadResult,
) -> anyhow::Result<UploadResult> {
    let key = sidecar.handle.clone();
    let upload = client
        .put_object()
        .bucket(bucket.clone())
        .metadata("Ct-Format", &sidecar.format)
        .metadata("Uploaded-By", "sns-worker")
        .metadata("Created-At", &sidecar.created_at)
        .metadata("Key-Id", &sidecar.key_id)
        .metadata("Transaction-Id", &sidecar.tx_hash)
        .metadata("Handle", &sidecar.handle)
        .metadata("Digest", &sidecar.digest)
        .metadata("Signed", &sidecar.signed_tuple)
        .metadata("Signer", &sidecar.signer)
        .key(&key)
        .body(ByteStream::from(ct_bytes))
        .send()
        .await;
    if let Err(err) = upload {
        error!(error = %err, bucket, ?sidecar, "Failed to upload ct");
        span.set_status(Status::error(err.to_string()));
        return Err(ExecutionError::S3TransientError(err.to_string()).into());
    }
    if extra_digest_as_key {
        let copy_source = format!("{}/{}", bucket, key);
        let upload_backward_compatible = client
            .copy_object()
            .copy_source(copy_source)
            .bucket(bucket.clone())
            .key(sidecar.digest.clone())
            .send()
            .await;
        if let Err(err) = upload_backward_compatible {
            error!(error = %err, bucket, ?sidecar, "Failed to upload ct for backcompatibility");
            span.set_status(Status::error(err.to_string()));
            return Err(ExecutionError::S3TransientError(err.to_string()).into());
        }
    }
    Ok(result)
}

fn s3_sidecar_key(sidecar: &CiphertextSideCar) -> String {
    format!("{}.json", &sidecar.handle)
}

async fn upload_sidecar(
    span: Span,
    client: Client,
    bucket: String,
    sidecar: CiphertextSideCar,
) -> anyhow::Result<()> {
    let result = client
        .put_object()
        .bucket(bucket.clone())
        .metadata("Uploaded-By", "sns-worker")
        .metadata("Transaction-Id", &sidecar.tx_hash) // for search
        .metadata("Handle", &sidecar.handle) // for search
        .metadata("Digest", &sidecar.digest) // for search
        .key(s3_sidecar_key(&sidecar))
        .body(ByteStream::from(sidecar.to_json_bytes()?))
        .send()
        .await;
    if let Err(err) = result {
        error!(error = %err, bucket, ?sidecar, "Failed to upload ciphertext sidecar");
        span.set_status(Status::error(err.to_string()));
        return Err(ExecutionError::S3TransientError(err.to_string()).into());
    }
    Ok(())
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
    signer: CoproSigner,
) -> anyhow::Result<()> {
    let handle_as_hex: String = to_hex(&task.handle);
    info!(handle = handle_as_hex, "Received task");

    let mut jobs = vec![];

    let key_id = task.key_id_gw.clone();
    let tx_hash = task.transaction_id.clone().unwrap_or_default();
    if !task.ct128.is_empty() && task.ct128.format() != Ciphertext128Format::Unknown {
        let ct128_bytes = task.ct128.bytes();
        // TODO check if sign_message is more appropriate (EIP-191 format)
        let ct128_digest = compute_digest(ct128_bytes);
        info!(
            handle = handle_as_hex,
            len = ?ByteSize::b(ct128_bytes.len() as u64),
            "Uploading ct128"
        );

        let format_as_str = task.ct128.format().to_string();

        let sidecar = CiphertextSideCar::new(
            &key_id,
            &tx_hash,
            &task.handle,
            &ct128_digest,
            &format_as_str,
            &signer,
        )
        .await?;

        let ct128_check_span = tracing::info_span!(
            "ct128_check_s3",
            ct_type = "ct128",
            exists = tracing::field::Empty,
        );
        let exists = match check_sidecar_exists(client, &conf.bucket_ct128, &sidecar)
            .instrument(ct128_check_span.clone())
            .await
        {
            Ok(v) => v,
            Err(err) => {
                ct128_check_span
                    .context()
                    .span()
                    .set_status(Status::error(err.to_string()));
                return Err(err.into());
            }
        };
        ct128_check_span.record("exists", tracing::field::display(exists));
        drop(ct128_check_span);

        if !exists {
            // TODO: check if sign_message is more appropriate (EIP-191 format)
            // TODO: check if the chain_id is needed (in sidecar + metadata + signing)
            let bucket = &conf.bucket_ct128;
            let ct128_upload_span = tracing::info_span!(
                "ct128_upload_s3",
                ct_type = "ct128",
                format = %format_as_str,
                len = ct128_bytes.len(),
            );
            let result = UploadResult::CtType128(ct128_digest);
            let upload_ct = upload_ct(
                ct128_upload_span.clone(),
                client.clone(),
                bucket.clone(),
                sidecar.clone(),
                ct128_bytes.to_vec(),
                true, // use digest as key for back-compatibility
                result,
            );
            let upload_sidecar = upload_sidecar(
                ct128_upload_span.clone(),
                client.clone(),
                bucket.clone(),
                sidecar.clone(),
            );
            let grouped_upload = tokio::spawn(
                async move {
                    let result = upload_ct.await?;
                    upload_sidecar.await?;
                    // sidecar validate the full write success
                    // in case of failure everything will be overwritten on the next upload attempt
                    anyhow::Ok(result)
                }
                .instrument(ct128_upload_span),
            );
            jobs.push(grouped_upload);
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
            "Uploading ct64",
        );

        let ct64_digest = compute_digest(ct64_compressed);

        let format_as_str = "ct64_compressed".to_string();

        let sidecar = CiphertextSideCar::new(
            &key_id,
            &tx_hash,
            &task.handle,
            &ct64_digest,
            &format_as_str,
            &signer,
        )
        .await?;

        let ct64_check_span = tracing::info_span!(
            "ct64_check_s3",
            ct_type = "ct64",
            exists = tracing::field::Empty,
        );
        let exists = match check_sidecar_exists(client, &conf.bucket_ct64, &sidecar)
            .instrument(ct64_check_span.clone())
            .await
        {
            Ok(v) => v,
            Err(err) => {
                ct64_check_span
                    .context()
                    .span()
                    .set_status(Status::error(err.to_string()));
                return Err(err.into());
            }
        };
        ct64_check_span.record("exists", tracing::field::display(exists));
        drop(ct64_check_span);

        if !exists {
            let bucket = &conf.bucket_ct64;
            let ct64_upload_span = tracing::info_span!(
                "ct64_upload_s3",
                ct_type = "ct64",
                len = ct64_compressed.len(),
            );
            let result = UploadResult::CtType64(ct64_digest);
            let upload_ct = upload_ct(
                ct64_upload_span.clone(),
                client.clone(),
                bucket.clone(),
                sidecar.clone(),
                ct64_compressed.clone(),
                false, // no need to use digest as key for ct64
                result,
            );
            let upload_sidecar = upload_sidecar(
                ct64_upload_span.clone(),
                client.clone(),
                bucket.clone(),
                sidecar.clone(),
            );
            let grouped_upload = tokio::spawn(
                async move {
                    let result = upload_ct.await?;
                    upload_sidecar.await?;
                    // if ct upload failed, having only the side car is ok
                    // everything will be overwritten on the next upload attempt
                    Ok(result)
                }
                .instrument(ct64_upload_span),
            );
            jobs.push(grouped_upload);
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

    let mut transient_error = anyhow::Ok(());
    let mut successful_uploads = 0;
    let jobs_is_empty = jobs.is_empty();

    // Wait all uploads results
    for result in join_all(jobs).await {
        match result {
            Ok(Ok(UploadResult::CtType128(digest))) => {
                successful_uploads += 1;
                task.update_ct128_uploaded(&mut trx, digest).await?
            }
            Ok(Ok(UploadResult::CtType64(digest))) => {
                successful_uploads += 1;
                task.update_ct64_uploaded(&mut trx, digest).await?
            }
            Ok(Err(err)) => {
                // already logged
                transient_error = Err(ExecutionError::S3TransientError(err.to_string()).into());
            }
            Err(err) => {
                transient_error = Err(ExecutionError::InternalError(err.to_string()).into());
                error!(error = %err, handle = handle_as_hex, "Upload task fail");
            }
        }
    }

    if successful_uploads > 0 || jobs_is_empty {
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(EVENT_CIPHERTEXTS_UPLOADED)
            .execute(trx.as_mut())
            .await?;
    }

    // db is updated only on successful cases
    trx.commit().await?;

    transient_error
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
        "SELECT handle, ciphertext, ciphertext128, ciphertext128_format, transaction_id, host_chain_id, key_id_gw
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
        let transaction_id = row.transaction_id;

        // Fetch missing ciphertext
        if ciphertext_digest.is_none() {
            if let Ok(row) = sqlx::query!(
                "SELECT ciphertext FROM ciphertexts WHERE handle = $1;",
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
                "SELECT ciphertext FROM ciphertexts128 WHERE handle = $1;",
                handle
            )
            .fetch_optional(db_pool)
            .await
            {
                if let Some(record) = row {
                    match record.ciphertext {
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
                        handle = to_hex(&handle),
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
            let recovery_span = tracing::info_span!(
                "recovery_task",
                txn_id = tracing::field::Empty,
                handle = tracing::field::Empty
            );
            telemetry::record_short_hex(&recovery_span, "handle", &handle);
            telemetry::record_short_hex_if_some(
                &recovery_span,
                "txn_id",
                transaction_id.as_deref(),
            );
            let item = HandleItem {
                host_chain_id: ChainId::try_from(row.host_chain_id)
                    .map_err(|e| ExecutionError::ConversionError(e.into()))?,
                key_id_gw: row.key_id_gw,
                handle: handle.clone(),
                ct64_compressed,
                ct128: Arc::new(ct128),
                span: recovery_span,
                transaction_id,
            };

            // Instruct the uploader to acquire DB lock when processing the item
            jobs.push(UploadJob::DatabaseLock(item));
        } else {
            // This should not happen as we are fetching rows with NULL ct64 or ct128
            error!(
                handle = hex::encode(&handle),
                "Both ciphertext and ciphertext128 are empty, skipping"
            );
        }
    }

    Ok(jobs)
}

/// Resubmit for uploading ciphertexts.
/// If a handle has a missing digest in ciphertext_digest table then
/// retry uploading the actual ciphertext.
async fn do_resubmits_loop(
    client: Arc<aws_sdk_s3::Client>,
    pool: Pool<Postgres>,
    conf: Config,
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
                    let (is_ready_res, _) = check_is_ready(&client, &conf).await;
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
                    pending_uploads = jobs.len(),
                    "Fetched pending uploads from the database"
                );
                let jobs_count = jobs.len();
                // Resubmit for uploading ciphertexts
                for task in jobs {
                    select! {
                        _ = tasks.send(task.clone()) => {
                            info!(handle = to_hex(task.handle()), "resubmitted");
                        },
                        _ = token.cancelled() => {
                            return Ok(());
                        }
                    }
                }

                if jobs_count < batch_size {
                    info!("No (more) pending uploads to resubmit");
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

async fn check_sidecar_exists(
    client: &Client,
    bucket: &str,
    sidecar: &CiphertextSideCar,
) -> Result<bool, ExecutionError> {
    let sidecar_key = s3_sidecar_key(sidecar);
    match client
        .head_object()
        .bucket(bucket)
        .key(sidecar_key)
        .send()
        .await
    {
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
