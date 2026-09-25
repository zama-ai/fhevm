use std::{collections::HashSet, fmt, sync::Arc, time::Duration};

use alloy_primitives::{Address, U256};
use aws_sdk_s3::Client;
#[cfg(test)]
use block_manifest::ManifestVersion;
use futures::stream::{self, FuturesUnordered, StreamExt, TryStreamExt};
use sqlx::{PgPool, Postgres, Transaction};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use fhevm_engine_common::pg_pool::is_fatal_connection_error;

use crate::manifest_consensus::{
    db_error::{classify_db_error, DbErrorClass},
    ExecutionError, ManifestWorkGate,
};

use super::consensus_analysis::{
    evaluate_quorum_with_history, CommitmentScope, HistoricalCommitmentEvidence, QuorumEvaluation,
    VerificationOutcome,
};
use super::drift_findings::{
    apply_evaluation_to_drift_handles, derive_historical_scope_digest, DriftLocalizationContext,
};
use super::history_fetch::{archive_history_for_disagreements, archive_peer_history_predecessors};
use super::localization_cache::load_completed_history;
use super::metrics::{
    DRIFT_LOCALIZATION_INCOMPLETE, PEER_MANIFEST_ARCHIVED, PEER_MANIFEST_DOWNLOAD_FAILURE,
    VERIFICATION_FAILURE, VERIFICATION_OUTCOMES,
};
#[cfg(test)]
use super::peer_manifest_source::{s3_bucket_location, S3BucketLocation};
use super::peer_manifest_source::{
    PeerDownloadRequest, PeerManifestObject, PeerManifestSource, S3PeerManifestSource,
};
use super::verification_evidence::{persist_verification_evidence, VerificationAttemptEvidence};
use super::verification_utils::{address, downloader_worker_id, duration_micros, internal};
use crate::manifest_consensus::manifest_archive::{
    load_highest_archived_revision, load_manifest_by_reference, load_previous_local_manifests,
    store_authenticated_manifest, AuthenticatedManifest, ManifestReference, ManifestSource,
    MAX_HISTORY_PREDECESSORS,
};

const DOWNLOAD_POLL_MARGIN: Duration = Duration::from_millis(100);
const DOWNLOAD_CLAIM: Duration = Duration::from_secs(5 * 60);
const PEER_LIST_TIMEOUT: Duration = Duration::from_secs(30);
const PEER_OBJECT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_REVISION_CANDIDATES_PER_ATTEMPT: usize = 5;
/// Peers downloaded at once within one verification attempt.
const PEER_DOWNLOAD_CONCURRENCY: usize = 8;

/// A peer did not contribute a usable manifest to this verification attempt.
///
/// Corruption identifies a listed object that cannot be used: authentication
/// failed, the payload is out of scope or oversized, or S3 returned `NoSuchKey` for an
/// immutable revision that will never appear. Incompleteness means a required
/// covering body is unavailable for this attempt (listing failed, or a
/// transient get of a higher revision). Both are peer-level failures: the
/// task still completes and follows its normal retry policy.
#[derive(Debug)]
pub(super) enum PeerManifestFailure {
    Corrupted { error: String },
    Incomplete { reason: String },
}

impl fmt::Display for PeerManifestFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Corrupted { error } => write!(formatter, "corrupted: {error}"),
            Self::Incomplete { reason } => write!(formatter, "incomplete: {reason}"),
        }
    }
}

