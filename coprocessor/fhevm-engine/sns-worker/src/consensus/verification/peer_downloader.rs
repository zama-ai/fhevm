use std::{collections::HashSet, sync::Arc, time::Duration};

#[cfg(test)]
use alloy_primitives::B256;
use alloy_primitives::{Address, U256};
use aws_sdk_s3::Client;
use ciphertext_attestation::manifest::ManifestReference;
#[cfg(test)]
use ciphertext_attestation::manifest::ManifestVersion;
use fhevm_engine_common::{
    pg_pool::{PostgresPoolManager, ServiceError},
    versioning::StackMode,
};
use sqlx::{PgPool, Postgres, Transaction};
use tokio::{task::JoinHandle, time::MissedTickBehavior};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::ExecutionError;

use super::consensus_analysis::{evaluate_quorum, QuorumEvaluation, VerificationOutcome};
use super::drift_findings::apply_evaluation_to_drift_handles;
use super::peer_manifest_source::{
    referenced_manifest_object_key, PeerDownloadRequest, PeerManifestObject, PeerManifestSource,
    S3PeerManifestSource,
};
#[cfg(test)]
use super::peer_manifest_source::{s3_bucket_location, S3BucketLocation};
use super::verification_evidence::{persist_verification_evidence, VerificationAttemptEvidence};
use super::verification_scope::VerificationScope;
use super::verification_utils::{
    address, b256, downloader_worker_id, duration_micros, internal, manifest_version, u256,
};
use crate::consensus::{
    manifest_archive::{
        load_manifest_by_reference, load_manifest_revision, load_tip_eligible_manifest,
        store_authenticated_manifest, AuthenticatedManifest, ManifestSource,
    },
    metrics::{
        DRIFT_LOCALIZATION_INCOMPLETE, PEER_MANIFEST_ARCHIVED, PEER_MANIFEST_DOWNLOAD_FAILURE,
        VERIFICATION_FAILURE, VERIFICATION_OUTCOMES,
    },
};

const DOWNLOAD_POLL_INTERVAL: Duration = Duration::from_secs(1);
const DOWNLOAD_CLAIM: Duration = Duration::from_secs(5 * 60);
const PEER_LIST_TIMEOUT: Duration = Duration::from_secs(30);
const PEER_OBJECT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_PREDECESSORS_PER_ATTEMPT: usize = 256;

#[derive(Clone, Debug)]
struct ClaimedPeer {
    publisher: Address,
    s3_bucket_url: String,
    known_revisions: HashSet<u64>,
    resume_revision: Option<u64>,
}

#[derive(Clone, Debug)]
pub(crate) struct VerificationClaim {
    task_id: i64,
    worker_id: String,
    attempt: i32,
    required_quorum: usize,
    scope: VerificationScope,
    peers: Vec<ClaimedPeer>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct VerificationRunResult {
    pub task_id: i64,
    pub attempt: i32,
    pub outcome: VerificationOutcome,
}

pub(crate) async fn spawn_peer_manifest_downloader(
    pool_mngr: &PostgresPoolManager,
    client: Arc<Client>,
    stack_mode: Arc<StackMode>,
) -> Result<JoinHandle<Result<(), ServiceError>>, ExecutionError> {
    let source = S3PeerManifestSource::new(client);
    let op = move |pool, token| {
        let source = source.clone();
        let stack_mode = Arc::clone(&stack_mode);
        async move {
            run_peer_manifest_downloader(pool, token, source, stack_mode)
                .await
                .map_err(ServiceError::from)
        }
    };
    Ok(pool_mngr
        .spawn_with_db_retry(op, "consensus_peer_manifest_downloader")
        .await)
}

async fn run_peer_manifest_downloader<S: PeerManifestSource>(
    pool: PgPool,
    token: CancellationToken,
    source: S,
    stack_mode: Arc<StackMode>,
) -> Result<(), ExecutionError> {
    while stack_mode.gcs_mode() {
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = tokio::time::sleep(Duration::from_secs(2)) => {}
        }
    }
    info!("Peer manifest verification enabled on the live stack");
    let worker_id = downloader_worker_id();
    let mut ticker = tokio::time::interval(DOWNLOAD_POLL_INTERVAL);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        if stack_mode.is_paused() {
            info!("Retired stack stopped peer manifest verification");
            return Ok(());
        }
        tokio::select! {
            _ = token.cancelled() => return Ok(()),
            _ = ticker.tick() => {}
        }

