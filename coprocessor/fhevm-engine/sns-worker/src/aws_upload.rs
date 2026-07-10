use crate::metrics::{
    AWS_UPLOAD_FAILURE_COUNTER, AWS_UPLOAD_SUCCESS_COUNTER,
    S3_CANONICAL_RECONCILER_MISMATCH_COUNTER, S3_CANONICAL_REPAIR_COMPLETED_COUNTER,
    S3_CANONICAL_REPAIR_ENQUEUED_COUNTER, S3_CANONICAL_REPAIR_FAILED_COUNTER,
    S3_CANONICAL_REPAIR_QUARANTINED_COUNTER, S3_PUBLICATION_BLOCKS_SETTLEMENT_COUNTER,
    STALE_S3_UPLOAD_AFTER_CLEANUP_COUNTER,
};
use crate::{
    BigCiphertext, Ciphertext128Format, Config, ExecutionError, HandleItem, S3Config, UploadJob,
    CURRENT_S3_FORMAT_VERSION, S3_FORMAT_VERSION_LEGACY,
};
use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_bucket::HeadBucketError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{ChecksumAlgorithm, ChecksumMode, MetadataDirective};
use aws_sdk_s3::Client;
use base64::Engine;
use bytesize::ByteSize;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextAttestationPayload, CiphertextFormat, Version,
    S3_METADATA_ATTESTATION_KEY,
};
use fhevm_engine_common::branch::{select_producer_candidate, ProducerBlockHashed};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::database::EVENT_CIPHERTEXTS_UPLOADED;
use fhevm_engine_common::pg_pool::{is_fatal_connection_error, PostgresPoolManager, ServiceError};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use fhevm_engine_common::types::CoproSigner;
use fhevm_engine_common::utils::to_hex;
use futures::future::join_all;
use opentelemetry::trace::{Status, TraceContextExt};
use sha2::Sha256;
use sha3::{Digest, Keccak256};
use sqlx::{Executor, PgPool, Pool, Postgres, Transaction};
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
const S3_CANONICAL_REPAIR_MAX_ATTEMPTS: i32 = 10;
pub(crate) const COPROCESSOR_CONTEXT_ID_1: U256 = U256::ONE;
const NO_SNS_CIPHERTEXT_DIGEST: [u8; 32] = [0; 32];
#[derive(Clone, Debug)]
struct UploadBytesCandidate {
    producer_block_hash: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl ProducerBlockHashed for UploadBytesCandidate {
    fn producer_block_hash(&self) -> &[u8] {
        &self.producer_block_hash
    }
}

pub(crate) async fn spawn_resubmit_task(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    jobs_tx: mpsc::Sender<UploadJob>,
    client: Arc<aws_sdk_s3::Client>,
    is_ready: Arc<AtomicBool>,
    signer: CoproSigner,
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
    let op = move |pool, token| {
        let client = client.clone();
        let is_ready = is_ready.clone();
        let conf = conf.clone();
        let jobs_tx = jobs_tx.clone();
        let signer = signer.clone();

        async move {
            do_resubmits_loop(client, pool, conf, jobs_tx, token, is_ready, signer)
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
        let conf = conf.clone();
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
    conf: Config,
    signer: CoproSigner,
) -> Result<(), ExecutionError> {
    let gcs_mode = conf.gcs_mode;
    let conf = conf.s3;
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

                let Some(mut trx) = fhevm_engine_common::versioning::begin_write_guarded(
                    &pool,
                    gcs_mode,
                )
                .await?
                else {
                    info!("Cutover completed — stopping retired SNS uploader");
                    return Ok(());
                };

                // Normal jobs defer their enqueue into the spawned task: the
                // provenance witness takes FOR KEY SHARE on the pbs row,
                // which blocks while the originating batch transaction still
                // holds its FOR UPDATE work locks — the dispatch loop must
                // never park head-of-line on that.
                let (item, needs_enqueue) = match job {
                    UploadJob::Normal(item) => (item, true),
                    UploadJob::DatabaseLock(mut item) => {
                        create_upload_task_savepoint(&mut trx).await?;
                        let row = match sqlx::query!(
                            r#"
                            SELECT d.ciphertext,
                                   d.ciphertext128,
                                   d.ciphertext128_format AS "ciphertext128_format?",
                                   d.s3_format_version
                            FROM ciphertext_digest_branch d
                            WHERE d.host_chain_id = $1
                              AND d.handle = $2
                              AND d.producer_block_hash = $3
                              AND d.block_hash = $4
                              AND (
                                EXISTS (
                                  SELECT 1
                                  FROM pbs_computations_branch p
                                  WHERE p.host_chain_id = d.host_chain_id
                                    AND p.handle = d.handle
                                    AND p.producer_block_hash = d.producer_block_hash
                                    AND p.block_hash = d.block_hash
                                    AND p.is_completed = TRUE
                                )
                                OR (
                                  d.producer_block_hash = ''::BYTEA
                                  AND d.block_hash = ''::BYTEA
                                  AND NOT EXISTS (
                                    SELECT 1
                                    FROM pbs_computations_branch p
                                    WHERE p.host_chain_id = d.host_chain_id
                                      AND p.handle = d.handle
                                      AND p.producer_block_hash = d.producer_block_hash
                                      AND p.block_hash = d.block_hash
                                  )
                                )
                              )
                              AND (
                                d.ciphertext IS NULL
                                OR (
                                  (
                                    d.ciphertext128 IS NULL
                                    OR d.ciphertext128 = decode(repeat('00', 32), 'hex')
                                  )
                                  AND EXISTS (
                                    SELECT 1
                                    FROM ciphertexts128_branch c
                                    WHERE c.handle = d.handle
                                      AND c.producer_block_hash = d.producer_block_hash
                                    AND c.ciphertext IS NOT NULL
                                  )
                                )
                                OR (
                                  d.ciphertext128_format IS NULL
                                  AND EXISTS (
                                    SELECT 1
                                    FROM ciphertexts128_branch c
                                    WHERE c.handle = d.handle
                                      AND c.producer_block_hash = d.producer_block_hash
                                      AND c.ciphertext IS NOT NULL
                                  )
                                )
                              )
                            FOR UPDATE SKIP LOCKED
                            "#,
                            item.host_chain_id.as_i64(),
                            &item.handle,
                            &item.producer_block_hash,
                            &item.block_hash,
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
                        let uploaded_ct64 = row.ciphertext;
                        let uploaded_ct128 = row.ciphertext128;
                        let ciphertext128_format = row.ciphertext128_format;
                        item.s3_format_version = row.s3_format_version;

                        // A non-null digest means another worker already uploaded that
                        // ciphertext variant, so this recovery job must not retry it.
                        if let Some(digest) = uploaded_ct64 {
                            item.ct64_digest = Some(digest);
                            item.ct64_compressed = Arc::new(Vec::new());
                        }

                        if let Some(digest) = uploaded_ct128 {
                            if !item.ct128.is_empty()
                                && digest.as_slice() == NO_SNS_CIPHERTEXT_DIGEST.as_slice()
                            {
                                // A previous recovery pass may have written the
                                // no-SNS sentinel while conversion was still
                                // incomplete. Recompute from the now-available
                                // ct128 bytes instead of preserving it.
                                item.ct128_digest = None;
                            } else {
                                item.ct128_digest = Some(digest);
                            }
                        }

                        let ct128_format = match ciphertext128_format {
                            Some(format) => Ciphertext128Format::from_i16(format).ok_or_else(|| {
                                ExecutionError::InvalidCiphertext128Format(format!(
                                    "pending ct128 has invalid format id, host_chain_id: {}, handle: {}, format_id: {}",
                                    item.host_chain_id.as_i64(),
                                    to_hex(&item.handle),
                                    format,
                                ))
                            })?,
                            None if !item.ct128.is_empty()
                                && item.ct128.format() != Ciphertext128Format::Unknown =>
                            {
                                warn!(
                                    handle = %to_hex(&item.handle),
                                    format = %item.ct128.format(),
                                    "Backfilling missing ciphertext128 format from SNS configuration"
                                );
                                item.ct128.format()
                            }
                            None if item.ct128.is_empty() => Ciphertext128Format::Unknown,
                            None => {
                                return Err(ExecutionError::InvalidCiphertext128Format(format!(
                                    "pending ct128 is missing format, host_chain_id: {}, handle: {}",
                                    item.host_chain_id.as_i64(),
                                    to_hex(&item.handle),
                                )));
                            }
                        };

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

                        (item, false)
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
                    let retry_identity = item.clone();
                    // Enqueue deferred from the dispatch loop (see above). A
                    // dead witness means reorg cleanup cancelled the
                    // publication while this job was queued: drop it.
                    let prep: anyhow::Result<bool> = async {
                        if needs_enqueue {
                            if !item.enqueue_upload_task(&mut trx).await? {
                                return Ok(false);
                            }
                            create_upload_task_savepoint(&mut trx).await?;
                        }
                        Ok(true)
                    }
                    .await;
                    let result = match prep {
                        Ok(false) => {
                            if let Err(err) = trx.rollback().await {
                                warn!(error = %err, "Failed to roll back cancelled upload");
                            }
                            drop(upload_span);
                            drop(permit);
                            return Ok(());
                        }
                        Ok(true) => {
                            upload_ciphertexts(&mut trx, item, &client, &conf, signer)
                                .instrument(upload_span.clone())
                                .await
                        }
                        Err(err) => Err(err),
                    };
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

                            preserve_upload_task_for_retry(trx, &retry_identity, &err).await;

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
    sqlx::query!("SAVEPOINT savepoint_upload_ciphertexts_task")
        .execute(trx.as_mut())
        .await?;

    Ok(())
}

async fn rollback_to_upload_task_savepoint(
    trx: &mut Transaction<'_, Postgres>,
) -> Result<(), ExecutionError> {
    sqlx::query!("ROLLBACK TO SAVEPOINT savepoint_upload_ciphertexts_task")
        .execute(trx.as_mut())
        .await?;

    Ok(())
}

async fn preserve_upload_task_for_retry(
    mut trx: Transaction<'_, Postgres>,
    task: &HandleItem,
    upload_error: &anyhow::Error,
) {
    if let Err(preserve_err) = async {
        rollback_to_upload_task_savepoint(&mut trx).await?;
        record_canonical_repair_failure(
            trx.as_mut(),
            task.host_chain_id.as_i64(),
            &task.handle,
            &task.producer_block_hash,
            &task.block_hash,
            &upload_error.to_string(),
        )
        .await?;
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

async fn record_canonical_repair_failure<'e, E>(
    executor: E,
    host_chain_id: i64,
    handle: &[u8],
    producer_block_hash: &[u8],
    block_hash: &[u8],
    repair_error: &str,
) -> Result<(), ExecutionError>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query_as::<_, (String, i32)>(
        r#"
        UPDATE s3_canonical_repair_queue
           SET status = CASE
                   WHEN attempts >= $6 THEN 'quarantined'
                   ELSE 'pending'
               END,
               locked_at = NULL,
               last_error = $5,
               last_error_at = NOW(),
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND handle = $2
           AND target_producer_block_hash = $3
           AND target_block_hash = $4
           AND status = 'pending'
         RETURNING status, attempts
        "#,
    )
    .bind(host_chain_id)
    .bind(handle)
    .bind(producer_block_hash)
    .bind(block_hash)
    .bind(repair_error)
    .bind(S3_CANONICAL_REPAIR_MAX_ATTEMPTS)
    .fetch_optional(executor)
    .await?;

    if let Some((status, attempts)) = row {
        S3_CANONICAL_REPAIR_FAILED_COUNTER.inc();
        if status == "quarantined" {
            S3_CANONICAL_REPAIR_QUARANTINED_COUNTER.inc();
            error!(
                host_chain_id,
                handle = %to_hex(handle),
                producer_block_hash = %to_hex(producer_block_hash),
                block_hash = %to_hex(block_hash),
                attempts,
                repair_error,
                "Quarantined canonical S3 repair after bounded retries"
            );
        }
    }

    Ok(())
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

/// How strictly an existing object's attestation is validated.
///
/// The attestation signature binds the ciphertext handle (RFC 023), which only
/// the canonical handle-keyed object can satisfy. Digest-keyed
/// backward-compatibility copies are content-addressed: every handle whose
/// ciphertext bytes are identical shares one copy, whose attestation is bound
/// to whichever handle last wrote it. Recovering the signature there against
/// another handle's payload yields a garbage address and the upload loop
/// re-uploads forever. Content-bound validation therefore checks every
/// attested field (digests, key id, format, signer) but skips signature
/// recovery; content identity is already pinned by the digest key, and the
/// canonical object still gets the handle-bound check.
#[derive(Clone, Copy, PartialEq)]
enum AttestationBinding {
    HandleBound,
    ContentBound,
}

fn validate_existing_attestation(
    expected: &CiphertextAttestationPayload,
    expected_signer: Address,
    actual: &CiphertextAttestation,
    binding: AttestationBinding,
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

    if binding == AttestationBinding::HandleBound {
        actual
            .verify(expected.handle, expected.coprocessor_context_id)
            .map_err(|err| format!("handle/context/signature mismatch: {err}"))?;
    }

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

async fn check_s3_publication(
    client: &Client,
    conf: &S3Config,
    task: &HandleItem,
    expected_attestation: &CiphertextAttestationPayload,
    expected_signer: Address,
    upload_material: &UploadMaterial,
    preserve_legacy_s3_format: bool,
) -> Result<bool, ExecutionError> {
    let key = s3_ciphertext_key(&task.handle, COPROCESSOR_CONTEXT_ID_1);
    let ct64_checksum_sha256 = conf
        .verify_sha256_checksum
        .then(|| match task.ct64_compressed.is_empty() {
            true => None,
            false => Some(sha256_checksum_header(task.ct64_compressed.as_ref())),
        })
        .flatten();

    let ct64_verified = check_attested_object_exists(
        client,
        &conf.bucket_ct64,
        &key,
        expected_attestation,
        expected_signer,
        ct64_checksum_sha256.as_deref(),
        AttestationBinding::HandleBound,
    )
    .await?;
    if !ct64_verified {
        return Ok(false);
    }

    if upload_material.ct128_digest.as_slice() != NO_SNS_CIPHERTEXT_DIGEST.as_slice()
        && !preserve_legacy_s3_format
    {
        let digest_key = hex::encode(&upload_material.ct128_digest);
        let ct128_checksum_sha256 = conf
            .verify_sha256_checksum
            .then(|| match task.ct128.is_empty() {
                true => None,
                false => Some(sha256_checksum_header(task.ct128.bytes())),
            })
            .flatten();

        let ct128_verified = check_ct128_objects_exist(
            client,
            &conf.bucket_ct128,
            &key,
            &digest_key,
            expected_attestation,
            expected_signer,
            ct128_checksum_sha256.as_deref(),
        )
        .await?;
        if !ct128_verified {
            return Ok(false);
        }
    }

    Ok(true)
}

async fn verify_s3_publication(
    client: &Client,
    conf: &S3Config,
    task: &HandleItem,
    expected_attestation: &CiphertextAttestationPayload,
    expected_signer: Address,
    upload_material: &UploadMaterial,
    preserve_legacy_s3_format: bool,
) -> Result<(), ExecutionError> {
    if !check_s3_publication(
        client,
        conf,
        task,
        expected_attestation,
        expected_signer,
        upload_material,
        preserve_legacy_s3_format,
    )
    .await?
    {
        return Err(ExecutionError::S3TransientError(format!(
            "S3 publication for handle {} was not verified after upload",
            to_hex(&task.handle)
        )));
    }

    Ok(())
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

    if !task.is_upload_publishable(trx).await? {
        if task
            .enqueue_canonical_repair(trx, "upload_preflight_not_publishable")
            .await?
        {
            S3_CANONICAL_REPAIR_ENQUEUED_COUNTER.inc();
        }
        STALE_S3_UPLOAD_AFTER_CLEANUP_COUNTER.inc();
        info!(
            handle = handle_as_hex,
            producer_block_hash = %to_hex(&task.producer_block_hash),
            block_hash = %to_hex(&task.block_hash),
            "Skipping S3 upload for non-publishable branch row"
        );
        return Ok(());
    }
    let repair_attempt = task.is_canonical_repair_task(trx).await?;

    let mut jobs = vec![];

    let upload_material = upload_material(&task)?;
    let ct128_digest = &upload_material.ct128_digest;
    let preserve_legacy_s3_format = should_preserve_legacy_s3_format(&task, ct128_digest);

    // Pure-ct64 objects carry compressed ct64 bytes with the no-SNS digest sentinel.
    // For real ct128 objects, attest the stored ct128 format.
    let attestation_format = if *ct128_digest == NO_SNS_CIPHERTEXT_DIGEST.to_vec() {
        CiphertextFormat::CompressedOnCpu
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
            let mut exists = object_check_result(
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

            if !exists {
                exists = object_check_result(
                    &ct128_check_span,
                    repair_ct128_object_pair_from_existing_copy(
                        client,
                        &conf.bucket_ct128,
                        &key,
                        &digest_key,
                        &expected_attestation,
                        expected_signer,
                        ct128_checksum_sha256.as_deref(),
                        conf.verify_sha256_checksum,
                        &s3_metadata,
                        &task.ct128.format().to_string(),
                    )
                    .instrument(ct128_check_span.clone())
                    .await,
                )?;
            }
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
                AttestationBinding::HandleBound,
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

    if let Err(err) = verify_s3_publication(
        client,
        conf,
        &task,
        &expected_attestation,
        expected_signer,
        &upload_material,
        preserve_legacy_s3_format,
    )
    .await
    {
        return Err(err.into());
    }

    let marked_uploaded = task
        .mark_ciphertexts_uploaded(
            trx,
            upload_material.ct64_digest,
            upload_material.ct128_digest,
            completed_s3_format_version(&task, preserve_legacy_s3_format),
        )
        .await?;
    if !marked_uploaded {
        if task
            .enqueue_canonical_repair(trx, "upload_postflight_not_publishable")
            .await?
        {
            S3_CANONICAL_REPAIR_ENQUEUED_COUNTER.inc();
        }
        STALE_S3_UPLOAD_AFTER_CLEANUP_COUNTER.inc();
        return Ok(());
    }
    if repair_attempt {
        S3_CANONICAL_REPAIR_COMPLETED_COUNTER.inc();
    }

    sqlx::query!("SELECT pg_notify($1, '')", EVENT_CIPHERTEXTS_UPLOADED)
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
pub(crate) async fn fetch_pending_uploads(
    db_pool: &Pool<Postgres>,
    limit: i64,
    expected_ct128_format: Ciphertext128Format,
) -> Result<Vec<UploadJob>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT d.handle,
               d.producer_block_hash,
               d.block_hash,
               d.block_number,
               d.ciphertext,
               d.ciphertext128,
               d.ciphertext128_format AS "ciphertext128_format?",
               d.s3_format_version,
               d.transaction_id,
               d.host_chain_id,
               d.key_id_gw,
               EXISTS (
                 SELECT 1
                 FROM ciphertexts128_branch c
                 WHERE c.handle = d.handle
                   AND c.producer_block_hash = d.producer_block_hash
                   AND c.ciphertext IS NOT NULL
               ) AS has_ct128_ciphertext
        FROM ciphertext_digest_branch d
        WHERE (
                EXISTS (
                  SELECT 1
                  FROM pbs_computations_branch p
                  WHERE p.host_chain_id = d.host_chain_id
                    AND p.handle = d.handle
                    AND p.producer_block_hash = d.producer_block_hash
                    AND p.block_hash = d.block_hash
                    AND p.is_completed = TRUE
                )
                OR (
                  d.producer_block_hash = ''::BYTEA
                  AND d.block_hash = ''::BYTEA
                  AND NOT EXISTS (
                    SELECT 1
                    FROM pbs_computations_branch p
                    WHERE p.host_chain_id = d.host_chain_id
                      AND p.handle = d.handle
                      AND p.producer_block_hash = d.producer_block_hash
                      AND p.block_hash = d.block_hash
                  )
                )
              )
          AND (
                d.ciphertext IS NULL
                OR (
                  (
                    d.ciphertext128 IS NULL
                    OR d.ciphertext128 = decode(repeat('00', 32), 'hex')
                  )
                  AND EXISTS (
                    SELECT 1
                    FROM ciphertexts128_branch c
                    WHERE c.handle = d.handle
                      AND c.producer_block_hash = d.producer_block_hash
                    AND c.ciphertext IS NOT NULL
                  )
                )
                OR (
                  d.ciphertext128_format IS NULL
                  AND EXISTS (
                    SELECT 1
                    FROM ciphertexts128_branch c
                    WHERE c.handle = d.handle
                      AND c.producer_block_hash = d.producer_block_hash
                      AND c.ciphertext IS NOT NULL
                  )
                )
              )
        FOR UPDATE SKIP LOCKED
        LIMIT $1
        "#,
        limit,
    )
    .fetch_all(db_pool)
    .await?;

    let mut jobs = Vec::new();

    for row in rows {
        let mut ct64_compressed = Arc::new(Vec::new());
        let mut ct128 = Vec::new();
        let ciphertext_digest = row.ciphertext;
        let mut ciphertext128_digest = row.ciphertext128;
        let s3_format_version = row.s3_format_version;
        let should_verify_existing_s3 = s3_format_version != Some(CURRENT_S3_FORMAT_VERSION);
        let handle = row.handle;
        let producer_block_hash = row.producer_block_hash;
        let block_hash = row.block_hash;
        let block_number = row.block_number;
        let transaction_id = row.transaction_id;
        let host_chain_id_raw = row.host_chain_id;
        let key_id_gw = row.key_id_gw;
        let ciphertext128_format = row.ciphertext128_format;
        let has_ct128_ciphertext = row.has_ct128_ciphertext.unwrap_or(false);
        if has_ct128_ciphertext
            && ciphertext128_digest.as_deref() == Some(NO_SNS_CIPHERTEXT_DIGEST.as_slice())
        {
            warn!(
                handle = %to_hex(&handle),
                "Replacing stale no-SNS digest from completed conversion"
            );
            ciphertext128_digest = None;
        }
        let row_incomplete = ciphertext_digest.is_none()
            || (has_ct128_ciphertext
                && (ciphertext128_digest.is_none() || ciphertext128_format.is_none()));

        // Fetch the ciphertext whenever the row is not fully committed. This
        // lets recovery revalidate both S3 objects before the single DB update.
        // Also fetch already-uploaded ciphertext for pre-v1 rows so the retry
        // can validate/rewrite old S3 objects.
        if row_incomplete || should_verify_existing_s3 {
            let mut candidates = Vec::new();
            let rows = sqlx::query!(
                r#"
                SELECT producer_block_hash AS "producer_block_hash!", ciphertext AS "ciphertext!"
                 FROM ciphertexts_branch
                 WHERE handle = $1
                   AND ciphertext IS NOT NULL
                   AND ciphertext_version = $2
                "#,
                &handle,
                current_ciphertext_version(),
            )
            .fetch_all(db_pool)
            .await?;
            for candidate_row in rows {
                candidates.push(UploadBytesCandidate {
                    producer_block_hash: candidate_row.producer_block_hash,
                    ciphertext: candidate_row.ciphertext,
                });
            }
            if let Some(record) = select_producer_candidate(&candidates, &producer_block_hash) {
                ct64_compressed = Arc::new(record.ciphertext.clone());
            } else {
                error!(handle = hex::encode(&handle), "Missing ciphertext");
            }
        }

        // Fetch ciphertext128 under the same rule: incomplete rows are retried
        // as a whole handle, and pre-v1 rows need the bytes for validation.
        // Ct64-only rows have no ciphertext128 material, so they are completed
        // by writing the zero ct128 digest after ct64 is verified/uploaded.
        if has_ct128_ciphertext && (row_incomplete || should_verify_existing_s3) {
            let mut candidates = Vec::new();
            let rows = sqlx::query!(
                r#"
                SELECT producer_block_hash AS "producer_block_hash!", ciphertext
                 FROM ciphertexts128_branch
                 WHERE handle = $1
                "#,
                &handle,
            )
            .fetch_all(db_pool)
            .await?;
            for candidate_row in rows {
                let ciphertext = candidate_row.ciphertext;
                if let Some(ciphertext) = ciphertext {
                    if !ciphertext.is_empty() {
                        candidates.push(UploadBytesCandidate {
                            producer_block_hash: candidate_row.producer_block_hash,
                            ciphertext,
                        });
                    }
                }
            }
            if let Some(record) = select_producer_candidate(&candidates, &producer_block_hash) {
                ct128 = record.ciphertext.clone();
            } else if !candidates.is_empty() {
                warn!(handle = hex::encode(&handle), "Fetched empty ct128");
            } else {
                error!(handle = hex::encode(&handle), "Missing ciphertext128");
            }
        }

        let is_ct128_empty = ct128.is_empty();

        let ct128_format = match ciphertext128_format {
            Some(format) => match Ciphertext128Format::from_i16(format) {
                Some(format) => format,
                None => {
                    error!(
                        handle = to_hex(&handle),
                        format_id = format,
                        "Failed to create a BigCiphertext from DB data",
                    );
                    continue;
                }
            },
            None if has_ct128_ciphertext => {
                warn!(
                    handle = %to_hex(&handle),
                    format = %expected_ct128_format,
                    "Recovering missing ciphertext128 format from SNS configuration"
                );
                expected_ct128_format
            }
            None => Ciphertext128Format::Unknown,
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
                host_chain_id: ChainId::try_from(host_chain_id_raw)
                    .map_err(|e| ExecutionError::ConversionError(e.into()))?,
                key_id_gw,
                handle: handle.clone(),
                producer_block_hash,
                block_hash,
                block_number,
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

    if jobs.len() < limit as usize {
        jobs.extend(fetch_pending_canonical_repairs(db_pool, limit - jobs.len() as i64).await?);
    }

    Ok(jobs)
}

async fn fetch_pending_canonical_repairs(
    db_pool: &Pool<Postgres>,
    limit: i64,
) -> Result<Vec<UploadJob>, ExecutionError> {
    if limit <= 0 {
        return Ok(vec![]);
    }

    let quarantined = sqlx::query_scalar::<_, i64>(
        r#"
        WITH quarantined AS (
            UPDATE s3_canonical_repair_queue
               SET status = 'quarantined',
                   locked_at = NULL,
                   last_error = COALESCE(
                       last_error,
                       'repair lease expired after maximum attempts'
                   ),
                   last_error_at = COALESCE(last_error_at, NOW()),
                   updated_at = NOW()
             WHERE status = 'pending'
               AND attempts >= $1
               AND (
                    locked_at IS NULL
                    OR locked_at < NOW() - INTERVAL '5 minutes'
               )
             RETURNING 1
        )
        SELECT COUNT(*) FROM quarantined
        "#,
    )
    .bind(S3_CANONICAL_REPAIR_MAX_ATTEMPTS)
    .fetch_one(db_pool)
    .await?;
    if quarantined > 0 {
        S3_CANONICAL_REPAIR_QUARANTINED_COUNTER.inc_by(quarantined as u64);
        error!(
            quarantined,
            "Quarantined stale canonical S3 repairs after bounded claims"
        );
    }

    let rows = sqlx::query!(
        r#"
        WITH selected AS (
            SELECT q.host_chain_id,
                   q.handle
              FROM s3_canonical_repair_queue q
              JOIN ciphertext_digest_branch d
                ON d.host_chain_id = q.host_chain_id
               AND d.handle = q.handle
               AND d.producer_block_hash = q.target_producer_block_hash
               AND d.block_hash = q.target_block_hash
             WHERE q.status = 'pending'
               AND q.attempts < $2
               AND (
                    q.locked_at IS NULL
                    OR q.locked_at < NOW() - INTERVAL '5 minutes'
                   )
               AND (
                    EXISTS (
                        SELECT 1
                          FROM pbs_computations_branch p
                         WHERE p.host_chain_id = d.host_chain_id
                           AND p.handle = d.handle
                           AND p.producer_block_hash = d.producer_block_hash
                           AND p.block_hash = d.block_hash
                           AND p.is_completed = TRUE
                    )
                    OR (
                        d.producer_block_hash = ''::BYTEA
                        AND d.block_hash = ''::BYTEA
                        AND NOT EXISTS (
                            SELECT 1
                              FROM pbs_computations_branch p
                             WHERE p.host_chain_id = d.host_chain_id
                               AND p.handle = d.handle
                               AND p.producer_block_hash = d.producer_block_hash
                               AND p.block_hash = d.block_hash
                        )
                    )
               )
               AND d.ciphertext IS NOT NULL
               AND (
                    d.ciphertext128 IS NOT NULL
                    OR NOT EXISTS (
                        SELECT 1
                          FROM ciphertexts128_branch c
                         WHERE c.handle = d.handle
                           AND c.producer_block_hash = d.producer_block_hash
                           AND c.ciphertext IS NOT NULL
                    )
               )
             ORDER BY q.updated_at ASC
             FOR UPDATE OF q SKIP LOCKED
             LIMIT $1
        ),
        locked AS (
            UPDATE s3_canonical_repair_queue q
               SET locked_at = NOW(),
                   attempts = attempts + 1,
                   updated_at = NOW()
              FROM selected s
             WHERE q.host_chain_id = s.host_chain_id
               AND q.handle = s.handle
             RETURNING q.host_chain_id,
                       q.handle,
                       q.target_producer_block_hash,
                       q.target_block_hash
        )
        SELECT d.handle,
               d.producer_block_hash,
               d.block_hash,
               d.block_number,
               d.ciphertext,
               d.ciphertext128,
               d.ciphertext128_format AS "ciphertext128_format?",
               d.s3_format_version,
               d.transaction_id,
               d.host_chain_id,
               d.key_id_gw,
               EXISTS (
                 SELECT 1
                   FROM ciphertexts128_branch c
                 WHERE c.handle = d.handle
                    AND c.producer_block_hash = d.producer_block_hash
                    AND c.ciphertext IS NOT NULL
               ) AS "has_ct128_ciphertext!"
          FROM locked q
          JOIN ciphertext_digest_branch d
            ON d.host_chain_id = q.host_chain_id
           AND d.handle = q.handle
           AND d.producer_block_hash = q.target_producer_block_hash
           AND d.block_hash = q.target_block_hash
        LIMIT $1
        "#,
        limit,
        S3_CANONICAL_REPAIR_MAX_ATTEMPTS,
    )
    .fetch_all(db_pool)
    .await?;

    let mut jobs = Vec::new();
    for row in rows {
        let handle = row.handle;
        let producer_block_hash = row.producer_block_hash;
        let block_hash = row.block_hash;
        let block_number = row.block_number;
        let transaction_id = row.transaction_id;
        let host_chain_id_raw = row.host_chain_id;
        let key_id_gw = row.key_id_gw;
        let ciphertext_digest = row.ciphertext;
        let ciphertext128_digest = row.ciphertext128;
        let ciphertext128_format = row.ciphertext128_format;
        let s3_format_version = row.s3_format_version;
        let has_ct128_ciphertext = row.has_ct128_ciphertext;

        let ct64_compressed = sqlx::query_scalar!(
            r#"
            SELECT ciphertext AS "ciphertext!"
              FROM ciphertexts_branch
             WHERE handle = $1
               AND producer_block_hash = $2
               AND ciphertext IS NOT NULL
               AND ciphertext_version = $3
            "#,
            &handle,
            &producer_block_hash,
            current_ciphertext_version(),
        )
        .fetch_optional(db_pool)
        .await?;

        let Some(ct64_compressed) = ct64_compressed else {
            let repair_error = "missing ct64 bytes for S3 canonical repair";
            error!(
                handle = hex::encode(&handle),
                producer_block_hash = %to_hex(&producer_block_hash),
                "{repair_error}"
            );
            record_canonical_repair_failure(
                db_pool,
                host_chain_id_raw,
                &handle,
                &producer_block_hash,
                &block_hash,
                repair_error,
            )
            .await?;
            continue;
        };

        let mut ct128 = Vec::new();
        if has_ct128_ciphertext {
            ct128 = sqlx::query_scalar!(
                r#"
                SELECT ciphertext AS "ciphertext!"
                  FROM ciphertexts128_branch
                 WHERE handle = $1
                   AND producer_block_hash = $2
                   AND ciphertext IS NOT NULL
                "#,
                &handle,
                &producer_block_hash,
            )
            .fetch_optional(db_pool)
            .await?
            .unwrap_or_default();
            if ct128.is_empty() {
                let repair_error = "missing ct128 bytes for S3 canonical repair";
                error!(
                    handle = hex::encode(&handle),
                    producer_block_hash = %to_hex(&producer_block_hash),
                    "{repair_error}"
                );
                record_canonical_repair_failure(
                    db_pool,
                    host_chain_id_raw,
                    &handle,
                    &producer_block_hash,
                    &block_hash,
                    repair_error,
                )
                .await?;
                continue;
            }
        }

        let has_committed_ct128 = ciphertext128_digest
            .as_deref()
            .is_some_and(|digest| digest != NO_SNS_CIPHERTEXT_DIGEST.as_slice());
        let requires_ct128_format = has_ct128_ciphertext || has_committed_ct128;
        let ct128_format = match ciphertext128_format.and_then(Ciphertext128Format::from_i16) {
            Some(Ciphertext128Format::Unknown) if requires_ct128_format => {
                let repair_error = "unknown ciphertext128 format for S3 canonical repair";
                error!(
                    handle = to_hex(&handle),
                    format_id = ?ciphertext128_format,
                    "{repair_error}"
                );
                record_canonical_repair_failure(
                    db_pool,
                    host_chain_id_raw,
                    &handle,
                    &producer_block_hash,
                    &block_hash,
                    repair_error,
                )
                .await?;
                continue;
            }
            Some(format) => format,
            None if !requires_ct128_format => Ciphertext128Format::Unknown,
            None => {
                let repair_error =
                    "missing or invalid ciphertext128 format for S3 canonical repair";
                error!(
                    handle = to_hex(&handle),
                    format_id = ?ciphertext128_format,
                    "{repair_error}"
                );
                record_canonical_repair_failure(
                    db_pool,
                    host_chain_id_raw,
                    &handle,
                    &producer_block_hash,
                    &block_hash,
                    repair_error,
                )
                .await?;
                continue;
            }
        };

        let repair_span = tracing::info_span!(
            "s3_canonical_repair_task",
            txn_id = tracing::field::Empty,
            handle = tracing::field::Empty
        );
        telemetry::record_short_hex(&repair_span, "handle", &handle);
        telemetry::record_short_hex_if_some(&repair_span, "txn_id", transaction_id.as_deref());

        jobs.push(UploadJob::Normal(HandleItem {
            host_chain_id: ChainId::try_from(host_chain_id_raw)
                .map_err(|e| ExecutionError::ConversionError(e.into()))?,
            key_id_gw,
            handle,
            producer_block_hash,
            block_hash,
            block_number,
            ct64_compressed: Arc::new(ct64_compressed),
            ct128: Arc::new(BigCiphertext::new(ct128, ct128_format)),
            ct64_digest: ciphertext_digest,
            ct128_digest: ciphertext128_digest,
            s3_format_version,
            span: repair_span,
            transaction_id,
        }));
    }

    Ok(jobs)
}

async fn reconcile_s3_canonical_publications(
    client: &Client,
    db_pool: &Pool<Postgres>,
    conf: &S3Config,
    signer: CoproSigner,
    limit: usize,
) -> Result<(), ExecutionError> {
    let enqueued = enqueue_unverified_settled_publications(db_pool, limit as i64).await?;
    if enqueued > 0 {
        S3_CANONICAL_REPAIR_ENQUEUED_COUNTER.inc_by(enqueued);
        S3_PUBLICATION_BLOCKS_SETTLEMENT_COUNTER.inc_by(enqueued);
    }

    reconcile_verified_settled_publications(client, db_pool, conf, signer, limit as i64).await
}

pub(crate) async fn enqueue_unverified_settled_publications(
    db_pool: &Pool<Postgres>,
    limit: i64,
) -> Result<u64, ExecutionError> {
    if limit <= 0 {
        return Ok(0);
    }

    let enqueued = sqlx::query_scalar::<_, i64>(
        r#"
        WITH candidates AS (
            SELECT d.host_chain_id,
                   d.handle,
                   d.producer_block_hash,
                   d.block_hash,
                   d.block_number
              FROM ciphertext_digest_branch d
              JOIN coprocessor_settlement s
                ON s.chain_id = d.host_chain_id
              LEFT JOIN host_chain_blocks_valid producer
                ON producer.chain_id = d.host_chain_id
               AND producer.block_hash = d.producer_block_hash
               AND d.producer_block_hash <> ''::BYTEA
              LEFT JOIN host_chain_blocks_valid event_block
                ON event_block.chain_id = d.host_chain_id
               AND event_block.block_hash = d.block_hash
               AND d.block_hash <> ''::BYTEA
             WHERE d.block_number IS NOT NULL
               AND d.block_number <= s.settled_height
               AND (
                    d.producer_block_hash = ''::BYTEA
                    OR COALESCE(producer.block_status, 'pending') <> 'orphaned'
               )
               AND (
                    d.block_hash = ''::BYTEA
                    OR COALESCE(event_block.block_status, 'pending') <> 'orphaned'
               )
               AND d.ciphertext IS NOT NULL
               AND (
                    d.ciphertext128 IS NOT NULL
                    OR NOT EXISTS (
                        SELECT 1
                          FROM ciphertexts128_branch c
                         WHERE c.handle = d.handle
                           AND c.producer_block_hash = d.producer_block_hash
                           AND c.ciphertext IS NOT NULL
                    )
               )
               AND (
                    d.s3_publication_verified_at IS NULL
                    OR d.s3_publication_verified_digest IS DISTINCT FROM d.ciphertext
                    OR d.s3_publication_verified_producer_block_hash IS DISTINCT FROM d.producer_block_hash
               )
             ORDER BY d.block_number ASC, d.created_at ASC
             LIMIT $1
        ),
        upserted AS (
            INSERT INTO s3_canonical_repair_queue (
                host_chain_id,
                handle,
                target_producer_block_hash,
                target_block_hash,
                target_block_number,
                reason
            )
            SELECT host_chain_id,
                   handle,
                   producer_block_hash,
                   block_hash,
                   block_number,
                   'settlement_unverified_publication'
              FROM candidates
            ON CONFLICT (host_chain_id, handle) DO UPDATE
            SET target_producer_block_hash = EXCLUDED.target_producer_block_hash,
                target_block_hash = EXCLUDED.target_block_hash,
                target_block_number = EXCLUDED.target_block_number,
                reason = EXCLUDED.reason,
                attempts = CASE
                    WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                      OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                    THEN 0
                    ELSE s3_canonical_repair_queue.attempts
                END,
                status = CASE
                    WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                      OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                    THEN 'pending'
                    ELSE s3_canonical_repair_queue.status
                END,
                last_error = CASE
                    WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                      OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                    THEN NULL
                    ELSE s3_canonical_repair_queue.last_error
                END,
                last_error_at = CASE
                    WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                      OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                    THEN NULL
                    ELSE s3_canonical_repair_queue.last_error_at
                END,
                locked_at = NULL,
                updated_at = NOW()
            RETURNING handle
        )
        SELECT COUNT(*)::BIGINT FROM upserted
        "#,
    )
    .bind(limit)
    .fetch_one(db_pool)
    .await?;

    Ok(enqueued.max(0) as u64)
}

async fn reconcile_verified_settled_publications(
    client: &Client,
    db_pool: &Pool<Postgres>,
    conf: &S3Config,
    signer: CoproSigner,
    limit: i64,
) -> Result<(), ExecutionError> {
    if limit <= 0 {
        return Ok(());
    }

    let rows = sqlx::query!(
        r#"
        SELECT d.host_chain_id AS "host_chain_id!",
               d.key_id_gw AS "key_id_gw!",
               d.handle AS "handle!",
               d.producer_block_hash AS "producer_block_hash!",
               d.block_hash AS "block_hash!",
               d.block_number,
               d.ciphertext AS "ciphertext!",
               d.ciphertext128,
               d.ciphertext128_format AS "ciphertext128_format?",
               d.s3_format_version,
               d.transaction_id
          FROM ciphertext_digest_branch d
          JOIN coprocessor_settlement s
            ON s.chain_id = d.host_chain_id
          LEFT JOIN host_chain_blocks_valid producer
            ON producer.chain_id = d.host_chain_id
           AND producer.block_hash = d.producer_block_hash
           AND d.producer_block_hash <> ''::BYTEA
          LEFT JOIN host_chain_blocks_valid event_block
            ON event_block.chain_id = d.host_chain_id
           AND event_block.block_hash = d.block_hash
           AND d.block_hash <> ''::BYTEA
         WHERE d.block_number IS NOT NULL
           AND d.block_number <= s.settled_height
           AND (
                d.producer_block_hash = ''::BYTEA
                OR COALESCE(producer.block_status, 'pending') <> 'orphaned'
           )
           AND (
                d.block_hash = ''::BYTEA
                OR COALESCE(event_block.block_status, 'pending') <> 'orphaned'
           )
           AND d.ciphertext IS NOT NULL
           AND (
                d.ciphertext128 IS NOT NULL
                OR NOT EXISTS (
                    SELECT 1
                      FROM ciphertexts128_branch c
                     WHERE c.handle = d.handle
                       AND c.producer_block_hash = d.producer_block_hash
                       AND c.ciphertext IS NOT NULL
                )
           )
           AND d.s3_publication_verified_at IS NOT NULL
           AND d.s3_publication_verified_digest IS NOT DISTINCT FROM d.ciphertext
           AND d.s3_publication_verified_producer_block_hash IS NOT DISTINCT FROM d.producer_block_hash
         ORDER BY d.s3_publication_verified_at ASC
         LIMIT $1
        "#,
        limit,
    )
    .fetch_all(db_pool)
    .await?;

    for row in rows {
        let host_chain_id_raw = row.host_chain_id;
        let key_id_gw = row.key_id_gw;
        let handle = row.handle;
        let producer_block_hash = row.producer_block_hash;
        let block_hash = row.block_hash;
        let block_number = row.block_number;
        let ct64_digest = row.ciphertext;
        let ct128_digest = row.ciphertext128;
        let ciphertext128_format = row.ciphertext128_format;
        let s3_format_version = row.s3_format_version;
        let transaction_id = row.transaction_id;

        let ct128_digest = ct128_digest.unwrap_or_else(|| NO_SNS_CIPHERTEXT_DIGEST.to_vec());
        let ct128_format = if ct128_digest.as_slice() == NO_SNS_CIPHERTEXT_DIGEST.as_slice() {
            Ciphertext128Format::Unknown
        } else {
            match ciphertext128_format.and_then(Ciphertext128Format::from_i16) {
                Some(format) => format,
                None => {
                    warn!(
                        handle = to_hex(&handle),
                        "Skipping S3 reconciliation for row with missing or invalid ct128 format"
                    );
                    continue;
                }
            }
        };

        let task = HandleItem {
            host_chain_id: ChainId::try_from(host_chain_id_raw)
                .map_err(|e| ExecutionError::ConversionError(e.into()))?,
            key_id_gw,
            handle: handle.clone(),
            producer_block_hash: producer_block_hash.clone(),
            block_hash: block_hash.clone(),
            block_number,
            ct64_compressed: Arc::new(Vec::new()),
            ct128: Arc::new(BigCiphertext::new(Vec::new(), ct128_format)),
            ct64_digest: Some(ct64_digest),
            ct128_digest: Some(ct128_digest),
            s3_format_version,
            span: Span::none(),
            transaction_id,
        };
        let upload_material =
            upload_material(&task).map_err(|err| ExecutionError::InternalError(err.to_string()))?;
        let attestation_format =
            if upload_material.ct128_digest.as_slice() == NO_SNS_CIPHERTEXT_DIGEST.as_slice() {
                CiphertextFormat::CompressedOnCpu
            } else {
                attestation_format(task.ct128.format())
                    .map_err(|err| ExecutionError::InternalError(err.to_string()))?
            };
        let expected_attestation = build_attestation_payload(
            &task,
            COPROCESSOR_CONTEXT_ID_1,
            &upload_material.ct64_digest,
            &upload_material.ct128_digest,
            attestation_format,
        )
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
        let preserve_legacy_s3_format =
            should_preserve_legacy_s3_format(&task, &upload_material.ct128_digest);

        let is_current = check_s3_publication(
            client,
            conf,
            &task,
            &expected_attestation,
            signer.address(),
            &upload_material,
            preserve_legacy_s3_format,
        )
        .await?;

        if is_current {
            mark_s3_publication_reverified(db_pool, &task).await?;
            continue;
        }

        enqueue_exact_s3_canonical_repair(db_pool, &task, "reconciler_attestation_mismatch")
            .await?;
        S3_CANONICAL_REPAIR_ENQUEUED_COUNTER.inc();
        S3_CANONICAL_RECONCILER_MISMATCH_COUNTER.inc();
    }

    Ok(())
}

async fn mark_s3_publication_reverified(
    db_pool: &Pool<Postgres>,
    task: &HandleItem,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        UPDATE ciphertext_digest_branch
           SET s3_publication_verified_at = NOW()
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = $3
           AND block_hash = $4
        "#,
        task.host_chain_id.as_i64(),
        &task.handle,
        &task.producer_block_hash,
        &task.block_hash,
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

async fn enqueue_exact_s3_canonical_repair(
    db_pool: &Pool<Postgres>,
    task: &HandleItem,
    reason: &str,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        INSERT INTO s3_canonical_repair_queue (
            host_chain_id,
            handle,
            target_producer_block_hash,
            target_block_hash,
            target_block_number,
            reason
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (host_chain_id, handle) DO UPDATE
        SET target_producer_block_hash = EXCLUDED.target_producer_block_hash,
            target_block_hash = EXCLUDED.target_block_hash,
            target_block_number = EXCLUDED.target_block_number,
            reason = EXCLUDED.reason,
            attempts = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN 0
                ELSE s3_canonical_repair_queue.attempts
            END,
            status = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN 'pending'
                ELSE s3_canonical_repair_queue.status
            END,
            last_error = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN NULL
                ELSE s3_canonical_repair_queue.last_error
            END,
            last_error_at = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN NULL
                ELSE s3_canonical_repair_queue.last_error_at
            END,
            locked_at = NULL,
            updated_at = NOW()
        "#,
        task.host_chain_id.as_i64(),
        &task.handle,
        &task.producer_block_hash,
        &task.block_hash,
        task.block_number,
        reason,
    )
    .execute(db_pool)
    .await?;

    Ok(())
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
    signer: CoproSigner,
) -> Result<(), ExecutionError> {
    let expected_ct128_format = if conf.enable_compression {
        Ciphertext128Format::CompressedOnCpu
    } else {
        Ciphertext128Format::UncompressedOnCpu
    };

    // Retry to resubmit all upload tasks at the start-up
    if is_ready.load(Ordering::Acquire) {
        reconcile_s3_canonical_publications(
            &client,
            &pool,
            &conf.s3,
            signer.clone(),
            DEFAULT_BATCH_SIZE,
        )
        .await
        .unwrap_or_else(|err| {
            error!(error = %err, "Failed to reconcile S3 canonical publications");
        });
    }
    try_resubmit(
        &pool,
        is_ready.clone(),
        tasks.clone(),
        token.clone(),
        DEFAULT_BATCH_SIZE,
        expected_ct128_format,
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
                        reconcile_s3_canonical_publications(&client, &pool, &conf.s3, signer.clone(), DEFAULT_BATCH_SIZE).await
                            .unwrap_or_else(|err| {
                                error!(error = %err, "Failed to reconcile S3 canonical publications");
                            });
                        try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone(), DEFAULT_BATCH_SIZE, expected_ct128_format).await
                            .unwrap_or_else(|err| {
                                error!(error = %err, "Failed to resubmit tasks");
                            });
                    }
                }
            }
            // A regular resubmit to ensure there no remaining tasks
            _ = resubmit_ticker.tick() => {
                info!("Retry resubmit ...");
                if is_ready.load(Ordering::Acquire) {
                    reconcile_s3_canonical_publications(&client, &pool, &conf.s3, signer.clone(), DEFAULT_BATCH_SIZE).await
                        .unwrap_or_else(|err| {
                            error!(error = %err, "Failed to reconcile S3 canonical publications");
                        });
                }
                try_resubmit(&pool, is_ready.clone(), tasks.clone(), token.clone(), DEFAULT_BATCH_SIZE, expected_ct128_format).await
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
    expected_ct128_format: Ciphertext128Format,
) -> Result<(), ExecutionError> {
    loop {
        if !is_ready.load(Ordering::SeqCst) {
            info!("S3 setup is not ready, skipping resubmit");
            return Ok(());
        }

        match fetch_pending_uploads(pool, batch_size as i64, expected_ct128_format).await {
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
    binding: AttestationBinding,
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
                validate_existing_attestation(expected, expected_signer, &attestation, binding)
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
        AttestationBinding::HandleBound,
    )
    .await?;
    let digest_key_exists = check_attested_object_exists(
        client,
        bucket,
        digest_key,
        expected,
        expected_signer,
        expected_checksum_sha256,
        AttestationBinding::ContentBound,
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

#[allow(clippy::too_many_arguments)]
async fn repair_ct128_object_pair_from_existing_copy(
    client: &Client,
    bucket: &str,
    key: &str,
    digest_key: &str,
    expected: &CiphertextAttestationPayload,
    expected_signer: Address,
    expected_checksum_sha256: Option<&str>,
    verify_sha256_checksum: bool,
    canonical_metadata: &S3ObjectMetadata,
    canonical_ct_format: &str,
) -> Result<bool, ExecutionError> {
    let key_exists = check_attested_object_exists(
        client,
        bucket,
        key,
        expected,
        expected_signer,
        expected_checksum_sha256,
        AttestationBinding::HandleBound,
    )
    .await?;
    let digest_key_exists = check_attested_object_exists(
        client,
        bucket,
        digest_key,
        expected,
        expected_signer,
        expected_checksum_sha256,
        AttestationBinding::ContentBound,
    )
    .await?;

    match (key_exists, digest_key_exists) {
        (true, true) => Ok(true),
        (false, true) => {
            // The digest copy's attestation may be bound to another handle
            // sharing the same ciphertext bytes; the canonical key must carry
            // an attestation for THIS handle or it fails every handle-bound
            // check after the copy. Rebind the metadata while copying.
            copy_s3_object(
                client,
                bucket,
                digest_key,
                key,
                verify_sha256_checksum,
                "canonical ct128 key",
                Some((canonical_metadata, canonical_ct_format)),
            )
            .await?;
            Ok(true)
        }
        (true, false) => {
            copy_s3_object(
                client,
                bucket,
                key,
                digest_key,
                verify_sha256_checksum,
                "ct128 digest compatibility key",
                None,
            )
            .await?;
            Ok(true)
        }
        (false, false) => Ok(false),
    }
}

async fn copy_s3_object(
    client: &Client,
    bucket: &str,
    source_key: &str,
    destination_key: &str,
    verify_sha256_checksum: bool,
    destination_description: &'static str,
    replace_metadata: Option<(&S3ObjectMetadata, &str)>,
) -> Result<(), ExecutionError> {
    let copy_source = format!("{}/{}", bucket, source_key);
    let mut copy = client
        .copy_object()
        .copy_source(copy_source)
        .bucket(bucket)
        .key(destination_key)
        .set_checksum_algorithm(verify_sha256_checksum.then_some(ChecksumAlgorithm::Sha256));
    if let Some((metadata, ct_format)) = replace_metadata {
        copy = copy
            .metadata_directive(MetadataDirective::Replace)
            .metadata("Ct-Format", ct_format)
            .metadata("Uploaded-By", "sns-worker")
            .metadata(S3_METADATA_ATTESTATION_KEY, &metadata.attestation_json)
            .metadata("Key-Id", &metadata.key_id)
            .metadata("Transaction-Id", &metadata.transaction_id)
            .metadata("Signer", &metadata.signer);
    }
    let copy_result = copy.send().await;

    match copy_result {
        Ok(_) => {
            info!(
                bucket,
                source_key,
                destination_key,
                destination_description,
                "Repaired S3 object with server-side copy"
            );
            Ok(())
        }
        Err(err) => {
            error!(
                bucket,
                source_key,
                destination_key,
                error = %err,
                "Failed to repair S3 object with server-side copy"
            );
            Err(ExecutionError::S3TransientError(err.to_string()))
        }
    }
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
    use serial_test::serial;
    use sqlx::Row;
    use std::time::Duration;
    use test_harness::{
        instance::{setup_test_db, ImportMode},
        localstack::{self, create_localstack_s3_client},
    };

    fn sample_handle_item() -> HandleItem {
        let ct128_format = Ciphertext128Format::CompressedOnCpu;

        HandleItem {
            host_chain_id: ChainId::try_from(1_i64).unwrap(),
            key_id_gw: vec![7; 32],
            handle: vec![2; 32],
            producer_block_hash: vec![3; 32],
            block_hash: vec![4; 32],
            block_number: Some(1),
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

        validate_existing_attestation(
            &expected,
            expected_signer,
            &attestation,
            AttestationBinding::HandleBound,
        )
        .unwrap();
    }

    #[tokio::test]
    async fn expected_attestation_rejects_stale_digest_metadata() {
        let (expected, expected_signer, mut attestation) = sample_attestation().await;
        attestation.sns_ciphertext_digest = B256::ZERO;

        for binding in [
            AttestationBinding::HandleBound,
            AttestationBinding::ContentBound,
        ] {
            let err =
                validate_existing_attestation(&expected, expected_signer, &attestation, binding)
                    .unwrap_err();
            assert!(err.contains("sns ciphertext digest mismatch"));
        }
    }

    #[tokio::test]
    async fn expected_attestation_rejects_wrong_context_metadata() {
        let (mut expected, expected_signer, attestation) = sample_attestation().await;
        expected.coprocessor_context_id = U256::ZERO;

        let err = validate_existing_attestation(
            &expected,
            expected_signer,
            &attestation,
            AttestationBinding::HandleBound,
        )
        .unwrap_err();
        assert!(err.contains("handle/context/signature mismatch"));
    }

    #[tokio::test]
    async fn expected_attestation_rejects_wrong_signer_metadata() {
        let (expected, expected_signer, _) = sample_attestation().await;
        let other_signer: CoproSigner = Arc::new(PrivateKeySigner::random());
        let attestation = build_attestation(&expected, &other_signer).await.unwrap();

        for binding in [
            AttestationBinding::HandleBound,
            AttestationBinding::ContentBound,
        ] {
            let err =
                validate_existing_attestation(&expected, expected_signer, &attestation, binding)
                    .unwrap_err();
            assert!(err.contains("signer mismatch"));
        }
    }

    /// A digest-keyed compatibility copy is shared by every handle whose
    /// ciphertext bytes are identical, so its attestation is signed for
    /// whichever handle wrote it. Content-bound validation must accept it;
    /// handle-bound validation must keep rejecting it (canonical objects are
    /// never shared).
    #[tokio::test]
    async fn cross_handle_attestation_passes_content_bound_only() {
        let (expected, _, _) = sample_attestation().await;
        let signer: CoproSigner = Arc::new(PrivateKeySigner::random());
        let expected_signer = signer.address();

        let mut other_task = sample_handle_item();
        other_task.handle = vec![9; 32];
        let upload_material = upload_material(&other_task).unwrap();
        let format = attestation_format(other_task.ct128.format()).unwrap();
        let other_payload = build_attestation_payload(
            &other_task,
            COPROCESSOR_CONTEXT_ID_1,
            &upload_material.ct64_digest,
            &upload_material.ct128_digest,
            format,
        )
        .unwrap();
        let attestation = build_attestation(&other_payload, &signer).await.unwrap();

        validate_existing_attestation(
            &expected,
            expected_signer,
            &attestation,
            AttestationBinding::ContentBound,
        )
        .unwrap();

        let err = validate_existing_attestation(
            &expected,
            expected_signer,
            &attestation,
            AttestationBinding::HandleBound,
        )
        .unwrap_err();
        assert!(err.contains("handle/context/signature mismatch"));
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
            producer_block_hash: vec![3; 32],
            block_hash: vec![4; 32],
            block_number: Some(1),
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
            producer_block_hash: vec![3; 32],
            block_hash: vec![4; 32],
            block_number: Some(1),
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
            producer_block_hash: vec![3; 32],
            block_hash: vec![4; 32],
            block_number: Some(1),
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
            producer_block_hash: vec![3; 32],
            block_hash: vec![4; 32],
            block_number: Some(1),
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
        let client = create_localstack_s3_client(localstack.host_port).await;
        let bucket_ct64 = "legacy-partial-ct64";
        let bucket_ct128 = "legacy-partial-ct128-not-created";
        client.create_bucket().bucket(bucket_ct64).send().await?;

        let handle = vec![0x42; 32];
        let producer_block_hash = vec![0x24; 32];
        let block_hash = vec![0x25; 32];
        let key_id_gw = vec![0x07; 32];
        let ct64 = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let ct128_digest = vec![0x11; 32];
        let ct128_format: i16 = Ciphertext128Format::CompressedOnCpu.into();

        sqlx::query(
            "INSERT INTO ciphertext_digest_branch (
                host_chain_id,
                key_id_gw,
                handle,
                producer_block_hash,
                block_hash,
                block_number,
                ciphertext128,
                ciphertext128_format,
                s3_format_version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(1_i64)
        .bind(&key_id_gw)
        .bind(&handle)
        .bind(&producer_block_hash)
        .bind(&block_hash)
        .bind(1_i64)
        .bind(&ct128_digest)
        .bind(ct128_format)
        .bind(S3_FORMAT_VERSION_LEGACY)
        .execute(&pool)
        .await?;

        sqlx::query(
            "INSERT INTO pbs_computations_branch (
                handle,
                host_chain_id,
                block_number,
                producer_block_hash,
                block_hash,
                is_completed
            )
            VALUES ($1, $2, $3, $4, $5, TRUE)",
        )
        .bind(&handle)
        .bind(1_i64)
        .bind(1_i64)
        .bind(&producer_block_hash)
        .bind(&block_hash)
        .execute(&pool)
        .await?;

        let task = HandleItem {
            host_chain_id: ChainId::try_from(1_i64).unwrap(),
            key_id_gw,
            handle: handle.clone(),
            producer_block_hash: producer_block_hash.clone(),
            block_hash: block_hash.clone(),
            block_number: Some(1),
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

        let row = sqlx::query(
            "SELECT ciphertext, ciphertext128, s3_format_version
             FROM ciphertext_digest_branch
             WHERE handle = $1
               AND producer_block_hash = $2
               AND block_hash = $3",
        )
        .bind(&handle)
        .bind(&producer_block_hash)
        .bind(&block_hash)
        .fetch_one(&pool)
        .await?;

        let ciphertext: Option<Vec<u8>> = row.try_get("ciphertext")?;
        let ciphertext128: Option<Vec<u8>> = row.try_get("ciphertext128")?;
        let s3_format_version: Option<i16> = row.try_get("s3_format_version")?;
        assert_eq!(
            ciphertext.as_deref(),
            Some(compute_digest(&ct64).as_slice())
        );
        assert_eq!(ciphertext128.as_deref(), Some(ct128_digest.as_slice()));
        assert_eq!(s3_format_version, Some(S3_FORMAT_VERSION_LEGACY));

        let ct64_key = s3_ciphertext_key(&handle, COPROCESSOR_CONTEXT_ID_1);
        client
            .head_object()
            .bucket(bucket_ct64)
            .key(ct64_key)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    #[serial(s3)]
    #[cfg(not(feature = "gpu"))]
    async fn ct128_pair_repair_copies_only_valid_attested_existing_object() -> anyhow::Result<()> {
        let localstack = localstack::start_localstack().await?;
        let client = create_localstack_s3_client(localstack.host_port).await;
        let bucket = "ct128-copy-repair";
        client.create_bucket().bucket(bucket).send().await?;

        let (expected, expected_signer, attestation) = sample_attestation().await;
        let attestation_json = serde_json::to_string(&attestation)?;
        let key = "canonical/1";
        let digest_key = "digest-key";
        let body = vec![0xCA_u8, 0xFE, 0xBA, 0xBE];

        client
            .put_object()
            .bucket(bucket)
            .key(digest_key)
            .metadata(S3_METADATA_ATTESTATION_KEY, &attestation_json)
            .body(ByteStream::from(body.clone()))
            .send()
            .await?;

        let canonical_metadata = S3ObjectMetadata {
            attestation_json: attestation_json.clone(),
            key_id: hex::encode(vec![7_u8; 32]),
            transaction_id: String::new(),
            signer: expected_signer.to_string(),
        };
        let canonical_ct_format = Ciphertext128Format::CompressedOnCpu.to_string();

        let repaired = repair_ct128_object_pair_from_existing_copy(
            &client,
            bucket,
            key,
            digest_key,
            &expected,
            expected_signer,
            None,
            false,
            &canonical_metadata,
            &canonical_ct_format,
        )
        .await?;
        assert!(repaired);

        let copied_head = client.head_object().bucket(bucket).key(key).send().await?;
        assert_eq!(
            copied_head
                .metadata()
                .and_then(|m| m.get(S3_METADATA_ATTESTATION_KEY)),
            Some(&attestation_json),
            "repaired canonical object must carry the rebound attestation"
        );
        let copied = client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?
            .body
            .collect()
            .await?
            .to_vec();
        assert_eq!(copied, body);

        let bad_key = "bad-canonical/1";
        let bad_digest_key = "bad-digest-key";
        let mut stale_attestation = attestation.clone();
        stale_attestation.sns_ciphertext_digest = B256::ZERO;
        client
            .put_object()
            .bucket(bucket)
            .key(bad_digest_key)
            .metadata(
                S3_METADATA_ATTESTATION_KEY,
                serde_json::to_string(&stale_attestation)?,
            )
            .body(ByteStream::from(vec![0x11_u8, 0x22]))
            .send()
            .await?;

        let repaired = repair_ct128_object_pair_from_existing_copy(
            &client,
            bucket,
            bad_key,
            bad_digest_key,
            &expected,
            expected_signer,
            None,
            false,
            &canonical_metadata,
            &canonical_ct_format,
        )
        .await?;
        assert!(
            !repaired,
            "stale attestation metadata must not be used as a repair source"
        );
        let missing = client
            .head_object()
            .bucket(bucket)
            .key(bad_key)
            .send()
            .await;
        assert!(
            matches!(missing, Err(SdkError::ServiceError(err)) if matches!(err.err(), HeadObjectError::NotFound(_))),
            "canonical key must remain absent when the only source is stale"
        );

        Ok(())
    }

    #[tokio::test]
    #[serial(db)]
    async fn canonical_repair_quarantines_and_only_new_target_requeues() -> anyhow::Result<()> {
        let db = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(db.db_url())
            .await?;
        let task = sample_handle_item();

        sqlx::query(
            "INSERT INTO s3_canonical_repair_queue(
                host_chain_id, handle, target_producer_block_hash, target_block_hash,
                target_block_number, reason, attempts, locked_at
             ) VALUES ($1, $2, $3, $4, $5, 'test', $6, NOW())",
        )
        .bind(task.host_chain_id.as_i64())
        .bind(&task.handle)
        .bind(&task.producer_block_hash)
        .bind(&task.block_hash)
        .bind(task.block_number)
        .bind(S3_CANONICAL_REPAIR_MAX_ATTEMPTS)
        .execute(&pool)
        .await?;

        record_canonical_repair_failure(
            &pool,
            task.host_chain_id.as_i64(),
            &task.handle,
            &task.producer_block_hash,
            &task.block_hash,
            "permanent repair failure",
        )
        .await?;

        let state: (String, i32, bool, Option<String>) = sqlx::query_as(
            "SELECT status, attempts, locked_at IS NULL, last_error
               FROM s3_canonical_repair_queue
              WHERE host_chain_id = $1 AND handle = $2",
        )
        .bind(task.host_chain_id.as_i64())
        .bind(&task.handle)
        .fetch_one(&pool)
        .await?;
        assert_eq!(state.0, "quarantined");
        assert_eq!(state.1, S3_CANONICAL_REPAIR_MAX_ATTEMPTS);
        assert!(state.2);
        assert_eq!(state.3.as_deref(), Some("permanent repair failure"));

        enqueue_exact_s3_canonical_repair(&pool, &task, "same_target").await?;
        let same_target: (String, i32) = sqlx::query_as(
            "SELECT status, attempts FROM s3_canonical_repair_queue
              WHERE host_chain_id = $1 AND handle = $2",
        )
        .bind(task.host_chain_id.as_i64())
        .bind(&task.handle)
        .fetch_one(&pool)
        .await?;
        assert_eq!(same_target.0, "quarantined");
        assert_eq!(same_target.1, S3_CANONICAL_REPAIR_MAX_ATTEMPTS);

        let mut replacement = task.clone();
        replacement.producer_block_hash = vec![0xA1; 32];
        replacement.block_hash = vec![0xA2; 32];
        replacement.block_number = Some(2);
        enqueue_exact_s3_canonical_repair(&pool, &replacement, "new_target").await?;

        let new_target: (String, i32, Option<String>) = sqlx::query_as(
            "SELECT status, attempts, last_error FROM s3_canonical_repair_queue
              WHERE host_chain_id = $1 AND handle = $2",
        )
        .bind(task.host_chain_id.as_i64())
        .bind(&task.handle)
        .fetch_one(&pool)
        .await?;
        assert_eq!(new_target.0, "pending");
        assert_eq!(new_target.1, 0);
        assert!(new_target.2.is_none());

        sqlx::query(
            "UPDATE s3_canonical_repair_queue
                SET attempts = $3, locked_at = NOW() - INTERVAL '6 minutes'
              WHERE host_chain_id = $1 AND handle = $2",
        )
        .bind(task.host_chain_id.as_i64())
        .bind(&task.handle)
        .bind(S3_CANONICAL_REPAIR_MAX_ATTEMPTS)
        .execute(&pool)
        .await?;

        let jobs = fetch_pending_canonical_repairs(&pool, 1).await?;
        assert!(jobs.is_empty());
        let expired_lease: (String, Option<String>) = sqlx::query_as(
            "SELECT status, last_error FROM s3_canonical_repair_queue
              WHERE host_chain_id = $1 AND handle = $2",
        )
        .bind(task.host_chain_id.as_i64())
        .bind(&task.handle)
        .fetch_one(&pool)
        .await?;
        assert_eq!(expired_lease.0, "quarantined");
        assert_eq!(
            expired_lease.1.as_deref(),
            Some("repair lease expired after maximum attempts")
        );

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
