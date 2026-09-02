use std::{sync::Arc, time::Duration};

use alloy_primitives::U256;
use aws_sdk_s3::{error::ProvideErrorMetadata, primitives::ByteStream, Client};
use block_manifest::{SignedManifest, MAX_MANIFEST_BYTES};
use fhevm_engine_common::{pg_pool::is_fatal_connection_error, types::CoproSigner};
use sqlx::PgPool;
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::{
    manifest_consensus::{
        db_error::{classify_db_error, DbErrorClass},
        Config as ManifestConsensusConfig, ExecutionError, ManifestWorkGate,
    },
    Config,
};

use crate::manifest_consensus::{
    manifest_archive::{manifest_object_key, store_authenticated_manifest, ManifestSource},
    publication::{
        block_discovery::{
            discover_blocks_for_generation, discover_children_for_generation, discover_children_of,
            lock_next_block_to_progress_for_generation, pending_chain_ids_for_generation,
            ManifestProgressCursor, PendingBlock,
        },
        manifest_builder::{
            is_block_manifest_ready, load_manifest_descriptors, missing_handles_are_uncomputed,
            prepare_manifest, seal_block_content,
        },
        publication_status::{
            exhaust_manifest_publication_error, mark_manifest_published,
            record_manifest_publication_error, retry_skipped_predecessor_once,
        },
    },
};

use super::metrics::{
    MANIFEST_PUBLICATION_FAILURE, MANIFEST_PUBLICATION_SUCCESS, MANIFEST_WORK_SELECTION_TIMEOUT,
};

// Manifests are small. Bound the entire immutable put/recovery operation so a
// slow S3 request cannot retain the selected database row indefinitely.
const MANIFEST_S3_OPERATION_TIMEOUT: Duration = Duration::from_secs(15);
/// Coprocessor context id committed in every V1 block-content digest and
/// signed manifest. V1 ciphertext objects are stored under context `1`
/// (`s3_ct64_key` / `s3_ct128_key`); publication must use the same value so
/// descriptors match those objects. Not operator-configurable.
const COPROCESSOR_CONTEXT_ID_1: U256 = U256::ONE;

struct ManifestPublisherContext {
    bucket: String,
    client: Arc<Client>,
    signer: CoproSigner,
    consensus: ManifestConsensusConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PublicationProgress {
    Advanced,
    Waiting,
    WorkBusy,
}

pub(crate) fn spawn_manifest_publisher(
    pool: PgPool,
    token: CancellationToken,
    conf: Config,
    client: Arc<Client>,
    signer: CoproSigner,
    work_gate: Arc<ManifestWorkGate>,
) -> JoinHandle<Result<(), ExecutionError>> {
    tokio::spawn(run_manifest_publisher(
        pool, token, conf, client, signer, work_gate,
    ))
}

async fn run_manifest_publisher(
    pool: PgPool,
    token: CancellationToken,
    conf: Config,
    client: Arc<Client>,
    signer: CoproSigner,
    work_gate: Arc<ManifestWorkGate>,
) -> Result<(), ExecutionError> {
    let poll_interval = conf.manifest_consensus.discovery_interval;
    if poll_interval.is_zero() {
        return Err(ExecutionError::InternalError(
            "--manifest-discovery-interval must be greater than zero".to_owned(),
        ));
    }
    run_manifest_publisher_with_poll_interval(
        pool,
        token,
        ManifestPublisherContext {
            bucket: conf.my_bucket.ok_or_else(|| {
                ExecutionError::InternalError(
                    "manifest publication requires consensus-detector --my-bucket".to_owned(),
                )
            })?,
            client,
            signer,
            consensus: conf.manifest_consensus,
        },
        poll_interval,
        work_gate,
    )
    .await
}

async fn run_manifest_publisher_with_poll_interval(
    pool: PgPool,
    token: CancellationToken,
    context: ManifestPublisherContext,
    poll_interval: Duration,
    work_gate: Arc<ManifestWorkGate>,
) -> Result<(), ExecutionError> {
    let ManifestPublisherContext {
        bucket,
        client,
        signer,
        consensus,
    } = context;
    info!(
        discovery_interval = ?poll_interval,
        "Manifest publication enabled for this stack generation"
    );

    let mut ticker = tokio::time::interval(poll_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = ticker.tick() => {}
        }

        match publish_due_work(&pool, &client, &bucket, &signer, &consensus, &work_gate).await {
            Ok(()) => {}
            Err(ExecutionError::DbError(err)) if is_fatal_connection_error(&err) => {
                return Err(ExecutionError::DbError(err));
            }
            Err(ExecutionError::DbError(err)) => {
                error!(
                    error = %err,
                    "Manifest publication database error; retrying on the next interval"
                );
            }
            Err(err) => return Err(err),
        }
    }
}

