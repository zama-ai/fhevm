use std::{sync::Arc, time::Duration};

use aws_sdk_s3::{error::ProvideErrorMetadata, primitives::ByteStream, Client};
use ciphertext_attestation::manifest::SignedManifest;
use fhevm_engine_common::{
    pg_pool::{PostgresPoolManager, ServiceError},
    types::CoproSigner,
    versioning::StackMode,
};
use sqlx::PgPool;
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::{aws_upload::COPROCESSOR_CONTEXT_ID_1, Config, ConsensusConfig, ExecutionError};

use super::manifest::{
    discover_block_children, discover_known_children, is_block_manifest_ready,
    load_manifest_descriptors, lock_next_block_to_progress, mark_manifest_published,
    pending_chain_ids, prepare_manifest, seal_block_content, ManifestProgressCursor, PendingBlock,
};
use super::manifest_archive::{manifest_object_key, store_authenticated_manifest};
use super::metrics::{MANIFEST_PUBLICATION_FAILURE, MANIFEST_PUBLICATION_SUCCESS};
use super::peer_downloader::schedule_manifest_verification;

const MANIFEST_POLL_INTERVAL: Duration = Duration::from_secs(5);
#[derive(Clone, Copy)]
struct PendingChain {
    host_chain_id: i64,
}

async fn pending_manifest_chains(pool: &PgPool) -> Result<Vec<PendingChain>, ExecutionError> {
    Ok(pending_chain_ids(pool)
        .await?
        .into_iter()
        .map(|host_chain_id| PendingChain { host_chain_id })
        .collect())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PublicationProgress {
    Advanced,
    Waiting,
    LockBusy,
}

pub(crate) async fn spawn_manifest_publisher(
    pool_mngr: &PostgresPoolManager,
    conf: Config,
    client: Arc<Client>,
    signer: CoproSigner,
    stack_mode: Arc<StackMode>,
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
    let op = move |pool, token| {
        let conf = conf.clone();
        let client = Arc::clone(&client);
        let signer = Arc::clone(&signer);
        let stack_mode = Arc::clone(&stack_mode);
        async move {
            run_manifest_publisher(pool, token, conf, client, signer, stack_mode)
                .await
                .map_err(ServiceError::from)
        }
    };

    Ok(pool_mngr
        .spawn_with_db_retry(op, "consensus_manifest_publisher")
        .await)
}

async fn run_manifest_publisher(
    pool: PgPool,
    token: CancellationToken,
    conf: Config,
    client: Arc<Client>,
    signer: CoproSigner,
    stack_mode: Arc<StackMode>,
) -> Result<(), ExecutionError> {
    while stack_mode.gcs_mode() {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = tokio::time::sleep(Duration::from_secs(2)) => {}
        }
    }
    info!("Manifest publication enabled on the live stack");

    let mut ticker = tokio::time::interval(MANIFEST_POLL_INTERVAL);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        if stack_mode.is_paused() {
            info!("Retired stack stopped consensus manifest publication");
            return Ok(());
        }
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = ticker.tick() => {}
        }
        if stack_mode.is_paused() {
            info!("Retired stack stopped consensus manifest publication");
            return Ok(());
        }

        let discovered = discover_known_children(&pool).await?;
        if discovered > 0 {
            debug!(discovered, "Discovered manifest lineage blocks");
        }

        loop {
            let chains = pending_manifest_chains(&pool).await?;
            if chains.is_empty() {
                break;
            }

            let mut advanced = false;
            for chain in chains {
                match progress_chain(
                    &pool,
                    &client,
                    &conf.s3.bucket_ct128,
                    chain.host_chain_id,
                    &signer,
                    &conf.consensus,
                )
                .await
                {
                    Ok(PublicationProgress::Advanced) => advanced = true,
                    Ok(PublicationProgress::Waiting | PublicationProgress::LockBusy) => {}
                    Err(ExecutionError::DbError(err)) => {
                        return Err(ExecutionError::DbError(err));
                    }
                    Err(err) => {
                        error!(host_chain_id = chain.host_chain_id, error = %err, "Manifest publication attempt failed");
                    }
                }
            }

            if !advanced {
                break;
            }
        }
    }
}