#[cfg(test)]
use super::verification_queue::ready_verification_tasks;
use super::verification_queue::{
    claim_verification_task, next_verification_delay, ready_verification_chain_tasks, ClaimedPeer,
    VerificationClaim,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Result returned after one claim has been finalized successfully.
///
/// This is an execution receipt for metrics and tests. Durable retry and outcome
/// state remains in `block_manifest_verification_task`.
pub(crate) struct VerificationRunResult {
    pub task_id: i64,
    pub attempt: i32,
    pub outcome: VerificationOutcome,
}

pub(crate) fn spawn_peer_manifest_downloader(
    pool: PgPool,
    token: CancellationToken,
    client: Arc<Client>,
    work_gate: Arc<ManifestWorkGate>,
    verification_delay: Duration,
) -> JoinHandle<Result<(), ExecutionError>> {
    let source = S3PeerManifestSource::new(client);
    tokio::spawn(run_peer_manifest_downloader(
        pool,
        token,
        source,
        work_gate,
        verification_delay,
    ))
}

async fn run_peer_manifest_downloader<S: PeerManifestSource>(
    pool: PgPool,
    token: CancellationToken,
    source: S,
    work_gate: Arc<ManifestWorkGate>,
    verification_delay: Duration,
) -> Result<(), ExecutionError> {
    info!("Peer manifest verification enabled for this stack consensus_epoch");
    let worker_id = downloader_worker_id();
    // One task in flight per host chain. A chain is relaunched at the next poll
    // after its own task ends, so a slow chain never delays the others.
    let mut in_flight = FuturesUnordered::new();
    let mut busy_chains = HashSet::new();
    loop {
        match ready_idle_chain_tasks(&pool, &work_gate, &mut busy_chains).await {
            Ok(ready) => {
                for (consensus_epoch, host_chain_id, task_id) in ready {
                    in_flight.push(run_chain_task(
                        &pool,
                        &source,
                        &worker_id,
                        consensus_epoch,
                        host_chain_id,
                        task_id,
                    ));
                }
            }
            Err(ExecutionError::DbError(db)) if is_fatal_connection_error(&db) => {
                return Err(ExecutionError::DbError(db));
            }
            Err(err) => {
                error!(error = %err, "Cannot select ready verification tasks; retrying later");
            }
        }
        let poll = wait_for_next_verification_poll(&pool, &work_gate, verification_delay);
        tokio::pin!(poll);
        loop {
            tokio::select! {
                _ = token.cancelled() => return Ok(()),
                Some((host_chain_id, consensus_epoch, result)) = in_flight.next() => {
                    busy_chains.remove(&host_chain_id);
                    report_verification_task(&consensus_epoch, result)?;
                }
                _ = &mut poll => break,
            }
        }
    }
}

fn verification_poll_delay(next_task: Option<Duration>, verification_delay: Duration) -> Duration {
    next_task
        .unwrap_or(verification_delay)
        .min(verification_delay)
        + DOWNLOAD_POLL_MARGIN
}

async fn wait_for_next_verification_poll(
    pool: &PgPool,
    work_gate: &ManifestWorkGate,
    verification_delay: Duration,
) {
    let next_task = if let Some(epoch) = work_gate.pinned_consensus_epoch() {
        match next_verification_delay(pool, &epoch).await {
            Ok(delay) => delay,
            Err(err) => {
                error!(error = %err, "Cannot read next verification time; using default polling delay");
                None
            }
        }
    } else {
        None
    };
    tokio::time::sleep(verification_poll_delay(next_task, verification_delay)).await;
}

/// Due tasks of chains that have no task in flight; marks those chains busy.
async fn ready_idle_chain_tasks(
    pool: &PgPool,
    work_gate: &ManifestWorkGate,
    busy_chains: &mut HashSet<i64>,
) -> Result<Vec<(String, i64, i64)>, ExecutionError> {
    let Some(consensus_epoch) = work_gate.pinned_consensus_epoch() else {
        return Ok(Vec::new());
    };
    if !work_gate.work_enabled_for(&consensus_epoch) {
        return Ok(Vec::new());
    }
    Ok(ready_verification_chain_tasks(pool, &consensus_epoch)
        .await?
        .into_iter()
        .filter(|(host_chain_id, _)| busy_chains.insert(*host_chain_id))
        .map(|(host_chain_id, task_id)| (consensus_epoch.clone(), host_chain_id, task_id))
        .collect())
}

async fn run_chain_task<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    worker_id: &str,
    consensus_epoch: String,
    host_chain_id: i64,
    task_id: i64,
) -> (
    i64,
    String,
    Result<Option<VerificationRunResult>, ExecutionError>,
) {
    let result = run_verification_task(
        pool,
        source,
        worker_id,
        DOWNLOAD_CLAIM,
        &consensus_epoch,
        task_id,
    )
    .await;
    (host_chain_id, consensus_epoch, result)
}

