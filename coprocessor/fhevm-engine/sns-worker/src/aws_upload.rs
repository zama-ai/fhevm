use crate::metrics::{AWS_UPLOAD_FAILURE_COUNTER, AWS_UPLOAD_SUCCESS_COUNTER};
use crate::{
    BigCiphertext, Ciphertext128Format, Config, ExecutionError, HandleItem, S3Config, UploadJob,
    CURRENT_S3_FORMAT_VERSION,
};
use alloy_primitives::{B256, U256};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_bucket::HeadBucketError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytesize::ByteSize;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextAttestationPayload, CiphertextFormat, Version,
    S3_METADATA_ATTESTATION_KEY,
};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::pg_pool::{is_fatal_connection_error, PostgresPoolManager, ServiceError};
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
pub(crate) const COPROCESSOR_CONTEXT_ID_1: U256 = U256::ONE;
const NO_SNS_CIPHERTEXT_DIGEST: [u8; 32] = [0; 32];

pub(crate) async fn spawn_resubmit_task(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    jobs_tx: mpsc::Sender<UploadJob>,
    client: Arc<aws_sdk_s3::Client>,
    is_ready: Arc<AtomicBool>,
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
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
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
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
                    UploadJob::Normal(item) => {
                        item.enqueue_upload_task(&mut trx).await?;
                        item
                    }
                    UploadJob::DatabaseLock(mut item) => {
                        let row = match sqlx::query!(
                            "SELECT ciphertext, ciphertext128, ciphertext128_format, s3_format_version
                             FROM ciphertext_digest
                             WHERE handle = $1 AND
                             (ciphertext128 IS NULL OR ciphertext IS NULL)
                             FOR UPDATE SKIP LOCKED",
                            item.handle,
                        )
                        .fetch_one(trx.as_mut())
                        .await
                        {
                            Ok(row) => row,
                            Err(err) => {
                                warn!(
                                    error = %err,
                                    handle = to_hex(&item.handle),
                                    "Failed to lock pending uploads",
                                );
                                trx.rollback().await?;
                                continue;
                            }
                        };

                        let s3_format_version = row.s3_format_version;
                        let should_verify_existing_s3 =
                            s3_format_version != Some(CURRENT_S3_FORMAT_VERSION);

                        // A non-null digest means another worker already uploaded that
                        // ciphertext variant. For pre-v1 rows, keep recovered bytes so
                        // this retry can validate and, if needed, rewrite old S3 objects.
                        if row.ciphertext.is_some() && !should_verify_existing_s3 {
                            item.ct64_compressed = Arc::new(Vec::new());
                        }

                        let ct128_format = Ciphertext128Format::from_i16(row.ciphertext128_format)
                            .ok_or_else(|| {
                                ExecutionError::InvalidCiphertext128Format(format!(
                                    "pending ct128 has invalid format id, host_chain_id: {}, handle: {}, format_id: {}",
                                    item.host_chain_id.as_i64(),
                                    to_hex(&item.handle),
                                    row.ciphertext128_format,
                                ))
                            })?;

                        if row.ciphertext128.is_some()
                            && (!should_verify_existing_s3 || item.ct128.is_empty())
                        {
                            // ct128 already uploaded; only ct64 may need retrying.
                            item.ct128 = Arc::new(BigCiphertext::new(Vec::new(), ct128_format));
                        } else if !item.ct128.is_empty() {
                            // Still need to upload ct128 bytes; reconstruct with DB format as truth.
                            item.ct128 = Arc::new(BigCiphertext::new(
                                item.ct128.bytes().to_vec(),
                                ct128_format,
                            ));
                        }

                        item
                    }
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
                let permit = semaphore.clone().acquire_owned().await.map_err(|err| ExecutionError::InternalError(err.to_string()))?;
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
                    let result = upload_ciphertexts(trx, item, &client, &conf, signer)
                        .instrument(upload_span.clone())
                        .await;
                    let outcome = match result {
                        Ok(()) => {
                            AWS_UPLOAD_SUCCESS_COUNTER.inc();
                            Ok(())
                        }
                        Err(err) => {
                            if err
                                .downcast_ref::<ExecutionError>()
                                .is_some_and(|err| matches!(err, ExecutionError::S3TransientError(_)))
                            {
                                ready_flag.store(false, Ordering::Release);
                                info!(error = %err, "S3 setup is not ready, due to transient error");
                            } else {
                                error!(error = %err, "Failed to upload ciphertexts");
                            }
                            upload_span
                                .context()
                                .span()
                                .set_status(Status::error(err.to_string()));
                            AWS_UPLOAD_FAILURE_COUNTER.inc();
                            // Only a lost connection is fatal; retry the rest.
                            match &err {
                                ExecutionError::DbError(e) if is_fatal_connection_error(e) => {
                                    Err(err)
                                }
                                _ => Ok(()),
                            }
                        }
                    };
                    drop(upload_span);
                    drop(permit);
                    outcome
                });

                ongoing_upload_tasks.push(h);
            },
            // Drain finished uploads; exit if one lost the connection.
            Some(joined) = ongoing_upload_tasks.join_next() => {
                if let Ok(Err(err)) = joined {
                    return Err(err);
                }
            }
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

