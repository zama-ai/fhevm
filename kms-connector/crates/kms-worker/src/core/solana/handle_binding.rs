//! Handle binding: the leaf that grants a key access to a handle, proven against the peaks of the
//! observed encrypted store. The coprocessors' leaf record supplies the path; the chain decides.
//!
//! A proof is verified before its age is considered: an append merges only some peaks, so a proof
//! built against an older leaf count often still verifies and must be accepted. Only when the
//! record has no proof does its leaf count matter: a record at least as long as the observed store
//! holds no grant, a shorter one may still catch up.

use super::encrypted_store::ResolvedEncryptedStore;
use super::proof::{HostProofReader, LeafProofOutcome, LeafQuery, ProofReadError, check_length};
use super::{HandleBytes, SolanaPubkeyBytes};
use zama_solana_acl::{
    AclError, EncryptedStore, MmrProof, authorize_state_historical, authorize_state_public,
};

/// Verifies every query's candidates from every peer against the same observation, then asks once
/// more for the queries that may still succeed. A failed second read never erases a first result.
/// Results are in batch order.
pub async fn verify_proofs_with_one_retry<P: HostProofReader, T: Sync>(
    reader: &P,
    batch: &[(LeafQuery, T)],
    verify: impl Fn(&T, &LeafProofOutcome) -> Result<(), HandleBindingFailure>,
) -> Result<Vec<Result<(), HandleBindingFailure>>, ProofReadError> {
    let queries: Vec<LeafQuery> = batch.iter().map(|(query, _)| *query).collect();
    let contexts = || batch.iter().map(|(_, context)| context);
    let response = reader.read_proofs(&queries).await?;
    let candidates = response.candidates;
    let mut unavailable = response.unavailable;
    check_length(queries.len(), candidates.len())?;
    let mut results: Vec<_> = candidates
        .iter()
        .zip(contexts())
        .map(|(candidates, context)| {
            verify_candidates(candidates, |candidate| verify(context, candidate))
        })
        .collect();
    let unresolved: Vec<_> = results
        .iter()
        .zip(contexts())
        .enumerate()
        .filter_map(|(position, (result, context))| match result {
            // A missing leaf is every peer's record agreeing with the observation, so asking the
            // same records again in this attempt cannot change it.
            Err(HandleBindingFailure::NoLeaf { .. }) if unavailable.is_none() => None,
            Err(error) if error.is_recoverable() || unavailable.is_some() => {
                Some((position, context))
            }
            _ => None,
        })
        .collect();
    if !unresolved.is_empty() {
        let queries: Vec<_> = unresolved
            .iter()
            .map(|&(position, _)| queries[position])
            .collect();
        if let Ok(again) = reader.read_proofs(&queries).await
            && check_length(queries.len(), again.candidates.len()).is_ok()
        {
            unavailable = again.unavailable;
            for ((position, context), candidates) in unresolved.into_iter().zip(again.candidates) {
                results[position] =
                    verify_candidates(&candidates, |candidate| verify(context, candidate));
            }
        }
    }
    if results.iter().any(Result::is_err)
        && let Some(error) = unavailable
    {
        return Err(error);
    }
    Ok(results)
}

fn verify_candidates(
    candidates: &[LeafProofOutcome],
    verify: impl Fn(&LeafProofOutcome) -> Result<(), HandleBindingFailure>,
) -> Result<(), HandleBindingFailure> {
    let mut failure: Option<HandleBindingFailure> = None;
    for candidate in candidates {
        match verify(candidate) {
            Ok(()) => return Ok(()),
            Err(error) => {
                if failure
                    .as_ref()
                    .is_none_or(|kept| precedence(&error) > precedence(kept))
                {
                    failure = Some(error);
                }
            }
        }
    }
    Err(failure.unwrap_or(HandleBindingFailure::AccountUnknownToProofRecord))
}

/// Which peer's failure a query reports. One peer cannot make another's temporary failure
/// permanent, and a missing leaf yields to any other temporary failure: peers that disagree are
/// worth a second read, and a missing leaf alone is not.
fn precedence(failure: &HandleBindingFailure) -> u8 {
    match failure {
        _ if !failure.is_recoverable() => 0,
        HandleBindingFailure::NoLeaf { .. } => 1,
        _ => 2,
    }
}