async fn progress_chain(
    pool: &PgPool,
    client: &Client,
    bucket: &str,
    host_chain_id: i64,
    signer: &CoproSigner,
    consensus: &ConsensusConfig,
) -> Result<PublicationProgress, ExecutionError> {
    let mut cursor = ManifestProgressCursor::start();
    let mut attempted_candidate = false;

    loop {
        let mut trx = pool.begin().await?;
        let Some(block) = lock_next_block_to_progress(&mut trx, host_chain_id, &cursor).await?
        else {
            trx.rollback().await?;
            return Ok(if attempted_candidate {
                PublicationProgress::Waiting
            } else {
                PublicationProgress::LockBusy
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
        match result {
            Ok(PublicationProgress::Advanced) => {
                trx.commit().await?;
                if block.block_content_digest.is_some() {
                    MANIFEST_PUBLICATION_SUCCESS.inc();
                }
                return Ok(PublicationProgress::Advanced);
            }
            Ok(PublicationProgress::Waiting) => {
                trx.commit().await?;
            }
            Ok(PublicationProgress::LockBusy) => {
                unreachable!("a locked block cannot report lock contention")
            }
            Err(err @ ExecutionError::DbError(_)) => return Err(err),
            Err(err) => {
                trx.rollback().await?;
                if block.block_content_digest.is_some() {
                    MANIFEST_PUBLICATION_FAILURE.inc();
                }
                error!(
                    host_chain_id,
                    block_number = block.block_number,
                    block_hash = %hex::encode(&block.block_hash),
                    error = %err,
                    "Manifest candidate failed; continuing with another lineage"
                );
            }
        }
    }
}

async fn progress_locked_block(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client: &Client,
    bucket: &str,
    host_chain_id: i64,
    block: &PendingBlock,
    signer: &CoproSigner,
    consensus: &ConsensusConfig,
) -> Result<PublicationProgress, ExecutionError> {
    if block.block_content_digest.is_none() {
        if !is_block_manifest_ready(trx, block).await? {
            return Ok(PublicationProgress::Waiting);
        }
        let descriptors = load_manifest_descriptors(trx, block).await?;
        seal_block_content(trx, block, COPROCESSOR_CONTEXT_ID_1, &descriptors).await?;
        let discovered = discover_block_children(trx, block).await?;
        debug!(
            host_chain_id,
            block_number = block.block_number,
            block_hash = %hex::encode(&block.block_hash),
            descriptor_count = descriptors.len(),
            discovered_children = discovered,
            "Sealed block manifest content"
        );
        return Ok(PublicationProgress::Advanced);
    }

    publish_block_manifest(trx, client, bucket, block, signer, consensus).await?;
    Ok(PublicationProgress::Advanced)
}

pub(crate) async fn publish_block_manifest(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    client: &Client,
    bucket: &str,
    block: &PendingBlock,
    signer: &CoproSigner,
    consensus: &ConsensusConfig,
) -> Result<(), ExecutionError> {
    let prepared = prepare_manifest(trx, block, COPROCESSOR_CONTEXT_ID_1, signer.address()).await?;
    let detailed_range_start = i64::try_from(prepared.payload.detailed_range.first_block_number)
        .map_err(|_| ExecutionError::InternalError("detailed range start exceeds BIGINT".into()))?;
    let detailed_range_digest = prepared.payload.detailed_range.digest;
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

    let stored_body = put_immutable_manifest(client, bucket, &key, &signed, &body).await?;
    let archived =
        store_authenticated_manifest(trx, signed.payload.publisher, &key, &stored_body).await?;
    if archived.manifest.digest != manifest_digest {
        return Err(ExecutionError::InternalError(format!(
            "stored manifest digest {} does not match published digest {manifest_digest}",
            archived.manifest.digest,
        )));
    }
    mark_manifest_published(
        trx,
        block,
        signed.payload.publisher,
        detailed_range_start,
        detailed_range_digest,
        manifest_digest,
    )
    .await?;
    if consensus.verify_others_party_manifests {
        schedule_manifest_verification(
            trx,
            &archived.manifest,
            consensus.verification_delay,
            consensus.verification_retry_delay,
            consensus.verification_retry_count,
        )
        .await?;
    }
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
            let existing = client
                .get_object()
                .bucket(bucket)
                .key(key)
                .send()
                .await
                .map_err(|get_err| ExecutionError::S3TransientError(get_err.to_string()))?;
            let existing = existing
                .body
                .collect()
                .await
                .map_err(|collect_err| ExecutionError::S3TransientError(collect_err.to_string()))?
                .into_bytes();
            validate_existing_manifest(&existing, intended, bucket, key)?;
            Ok(existing.to_vec())
        }
        Err(err) => Err(ExecutionError::S3TransientError(err.to_string())),
    }
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