        loop {
            match run_peer_manifest_download_once(&pool, &source, &worker_id, DOWNLOAD_CLAIM).await
            {
                Ok(Some(result)) => {
                    debug!(
                        task_id = result.task_id,
                        attempt = result.attempt,
                        outcome = ?result.outcome,
                        "Completed peer manifest download attempt"
                    );
                }
                Ok(None) => break,
                Err(ExecutionError::DbError(err)) => {
                    VERIFICATION_FAILURE.inc();
                    return Err(ExecutionError::DbError(err));
                }
                Err(err) => {
                    VERIFICATION_FAILURE.inc();
                    error!(error = %err, "Peer manifest verification attempt failed");
                    break;
                }
            }
        }
    }
}

pub(crate) async fn schedule_manifest_verification(
    trx: &mut Transaction<'_, Postgres>,
    local_manifest_id: i64,
    verification_delay: Duration,
    retry_delay: Duration,
    retry_count: u32,
) -> Result<i64, ExecutionError> {
    super::verification_schedule::schedule_manifest_verification(
        trx,
        local_manifest_id,
        verification_delay,
        retry_delay,
        retry_count,
    )
    .await
}

async fn run_peer_manifest_download_once<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    worker_id: &str,
    claim_duration: Duration,
) -> Result<Option<VerificationRunResult>, ExecutionError> {
    bind_one_unbound_pending_task(pool).await?;
    let Some(claim) = claim_due_task(pool, worker_id, claim_duration).await? else {
        return Ok(None);
    };

    for peer in &claim.peers {
        download_claimed_peer(pool, source, &claim, peer).await?;
    }
    finish_claim(pool, &claim).await.map(Some)
}

async fn bind_one_unbound_pending_task(pool: &PgPool) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    let task_id = sqlx::query_scalar!(
        r#"
        SELECT id
          FROM block_manifest_verification_task
         WHERE state = 'pending'
           AND required_quorum IS NULL
           AND next_attempt_at <= NOW()
         ORDER BY next_attempt_at, id
         FOR UPDATE SKIP LOCKED
         LIMIT 1
        "#,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    if let Some(task_id) = task_id {
        bind_task_to_current_registry(&mut trx, task_id).await?;
    }
    trx.commit().await?;
    Ok(())
}