fn report_verification_task(
    consensus_epoch: &str,
    result: Result<Option<VerificationRunResult>, ExecutionError>,
) -> Result<(), ExecutionError> {
    match result {
        Ok(Some(result)) => debug!(task_id = result.task_id, attempt = result.attempt,
            outcome = ?result.outcome, "Completed peer manifest download attempt"),
        Ok(None) => {}
        Err(err) => {
            VERIFICATION_FAILURE
                .with_label_values(&[consensus_epoch])
                .inc();
            error!(error = %err, "Peer manifest verification attempt failed");
            if matches!(&err, ExecutionError::DbError(db) if is_fatal_connection_error(db)) {
                return Err(err);
            }
        }
    }
    Ok(())
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

async fn run_verification_task<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    worker_id: &str,
    claim_duration: Duration,
    consensus_epoch: &str,
    task_id: i64,
) -> Result<Option<VerificationRunResult>, ExecutionError> {
    let Some(claim) =
        claim_verification_task(pool, worker_id, claim_duration, consensus_epoch, task_id).await?
    else {
        return Ok(None);
    };
    verify_claim(pool, source, &claim).await
}

async fn verify_claim<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
) -> Result<Option<VerificationRunResult>, ExecutionError> {
    info!(
        task_id = claim.task_id,
        attempt = claim.attempt,
        consensus_epoch = %claim.scope.consensus_epoch,
        host_chain_id = claim.scope.host_chain_id,
        block_number = claim.scope.publication_block_number,
        block_hash = %claim.scope.publication_block_hash,
        revision = claim.scope.revision,
        required_quorum = claim.required_quorum,
        "Starting peer manifest verification attempt"
    );
    let result = async {
        stream::iter(claim.peers.iter().map(Ok))
            .try_for_each_concurrent(PEER_DOWNLOAD_CONCURRENCY, |peer| {
                download_claimed_peer(pool, source, claim, peer)
            })
            .await?;
        archive_history_for_disagreements(pool, source, claim).await?;
        finish_claim(pool, claim).await
    }
    .await;
    match result {
        Ok(outcome) => Ok(Some(outcome)),
        Err(ExecutionError::DbError(err)) if is_fatal_connection_error(&err) => {
            Err(ExecutionError::DbError(err))
        }
        Err(err) => {
            let applied = match &err {
                ExecutionError::DbError(db) => apply_claimed_db_error(pool, claim, db).await,
                error => apply_claimed_error(pool, claim, error).await,
            };
            if let Err(apply_error) = applied {
                if let ExecutionError::DbError(apply_db) = &apply_error {
                    if is_fatal_connection_error(apply_db) {
                        return Err(apply_error);
                    }
                }
                error!(
                    task_id = claim.task_id,
                    error = %apply_error,
                    "Failed to record verification error on the claimed task"
                );
            }
            Ok(None)
        }
    }
}