#[derive(Clone)]
struct S3ObjectMetadata {
    attestation_json: String,
    key_id: String,
    transaction_id: String,
    signer: String,
}

struct UploadMaterial {
    ct64_digest: Vec<u8>,
    ct128_digest: Vec<u8>, // 0 if no ct128
}

fn build_attestation_payload(
    task: &HandleItem,
    context_id: U256,
    ct64_digest: &[u8],
    ct128_digest: &[u8],
    format: CiphertextFormat,
) -> anyhow::Result<CiphertextAttestationPayload> {
    Ok(CiphertextAttestationPayload::new(
        Version::V1,
        b256_from_bytes("handle", &task.handle)?,
        u256_from_bytes("key_id_gw", &task.key_id_gw)?,
        context_id,
        b256_from_bytes("ciphertext digest", ct64_digest)?,
        b256_from_bytes("sns ciphertext digest", ct128_digest)?,
        format,
    ))
}

fn validate_existing_attestation(
    expected: &CiphertextAttestationPayload,
    actual: &CiphertextAttestation,
) -> Result<(), String> {
    if actual.version != expected.version {
        return Err(format!(
            "version mismatch: expected {:?}, got {:?}",
            expected.version, actual.version
        ));
    }
    if actual.key_id != expected.key_id {
        return Err(format!(
            "key_id mismatch: expected {}, got {}",
            expected.key_id, actual.key_id
        ));
    }
    if actual.ciphertext_digest != expected.ciphertext_digest {
        return Err(format!(
            "ciphertext digest mismatch: expected {}, got {}",
            expected.ciphertext_digest, actual.ciphertext_digest
        ));
    }
    if actual.sns_ciphertext_digest != expected.sns_ciphertext_digest {
        return Err(format!(
            "sns ciphertext digest mismatch: expected {}, got {}",
            expected.sns_ciphertext_digest, actual.sns_ciphertext_digest
        ));
    }
    if actual.format != expected.format {
        return Err(format!(
            "format mismatch: expected {:?}, got {:?}",
            expected.format, actual.format
        ));
    }

    actual
        .verify(expected.handle, expected.coprocessor_context_id)
        .map_err(|err| format!("handle/context/signature mismatch: {err}"))?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn upload_ct(
    span: Span,
    client: Client,
    bucket: String,
    key: String,
    ct_format: String,
    metadata: S3ObjectMetadata,
    ct_bytes: Vec<u8>,
    extra_digest_key: Option<String>,
    result: UploadResult,
) -> anyhow::Result<UploadResult> {
    let upload = client
        .put_object()
        .bucket(bucket.clone())
        .metadata("Ct-Format", ct_format)
        .metadata("Uploaded-By", "sns-worker")
        .metadata(S3_METADATA_ATTESTATION_KEY, &metadata.attestation_json)
        .metadata("Key-Id", metadata.key_id)
        .metadata("Transaction-Id", metadata.transaction_id)
        .metadata("Signer", metadata.signer)
        .key(&key)
        .body(ByteStream::from(ct_bytes));
    let upload = upload.send().await;
    if let Err(err) = upload {
        error!(error = %err, bucket, key, metadata.attestation_json, "Failed to upload ct");
        span.set_status(Status::error(err.to_string()));
        return Err(ExecutionError::S3TransientError(err.to_string()).into());
    }
    if let Some(extra_digest_key) = extra_digest_key {
        let copy_source = format!("{}/{}", bucket, key);
        let upload_backward_compatible = client
            .copy_object()
            .copy_source(copy_source)
            .bucket(bucket.clone())
            .key(&extra_digest_key)
            .send()
            .await;
        if let Err(err) = upload_backward_compatible {
            error!(error = %err, bucket, extra_digest_key, metadata.attestation_json, "Failed to upload ct for backcompatibility");
            span.set_status(Status::error(err.to_string()));
            return Err(ExecutionError::S3TransientError(err.to_string()).into());
        }
    }
    Ok(result)
}

pub(crate) fn s3_ciphertext_key(handle: &[u8], context_id: U256) -> String {
    hex::encode(handle) + "/" + &context_id.to_string()
}

fn b256_from_bytes(field: &str, bytes: &[u8]) -> anyhow::Result<B256> {
    let bytes: [u8; 32] = bytes.try_into().map_err(|_| {
        anyhow::anyhow!(
            "{} must be 32 bytes for ciphertext attestation, got {}",
            field,
            bytes.len()
        )
    })?;
    Ok(B256::from(bytes))
}

fn u256_from_bytes(field: &str, bytes: &[u8]) -> anyhow::Result<U256> {
    if bytes.len() > 32 {
        anyhow::bail!(
            "{} must be at most 32 bytes for ciphertext attestation, got {}",
            field,
            bytes.len()
        );
    }
    Ok(U256::from_be_slice(bytes))
}

fn attestation_format(format: Ciphertext128Format) -> anyhow::Result<CiphertextFormat> {
    match format {
        Ciphertext128Format::UncompressedOnCpu => Ok(CiphertextFormat::UncompressedOnCpu),
        Ciphertext128Format::CompressedOnCpu => Ok(CiphertextFormat::CompressedOnCpu),
        Ciphertext128Format::UncompressedOnGpu => Ok(CiphertextFormat::UncompressedOnGpu),
        Ciphertext128Format::CompressedOnGpu => Ok(CiphertextFormat::CompressedOnGpu),
        Ciphertext128Format::Unknown => {
            anyhow::bail!("Cannot build ciphertext attestation with unknown ct128 format")
        }
    }
}

fn upload_material(task: &HandleItem) -> anyhow::Result<UploadMaterial> {
    if task.ct64_compressed.is_empty() && task.ct128.is_empty() {
        let err = anyhow::anyhow!("Invalid upload task without ciphertext bytes: {:?}", task);
        error!(?task, error = %err, "Upload task has no ciphertext bytes");
        return Err(err);
    }

    if task.ct64_compressed.is_empty() && task.ct64_digest.is_none() {
        let err = anyhow::anyhow!(
            "Invalid upload task for handle {}: missing ct64 digest and ciphertext bytes",
            to_hex(&task.handle),
        );
        error!(?task, error = %err, "Upload task has no ct64 material");
        return Err(err);
    }

    let ct64_digest = if let Some(digest) = &task.ct64_digest {
        digest.clone()
    } else {
        compute_digest(task.ct64_compressed.as_ref())
    };

    let ct128_digest = if let Some(digest) = &task.ct128_digest {
        digest.clone()
    } else if !task.ct128.is_empty() {
        compute_digest(task.ct128.bytes())
    } else {
        NO_SNS_CIPHERTEXT_DIGEST.to_vec()
    };

    Ok(UploadMaterial {
        ct64_digest,
        ct128_digest,
    })
}

async fn build_attestation(
    payload: &CiphertextAttestationPayload,
    signer: &CoproSigner,
) -> anyhow::Result<CiphertextAttestation> {
    let signature = signer.sign_hash(&payload.canonical_digest()).await?;

    Ok(CiphertextAttestation {
        version: payload.version,
        key_id: payload.key_id,
        ciphertext_digest: payload.ciphertext_digest,
        sns_ciphertext_digest: payload.sns_ciphertext_digest,
        format: payload.format,
        signer: signer.address(),
        signature: signature.as_bytes().to_vec(),
    })
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
    let context_id = COPROCESSOR_CONTEXT_ID_1;
    let handle_as_hex: String = to_hex(&task.handle);
    info!(handle = handle_as_hex, "Received task");

    let mut jobs = vec![];

    let upload_material = upload_material(&task)?;
    let ct128_digest = &upload_material.ct128_digest;

    // The ct128 format is only required for the attestation when we actually have
    // a real (non-zero) ct128 digest. For pure-ct64 handles or partial recovery
    // (ct128 already succeeded, only retrying ct64), the format is not essential.
    let attestation_format = if *ct128_digest == NO_SNS_CIPHERTEXT_DIGEST.to_vec() {
        CiphertextFormat::UncompressedOnCpu
    } else {
        attestation_format(task.ct128.format())?
    };

    let expected_attestation = build_attestation_payload(
        &task,
        context_id,
        &upload_material.ct64_digest,
        ct128_digest,
        attestation_format,
    )?;
    let attestation = build_attestation(&expected_attestation, &signer).await?;
    let attestation_json = serde_json::to_string(&attestation)?;

    let s3_metadata = S3ObjectMetadata {
        attestation_json,
        key_id: hex::encode(&task.key_id_gw),
        transaction_id: hex::encode(task.transaction_id.as_deref().unwrap_or_default()),
        signer: signer.address().to_string(),
    };

    if *ct128_digest != NO_SNS_CIPHERTEXT_DIGEST.to_vec() {
        let key = s3_ciphertext_key(&task.handle, context_id);
        let digest_key = hex::encode(ct128_digest);

        let ct128_check_span = tracing::info_span!(
            "ct128_check_s3",
            ct_type = "ct128",
            exists = tracing::field::Empty,
        );
        let exists = object_check_result(
            &ct128_check_span,
            check_ct128_objects_exist(
                client,
                &conf.bucket_ct128,
                &key,
                &digest_key,
                &expected_attestation,
            )
            .instrument(ct128_check_span.clone())
            .await,
        )?;
        ct128_check_span.record("exists", tracing::field::display(exists));
        drop(ct128_check_span);

        if !exists {
            if task.ct128.is_empty() {
                anyhow::bail!(
                    "ct128 S3 object needs upload for handle {}, but ct128 bytes are unavailable",
                    handle_as_hex,
                );
            }

            let ct128_format = task.ct128.format();
            let ct128_bytes = task.ct128.bytes();
            info!(
                handle = handle_as_hex,
                len = ?ByteSize::b(ct128_bytes.len() as u64),
                format = %ct128_format,
                host_chain_id = task.host_chain_id.as_i64(),
                "Uploading ct128"
            );

            let bucket = &conf.bucket_ct128;
            let ct_format = ct128_format.to_string();
            let ct128_upload_span = tracing::info_span!(
                "ct128_upload_s3",
                ct_type = "ct128",
                format = %ct_format,
                len = ct128_bytes.len(),
            );
            let result = UploadResult::CtType128(ct128_digest.clone());
            let upload_ct = upload_ct(
                ct128_upload_span.clone(),
                client.clone(),
                bucket.clone(),
                key,
                ct_format,
                s3_metadata.clone(),
                ct128_bytes.to_vec(),
                Some(digest_key),
                result,
            );
            let grouped_upload = tokio::spawn(
                async move {
                    let result = upload_ct.await?;
                    anyhow::Ok(result)
                }
                .instrument(ct128_upload_span),
            );
            jobs.push(grouped_upload);
        } else {
            info!(
                handle = handle_as_hex,
                ct128_digest = hex::encode(ct128_digest),
                "ct128 already exists in S3",
            );

            // In case of a sns-worker failure after uploading to S3,
            // the state between both storages may become inconsistent
            task.update_ct128_uploaded(&mut trx, ct128_digest.clone())
                .await?;
        }
    }

    {
        let ct64_compressed = task.ct64_compressed.as_ref();
        let ct64_digest = &upload_material.ct64_digest;
        let key = s3_ciphertext_key(&task.handle, context_id);

        let ct64_check_span = tracing::info_span!(
            "ct64_check_s3",
            ct_type = "ct64",
            exists = tracing::field::Empty,
        );
        let exists = object_check_result(
            &ct64_check_span,
            check_attested_object_exists(client, &conf.bucket_ct64, &key, &expected_attestation)
                .instrument(ct64_check_span.clone())
                .await,
        )?;
        ct64_check_span.record("exists", tracing::field::display(exists));
        drop(ct64_check_span);

        if !exists {
            if ct64_compressed.is_empty() {
                anyhow::bail!(
                    "ct64 S3 object needs upload for handle {}, but ct64 bytes are unavailable",
                    handle_as_hex,
                );
            }

            info!(
                handle = handle_as_hex,
                len = ?ByteSize::b(ct64_compressed.len() as u64),
                "Uploading ct64",
            );

            let bucket = &conf.bucket_ct64;
            let ct64_upload_span = tracing::info_span!(
                "ct64_upload_s3",
                ct_type = "ct64",
                len = ct64_compressed.len(),
            );
            let result = UploadResult::CtType64(ct64_digest.clone());
            let upload_ct = upload_ct(
                ct64_upload_span.clone(),
                client.clone(),
                bucket.clone(),
                key,
                "ct64_compressed".to_string(),
                s3_metadata.clone(),
                ct64_compressed.clone(),
                None,
                result,
            );
            let grouped_upload = tokio::spawn(
                async move {
                    let result = upload_ct.await?;
                    Ok(result)
                }
                .instrument(ct64_upload_span),
            );
            jobs.push(grouped_upload);
        } else {
            info!(
                handle = handle_as_hex,
                ct64_digest = hex::encode(ct64_digest),
                "ct64 already exists in S3",
            );

            // In case of a sns-worker failure after uploading to S3,
            // the state between both storages may become inconsistent
            task.update_ct64_uploaded(&mut trx, ct64_digest.clone())
                .await?;
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
        "SELECT handle, ciphertext, ciphertext128, ciphertext128_format, s3_format_version,
            transaction_id, host_chain_id, key_id_gw
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
        let s3_format_version = row.s3_format_version;
        let should_verify_existing_s3 = s3_format_version != Some(CURRENT_S3_FORMAT_VERSION);
        let handle = row.handle;
        let transaction_id = row.transaction_id;

        // Fetch missing ciphertext, and also fetch an already-uploaded ciphertext
        // for pre-v1 rows so the retry can validate/rewrite its old S3 object.
        if ciphertext_digest.is_none() || (should_verify_existing_s3 && ciphertext_digest.is_some())
        {
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
            } else {
                error!(handle = hex::encode(&handle), "Failed to fetch ciphertext");
            }
        }

        // Fetch missing ciphertext128, and also fetch an already-uploaded
        // ciphertext128 for pre-v1 rows so the retry can validate/rewrite its
        // old S3 object.
        if ciphertext128_digest.is_none()
            || (should_verify_existing_s3 && ciphertext128_digest.is_some())
        {
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
            } else {
                error!(
                    handle = hex::encode(&handle),
                    "Failed to fetch ciphertext128"
                );
            }
        }

        let is_ct128_empty = ct128.is_empty();

        let ct128_format = match Ciphertext128Format::from_i16(row.ciphertext128_format) {
            Some(format) => format,
            None => {
                error!(
                    handle = to_hex(&handle),
                    format_id = row.ciphertext128_format,
                    "Failed to create a BigCiphertext from DB data",
                );
                continue;
            }
        };

        let ct128 = if !is_ct128_empty {
            let ct = BigCiphertext::new(ct128, ct128_format);
            info!(
                handle = to_hex(&handle),
                format = %ct.format(),
                "Recovered pending ct128 upload from DB"
            );
            ct
        } else {
            // Already uploaded; retain the stored format for attestation.
            BigCiphertext::new(Vec::new(), ct128_format)
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
                ct64_digest: ciphertext_digest,
                ct128_digest: ciphertext128_digest,
                span: recovery_span,
                transaction_id,
            };

            // Instruct the uploader to acquire DB lock when processing the item
            jobs.push(UploadJob::DatabaseLock(item));
        } else if ciphertext_digest.is_some() {
            debug!(
                handle = hex::encode(&handle),
                "No ciphertext material to resubmit"
            );
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

async fn check_attested_object_exists(
    client: &Client,
    bucket: &str,
    key: &str,
    expected: &CiphertextAttestationPayload,
) -> Result<bool, ExecutionError> {
    match client.head_object().bucket(bucket).key(key).send().await {
        Ok(output) => {
            let Some(attestation_json) = output
                .metadata()
                .and_then(|metadata| metadata.get(S3_METADATA_ATTESTATION_KEY))
            else {
                warn!(
                    bucket,
                    key, "S3 object exists without ciphertext attestation metadata, reuploading"
                );
                return Ok(false);
            };

            let attestation = match serde_json::from_str::<CiphertextAttestation>(attestation_json)
            {
                Ok(attestation) => attestation,
                Err(err) => {
                    warn!(
                        bucket,
                        key,
                        error = %err,
                        "S3 object has invalid ciphertext attestation metadata, reuploading"
                    );
                    return Ok(false);
                }
            };

            if let Err(reason) = validate_existing_attestation(expected, &attestation) {
                warn!(
                    bucket,
                    key,
                    reason = %reason,
                    "S3 object has stale ciphertext attestation metadata, reuploading"
                );
                return Ok(false);
            }

            Ok(true)
        }
        Err(SdkError::ServiceError(err)) if matches!(err.err(), HeadObjectError::NotFound(_)) => {
            Ok(false)
        }
        Err(err) => {
            error!(error = %err, "Failed to check object existence");
            Err(ExecutionError::S3TransientError(err.to_string()))
        }
    }
}

fn object_check_result(
    span: &Span,
    result: Result<bool, ExecutionError>,
) -> Result<bool, ExecutionError> {
    match result {
        Ok(exists) => Ok(exists),
        Err(err) => {
            span.context()
                .span()
                .set_status(Status::error(err.to_string()));
            Err(err)
        }
    }
}

async fn check_ct128_objects_exist(
    client: &Client,
    bucket: &str,
    key: &str,
    digest_key: &str,
    expected: &CiphertextAttestationPayload,
) -> Result<bool, ExecutionError> {
    let key_exists = check_attested_object_exists(client, bucket, key, expected).await?;
    let digest_key_exists =
        check_attested_object_exists(client, bucket, digest_key, expected).await?;

    if key_exists && !digest_key_exists {
        warn!(
            bucket,
            key,
            digest_key,
            "ct128 object exists without digest-key compatibility copy, reuploading"
        );
    }

    Ok(key_exists && digest_key_exists)
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::local::PrivateKeySigner;

    fn sample_handle_item() -> HandleItem {
        let ct128_format = Ciphertext128Format::CompressedOnCpu;

        HandleItem {
            host_chain_id: ChainId::try_from(1_i64).unwrap(),
            key_id_gw: vec![7; 32],
            handle: vec![2; 32],
            ct64_compressed: Arc::new(vec![1, 2, 3]),
            ct128: Arc::new(BigCiphertext::new(vec![4, 5, 6], ct128_format)),
            ct64_digest: None,
            ct128_digest: None,
            span: Span::none(),
            transaction_id: None,
        }
    }

    async fn sample_attestation() -> (CiphertextAttestationPayload, CiphertextAttestation) {
        let task = sample_handle_item();
        let upload_material = upload_material(&task).unwrap();
        let format = attestation_format(task.ct128.format()).unwrap();
        let expected = build_attestation_payload(
            &task,
            COPROCESSOR_CONTEXT_ID_1,
            &upload_material.ct64_digest,
            &upload_material.ct128_digest,
            format,
        )
        .unwrap();
        let signer: CoproSigner = Arc::new(PrivateKeySigner::random());
        let attestation = build_attestation(&expected, &signer).await.unwrap();

        (expected, attestation)
    }

    #[tokio::test]
    async fn expected_attestation_accepts_current_metadata() {
        let (expected, attestation) = sample_attestation().await;

        validate_existing_attestation(&expected, &attestation).unwrap();
    }

    #[tokio::test]
    async fn expected_attestation_rejects_stale_digest_metadata() {
        let (expected, mut attestation) = sample_attestation().await;
        attestation.sns_ciphertext_digest = B256::ZERO;

        let err = validate_existing_attestation(&expected, &attestation).unwrap_err();
        assert!(err.contains("sns ciphertext digest mismatch"));
    }

    #[tokio::test]
    async fn expected_attestation_rejects_wrong_context_metadata() {
        let (mut expected, attestation) = sample_attestation().await;
        expected.coprocessor_context_id = U256::ZERO;

        let err = validate_existing_attestation(&expected, &attestation).unwrap_err();
        assert!(err.contains("handle/context/signature mismatch"));
    }

    #[test]
    fn handle_item_debug_redacts_ciphertext_bytes() {
        let task = sample_handle_item();

        let debug = format!("{task:?}");

        assert!(debug.contains("host_chain_id: 1"));
        assert!(debug.contains(
            "handle: \"0x0202020202020202020202020202020202020202020202020202020202020202\""
        ));
        assert!(debug.contains("ct64_compressed_len: 3"));
        assert!(debug.contains("BigCiphertext"));
        assert!(debug.contains("bytes_len: 3"));
        assert!(!debug.contains("ct64_compressed:"));
        assert!(!debug.contains("bytes: ["));
        assert!(!debug.contains("[1, 2, 3]"));
        assert!(!debug.contains("[4, 5, 6]"));
    }

    #[test]
    fn ct64_only_upload_material_uses_zero_sns_ciphertext_digest() {
        let ct64 = vec![1, 2, 3];
        let task = HandleItem {
            host_chain_id: ChainId::try_from(1_i64).unwrap(),
            key_id_gw: vec![1],
            handle: vec![2; 32],
            ct64_compressed: Arc::new(ct64.clone()),
            ct128: Arc::new(BigCiphertext::default()),
            ct64_digest: None,
            ct128_digest: None,
            span: Span::none(),
            transaction_id: None,
        };

        let material = upload_material(&task).unwrap();

        assert_eq!(material.ct64_digest, compute_digest(&ct64));
        assert_eq!(material.ct128_digest, NO_SNS_CIPHERTEXT_DIGEST.to_vec());
    }

    #[test]
    fn partial_upload_recovery_uses_ct128_format_from_db_when_bytes_are_absent() {
        // Regression test for the partial-upload recovery bug.
        //
        // Scenario: ct128 was successfully uploaded earlier, but ct64 is still
        // pending. The recovery path (DatabaseLock) clears the ct128 *bytes*
        // (see BigCiphertext::default()), but must still carry the format that
        // was stored in ciphertext_digest.ciphertext128_format.
        //
        // Before the fix, this would reach `attestation_format(Unknown)` and fail
        // with "Cannot build ciphertext attestation with unknown ct128 format".

        let ct64 = vec![0xAA, 0xBB, 0xCC];
        let real_ct128_digest = vec![0x11; 32];
        let stored_format = Ciphertext128Format::CompressedOnCpu;

        let task = HandleItem {
            host_chain_id: ChainId::try_from(42_i64).unwrap(),
            key_id_gw: vec![7; 32],
            handle: vec![0xDE, 0xAD, 0xBE, 0xEF],
            ct64_compressed: Arc::new(ct64.clone()),
            // This is exactly what the recovery path sets when ciphertext128 IS NOT NULL:
            // empty bytes, but the DB format retained inside BigCiphertext.
            ct128: Arc::new(BigCiphertext::new(Vec::new(), stored_format)),
            ct64_digest: None,
            ct128_digest: Some(real_ct128_digest.clone()),
            span: Span::none(),
            transaction_id: None,
        };

        let material = upload_material(&task).unwrap();
        assert_eq!(material.ct64_digest, compute_digest(&ct64));
        assert_eq!(material.ct128_digest, real_ct128_digest);

        // This is the call that used to blow up. It must succeed because the
        // BigCiphertext created from the DB row retains its stored format.
        let fmt = attestation_format(task.ct128.format())
            .expect("ct128_format from DB must be valid when a real ct128 digest exists");

        assert_eq!(fmt, CiphertextFormat::CompressedOnCpu);
    }

    #[test]
    fn s3_key_v1() {
        let handle = B256::ZERO;
        let coprocessor_context_id = U256::ZERO;
        let s3_key = s3_ciphertext_key(handle.as_ref(), coprocessor_context_id);
        assert_eq!(
            "0000000000000000000000000000000000000000000000000000000000000000/0",
            &s3_key
        );
    }
}