async fn publish_due_work(
    pool: &PgPool,
    client: &Client,
    bucket: &str,
    signer: &CoproSigner,
    consensus: &ManifestConsensusConfig,
    work_gate: &ManifestWorkGate,
) -> Result<(), ExecutionError> {
    let Some(generation) = work_gate.pinned_generation() else {
        return Ok(());
    };

    let discovered =
        discover_blocks_for_generation(pool, &generation, &consensus.publication_cadence_overrides)
            .await?;
    if discovered > 0 {
        debug!(discovered, "Discovered manifest blocks");
    }
    if !work_gate.work_enabled_for(&generation) {
        return Ok(());
    }
    let descendants = discover_children_for_generation(pool, &generation).await?;
    if descendants > 0 {
        debug!(descendants, "Discovered manifest lineage blocks");
    }

    loop {
        if !work_gate.work_enabled_for(&generation) {
            return Ok(());
        }
        let chain_ids = pending_chain_ids_for_generation(pool, &generation).await?;
        if chain_ids.is_empty() {
            return Ok(());
        }

        let mut advanced = false;
        for host_chain_id in chain_ids {
            if !work_gate.work_enabled_for(&generation) {
                return Ok(());
            }
            match progress_chain(
                pool,
                client,
                bucket,
                host_chain_id,
                signer,
                consensus,
                work_gate,
                &generation,
            )
            .await
            {
                Ok(PublicationProgress::Advanced) => advanced = true,
                Ok(PublicationProgress::Waiting | PublicationProgress::WorkBusy) => {}
                Err(ExecutionError::DbError(err)) if is_fatal_connection_error(&err) => {
                    return Err(ExecutionError::DbError(err));
                }
                Err(ExecutionError::DbError(err)) => {
                    error!(
                        host_chain_id,
                        error = %err,
                        "Manifest publication database error; continuing with another chain"
                    );
                }
                Err(err) => {
                    error!(host_chain_id, error = %err, "Manifest publication attempt failed");
                }
            }
        }

        if !advanced {
            return Ok(());
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn progress_chain(
    pool: &PgPool,
    client: &Client,
    bucket: &str,
    host_chain_id: i64,
    signer: &CoproSigner,
    consensus: &ManifestConsensusConfig,
    work_gate: &ManifestWorkGate,
    generation: &str,
) -> Result<PublicationProgress, ExecutionError> {
    let mut cursor = ManifestProgressCursor::start();
    let mut attempted_candidate = false;
    let max_attempts = i64::from(consensus.publication_retry_count) + 1;
    let retry_delay_micros =
        i64::try_from(consensus.publication_retry_delay.as_micros()).map_err(|_| {
            ExecutionError::InternalError("publication retry delay exceeds BIGINT".into())
        })?;

    loop {
        if !work_gate.work_enabled_for(generation) {
            return Ok(PublicationProgress::Waiting);
        }
        let mut trx = pool.begin().await?;
        let selected = match lock_next_block_to_progress_for_generation(
            &mut trx,
            host_chain_id,
            &cursor,
            generation,
        )
        .await
        {
            Ok(selected) => selected,
            Err(ExecutionError::DbError(error)) if is_statement_timeout(&error) => {
                trx.rollback().await?;
                MANIFEST_WORK_SELECTION_TIMEOUT
                    .with_label_values(&[generation])
                    .inc();
                warn!(
                    host_chain_id,
                    "Manifest work selection timed out; retrying later"
                );
                return Ok(PublicationProgress::Waiting);
            }
            Err(ExecutionError::DbError(error)) if is_fatal_connection_error(&error) => {
                return Err(ExecutionError::DbError(error));
            }
            Err(ExecutionError::DbError(error)) => {
                trx.rollback().await.ok();
                warn!(
                    host_chain_id,
                    error = %error,
                    "Manifest work selection failed; retrying later"
                );
                return Ok(PublicationProgress::Waiting);
            }
            Err(error) => return Err(error),
        };
        let Some(block) = selected else {
            trx.rollback().await?;
            return Ok(if attempted_candidate {
                PublicationProgress::Waiting
            } else {
                PublicationProgress::WorkBusy
            });
        };
        attempted_candidate = true;
        cursor.advance_to(&block);

        let result = progress_locked_block(
            &mut trx,
            client,
            bucket,
            host_chain_id,
            &block,
            signer,
            consensus,
        )
        .await;
        if !work_gate.work_enabled_for(generation) {
            trx.rollback().await?;
            return Ok(PublicationProgress::Waiting);
        }
        match result {
            Ok(PublicationProgress::Advanced) => {
                trx.commit().await?;
                if block.block_content_digest.is_some() {
                    MANIFEST_PUBLICATION_SUCCESS
                        .with_label_values(&[generation])
                        .inc();
                }
                return Ok(PublicationProgress::Advanced);
            }
            Ok(PublicationProgress::Waiting) => {
                trx.commit().await?;
            }
            Ok(PublicationProgress::WorkBusy) => {
                unreachable!("a selected work row cannot report row contention")
            }
            Err(ExecutionError::DbError(err)) if is_fatal_connection_error(&err) => {
                return Err(ExecutionError::DbError(err));
            }
            Err(err) => {
                trx.rollback().await?;
                record_candidate_failure(pool, &block, &err, max_attempts, retry_delay_micros)
                    .await;
            }
        }
    }
}

async fn record_candidate_failure(
    pool: &PgPool,
    block: &PendingBlock,
    err: &ExecutionError,
    max_attempts: i64,
    retry_delay_micros: i64,
) {
    let db_class = match err {
        ExecutionError::DbError(error) => Some(classify_db_error(error)),
        _ => None,
    };
    if db_class == Some(DbErrorClass::Transient) {
        warn!(
            host_chain_id = block.host_chain_id,
            block_number = block.block_number,
            block_hash = %hex::encode(&block.block_hash),
            error = %err,
            "Transient manifest database error; retrying later without charging the budget"
        );
        return;
    }

    let exhaust_now = db_class == Some(DbErrorClass::Definitive);
    if block.block_content_digest.is_some() {
        MANIFEST_PUBLICATION_FAILURE
            .with_label_values(&[&block.generation])
            .inc();
        let recorded = if exhaust_now {
            exhaust_manifest_publication_error(pool, block, &err.to_string(), max_attempts).await
        } else {
            record_manifest_publication_error(
                pool,
                block,
                &err.to_string(),
                max_attempts,
                retry_delay_micros,
            )
            .await
        };
        if let Err(record_error) = recorded {
            error!(
                host_chain_id = block.host_chain_id,
                block_number = block.block_number,
                block_hash = %hex::encode(&block.block_hash),
                error = %record_error,
                "Failed to record manifest publication error"
            );
        }
    }
    error!(
        host_chain_id = block.host_chain_id,
        block_number = block.block_number,
        block_hash = %hex::encode(&block.block_hash),
        error = %err,
        exhaust_now,
        "Manifest candidate failed; continuing with another lineage"
    );
}

fn is_statement_timeout(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|error| error.code())
        .is_some_and(|code| code == "57014")
}

async fn progress_locked_block(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client: &Client,
    bucket: &str,
    host_chain_id: i64,
    block: &PendingBlock,
    signer: &CoproSigner,
    consensus: &ManifestConsensusConfig,
) -> Result<PublicationProgress, ExecutionError> {
    if block.block_content_digest.is_none() {
        let allow_uncomputed = missing_handles_are_uncomputed(trx, block, consensus).await?;
        if !is_block_manifest_ready(trx, block).await? && !allow_uncomputed {
            return Ok(PublicationProgress::Waiting);
        }
        let descriptors = load_manifest_descriptors(trx, block, allow_uncomputed).await?;
        seal_block_content(trx, block, COPROCESSOR_CONTEXT_ID_1, &descriptors).await?;
        let discovered = discover_children_of(trx, block).await?;
        debug!(
            host_chain_id,
            block_number = block.block_number,
            block_hash = %hex::encode(&block.block_hash),
            block_handle_count = descriptors.len(),
            discovered_children = discovered,
            "Sealed block manifest content"
        );
        return Ok(PublicationProgress::Advanced);
    }

    match publish_block_manifest(trx, client, bucket, block, signer, consensus).await {
        Ok(()) => Ok(PublicationProgress::Advanced),
        Err(ExecutionError::StaleBlockSeal {
            host_chain_id,
            block_number,
            ref reason,
        }) => {
            debug!(
                host_chain_id,
                block_number, reason, "Discarded stale unpublished seal; block will reseal"
            );
            Ok(PublicationProgress::Waiting)
        }
        Err(ExecutionError::PredecessorUnsealed {
            host_chain_id,
            block_number,
        }) => {
            debug!(
                host_chain_id,
                block_number, "Waiting for an unsealed predecessor before publishing this manifest"
            );
            Ok(PublicationProgress::Waiting)
        }
        Err(error) => Err(error),
    }
}

pub(crate) async fn publish_block_manifest(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client: &Client,
    bucket: &str,
    block: &PendingBlock,
    signer: &CoproSigner,
    consensus: &ManifestConsensusConfig,
) -> Result<(), ExecutionError> {
    let prepared = prepare_manifest(trx, block, COPROCESSOR_CONTEXT_ID_1, signer.address()).await?;
    let frontier_range_count = prepared.history_frontier.as_slice().len();
    let signed = prepared
        .payload
        .sign(signer.as_ref())
        .await
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
    signed
        .verify()
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
    let manifest_digest = signed
        .digest()
        .map_err(|err| ExecutionError::InternalError(err.to_string()))?;
    let body = serde_json::to_vec(&signed)
        .map_err(|err| ExecutionError::SerializationError(err.to_string()))?;
    let key = manifest_object_key(&signed);
    ensure_manifest_size(body.len(), bucket, &key)?;

    let stored_body = await_manifest_s3_operation(
        MANIFEST_S3_OPERATION_TIMEOUT,
        put_immutable_manifest(client, bucket, &key, &signed, &body),
    )
    .await?;
    let archived = store_authenticated_manifest(
        trx,
        signed.payload.publisher,
        &key,
        &stored_body,
        ManifestSource::Local,
    )
    .await?;
    if archived.manifest.digest != manifest_digest {
        return Err(ExecutionError::InternalError(format!(
            "stored manifest digest {} does not match published digest {manifest_digest}",
            archived.manifest.digest,
        )));
    }
    mark_manifest_published(trx, block, signed.payload.publisher, manifest_digest).await?;
    retry_skipped_predecessor_once(trx, block, i64::from(consensus.publication_retry_count) + 1)
        .await?;
    info!(
        host_chain_id = block.host_chain_id,
        block_number = block.block_number,
        block_hash = %hex::encode(&block.block_hash),
        revision = block.manifest_revision,
        manifest_digest = %manifest_digest,
        frontier_range_count,
        bucket,
        key,
        archive_outcome = ?archived.outcome,
        "Published immutable consensus manifest"
    );
    Ok(())
}

async fn await_manifest_s3_operation<T>(
    timeout: Duration,
    operation: impl std::future::Future<Output = Result<T, ExecutionError>>,
) -> Result<T, ExecutionError> {
    tokio::time::timeout(timeout, operation)
        .await
        .map_err(|_| {
            ExecutionError::S3TransientError(format!(
                "manifest S3 operation timed out after {} seconds",
                timeout.as_secs_f64(),
            ))
        })?
}

async fn put_immutable_manifest(
    client: &Client,
    bucket: &str,
    key: &str,
    intended: &SignedManifest,
    body: &[u8],
) -> Result<Vec<u8>, ExecutionError> {
    let result = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .content_type("application/json")
        .if_none_match("*")
        .body(ByteStream::from(body.to_vec()))
        .send()
        .await;

    match result {
        Ok(_) => Ok(body.to_vec()),
        Err(err)
            if err.as_service_error().and_then(ProvideErrorMetadata::code)
                == Some("PreconditionFailed") =>
        {
            let existing = download_existing_manifest(client, bucket, key).await?;
            validate_existing_manifest(&existing, intended, bucket, key)?;
            Ok(existing)
        }
        Err(err) => Err(ExecutionError::S3TransientError(err.to_string())),
    }
}

async fn download_existing_manifest(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<Vec<u8>, ExecutionError> {
    let response = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|err| ExecutionError::S3TransientError(err.to_string()))?;
    ensure_manifest_length(response.content_length(), bucket, key)?;

    let mut stream = response.body;
    let mut body = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|err| ExecutionError::S3TransientError(err.to_string()))?;
        ensure_manifest_size(body.len().saturating_add(chunk.len()), bucket, key)?;
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn ensure_manifest_length(
    content_length: Option<i64>,
    bucket: &str,
    key: &str,
) -> Result<(), ExecutionError> {
    if content_length.is_some_and(|length| length < 0 || length as usize > MAX_MANIFEST_BYTES) {
        return Err(manifest_too_large_error(bucket, key));
    }
    Ok(())
}

fn ensure_manifest_size(size: usize, bucket: &str, key: &str) -> Result<(), ExecutionError> {
    if size > MAX_MANIFEST_BYTES {
        return Err(manifest_too_large_error(bucket, key));
    }
    Ok(())
}

fn manifest_too_large_error(bucket: &str, key: &str) -> ExecutionError {
    ExecutionError::InternalError(format!(
        "immutable manifest object at s3://{bucket}/{key} exceeds {MAX_MANIFEST_BYTES} bytes",
    ))
}

fn validate_existing_manifest(
    existing: &[u8],
    intended: &SignedManifest,
    bucket: &str,
    key: &str,
) -> Result<(), ExecutionError> {
    let existing: SignedManifest = serde_json::from_slice(existing).map_err(|err| {
        ExecutionError::InternalError(format!(
            "immutable manifest object at s3://{bucket}/{key} is not valid JSON: {err}",
        ))
    })?;
    existing.verify().map_err(|err| {
        ExecutionError::InternalError(format!(
            "immutable manifest object at s3://{bucket}/{key} has an invalid signature: {err}",
        ))
    })?;
    if existing.payload != intended.payload {
        return Err(ExecutionError::InternalError(format!(
            "immutable manifest object already exists with a different payload at s3://{bucket}/{key}",
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "publisher_tests.rs"]
mod tests;