async fn download_claimed_peer<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
) -> Result<(), ExecutionError> {
    if peer.current_download_complete {
        return Ok(());
    }
    let request = peer_download_request(claim, peer);

    let Some(object_keys) = list_peer_manifests(pool, source, claim, peer, &request).await? else {
        return Ok(());
    };

    if !object_keys.is_empty() {
        let Some(manifest) = download_highest_verified_manifest(
            pool,
            source,
            claim,
            peer,
            &request,
            object_keys,
            |manifest| claim.scope.matches_manifest(&manifest.signed),
            "verification task",
        )
        .await?
        else {
            archive_peer_history_predecessors(pool, source, claim, peer).await?;
            return Ok(());
        };
        let revision = manifest.signed.payload.revision;
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
    } else {
        archive_peer_history_predecessors(pool, source, claim, peer).await?;
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

#[allow(clippy::too_many_arguments)]
pub(super) async fn download_highest_verified_manifest<S, M>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    request: &PeerDownloadRequest,
    object_keys: Vec<String>,
    matches_scope: M,
    expected_scope: &str,
) -> Result<Option<AuthenticatedManifest>, ExecutionError>
where
    S: PeerManifestSource,
    M: Fn(&AuthenticatedManifest) -> bool,
{
    let mut last_rejection = None;
    let mut rejected = 0;
    for object_key in object_keys
        .into_iter()
        .take(MAX_REVISION_CANDIDATES_PER_ATTEMPT)
    {
        let object =
            match fetch_peer_manifest(pool, source, claim, peer, request, &object_key).await? {
                FetchedPeerObject::Body(object) => object,
                FetchedPeerObject::Unusable(reason) => {
                    rejected += 1;
                    last_rejection = Some(reason);
                    remember_rejected_object(pool, claim, peer, &object_key).await?;
                    warn!(
                        task_id = claim.task_id,
                        publisher = %peer.publisher,
                        object_key,
                        "Peer manifest revision is unusable; trying an older revision"
                    );
                    continue;
                }
                FetchedPeerObject::Unavailable => return Ok(None),
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
            Err(error @ ExecutionError::DbError(_)) => return Err(error),
            Err(error) => {
                trx.rollback().await?;
                rejected += 1;
                last_rejection = Some(error.to_string());
                remember_rejected_object(pool, claim, peer, &object_key).await?;
                warn!(
                    task_id = claim.task_id,
                    publisher = %peer.publisher,
                    object_key,
                    error = %error,
                    "Rejected peer manifest revision; trying an older revision"
                );
                continue;
            }
        };
        if !matches_scope(&stored.manifest) {
            trx.rollback().await?;
            rejected += 1;
            let error = format!("manifest does not match expected {expected_scope}");
            last_rejection = Some(error.clone());
            remember_rejected_object(pool, claim, peer, &object_key).await?;
            warn!(
                task_id = claim.task_id,
                publisher = %peer.publisher,
                object_key,
                error,
                "Rejected peer manifest revision; trying an older revision"
            );
            continue;
        }
        trx.commit().await?;
        PEER_MANIFEST_ARCHIVED
            .with_label_values(&[&claim.scope.consensus_epoch])
            .inc();
        return Ok(Some(stored.manifest));
    }

    if rejected > 0 {
        let failure = PeerManifestFailure::Corrupted {
            error: format!(
                "no valid manifest in the {rejected} highest revision candidate(s); last rejection: {}",
                last_rejection.unwrap_or_else(|| "unknown rejection".to_owned()),
            ),
        };
        record_peer_failure(pool, claim, peer.publisher, &failure).await?;
    }
    Ok(None)
}

fn peer_download_request(claim: &VerificationClaim, peer: &ClaimedPeer) -> PeerDownloadRequest {
    PeerDownloadRequest {
        publisher: peer.publisher,
        s3_bucket_url: peer.s3_bucket_url.clone(),
        version: claim.scope.version,
        consensus_epoch: claim.scope.consensus_epoch.clone(),
        coprocessor_context_id: claim.scope.coprocessor_context_id,
        host_chain_id: claim.scope.host_chain_id,
        publication_block_number: claim.scope.publication_block_number,
        publication_block_hash: claim.scope.publication_block_hash,
        highest_archived_revision: peer.known_revisions.iter().copied().max(),
        rejected_object_keys: peer.rejected_object_keys.clone(),
    }
}

/// Renew before each bounded list/GET, including historical and fallback work.
/// The UPDATE commits before network I/O. Expired or stolen claims cannot revive.
async fn renew_download_claim(
    pool: &PgPool,
    claim: &VerificationClaim,
) -> Result<(), ExecutionError> {
    let lease_micros = duration_micros("verification lease", claim.lease_duration)?;
    let result = sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET claim_expires_at = clock_timestamp() + $3::BIGINT * INTERVAL '1 microsecond',
               updated_at = clock_timestamp()
         WHERE id = $1
           AND state = 'claimed'
           AND claim_owner = $2
           AND claim_expires_at > clock_timestamp()
           AND attempt_count = $4
        "#,
        claim.task_id,
        &claim.worker_id,
        lease_micros,
        claim.attempt - 1,
    )
    .execute(pool)
    .await?;
    if result.rows_affected() != 1 {
        return Err(internal(format!(
            "verification claim for task {} is no longer owned by {}",
            claim.task_id, claim.worker_id,
        )));
    }
    Ok(())
}

