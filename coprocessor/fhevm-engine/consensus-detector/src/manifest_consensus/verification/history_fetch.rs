use std::collections::HashSet;

use alloy_primitives::{B256, U256};
use sqlx::PgPool;

use crate::manifest_consensus::manifest_archive::{
    load_highest_archived_revision, load_local_manifest_covering_block,
    load_previous_local_manifests, AuthenticatedManifest, MAX_HISTORY_PREDECESSORS,
};
use crate::manifest_consensus::ExecutionError;

use super::consensus_analysis::{
    evaluate_quorum, evaluate_quorum_with_history, CommitmentScope, QuorumEvaluation,
    VerificationOutcome,
};
use super::localization_cache::{load_completed_history, without_completed_history};
use super::peer_downloader::{
    download_highest_verified_manifest, list_peer_manifests, load_claim_derived_history,
    load_claim_manifests, record_peer_failure, require_active_claim, PeerManifestFailure,
};
use super::peer_manifest_source::{PeerDownloadRequest, PeerManifestSource};
use super::verification_queue::{load_known_peer_revisions_at, ClaimedPeer, VerificationClaim};
use super::verification_utils::internal;
use tracing::debug;

pub(super) async fn archive_peer_history_predecessors<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
) -> Result<(), ExecutionError> {
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    let manifests = load_claim_manifests(&mut trx, claim).await?;
    let local = manifests
        .iter()
        .find(|manifest| manifest.signed.payload.publisher == claim.scope.local_publisher)
        .ok_or_else(|| internal("local manifest is missing from history fallback"))?;
    let predecessors = load_previous_local_manifests(
        &mut trx,
        claim.scope.local_publisher,
        local,
        MAX_HISTORY_PREDECESSORS,
    )
    .await?;
    trx.commit().await?;

    for predecessor in predecessors {
        let _ = load_or_archive_peer_manifest_at(pool, source, claim, peer, &predecessor).await?;
    }
    Ok(())
}

pub(super) async fn archive_history_for_disagreements<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
) -> Result<(), ExecutionError> {
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
    let evaluation = without_completed_history(&evaluation, &completed_history);
    trx.commit().await?;

    let mut visited = HashSet::new();
    archive_historical_disagreements(
        pool,
        source,
        claim,
        &manifests,
        &evaluation,
        HistoricalWindow {
            first: U256::ZERO,
            last: U256::MAX,
        },
        &mut visited,
    )
    .await?;
    Ok(())
}

/// Inclusive block interval currently being localized in historical evidence.
///
/// Recursive fallback intersects child ranges with this window so disagreement
/// localization cannot escape into an adjacent dyadic range.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct HistoricalWindow {
    first: U256,
    last: U256,
}

async fn archive_historical_disagreements<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    manifests: &[AuthenticatedManifest],
    evaluation: &QuorumEvaluation,
    window: HistoricalWindow,
    visited: &mut HashSet<(B256, HistoricalWindow)>,
) -> Result<(), ExecutionError> {
    let local = manifests
        .iter()
        .find(|manifest| manifest.signed.payload.publisher == claim.scope.local_publisher)
        .ok_or_else(|| internal("local manifest is missing from historical scan"))?;

    for scope_evaluation in &evaluation.scopes {
        let CommitmentScope::Historical {
            first,
            last,
            end_block_hash,
            ..
        } = &scope_evaluation.scope
        else {
            continue;
        };
        let Some(window) = intersect_historical_window(window, *first, *last) else {
            continue;
        };
        if scope_evaluation.local_digest.is_none() || scope_evaluation.groups.len() <= 1 {
            continue;
        }

        if !local
            .signed
            .payload
            .historical_ranges
            .iter()
            .any(|range| range.end_block_number == *last && range.end_block_hash == *end_block_hash)
        {
            return Err(internal("local historical range is missing"));
        }

        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        let covering = load_local_manifest_covering_block(
            &mut trx,
            claim.scope.local_publisher,
            local.signed.payload.version,
            local.signed.payload.coprocessor_context_id,
            claim.scope.host_chain_id,
            &local.signed.payload.consensus_epoch,
            *last,
            *end_block_hash,
        )
        .await?;
        trx.commit().await?;
        let Some(covering) = covering else {
            continue;
        };
        let mut trx = pool.begin().await?;
        require_active_claim(&mut trx, claim).await?;
        let predecessors = load_previous_local_manifests(
            &mut trx,
            claim.scope.local_publisher,
            &covering,
            MAX_HISTORY_PREDECESSORS,
        )
        .await?;
        let mut anchors = vec![covering];
        anchors.extend(predecessors);
        trx.commit().await?;

        for anchor in anchors {
            if !visited.insert((anchor.digest, window)) {
                continue;
            }
            let mut comparison_manifests = vec![anchor.clone()];
            for peer in &claim.peers {
                if let Some(manifest) =
                    load_or_archive_peer_manifest_at(pool, source, claim, peer, &anchor).await?
                {
                    comparison_manifests.push(manifest);
                }
            }
            let comparison = evaluate_quorum(
                &comparison_manifests,
                claim.scope.local_publisher,
                claim.required_quorum,
            );
            if comparison.outcome == VerificationOutcome::Drift {
                Box::pin(archive_historical_disagreements(
                    pool,
                    source,
                    claim,
                    &comparison_manifests,
                    &comparison,
                    window,
                    visited,
                ))
                .await?;
                break;
            }
            if comparison.outcome == VerificationOutcome::Consensus {
                break;
            }
        }
    }
    Ok(())
}

