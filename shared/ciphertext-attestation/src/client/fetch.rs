//! Fans out a handle's attestation fetch across every registered Coprocessor bucket and
//! evaluates consensus over the results.

use crate::{
    client::{registry::CoprocessorRegistrySnapshot, s3::BoundedClient},
    consensus::{ConsensusOutcome, ConsensusRound},
};
use alloy::primitives::B256;
use tokio::task::JoinSet;
use tracing::warn;

/// Fetches the attestation for a `handle` and evaluates the consensus among coprocessors.
///
/// Consensus is evaluated as soon as enough attestations are received.
pub async fn fetch_attestations_and_check_consensus(
    client: &BoundedClient,
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
) -> ConsensusOutcome {
    let mut fetch_attestation_tasks = JoinSet::new();
    for entry in &registry.coprocessors {
        let client = client.clone();
        let (bucket, signer) = (entry.bucket.clone(), entry.signer);
        fetch_attestation_tasks.spawn(async move {
            let result = client.fetch_single_attestation(&bucket, handle).await;
            (signer, result)
        });
    }

    let entries = registry.coprocessors.iter().cloned();
    let mut round = ConsensusRound::new(handle, client.context_id(), entries, registry.threshold);

    while let Some(joined) = fetch_attestation_tasks.join_next().await {
        let (signer, fetch_result) = match joined {
            Ok(joined) => joined,
            Err(e) => {
                warn!(%handle, "Attestation fetch task panicked: {e}");
                continue;
            }
        };
        let outcome = match fetch_result {
            Ok(attestation) => round.record_attestation(signer, &attestation),
            Err(e) => {
                warn!(%signer, %handle, "Failed to fetch attestation: {e}");
                round.record_no_reply(signer)
            }
        };
        if let Some(outcome) = outcome {
            return outcome;
        }
    }

    round.close()
}