pub(super) async fn list_peer_manifests<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    request: &PeerDownloadRequest,
) -> Result<Option<Vec<String>>, ExecutionError> {
    renew_download_claim(pool, claim).await?;
    let result = tokio::time::timeout(PEER_LIST_TIMEOUT, source.list_manifests(request)).await;
    let error = match result {
        Ok(Ok(object_keys)) => return Ok(Some(object_keys)),
        Ok(Err(error)) => error,
        Err(_) => ExecutionError::S3TransientError(format!(
            "peer manifest listing timed out after {PEER_LIST_TIMEOUT:?}"
        )),
    };

    let failure = PeerManifestFailure::Incomplete {
        reason: error.to_string(),
    };
    record_peer_failure(pool, claim, peer.publisher, &failure).await?;
    warn!(
        task_id = claim.task_id,
        publisher = %peer.publisher,
        error = %failure,
        "Peer manifest download failed"
    );
    Ok(None)
}

enum FetchedPeerObject {
    Body(PeerManifestObject),
    /// Listed immutable key is missing or oversized; try an older revision.
    Unusable(String),
    /// Transient get failure. Incomplete is already recorded; do not fall back.
    Unavailable,
}

async fn fetch_peer_manifest<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    request: &PeerDownloadRequest,
    object_key: &str,
) -> Result<FetchedPeerObject, ExecutionError> {
    renew_download_claim(pool, claim).await?;
    let result = tokio::time::timeout(
        PEER_OBJECT_TIMEOUT,
        source.fetch_manifest(request, object_key),
    )
    .await;
    let error = match result {
        Ok(Ok(object)) => return Ok(FetchedPeerObject::Body(object)),
        Ok(Err(ExecutionError::DbError(error))) => return Err(ExecutionError::DbError(error)),
        Ok(Err(
            error @ (ExecutionError::S3ObjectNotFound(_) | ExecutionError::PeerManifestTooLarge(_)),
        )) => {
            return Ok(FetchedPeerObject::Unusable(error.to_string()));
        }
        Ok(Err(error)) => error,
        Err(_) => ExecutionError::S3TransientError(format!(
            "peer manifest object fetch timed out after {PEER_OBJECT_TIMEOUT:?}"
        )),
    };
    let failure = PeerManifestFailure::Incomplete {
        reason: error.to_string(),
    };
    record_peer_failure(pool, claim, peer.publisher, &failure).await?;
    warn!(
        task_id = claim.task_id,
        publisher = %peer.publisher,
        object_key,
        error = %failure,
        "Peer manifest object download failed; retrying the same revision later"
    );
    Ok(FetchedPeerObject::Unavailable)
}

