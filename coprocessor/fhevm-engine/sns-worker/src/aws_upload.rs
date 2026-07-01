use crate::metrics::{AWS_UPLOAD_FAILURE_COUNTER, AWS_UPLOAD_SUCCESS_COUNTER};
use crate::{
    BigCiphertext, Ciphertext128Format, Config, ExecutionError, HandleItem, S3Config, UploadJob,
    CURRENT_S3_FORMAT_VERSION, S3_FORMAT_VERSION_LEGACY,
};
use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_bucket::HeadBucketError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{ChecksumAlgorithm, ChecksumMode};
use aws_sdk_s3::Client;
use base64::Engine;
use bytesize::ByteSize;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextAttestationPayload, CiphertextFormat, Version,
    S3_METADATA_ATTESTATION_KEY,
};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::database::EVENT_CIPHERTEXTS_UPLOADED;
use fhevm_engine_common::pg_pool::{is_fatal_connection_error, PostgresPoolManager, ServiceError};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::CoproSigner;
use fhevm_engine_common::utils::to_hex;
use futures::future::join_all;
use opentelemetry::trace::{Status, TraceContextExt};
use sha2::Sha256;
use sha3::{Digest, Keccak256};
use sqlx::{PgPool, Pool, Postgres, Transaction};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::select;
use tokio::sync::{mpsc, RwLock, Semaphore};
use tokio::task::{JoinError, JoinHandle, JoinSet};
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, error_span, info, warn, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