pub(super) async fn bind_task_to_current_registry(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
) -> Result<bool, ExecutionError> {
    let target = sqlx::query!(
        r#"
        SELECT manifest.publisher AS local_publisher,
               manifest.version,
               manifest.coprocessor_context_id,
               manifest.host_chain_id,
               manifest.publication_block_number,
               manifest.publication_block_hash,
               task.state
          FROM block_manifest_verification_task task
          JOIN block_manifest manifest ON manifest.id = task.local_manifest_id
         WHERE task.id = $1
         FOR UPDATE
        "#,
        task_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if target.state != "pending" {
        return Ok(false);
    }

    let registry = sqlx::query!(
        r#"
        SELECT signer_address,
               s3_bucket_url,
               coprocessor_threshold,
               gateway_chain_id,
               gateway_config_address,
               snapshot_block_number,
               snapshot_block_hash
          FROM gateway_config_coprocessors
         ORDER BY signer_address
        "#,
    )
    .fetch_all(trx.as_mut())
    .await?;
    if registry.is_empty() {
        postpone_registry_binding(trx, task_id, "GatewayConfig registry snapshot is empty").await?;
        return Ok(false);
    }

    let local_publisher = address("verification task local publisher", &target.local_publisher)?;
    let first = &registry[0];
    let threshold = first.coprocessor_threshold;
    let coprocessor_count = i32::try_from(registry.len())
        .map_err(|_| internal("registered coprocessor count exceeds INTEGER"))?;
    let required_quorum = i32::try_from(threshold)
        .map_err(|_| internal("GatewayConfig threshold exceeds INTEGER"))?;
    if required_quorum <= 0 || required_quorum > coprocessor_count {
        postpone_registry_binding(
            trx,
            task_id,
            &format!(
                "invalid GatewayConfig threshold {required_quorum} for {coprocessor_count} coprocessors"
            ),
        )
        .await?;
        return Ok(false);
    }

    let gateway_chain_id = first.gateway_chain_id;
    let gateway_config_address = first.gateway_config_address.clone();
    let snapshot_block_number = first.snapshot_block_number;
    let snapshot_block_hash = first.snapshot_block_hash.clone();
    let mut local_registered = false;
    let mut peers = Vec::with_capacity(registry.len().saturating_sub(1));
    for row in registry {
        if row.coprocessor_threshold != threshold
            || row.gateway_chain_id != gateway_chain_id
            || row.gateway_config_address != gateway_config_address
            || row.snapshot_block_number != snapshot_block_number
            || row.snapshot_block_hash != snapshot_block_hash
        {
            return Err(internal(
                "GatewayConfig registry rows do not form one consistent snapshot",
            ));
        }
        let publisher = address("GatewayConfig signer address", &row.signer_address)?;
        if publisher == local_publisher {
            local_registered = true;
        } else {
            peers.push((publisher, row.s3_bucket_url));
        }
    }
    if !local_registered {
        postpone_registry_binding(
            trx,
            task_id,
            "local manifest publisher is absent from GatewayConfig registry",
        )
        .await?;
        return Ok(false);
    }

    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET state = 'pending',
               gateway_chain_id = $2,
               gateway_config_address = $3,
               registry_block_number = $4,
               registry_block_hash = $5,
               registered_coprocessor_count = $6,
               required_quorum = $7,
               last_error = NULL,
               updated_at = NOW()
         WHERE id = $1
        "#,
        task_id,
        gateway_chain_id,
        &gateway_config_address,
        snapshot_block_number,
        &snapshot_block_hash,
        coprocessor_count,
        required_quorum,
    )
    .execute(trx.as_mut())
    .await?;
    for (publisher, s3_bucket_url) in peers {
        sqlx::query!(
            r#"
            INSERT INTO block_manifest_peer_download (
                task_id,
                publisher,
                s3_bucket_url
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (task_id, publisher) DO NOTHING
            "#,
            task_id,
            publisher.as_slice(),
            s3_bucket_url,
        )
        .execute(trx.as_mut())
        .await?;
    }
    Ok(true)
}

async fn postpone_registry_binding(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
    error: &str,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET next_attempt_at = GREATEST(
                   eligible_at,
                   NOW() + retry_delay_micros * INTERVAL '1 microsecond'
               ),
               last_error = $2,
               updated_at = NOW()
         WHERE id = $1
        "#,
        task_id,
        error,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

pub(crate) async fn claim_due_task(
    pool: &PgPool,
    worker_id: &str,
    claim_duration: Duration,
) -> Result<Option<VerificationClaim>, ExecutionError> {
    let claim_micros = duration_micros("verification claim", claim_duration)?;
    let mut trx = pool.begin().await?;
    let row = sqlx::query!(
        r#"
        WITH selected AS (
            SELECT id
              FROM block_manifest_verification_task
             WHERE required_quorum IS NOT NULL
               AND next_attempt_at <= NOW()
               AND (
                    state = 'pending'
                    OR (state = 'claimed' AND claim_expires_at <= NOW())
               )
             ORDER BY next_attempt_at, id
             FOR UPDATE SKIP LOCKED
             LIMIT 1
        )
        UPDATE block_manifest_verification_task target
           SET state = 'claimed',
               claim_owner = $1,
               claim_expires_at = NOW() + $2::BIGINT * INTERVAL '1 microsecond',
               updated_at = NOW()
          FROM selected, block_manifest manifest
         WHERE target.id = selected.id
           AND manifest.id = target.local_manifest_id
        RETURNING target.id,
                  target.attempt_count + 1 AS "attempt!",
                  target.required_quorum AS "required_quorum!",
                  manifest.publisher AS "local_publisher!",
                  manifest.version AS "version!",
                  manifest.coprocessor_context_id AS "coprocessor_context_id!",
                  manifest.host_chain_id AS "host_chain_id!",
                  manifest.publication_block_number AS "publication_block_number!",
                  manifest.publication_block_hash AS "publication_block_hash!",
                  manifest.revision AS "revision!",
                  manifest.manifest_digest AS "local_manifest_digest!"
        "#,
        worker_id,
        claim_micros,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    let Some(row) = row else {
        trx.commit().await?;
        return Ok(None);
    };
    let task_id = row.id;
    let attempt = row.attempt;
    let context = u256("verification task context", &row.coprocessor_context_id)?;
    let version = manifest_version(row.version)?;
    let scope = VerificationScope {
        local_publisher: address("verification task local publisher", &row.local_publisher)?,
        version,
        coprocessor_context_id: context,
        host_chain_id: row.host_chain_id,
        publication_block_number: row.publication_block_number,
        publication_block_hash: b256(
            "verification task publication block hash",
            &row.publication_block_hash,
        )?,
        revision: u64::try_from(row.revision)
            .map_err(|_| internal("verification task revision is negative"))?,
        local_manifest_digest: b256(
            "verification task local manifest digest",
            &row.local_manifest_digest,
        )?,
    };

    let peer_rows = sqlx::query!(
        r#"
        SELECT publisher, s3_bucket_url
          FROM block_manifest_peer_download
         WHERE task_id = $1
           AND completed_attempt < $2
         ORDER BY publisher
        "#,
        task_id,
        attempt,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let mut peers = Vec::with_capacity(peer_rows.len());
    for peer in peer_rows {
        let publisher = address("peer download publisher", &peer.publisher)?;
        let known_revisions = load_known_peer_revisions(&mut trx, &scope, publisher).await?;
        let resume_revision = known_revisions.iter().max().copied();
        peers.push(ClaimedPeer {
            publisher,
            s3_bucket_url: peer.s3_bucket_url,
            known_revisions,
            resume_revision,
        });
    }
    trx.commit().await?;
    Ok(Some(VerificationClaim {
        task_id,
        worker_id: worker_id.to_owned(),
        attempt,
        required_quorum: usize::try_from(row.required_quorum)
            .map_err(|_| internal("required quorum is negative"))?,
        scope,
        peers,
    }))
}

async fn load_known_peer_revisions(
    trx: &mut Transaction<'_, Postgres>,
    scope: &VerificationScope,
    publisher: Address,
) -> Result<HashSet<u64>, ExecutionError> {
    let context = scope.coprocessor_context_id.to_be_bytes::<32>();
    let revisions = sqlx::query_scalar!(
        r#"
            SELECT revision
              FROM block_manifest
             WHERE publisher = $1
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND publication_block_number = $5
               AND publication_block_hash = $6
            "#,
        publisher.as_slice(),
        i16::from(u8::from(scope.version)),
        context.as_slice(),
        scope.host_chain_id,
        scope.publication_block_number,
        scope.publication_block_hash.as_slice(),
    )
    .fetch_all(trx.as_mut())
    .await?;
    revisions
        .into_iter()
        .map(|revision| {
            u64::try_from(revision)
                .map_err(|_| internal("archived peer manifest revision is negative"))
        })
        .collect()
}

async fn download_claimed_peer<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
) -> Result<(), ExecutionError> {
    let mut request = peer_download_request(claim, peer);
    if let Some(revision) = peer.resume_revision {
        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        let archived = load_manifest_revision(
            &mut trx,
            peer.publisher,
            claim.scope.version,
            claim.scope.coprocessor_context_id,
            claim.scope.host_chain_id,
            claim.scope.publication_block_number,
            claim.scope.publication_block_hash,
            i64::try_from(revision).map_err(|_| internal("peer revision exceeds BIGINT"))?,
        )
        .await?;
        trx.commit().await?;
        if let Some(archived) = archived {
            if !archive_missing_predecessors(pool, source, claim, peer, &request, &archived).await?
            {
                return Ok(());
            }
            request.known_revisions.extend(0..=revision);
        }
    }

    let Some(object_keys) = list_peer_manifests(pool, source, claim, peer, &request).await? else {
        return Ok(());
    };

    if let Some(object_key) = object_keys.into_iter().next() {
        let Some(object) =
            fetch_peer_manifest(pool, source, claim, peer, &request, &object_key).await?
        else {
            return Ok(());
        };
        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        let stored = match store_authenticated_manifest(
            &mut trx,
            peer.publisher,
            &object.object_key,
            &object.signed_bytes,
            ManifestSource::Peer,
        )
        .await
        {
            Ok(stored) => stored,
            Err(err) => {
                trx.rollback().await?;
                record_peer_failure(pool, claim, peer.publisher, &err.to_string()).await?;
                warn!(
                    task_id = claim.task_id,
                    publisher = %peer.publisher,
                    error = %err,
                    "Rejected downloaded peer manifest"
                );
                return Ok(());
            }
        };
        if !claim.scope.matches_manifest(&stored.manifest.signed) {
            let error = format!(
                "peer manifest {} does not match verification task {}",
                object.object_key, claim.task_id,
            );
            trx.rollback().await?;
            record_peer_failure(pool, claim, peer.publisher, &error).await?;
            return Ok(());
        }
        let revision = stored.manifest.signed.payload.revision;
        trx.commit().await?;
        PEER_MANIFEST_ARCHIVED.inc();
        if !archive_missing_predecessors(pool, source, claim, peer, &request, &stored.manifest)
            .await?
        {
            return Ok(());
        }
        let revision = i64::try_from(revision)
            .map_err(|_| internal("downloaded manifest revision exceeds BIGINT"))?;
        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        sqlx::query!(
            r#"
            UPDATE block_manifest_peer_download
               SET latest_revision = GREATEST(COALESCE(latest_revision, -1), $3),
                   last_attempt_at = NOW(),
                   updated_at = NOW()
             WHERE task_id = $1
               AND publisher = $2
            "#,
            claim.task_id,
            peer.publisher.as_slice(),
            revision,
        )
        .execute(trx.as_mut())
        .await?;
        trx.commit().await?;
        // The newest authenticated tip recursively retrieves its complete
        // supersession and history graph. Older listed keys are therefore
        // redundant and may legitimately have been signed before key rotation.
    }

    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    sqlx::query!(
        r#"
        UPDATE block_manifest_peer_download
           SET completed_attempt = $3,
               last_attempt_at = NOW(),
               last_error = NULL,
               updated_at = NOW()
         WHERE task_id = $1
           AND publisher = $2
        "#,
        claim.task_id,
        peer.publisher.as_slice(),
        claim.attempt,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

fn peer_download_request(claim: &VerificationClaim, peer: &ClaimedPeer) -> PeerDownloadRequest {
    PeerDownloadRequest {
        publisher: peer.publisher,
        s3_bucket_url: peer.s3_bucket_url.clone(),
        version: claim.scope.version,
        coprocessor_context_id: claim.scope.coprocessor_context_id,
        host_chain_id: claim.scope.host_chain_id,
        publication_block_number: claim.scope.publication_block_number,
        publication_block_hash: claim.scope.publication_block_hash,
        known_revisions: peer.known_revisions.clone(),
    }
}

async fn list_peer_manifests<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    request: &PeerDownloadRequest,
) -> Result<Option<Vec<String>>, ExecutionError> {
    let result = tokio::time::timeout(PEER_LIST_TIMEOUT, source.list_manifests(request)).await;
    let error = match result {
        Ok(Ok(object_keys)) => return Ok(Some(object_keys)),
        Ok(Err(error)) => error,
        Err(_) => ExecutionError::S3TransientError(format!(
            "peer manifest listing timed out after {PEER_LIST_TIMEOUT:?}"
        )),
    };

    record_peer_failure(pool, claim, peer.publisher, &error.to_string()).await?;
    warn!(
        task_id = claim.task_id,
        publisher = %peer.publisher,
        error = %error,
        "Peer manifest download failed"
    );
    Ok(None)
}

async fn fetch_peer_manifest<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    request: &PeerDownloadRequest,
    object_key: &str,
) -> Result<Option<PeerManifestObject>, ExecutionError> {
    let result = tokio::time::timeout(
        PEER_OBJECT_TIMEOUT,
        source.fetch_manifest(request, object_key),
    )
    .await;
    let error = match result {
        Ok(Ok(object)) => return Ok(Some(object)),
        Ok(Err(error)) => error,
        Err(_) => ExecutionError::S3TransientError(format!(
            "peer manifest object fetch timed out after {PEER_OBJECT_TIMEOUT:?}"
        )),
    };
    record_peer_failure(pool, claim, peer.publisher, &error.to_string()).await?;
    warn!(
        task_id = claim.task_id,
        publisher = %peer.publisher,
        object_key,
        error = %error,
        "Peer manifest object download failed"
    );
    Ok(None)
}

async fn archive_missing_predecessors<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    request: &PeerDownloadRequest,
    manifest: &AuthenticatedManifest,
) -> Result<bool, ExecutionError> {
    let mut pending = Vec::new();
    enqueue_manifest_references(&mut pending, manifest);
    let mut visited = HashSet::new();

    while let Some(reference) = pending.pop() {
        let identity = (
            reference.publisher,
            reference.block_hash,
            reference.revision,
            reference.manifest_digest,
        );
        if !visited.insert(identity) {
            continue;
        }
        if visited.len() > MAX_PREDECESSORS_PER_ATTEMPT {
            return Err(internal(format!(
                "peer manifest predecessor chain exceeds {MAX_PREDECESSORS_PER_ATTEMPT} objects"
            )));
        }

        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        if let Some(stored) = load_manifest_by_reference(
            &mut trx,
            request.version,
            request.coprocessor_context_id,
            request.host_chain_id,
            &reference,
        )
        .await?
        {
            trx.commit().await?;
            enqueue_manifest_references(&mut pending, &stored);
            continue;
        }
        trx.commit().await?;

        let object_key = referenced_manifest_object_key(request, &reference);
        let Some(object) =
            fetch_peer_manifest(pool, source, claim, peer, request, &object_key).await?
        else {
            return Ok(false);
        };
        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        let stored = store_authenticated_manifest(
            &mut trx,
            reference.publisher,
            &object.object_key,
            &object.signed_bytes,
            ManifestSource::Peer,
        )
        .await?;
        if stored.manifest.digest != reference.manifest_digest {
            return Err(internal(format!(
                "downloaded predecessor {} does not match its signed reference",
                object.object_key
            )));
        }
        trx.commit().await?;
        PEER_MANIFEST_ARCHIVED.inc();
        enqueue_manifest_references(&mut pending, &stored.manifest);
    }
    Ok(true)
}

fn enqueue_manifest_references(
    pending: &mut Vec<ManifestReference>,
    manifest: &AuthenticatedManifest,
) {
    if let Some(reference) = manifest.signed.payload.previous_manifest.clone() {
        pending.push(reference);
    }
    if let Some(reference) = manifest.signed.payload.supersedes.clone() {
        pending.push(reference);
    }
}

async fn record_peer_failure(
    pool: &PgPool,
    claim: &VerificationClaim,
    publisher: Address,
    error: &str,
) -> Result<(), ExecutionError> {
    PEER_MANIFEST_DOWNLOAD_FAILURE.inc();
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    sqlx::query!(
        r#"
        UPDATE block_manifest_peer_download
           SET completed_attempt = $3,
               last_attempt_at = NOW(),
               last_error = $4,
               updated_at = NOW()
         WHERE task_id = $1
           AND publisher = $2
        "#,
        claim.task_id,
        publisher.as_slice(),
        claim.attempt,
        error,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

async fn require_active_claim(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
) -> Result<(), ExecutionError> {
    let active = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
              FROM block_manifest_verification_task
             WHERE id = $1
               AND state = 'claimed'
               AND claim_owner = $2
               AND claim_expires_at > NOW()
             FOR UPDATE
        ) AS "active!"
        "#,
        claim.task_id,
        &claim.worker_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if !active {
        return Err(internal(format!(
            "verification claim for task {} is no longer owned by {}",
            claim.task_id, claim.worker_id,
        )));
    }
    Ok(())
}

async fn finish_claim(
    pool: &PgPool,
    claim: &VerificationClaim,
) -> Result<VerificationRunResult, ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    let manifests = load_claim_manifests(&mut trx, claim).await?;
    let evaluation = evaluate_quorum(
        &manifests,
        claim.scope.local_publisher,
        claim.required_quorum,
    );
    let localization = apply_evaluation_to_drift_handles(
        &mut trx,
        claim.task_id,
        &manifests,
        claim.scope.local_publisher,
        &evaluation,
    )
    .await?;
    persist_verification_evidence(
        &mut trx,
        VerificationAttemptEvidence {
            task_id: claim.task_id,
            attempt: claim.attempt,
            required_quorum: claim.required_quorum,
            evaluation: &evaluation,
            localization_complete: localization.complete,
            drifted_block_count: localization.drifted_block_count,
            drifted_handle_count: localization.drifted_handle_count,
        },
    )
    .await?;
    persist_claim_outcome(&mut trx, claim, &evaluation).await?;
    trx.commit().await?;
    VERIFICATION_OUTCOMES
        .with_label_values(&[evaluation.outcome.as_db_str()])
        .inc();
    if !localization.complete {
        DRIFT_LOCALIZATION_INCOMPLETE.inc();
    }
    Ok(VerificationRunResult {
        task_id: claim.task_id,
        attempt: claim.attempt,
        outcome: evaluation.outcome,
    })
}

async fn load_claim_manifests(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
) -> Result<Vec<AuthenticatedManifest>, ExecutionError> {
    let local_reference = ManifestReference {
        publisher: claim.scope.local_publisher,
        block_number: U256::from(
            u64::try_from(claim.scope.publication_block_number)
                .map_err(|_| internal("publication block number is negative"))?,
        ),
        block_hash: claim.scope.publication_block_hash,
        revision: claim.scope.revision,
        manifest_digest: claim.scope.local_manifest_digest,
    };
    let local = load_manifest_by_reference(
        trx,
        claim.scope.version,
        claim.scope.coprocessor_context_id,
        claim.scope.host_chain_id,
        &local_reference,
    )
    .await?
    .ok_or_else(|| internal("verification task local manifest is absent from archive"))?;
    let peer_publishers = sqlx::query_scalar!(
        "SELECT publisher FROM block_manifest_peer_download WHERE task_id = $1",
        claim.task_id,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let mut manifests = vec![local];
    for publisher_bytes in peer_publishers {
        let publisher = address("peer download publisher", &publisher_bytes)?;
        if let Some(manifest) = load_tip_eligible_manifest(
            trx,
            publisher,
            claim.scope.version,
            claim.scope.coprocessor_context_id,
            claim.scope.host_chain_id,
            claim.scope.publication_block_number,
            claim.scope.publication_block_hash,
        )
        .await?
        {
            manifests.push(manifest);
        }
    }
    Ok(manifests)
}

async fn persist_claim_outcome(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
    evaluation: &QuorumEvaluation,
) -> Result<(), ExecutionError> {
    let target = sqlx::query!(
        "SELECT max_attempts, retry_delay_micros FROM block_manifest_verification_task WHERE id = $1",
        claim.task_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    let max_attempts = target.max_attempts;
    let retry_delay_micros = target.retry_delay_micros;
    let state = if evaluation.outcome == VerificationOutcome::Consensus {
        "consensus"
    } else if claim.attempt >= max_attempts {
        "retry_exhausted"
    } else {
        // Drift is reportable immediately, but retries remain available so a
        // newer peer revision can demonstrate remission.
        "pending"
    };
    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET attempt_count = $3,
               state = $4,
               latest_outcome = $5,
               last_attempt_at = NOW(),
               next_attempt_at = CASE
                   WHEN $4 = 'pending'
                   THEN NOW() + $6::BIGINT * INTERVAL '1 microsecond'
                   ELSE NULL
               END,
               claim_owner = NULL,
               claim_expires_at = NULL,
               last_error = NULL,
               updated_at = NOW()
         WHERE id = $1
           AND claim_owner = $2
        "#,
        claim.task_id,
        &claim.worker_id,
        claim.attempt,
        state,
        evaluation.outcome.as_db_str(),
        retry_delay_micros,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

#[cfg(test)]
#[path = "peer_downloader_tests.rs"]
mod tests;