async fn remember_rejected_object(
    pool: &PgPool,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    object_key: &str,
) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    sqlx::query!(
        r#"
        UPDATE block_manifest_peer_download
           SET rejected_object_keys = CASE
                   WHEN $3 = ANY(rejected_object_keys) THEN rejected_object_keys
                   ELSE array_append(rejected_object_keys, $3)
               END,
               updated_at = NOW()
         WHERE task_id = $1
           AND publisher = $2
        "#,
        claim.task_id,
        peer.publisher.as_slice(),
        object_key,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

pub(super) async fn record_peer_failure(
    pool: &PgPool,
    claim: &VerificationClaim,
    publisher: Address,
    failure: &PeerManifestFailure,
) -> Result<(), ExecutionError> {
    PEER_MANIFEST_DOWNLOAD_FAILURE
        .with_label_values(&[&claim.scope.consensus_epoch])
        .inc();
    let error = failure.to_string();
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
        &error,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

pub(super) async fn require_active_claim(
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
    let derived_history = load_claim_derived_history(&mut trx, claim, &manifests).await?;
    let evaluation = evaluate_quorum_with_history(
        &manifests,
        &derived_history,
        claim.scope.local_publisher,
        claim.required_quorum,
    );
    let completed_history = load_completed_history(&mut trx, claim, &evaluation).await?;
    let peer_publishers = claim
        .peers
        .iter()
        .map(|peer| peer.publisher)
        .collect::<Vec<_>>();
    let localization = apply_evaluation_to_drift_handles(
        &mut trx,
        claim.task_id,
        &manifests,
        claim.scope.local_publisher,
        &evaluation,
        DriftLocalizationContext {
            peer_publishers: &peer_publishers,
            required_quorum: claim.required_quorum,
            completed_history: &completed_history,
        },
    )
    .await?;
    persist_verification_evidence(
        &mut trx,
        VerificationAttemptEvidence {
            task_id: claim.task_id,
            consensus_epoch: claim.scope.consensus_epoch.clone(),
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
    info!(
        task_id = claim.task_id,
        attempt = claim.attempt,
        consensus_epoch = %claim.scope.consensus_epoch,
        host_chain_id = claim.scope.host_chain_id,
        block_number = claim.scope.publication_block_number,
        block_hash = %claim.scope.publication_block_hash,
        revision = claim.scope.revision,
        local_publisher = %claim.scope.local_publisher,
        outcome = evaluation.outcome.as_db_str(),
        local_quorum_status = evaluation.local_quorum_status.as_db_str(),
        quorum_from_block = ?evaluation.coverage.quorum_from_block,
        quorum_through_block = ?evaluation.coverage.quorum_through_block,
        unverified_prefix_from_block = ?evaluation.coverage.unverified_prefix_from_block,
        unverified_prefix_through_block = ?evaluation.coverage.unverified_prefix_through_block,
        required_quorum = claim.required_quorum,
        archived_publishers = ?manifests.iter().map(|manifest| manifest.signed.payload.publisher).collect::<Vec<_>>(),
        localization_complete = localization.complete,
        drifted_block_count = ?localization.drifted_block_count,
        drifted_handle_count = ?localization.drifted_handle_count,
        "Completed peer manifest verification attempt"
    );
    VERIFICATION_OUTCOMES
        .with_label_values(&[&claim.scope.consensus_epoch, evaluation.outcome.as_db_str()])
        .inc();
    if !localization.complete {
        DRIFT_LOCALIZATION_INCOMPLETE
            .with_label_values(&[&claim.scope.consensus_epoch])
            .inc();
    }
    Ok(VerificationRunResult {
        task_id: claim.task_id,
        attempt: claim.attempt,
        outcome: evaluation.outcome,
    })
}

pub(super) async fn load_claim_manifests(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
) -> Result<Vec<AuthenticatedManifest>, ExecutionError> {
    let local_reference = ManifestReference {
        consensus_epoch: claim.scope.consensus_epoch.clone(),
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
        if let Some(manifest) = load_highest_archived_revision(
            trx,
            publisher,
            claim.scope.version,
            claim.scope.coprocessor_context_id,
            claim.scope.host_chain_id,
            &claim.scope.consensus_epoch,
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

pub(super) async fn load_claim_derived_history(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
    manifests: &[AuthenticatedManifest],
) -> Result<Vec<HistoricalCommitmentEvidence>, ExecutionError> {
    let local = manifests
        .iter()
        .find(|manifest| manifest.signed.payload.publisher == claim.scope.local_publisher)
        .ok_or_else(|| internal("local manifest is missing from derived history"))?;
    let predecessors = load_previous_local_manifests(
        trx,
        claim.scope.local_publisher,
        local,
        MAX_HISTORY_PREDECESSORS,
    )
    .await?;
    let scopes = local
        .signed
        .payload
        .historical_ranges
        .iter()
        .map(|range| CommitmentScope::Historical {
            first: range.start_block_number,
            last: range.end_block_number,
            scale: range.scale,
            end_block_hash: range.end_block_hash,
        })
        .collect::<Vec<_>>();
    let mut evidence = Vec::new();
    for peer in &claim.peers {
        if manifests
            .iter()
            .any(|manifest| manifest.signed.payload.publisher == peer.publisher)
        {
            continue;
        }
        let mut unresolved = scopes.iter().cloned().collect::<HashSet<_>>();
        for predecessor in &predecessors {
            let payload = &predecessor.signed.payload;
            let publication_block_number = i64::try_from(payload.publication_block_number)
                .map_err(|_| internal("predecessor publication block exceeds BIGINT"))?;
            let Some(peer_manifest) = load_highest_archived_revision(
                trx,
                peer.publisher,
                payload.version,
                payload.coprocessor_context_id,
                claim.scope.host_chain_id,
                &payload.consensus_epoch,
                publication_block_number,
                payload.publication_block_hash,
            )
            .await?
            else {
                continue;
            };
            let resolved = unresolved
                .iter()
                .filter_map(|scope| {
                    derive_historical_scope_digest(&peer_manifest.signed.payload, scope)
                        .map(|digest| (scope.clone(), digest))
                })
                .collect::<Vec<_>>();
            for (scope, digest) in resolved {
                unresolved.remove(&scope);
                evidence.push(HistoricalCommitmentEvidence {
                    publisher: peer.publisher,
                    scope,
                    digest,
                });
            }
            if unresolved.is_empty() {
                break;
            }
        }
    }
    Ok(evidence)
}

async fn apply_claimed_db_error(
    pool: &PgPool,
    claim: &VerificationClaim,
    error: &sqlx::Error,
) -> Result<(), ExecutionError> {
    match classify_db_error(error) {
        DbErrorClass::Transient => {
            warn!(
                task_id = claim.task_id,
                attempt = claim.attempt,
                host_chain_id = claim.scope.host_chain_id,
                block_number = claim.scope.publication_block_number,
                error = %error,
                "Transient verification database error; retrying later without charging the budget"
            );
            release_claimed_task_uncharged(pool, claim, &error.to_string()).await
        }
        DbErrorClass::Integrity => {
            error!(
                task_id = claim.task_id,
                attempt = claim.attempt,
                host_chain_id = claim.scope.host_chain_id,
                block_number = claim.scope.publication_block_number,
                error = %error,
                "Integrity verification database error"
            );
            VERIFICATION_FAILURE
                .with_label_values(&[&claim.scope.consensus_epoch])
                .inc();
            fail_claimed_task(pool, claim, &error.to_string(), false).await
        }
        DbErrorClass::Definitive => {
            error!(
                task_id = claim.task_id,
                attempt = claim.attempt,
                host_chain_id = claim.scope.host_chain_id,
                block_number = claim.scope.publication_block_number,
                error = %error,
                "Definitive verification database error"
            );
            VERIFICATION_FAILURE
                .with_label_values(&[&claim.scope.consensus_epoch])
                .inc();
            fail_claimed_task(pool, claim, &error.to_string(), true).await
        }
    }
}

/// Only a clearly transient failure keeps the budget. Any other error charges
/// an attempt, so a persistent one ends in `retry_exhausted` instead of being
/// reclaimed with the same attempt after every lease expiry.
async fn apply_claimed_error(
    pool: &PgPool,
    claim: &VerificationClaim,
    error: &ExecutionError,
) -> Result<(), ExecutionError> {
    if let ExecutionError::S3TransientError(_) = error {
        warn!(
            task_id = claim.task_id,
            attempt = claim.attempt,
            host_chain_id = claim.scope.host_chain_id,
            block_number = claim.scope.publication_block_number,
            error = %error,
            "Transient verification error; retrying later without charging the budget"
        );
        return release_claimed_task_uncharged(pool, claim, &error.to_string()).await;
    }
    error!(
        task_id = claim.task_id,
        attempt = claim.attempt,
        host_chain_id = claim.scope.host_chain_id,
        block_number = claim.scope.publication_block_number,
        error = %error,
        "Verification attempt failed"
    );
    VERIFICATION_FAILURE
        .with_label_values(&[&claim.scope.consensus_epoch])
        .inc();
    fail_claimed_task(pool, claim, &error.to_string(), false).await
}

/// Keeps the attempt budget but still waits the task's retry delay, at least
/// one second, so an error that repeats forever cannot re-run the task at the
/// poll's 100 ms slack.
async fn release_claimed_task_uncharged(
    pool: &PgPool,
    claim: &VerificationClaim,
    error: &str,
) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET state = 'pending',
               last_error = $3,
               next_attempt_at = NOW() + GREATEST(retry_delay_secs, 1) * INTERVAL '1 second',
               claim_owner = NULL,
               claim_expires_at = NULL,
               updated_at = NOW()
         WHERE id = $1
           AND claim_owner = $2
        "#,
        claim.task_id,
        &claim.worker_id,
        error,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

async fn fail_claimed_task(
    pool: &PgPool,
    claim: &VerificationClaim,
    error: &str,
    exhaust: bool,
) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    let target = sqlx::query!(
        "SELECT max_attempts, retry_delay_secs FROM block_manifest_verification_task WHERE id = $1",
        claim.task_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    let state = if exhaust || claim.attempt >= target.max_attempts {
        "retry_exhausted"
    } else {
        "pending"
    };
    sqlx::query!(
        r#"
        UPDATE block_manifest_verification_task
           SET attempt_count = $3,
               state = $4,
               last_attempt_at = NOW(),
               next_attempt_at = CASE
                   WHEN $4 = 'pending'
                   THEN NOW() + $6::BIGINT * INTERVAL '1 second'
                   ELSE NULL
               END,
               claim_owner = NULL,
               claim_expires_at = NULL,
               last_error = $5,
               updated_at = NOW()
         WHERE id = $1
           AND claim_owner = $2
        "#,
        claim.task_id,
        &claim.worker_id,
        claim.attempt,
        state,
        error,
        target.retry_delay_secs,
    )
    .execute(trx.as_mut())
    .await?;
    trx.commit().await?;
    Ok(())
}

async fn persist_claim_outcome(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
    evaluation: &QuorumEvaluation,
) -> Result<(), ExecutionError> {
    let target = sqlx::query!(
        "SELECT max_attempts, retry_delay_secs FROM block_manifest_verification_task WHERE id = $1",
        claim.task_id,
    )
    .fetch_one(trx.as_mut())
    .await?;
    let max_attempts = target.max_attempts;
    let retry_delay_secs = target.retry_delay_secs;
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
                   THEN NOW() + $6::BIGINT * INTERVAL '1 second'
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
        retry_delay_secs,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

#[cfg(test)]
#[path = "peer_downloader_tests.rs"]
mod tests;
