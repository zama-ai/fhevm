//! Fans out a handle's attestation fetch across every registered Coprocessor bucket and
//! evaluates consensus over the results.

use crate::{registry::CoprocessorRegistrySnapshot, s3};
use alloy::{
    primitives::{B256, U256},
    transports::http::Client,
};
use ciphertext_attestation::{
    consensus::Consensus,
    tracker::{ConsensusStatus, ConsensusTracker, ValidAttestation},
};
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::warn;

/// Why a handle has no attestation consensus this round.
///
/// The two variants call for opposite responses, which is why they are distinguished here rather
/// than collapsed into one opaque error: [`Self::Starved`] is the expected early state, since
/// Coprocessors publish attestations asynchronously, so it is worth retrying. [`Self::Split`]
/// means the signers that answered disagree and no vote still to come could break the tie —
/// retrying re-reads the same disagreement.
#[derive(Debug, thiserror::Error)]
pub enum ConsensusCheckError {
    /// Too few usable replies for this round to reach the threshold. Retriable: attestations are
    /// published asynchronously, so this is the normal early state.
    #[error("only {valid} valid attestation(s), need {required}")]
    Starved { valid: usize, required: usize },

    /// The signers that answered disagree, and even every Coprocessor yet to vote joining the
    /// largest group would fall short of the threshold. Terminal: cast votes do not change.
    #[error("{valid} valid attestation(s), but no group of {largest} reached the threshold")]
    Split { valid: usize, largest: usize },
}

/// A reached consensus together with the buckets to fetch the ciphertext from.
///
/// `winning_buckets` (the buckets whose registered signer is in the winning group) exists only so
/// a ciphertext-retrieving consumer (e.g. the KMS Connector) can fetch the ciphertext bytes from a
/// winning-group bucket; a consumer that only needs the consensus verdict can ignore it.
#[derive(Debug)]
pub struct ResolvedConsensus {
    pub consensus: Consensus,
    pub winning_buckets: Vec<String>,
}

/// Fetches the attestation for a `handle` from the registered Coprocessor buckets concurrently
/// and evaluates the consensus.
///
/// Tries to evaluate the consensus as soon as enough attestations are received, without waiting
/// for slow or unreachable buckets.
///
/// On success returns the winning [`Consensus`] together with the URLs of the winning-group
/// buckets (those whose registered signer vouches for the winning material).
pub async fn fetch_attestations_and_check_consensus(
    client: &Client,
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
    head_timeout: Duration,
    context_id: U256,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    let mut fetch_attestation_tasks = JoinSet::new();
    for entry in &registry.coprocessors {
        let (client, head_timeout) = (client.clone(), head_timeout);
        let (bucket, signer) = (entry.bucket.clone(), entry.signer);
        fetch_attestation_tasks.spawn(async move {
            let result =
                s3::fetch_single_attestation(&client, &bucket, handle, head_timeout, context_id)
                    .await;
            (signer, result)
        });
    }

    let mut tracker = ConsensusTracker::new(registry.coprocessors.len(), registry.threshold);

    // Every one of the `registry.coprocessors.len()` spawned tasks must feed the tracker exactly
    // one event below (a vote or a failure) — that is what makes the tracker's pending count, and
    // therefore its verdict, correct.
    while let Some(joined) = fetch_attestation_tasks.join_next().await {
        let status = match joined {
            Err(e) => {
                warn!("Attestation fetch task panicked: {e}");
                tracker.record_failure()
            }
            Ok((signer, Err(e))) => {
                warn!(%signer, %handle, "Failed to fetch attestation: {e}");
                tracker.record_failure()
            }
            Ok((signer, Ok(attestation))) => {
                match ValidAttestation::validate(&attestation, handle, context_id, signer) {
                    Ok(vote) => tracker.add_vote(vote),
                    Err(e) => {
                        warn!(%signer, %handle, "Discarding invalid attestation: {e}");
                        tracker.record_failure()
                    }
                }
            }
        };

        match status {
            ConsensusStatus::Reached(consensus) => {
                let winning_buckets = winning_buckets(registry, &consensus);
                // Early-exit: dropping remaining tasks aborts the other in-flight HEAD requests.
                return Ok(ResolvedConsensus {
                    consensus,
                    winning_buckets,
                });
            }
            ConsensusStatus::Pending => continue,
            ConsensusStatus::Starved { valid, required } => {
                return Err(ConsensusCheckError::Starved { valid, required });
            }
            ConsensusStatus::Split { valid, largest } => {
                return Err(ConsensusCheckError::Split { valid, largest });
            }
        }
    }

    // Only reachable with an empty registry: with at least one Coprocessor the invariant above
    // leaves the tracker `Reached`, `Starved` or `Split` once the JoinSet drains, and all three
    // return from inside the loop. `load` rejects an empty registry, but this runs on the
    // synchronous request path, so it fails closed instead of panicking.
    Err(ConsensusCheckError::Starved {
        valid: 0,
        required: registry.threshold.get(),
    })
}

/// Collects the URLs of the buckets whose registered signer is in the winning group.
fn winning_buckets(registry: &CoprocessorRegistrySnapshot, consensus: &Consensus) -> Vec<String> {
    registry
        .coprocessors
        .iter()
        .filter(|entry| consensus.signers.contains(&entry.signer))
        .map(|entry| entry.bucket.clone())
        .collect()
}