fn intersect_historical_window(
    window: HistoricalWindow,
    first: U256,
    last: U256,
) -> Option<HistoricalWindow> {
    let first = first.max(window.first);
    let last = last.min(window.last);
    (first <= last).then_some(HistoricalWindow { first, last })
}

async fn load_or_archive_peer_manifest_at<S: PeerManifestSource>(
    pool: &PgPool,
    source: &S,
    claim: &VerificationClaim,
    peer: &ClaimedPeer,
    local_covering_manifest: &AuthenticatedManifest,
) -> Result<Option<AuthenticatedManifest>, ExecutionError> {
    let payload = &local_covering_manifest.signed.payload;
    let publication_block_number = i64::try_from(payload.publication_block_number)
        .map_err(|_| internal("covering manifest publication block number exceeds BIGINT"))?;
    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    if let Some(manifest) = load_highest_archived_revision(
        &mut trx,
        peer.publisher,
        payload.version,
        payload.coprocessor_context_id,
        claim.scope.host_chain_id,
        &payload.consensus_epoch,
        publication_block_number,
        payload.publication_block_hash,
    )
    .await?
    {
        trx.commit().await?;
        return Ok(Some(manifest));
    }
    if claim.attempt > 1 {
        trx.commit().await?;
        debug!(
            task_id = claim.task_id,
            publisher = %peer.publisher,
            publication_block_number,
            attempt = claim.attempt,
            "Skipping covering S3 fetch on same-task retry; missing covering is retried after a later publication"
        );
        return Ok(None);
    }
    let known_revisions = load_known_peer_revisions_at(
        &mut trx,
        peer.publisher,
        payload.version,
        payload.coprocessor_context_id,
        claim.scope.host_chain_id,
        &payload.consensus_epoch,
        publication_block_number,
        payload.publication_block_hash,
    )
    .await?;
    trx.commit().await?;

    let request = peer_download_request_for_manifest(peer, payload, known_revisions)?;
    let Some(object_keys) = list_peer_manifests(pool, source, claim, peer, &request).await? else {
        return Ok(None);
    };
    if object_keys.is_empty() {
        let failure = PeerManifestFailure::Incomplete {
            reason: format!(
                "peer has no archived manifest covering block {}",
                payload.publication_block_number
            ),
        };
        record_peer_failure(pool, claim, peer.publisher, &failure).await?;
        return Ok(None);
    }
    let Some(_) = download_highest_verified_manifest(
        pool,
        source,
        claim,
        peer,
        &request,
        object_keys,
        |manifest| request_matches_manifest(&request, manifest),
        "local covering manifest",
    )
    .await?
    else {
        return Ok(None);
    };

    let mut trx = pool.begin().await?;
    require_active_claim(&mut trx, claim).await?;
    let manifest = load_highest_archived_revision(
        &mut trx,
        peer.publisher,
        payload.version,
        payload.coprocessor_context_id,
        claim.scope.host_chain_id,
        &payload.consensus_epoch,
        publication_block_number,
        payload.publication_block_hash,
    )
    .await?;
    trx.commit().await?;
    Ok(manifest)
}

fn peer_download_request_for_manifest(
    peer: &ClaimedPeer,
    payload: &block_manifest::ManifestPayload,
    known_revisions: HashSet<u64>,
) -> Result<PeerDownloadRequest, ExecutionError> {
    Ok(PeerDownloadRequest {
        publisher: peer.publisher,
        s3_bucket_url: peer.s3_bucket_url.clone(),
        version: payload.version,
        generation: payload.consensus_epoch.clone(),
        coprocessor_context_id: payload.coprocessor_context_id,
        host_chain_id: i64::try_from(payload.host_chain_id)
            .map_err(|_| internal("covering manifest host chain id exceeds BIGINT"))?,
        publication_block_number: i64::try_from(payload.publication_block_number)
            .map_err(|_| internal("covering manifest publication block number exceeds BIGINT"))?,
        publication_block_hash: payload.publication_block_hash,
        highest_archived_revision: known_revisions.iter().copied().max(),
        rejected_object_keys: peer.rejected_object_keys.clone(),
    })
}

fn request_matches_manifest(
    request: &PeerDownloadRequest,
    manifest: &AuthenticatedManifest,
) -> bool {
    let Ok(host_chain_id) = u64::try_from(request.host_chain_id) else {
        return false;
    };
    let Ok(publication_block_number) = u64::try_from(request.publication_block_number) else {
        return false;
    };
    let payload = &manifest.signed.payload;
    payload.publisher == request.publisher
        && payload.version == request.version
        && payload.consensus_epoch == request.generation
        && payload.coprocessor_context_id == request.coprocessor_context_id
        && payload.host_chain_id == U256::from(host_chain_id)
        && payload.publication_block_number == U256::from(publication_block_number)
        && payload.publication_block_hash == request.publication_block_hash
}