// TODO: Use a config TOML to set these values
// Default batch size for fetching pending uploads
// There might be pending uploads in the database
// with sizes of 32MiB so the batch size is set to 10
const DEFAULT_BATCH_SIZE: usize = 10;
pub(crate) const COPROCESSOR_CONTEXT_ID_1: U256 = U256::ONE;
const NO_SNS_CIPHERTEXT_DIGEST: [u8; 32] = [0; 32];
const UPLOAD_CIPHERTEXTS_TASK_SAVEPOINT: &str = "savepoint_upload_ciphertexts_task";

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
    let mut ongoing_upload_tasks: JoinSet<anyhow::Result<()>> = JoinSet::new();
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
                        create_upload_task_savepoint(&mut trx).await?;
                        item
                    }
                    UploadJob::DatabaseLock(mut item) => {
                        create_upload_task_savepoint(&mut trx).await?;
                        let row = match sqlx::query!(
                            "
                            SELECT d.ciphertext128_format,
                                   d.s3_format_version
                            FROM ciphertext_digest d
                            WHERE d.handle = $1
                              AND (
                                d.ciphertext IS NULL
                                OR (
                                  d.ciphertext128 IS NULL
                                  AND EXISTS (
                                    SELECT 1
                                    FROM ciphertexts128 c
                                    WHERE c.handle = d.handle
                                      AND c.ciphertext IS NOT NULL
                                  )
                                )
                            )
                            FOR UPDATE SKIP LOCKED
                            ",
                            &item.handle,
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

                        let ciphertext128_format = row.ciphertext128_format;
                        item.s3_format_version = row.s3_format_version;
                        let ct128_format = Ciphertext128Format::from_i16(ciphertext128_format)
                            .ok_or_else(|| {
                                ExecutionError::InvalidCiphertext128Format(format!(
                                    "pending ct128 has invalid format id, host_chain_id: {}, handle: {}, format_id: {}",
                                    item.host_chain_id.as_i64(),
                                    to_hex(&item.handle),
                                    ciphertext128_format,
                                ))
                            })?;

                        if !item.ct128.is_empty() {
                            // Reconstruct with DB format as truth. Even if a digest is already
                            // present, incomplete rows retry the whole handle before committing.
                            item.ct128 = Arc::new(BigCiphertext::new(
                                item.ct128.bytes().to_vec(),
                                ct128_format,
                            ));
                        } else {
                            item.ct128 = Arc::new(BigCiphertext::new(Vec::new(), ct128_format));
                        }

                        item
                    }
                };
                debug!(handle = hex::encode(&item.handle), "Received task, handle");

                // Cleanup completed tasks
                while let Some(joined) = ongoing_upload_tasks.try_join_next() {
                    propagate_joined_upload_error(joined)?;
                }
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
                ongoing_upload_tasks.spawn(async move {
                    // Cross-boundary: spawned task; restore the OTel context
                    // that was captured when the upload item was created.
                    let upload_span = error_span!("upload_s3");
                    upload_span.set_parent(item.span.context());
                    let result = upload_ciphertexts(&mut trx, item, &client, &conf, signer)
                        .instrument(upload_span.clone())
                        .await;
                    let outcome = match result {
                        Ok(()) => {
                            if let Err(err) = trx.commit().await {
                                let err = anyhow::Error::from(err);
                                let is_fatal_db_error = is_fatal_upload_db_error(&err);
                                error!(
                                    error = %err,
                                    "Failed to commit uploaded ciphertexts"
                                );
                                upload_span
                                    .context()
                                    .span()
                                    .set_status(Status::error(err.to_string()));
                                AWS_UPLOAD_FAILURE_COUNTER.inc();

                                if is_fatal_db_error {
                                    Err(err)
                                } else {
                                    Ok(())
                                }
                            } else {
                                AWS_UPLOAD_SUCCESS_COUNTER.inc();
                                Ok(())
                            }
                        }
                        Err(err) => {
                            let is_s3_transient_error =
                                err.downcast_ref::<ExecutionError>().is_some_and(|err| {
                                    matches!(err, ExecutionError::S3TransientError(_))
                                });
                            let is_fatal_db_error = is_fatal_upload_db_error(&err);

                            preserve_upload_task_for_retry(trx, &err).await;

                            if is_s3_transient_error {
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
                            if is_fatal_db_error {
                                Err(err)
                            } else {
                                Ok(())
                            }
                        }
                    };
                    drop(upload_span);
                    drop(permit);
                    outcome
                });
            },
            // Drain finished uploads; exit if one lost the connection.
            Some(joined) = ongoing_upload_tasks.join_next() => {
                propagate_joined_upload_error(joined)?;
            }
            _ = token.cancelled() => {
                info!("Waiting for all uploads to finish...");

                while let Some(joined) = ongoing_upload_tasks.join_next().await {
                    if let Err(err) = propagate_joined_upload_error(joined) {
                        error!(error = %err, "Upload task failed while shutting down");
                    }
                }

                return Ok(())
            }
        }
    }
}

fn is_fatal_upload_db_error(err: &anyhow::Error) -> bool {
    err.downcast_ref::<ExecutionError>().is_some_and(
        |err| matches!(err, ExecutionError::DbError(db_err) if is_fatal_connection_error(db_err)),
    ) || err
        .downcast_ref::<sqlx::Error>()
        .is_some_and(is_fatal_connection_error)
}

fn propagate_joined_upload_error(
    joined: Result<anyhow::Result<()>, JoinError>,
) -> Result<(), ExecutionError> {
    match joined {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(upload_task_error_into_execution_error(err)),
        Err(err) => {
            error!(error = %err, "Failed to join upload task");
            Ok(())
        }
    }
}

fn upload_task_error_into_execution_error(err: anyhow::Error) -> ExecutionError {
    match err.downcast::<ExecutionError>() {
        Ok(err) => err,
        Err(err) => match err.downcast::<sqlx::Error>() {
            Ok(err) => ExecutionError::DbError(err),
            Err(err) => ExecutionError::InternalError(err.to_string()),
        },
    }
}

async fn create_upload_task_savepoint(
    trx: &mut Transaction<'_, Postgres>,
) -> Result<(), ExecutionError> {
    sqlx::query(&format!("SAVEPOINT {UPLOAD_CIPHERTEXTS_TASK_SAVEPOINT}"))
        .execute(trx.as_mut())
        .await?;

    Ok(())
}