/// Establishes that `owner_address` may decrypt `handle` under this encrypted store. Taking the
/// resolved store means the proof is checked against validated peaks.
pub fn check_handle_binding(
    encrypted_store: &ResolvedEncryptedStore,
    handle: HandleBytes,
    owner_address: SolanaPubkeyBytes,
    outcome: &LeafProofOutcome,
) -> Result<(), HandleBindingFailure> {
    check_leaf(encrypted_store, outcome, |state, proof| {
        authorize_state_historical(
            encrypted_store.account_key(),
            state,
            handle,
            owner_address,
            proof,
        )
    })
}

/// Establishes that `handle` was made public under this encrypted store.
pub fn check_public_binding(
    encrypted_store: &ResolvedEncryptedStore,
    handle: HandleBytes,
    outcome: &LeafProofOutcome,
) -> Result<(), HandleBindingFailure> {
    check_leaf(encrypted_store, outcome, |state, proof| {
        authorize_state_public(encrypted_store.account_key(), state, handle, proof)
    })
}

fn check_leaf(
    encrypted_store: &ResolvedEncryptedStore,
    outcome: &LeafProofOutcome,
    verify: impl Fn(&EncryptedStore, &MmrProof) -> Result<(), AclError>,
) -> Result<(), HandleBindingFailure> {
    let state = encrypted_store.encrypted_store();
    let live_leaf_count = state.leaf_count;

    let (leaf_index, siblings, record_leaf_count) = match outcome {
        LeafProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => (*leaf_index, siblings, *leaf_count),
        LeafProofOutcome::NotFound { leaf_count } if *leaf_count >= live_leaf_count => {
            return Err(HandleBindingFailure::NoLeaf {
                record_leaf_count: *leaf_count,
                live_leaf_count,
            });
        }
        LeafProofOutcome::NotFound { leaf_count } => {
            return Err(HandleBindingFailure::ProofRecordBehind {
                record_leaf_count: *leaf_count,
                live_leaf_count,
            });
        }
        LeafProofOutcome::UnknownAccount => {
            return Err(HandleBindingFailure::AccountUnknownToProofRecord);
        }
        LeafProofOutcome::HistoryIncomplete => return Err(HandleBindingFailure::HistoryIncomplete),
    };

    // The record is ahead of this observation.
    if leaf_index >= live_leaf_count {
        return Err(HandleBindingFailure::LeafIndexOutOfRange {
            leaf_index,
            leaf_count: live_leaf_count,
        });
    }

    let proof = MmrProof {
        leaf_index,
        // The mountain containing this leaf only grows on append. A proof for a later
        // tree therefore starts with the path to the observed mountain's peak.
        siblings: siblings
            .iter()
            .take((live_leaf_count ^ leaf_index).ilog2() as usize)
            .copied()
            .collect(),
    };
    // The only way verification fails is an invalid proof.
    verify(state, &proof).map_err(|_| HandleBindingFailure::ProofDoesNotVerify {
        record_leaf_count,
        live_leaf_count,
    })
}

/// `record_leaf_count` is what the coprocessors' leaf record had sealed; `live_leaf_count` is what
/// the observed account shows.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum HandleBindingFailure {
    /// The record has sealed at least the observed history, and no grant is in it.
    #[error("no leaf for this key and handle in {record_leaf_count} of {live_leaf_count} leaves")]
    NoLeaf {
        record_leaf_count: u64,
        live_leaf_count: u64,
    },
    #[error("leaf record is behind the chain ({record_leaf_count} of {live_leaf_count} leaves)")]
    ProofRecordBehind {
        record_leaf_count: u64,
        live_leaf_count: u64,
    },
    #[error("the leaf record does not know this encrypted store")]
    AccountUnknownToProofRecord,
    #[error("the leaf record's history for this encrypted store is incomplete")]
    HistoryIncomplete,
    #[error(
        "leaf proof does not verify against the observed peaks (record {record_leaf_count} \
         leaves, chain {live_leaf_count})"
    )]
    ProofDoesNotVerify {
        record_leaf_count: u64,
        live_leaf_count: u64,
    },
    #[error("leaf index {leaf_index} is not below the observed leaf count {leaf_count}")]
    LeafIndexOutOfRange { leaf_index: u64, leaf_count: u64 },
}