async fn rollback_to_upload_task_savepoint(
    trx: &mut Transaction<'_, Postgres>,
) -> Result<(), ExecutionError> {
    sqlx::query(&format!(
        "ROLLBACK TO SAVEPOINT {UPLOAD_CIPHERTEXTS_TASK_SAVEPOINT}"
    ))
    .execute(trx.as_mut())
    .await?;

    Ok(())
}

async fn preserve_upload_task_for_retry(
    mut trx: Transaction<'_, Postgres>,
    upload_error: &anyhow::Error,
) {
    if let Err(preserve_err) = async {
        rollback_to_upload_task_savepoint(&mut trx).await?;
        trx.commit().await?;
        anyhow::Ok(())
    }
    .await
    {
        error!(
            error = %preserve_err,
            upload_error = %upload_error,
            "Failed to preserve incomplete upload state",
        );
    }
}

enum UploadResult {
    CtType128,
    CtType64,
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
    expected_signer: Address,
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
    if actual.signer != expected_signer {
        return Err(format!(
            "signer mismatch: expected {}, got {}",
            expected_signer, actual.signer
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
    verify_sha256_checksum: bool,
) -> anyhow::Result<UploadResult> {
    let checksum_sha256 = verify_sha256_checksum.then(|| sha256_checksum_header(&ct_bytes));
    let mut upload = client
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
    if let Some(checksum_sha256) = checksum_sha256 {
        upload = upload.checksum_sha256(checksum_sha256);
    }
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
            .set_checksum_algorithm(verify_sha256_checksum.then_some(ChecksumAlgorithm::Sha256))
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

fn should_preserve_legacy_s3_format(task: &HandleItem, ct128_digest: &[u8]) -> bool {
    let has_legacy_s3_format = task.s3_format_version != Some(CURRENT_S3_FORMAT_VERSION);
    let has_existing_ct128_digest =
        task.ct128_digest.is_some() && ct128_digest != NO_SNS_CIPHERTEXT_DIGEST.as_slice();

    has_legacy_s3_format && has_existing_ct128_digest && task.ct128.is_empty()
}

fn completed_s3_format_version(task: &HandleItem, preserve_legacy_s3_format: bool) -> i16 {
    if preserve_legacy_s3_format {
        task.s3_format_version.unwrap_or(S3_FORMAT_VERSION_LEGACY)
    } else {
        CURRENT_S3_FORMAT_VERSION
    }
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
/// buckets. If both uploads succeed, it stores their digests in the database.
///
/// Guarantees:
/// - If any required upload or verification fails, the function will not mark
///   either ciphertext as uploaded in the database.
async fn upload_ciphertexts(
    trx: &mut Transaction<'_, Postgres>,
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
    let preserve_legacy_s3_format = should_preserve_legacy_s3_format(&task, ct128_digest);

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
    let expected_signer = signer.address();
    let attestation = build_attestation(&expected_attestation, &signer).await?;
    let attestation_json = serde_json::to_string(&attestation)?;

    let s3_metadata = S3ObjectMetadata {
        attestation_json,
        key_id: hex::encode(&task.key_id_gw),
        transaction_id: hex::encode(task.transaction_id.as_deref().unwrap_or_default()),
        signer: expected_signer.to_string(),
    };

    if *ct128_digest != NO_SNS_CIPHERTEXT_DIGEST.to_vec() {
        let key = s3_ciphertext_key(&task.handle, context_id);
        let digest_key = hex::encode(ct128_digest);

        if preserve_legacy_s3_format {
            info!(
                handle = handle_as_hex,
                ct128_digest = hex::encode(ct128_digest),
                s3_format_version = task.s3_format_version,
                "Preserving legacy ct128 S3 format for later migration"
            );
        } else {
            let ct128_checksum_sha256 = conf
                .verify_sha256_checksum
                .then(|| match task.ct128.is_empty() {
                    true => None,
                    false => Some(sha256_checksum_header(task.ct128.bytes())),
                })
                .flatten();

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
                    expected_signer,
                    ct128_checksum_sha256.as_deref(),
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
                let result = UploadResult::CtType128;
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
                    conf.verify_sha256_checksum,
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
            }
        }
    }

    {
        let ct64_compressed = task.ct64_compressed.as_ref();
        let ct64_digest = &upload_material.ct64_digest;
        let key = s3_ciphertext_key(&task.handle, context_id);
        let ct64_checksum_sha256 = conf
            .verify_sha256_checksum
            .then(|| match ct64_compressed.is_empty() {
                true => None,
                false => Some(sha256_checksum_header(ct64_compressed)),
            })
            .flatten();

        let ct64_check_span = tracing::info_span!(
            "ct64_check_s3",
            ct_type = "ct64",
            exists = tracing::field::Empty,
        );
        let exists = object_check_result(
            &ct64_check_span,
            check_attested_object_exists(
                client,
                &conf.bucket_ct64,
                &key,
                &expected_attestation,
                expected_signer,
                ct64_checksum_sha256.as_deref(),
            )
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
            let result = UploadResult::CtType64;
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
                conf.verify_sha256_checksum,
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
        }
    }

    let mut transient_error = anyhow::Ok(());

    // Wait all uploads results
    for result in join_all(jobs).await {
        match result {
            Ok(Ok(UploadResult::CtType128)) | Ok(Ok(UploadResult::CtType64)) => {}
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

    transient_error?;

    task.mark_ciphertexts_uploaded(
        trx,
        upload_material.ct64_digest,
        upload_material.ct128_digest,
        completed_s3_format_version(&task, preserve_legacy_s3_format),
    )
    .await?;

    sqlx::query("SELECT pg_notify($1, '')")
        .bind(EVENT_CIPHERTEXTS_UPLOADED)
        .execute(trx.as_mut())
        .await?;

    Ok(())
}

pub fn compute_digest(ct: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    hasher.finalize().to_vec()
}

fn sha256_checksum_header(ct: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(Sha256::digest(ct))
}

/// Fetches incomplete upload tasks from the database.
///
/// An incomplete upload task is defined as a task missing its ct64 digest, or
/// missing its ct128 digest while ct128 ciphertext material exists.
async fn fetch_pending_uploads(
    db_pool: &Pool<Postgres>,
    limit: i64,
) -> Result<Vec<UploadJob>, ExecutionError> {
    let rows = sqlx::query!(
        "
        SELECT d.handle,
               d.ciphertext,
               d.ciphertext128,
               d.ciphertext128_format,
               d.s3_format_version,
               d.transaction_id,
               d.host_chain_id,
               d.key_id_gw,
               EXISTS (
                 SELECT 1
                 FROM ciphertexts128 c
                 WHERE c.handle = d.handle
                   AND c.ciphertext IS NOT NULL
               ) AS \"has_ct128_ciphertext!\"
        FROM ciphertext_digest d
        WHERE d.ciphertext IS NULL
           OR (
             d.ciphertext128 IS NULL
             AND EXISTS (
               SELECT 1
               FROM ciphertexts128 c
               WHERE c.handle = d.handle
                 AND c.ciphertext IS NOT NULL
             )
           )
        FOR UPDATE SKIP LOCKED
        LIMIT $1;
        ",
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
        let has_ct128_ciphertext = row.has_ct128_ciphertext;
        let row_incomplete =
            ciphertext_digest.is_none() || (has_ct128_ciphertext && ciphertext128_digest.is_none());

        // Fetch the ciphertext whenever the row is not fully committed. This
        // lets recovery revalidate both S3 objects before the single DB update.
        // Also fetch already-uploaded ciphertext for pre-v1 rows so the retry
        // can validate/rewrite old S3 objects.
        if row_incomplete || should_verify_existing_s3 {
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

        // Fetch ciphertext128 under the same rule: incomplete rows are retried
        // as a whole handle, and pre-v1 rows need the bytes for validation.
        // Ct64-only rows have no ciphertext128 material, so they are completed
        // by writing the zero ct128 digest after ct64 is verified/uploaded.
        if has_ct128_ciphertext && (row_incomplete || should_verify_existing_s3) {
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
                s3_format_version,
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
                    let (is_ready_res, _) = check_is_ready(&client, &conf.s3).await;
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
pub(crate) async fn check_is_ready(client: &Client, conf: &S3Config) -> (bool, bool) {
    // Check if the S3 client is ready
    //
    // By checking the existence of both ct64 and ct128 buckets here,
    // we also incorporate the aws-sdk connection retry
    let (ct64_exists, _) = check_bucket_exists(client, &conf.bucket_ct64).await;
    let (ct128_exists, conn) = check_bucket_exists(client, &conf.bucket_ct128).await;

    ((ct64_exists && ct128_exists), conn)
}

async fn check_attested_object_exists(
    client: &Client,
    bucket: &str,
    key: &str,
    expected: &CiphertextAttestationPayload,
    expected_signer: Address,
    expected_checksum_sha256: Option<&str>,
) -> Result<bool, ExecutionError> {
    let mut head_object = client.head_object().bucket(bucket).key(key);
    if expected_checksum_sha256.is_some() {
        head_object = head_object.checksum_mode(ChecksumMode::Enabled);
    }

    match head_object.send().await {
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

            if let Err(reason) =
                validate_existing_attestation(expected, expected_signer, &attestation)
            {
                warn!(
                    bucket,
                    key,
                    reason = %reason,
                    "S3 object has stale ciphertext attestation metadata, reuploading"
                );
                return Ok(false);
            }

            if let Some(expected_checksum_sha256) = expected_checksum_sha256 {
                match output.checksum_sha256() {
                    Some(actual_checksum_sha256)
                        if actual_checksum_sha256 == expected_checksum_sha256 => {}
                    Some(actual_checksum_sha256) => {
                        error!(
                            bucket,
                            key,
                            expected_checksum_sha256,
                            actual_checksum_sha256,
                            "S3 object has stale SHA256 checksum, reuploading"
                        );
                        return Ok(false);
                    }
                    None => {
                        warn!(
                            bucket,
                            key, "S3 object exists without SHA256 checksum, reuploading"
                        );
                        return Ok(false);
                    }
                }
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
    expected_signer: Address,
    expected_checksum_sha256: Option<&str>,
) -> Result<bool, ExecutionError> {
    let key_exists = check_attested_object_exists(
        client,
        bucket,
        key,
        expected,
        expected_signer,
        expected_checksum_sha256,
    )
    .await?;
    let digest_key_exists = check_attested_object_exists(
        client,
        bucket,
        digest_key,
        expected,
        expected_signer,
        expected_checksum_sha256,
    )
    .await?;

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

pub async fn check_bucket_exists(
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
    use crate::S3RetryPolicy;
    use alloy::signers::local::PrivateKeySigner;
    use aws_config::BehaviorVersion;
    use serial_test::serial;
    use std::time::Duration;
    use test_harness::{
        instance::{setup_test_db, ImportMode},
        localstack,
    };

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
            s3_format_version: None,
            span: Span::none(),
            transaction_id: None,
        }
    }

    async fn sample_attestation() -> (CiphertextAttestationPayload, Address, CiphertextAttestation)
    {
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
        let expected_signer = signer.address();
        let attestation = build_attestation(&expected, &signer).await.unwrap();

        (expected, expected_signer, attestation)
    }

    #[tokio::test]
    async fn expected_attestation_accepts_current_metadata() {
        let (expected, expected_signer, attestation) = sample_attestation().await;

        validate_existing_attestation(&expected, expected_signer, &attestation).unwrap();
    }

    #[tokio::test]
    async fn expected_attestation_rejects_stale_digest_metadata() {
        let (expected, expected_signer, mut attestation) = sample_attestation().await;
        attestation.sns_ciphertext_digest = B256::ZERO;

        let err =
            validate_existing_attestation(&expected, expected_signer, &attestation).unwrap_err();
        assert!(err.contains("sns ciphertext digest mismatch"));
    }

    #[tokio::test]
    async fn expected_attestation_rejects_wrong_context_metadata() {
        let (mut expected, expected_signer, attestation) = sample_attestation().await;
        expected.coprocessor_context_id = U256::ZERO;

        let err =
            validate_existing_attestation(&expected, expected_signer, &attestation).unwrap_err();
        assert!(err.contains("handle/context/signature mismatch"));
    }

    #[tokio::test]
    async fn expected_attestation_rejects_wrong_signer_metadata() {
        let (expected, expected_signer, _) = sample_attestation().await;
        let other_signer: CoproSigner = Arc::new(PrivateKeySigner::random());
        let attestation = build_attestation(&expected, &other_signer).await.unwrap();

        let err =
            validate_existing_attestation(&expected, expected_signer, &attestation).unwrap_err();
        assert!(err.contains("signer mismatch"));
    }

    #[test]
    fn sha256_checksum_header_is_base64_encoded() {
        assert_eq!(
            sha256_checksum_header(b"Hello world"),
            "ZOyIygCyaOW6GjVnihtTFtIS9PNmskdyMlNKiuyjfzw="
        );
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
            s3_format_version: None,
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
            s3_format_version: Some(S3_FORMAT_VERSION_LEGACY),
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
    fn legacy_partial_ct128_without_bytes_preserves_s3_format_version() {
        let ct64 = vec![0xAA, 0xBB, 0xCC];
        let real_ct128_digest = vec![0x11; 32];
        let task = HandleItem {
            host_chain_id: ChainId::try_from(42_i64).unwrap(),
            key_id_gw: vec![7; 32],
            handle: vec![0xDE, 0xAD, 0xBE, 0xEF],
            ct64_compressed: Arc::new(ct64),
            ct128: Arc::new(BigCiphertext::new(
                Vec::new(),
                Ciphertext128Format::CompressedOnCpu,
            )),
            ct64_digest: None,
            ct128_digest: Some(real_ct128_digest.clone()),
            s3_format_version: Some(S3_FORMAT_VERSION_LEGACY),
            span: Span::none(),
            transaction_id: None,
        };

        assert!(should_preserve_legacy_s3_format(&task, &real_ct128_digest));
        assert_eq!(
            completed_s3_format_version(&task, true),
            S3_FORMAT_VERSION_LEGACY
        );
    }

    #[test]
    fn legacy_partial_ct128_with_bytes_updates_to_current_s3_format_version() {
        let real_ct128_digest = vec![0x11; 32];
        let task = HandleItem {
            host_chain_id: ChainId::try_from(42_i64).unwrap(),
            key_id_gw: vec![7; 32],
            handle: vec![0xDE, 0xAD, 0xBE, 0xEF],
            ct64_compressed: Arc::new(vec![0xAA, 0xBB, 0xCC]),
            ct128: Arc::new(BigCiphertext::new(
                vec![0x44, 0x55],
                Ciphertext128Format::CompressedOnCpu,
            )),
            ct64_digest: None,
            ct128_digest: Some(real_ct128_digest.clone()),
            s3_format_version: Some(S3_FORMAT_VERSION_LEGACY),
            span: Span::none(),
            transaction_id: None,
        };

        assert!(!should_preserve_legacy_s3_format(&task, &real_ct128_digest));
        assert_eq!(
            completed_s3_format_version(&task, false),
            CURRENT_S3_FORMAT_VERSION
        );
    }

    #[tokio::test]
    #[serial(s3)]
    #[cfg(not(feature = "gpu"))]
    async fn legacy_partial_ct128_without_bytes_uploads_ct64_and_preserves_legacy_s3_format(
    ) -> anyhow::Result<()> {
        let db_instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!("setup test db: {err}"))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(db_instance.db_url())
            .await?;

        let localstack = localstack::start_localstack().await?;
        let endpoint_url = format!("http://127.0.0.1:{}", localstack.host_port);
        std::env::set_var("AWS_ENDPOINT_URL", endpoint_url);
        std::env::set_var("AWS_REGION", "us-east-1");
        std::env::set_var("AWS_ACCESS_KEY_ID", "test");
        std::env::set_var("AWS_SECRET_ACCESS_KEY", "test");

        let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = aws_sdk_s3::Client::new(&aws_conf);
        let bucket_ct64 = "legacy-partial-ct64";
        let bucket_ct128 = "legacy-partial-ct128-not-created";
        client.create_bucket().bucket(bucket_ct64).send().await?;

        let handle = vec![0x42; 32];
        let key_id_gw = vec![0x07; 32];
        let ct64 = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let ct128_digest = vec![0x11; 32];
        let ct128_format: i16 = Ciphertext128Format::CompressedOnCpu.into();

        sqlx::query!(
            "INSERT INTO ciphertext_digest (
                host_chain_id, key_id_gw, handle, ciphertext128, ciphertext128_format, s3_format_version
            )
            VALUES ($1, $2, $3, $4, $5, $6)",
            1_i64,
            &key_id_gw,
            &handle,
            &ct128_digest,
            ct128_format,
            S3_FORMAT_VERSION_LEGACY,
        )
        .execute(&pool)
        .await?;

        let task = HandleItem {
            host_chain_id: ChainId::try_from(1_i64).unwrap(),
            key_id_gw,
            handle: handle.clone(),
            ct64_compressed: Arc::new(ct64.clone()),
            ct128: Arc::new(BigCiphertext::new(
                Vec::new(),
                Ciphertext128Format::CompressedOnCpu,
            )),
            ct64_digest: None,
            ct128_digest: Some(ct128_digest.clone()),
            s3_format_version: Some(S3_FORMAT_VERSION_LEGACY),
            span: Span::none(),
            transaction_id: None,
        };
        let conf = S3Config {
            bucket_ct128: bucket_ct128.to_owned(),
            bucket_ct64: bucket_ct64.to_owned(),
            max_concurrent_uploads: 1,
            retry_policy: S3RetryPolicy {
                max_retries_per_upload: 1,
                max_backoff: Duration::from_millis(1),
                max_retries_timeout: Duration::from_secs(1),
                recheck_duration: Duration::from_secs(1),
                regular_recheck_duration: Duration::from_secs(1),
            },
            verify_sha256_checksum: false,
        };

        let mut trx = pool.begin().await?;
        create_upload_task_savepoint(&mut trx).await?;
        upload_ciphertexts(
            &mut trx,
            task,
            &client,
            &conf,
            Arc::new(PrivateKeySigner::random()),
        )
        .await?;
        trx.commit().await?;

        let row = sqlx::query!(
            "SELECT ciphertext, ciphertext128, s3_format_version
             FROM ciphertext_digest
             WHERE handle = $1",
            &handle
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            row.ciphertext.as_deref(),
            Some(compute_digest(&ct64).as_slice())
        );
        assert_eq!(row.ciphertext128.as_deref(), Some(ct128_digest.as_slice()));
        assert_eq!(row.s3_format_version, Some(S3_FORMAT_VERSION_LEGACY));

        let ct64_key = s3_ciphertext_key(&handle, COPROCESSOR_CONTEXT_ID_1);
        client
            .head_object()
            .bucket(bucket_ct64)
            .key(ct64_key)
            .send()
            .await?;

        Ok(())
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
