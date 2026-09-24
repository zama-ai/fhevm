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

/// Asks the coprocessors in configured order until every query is resolved, and returns one result
/// per query in batch order. Each coprocessor is asked only for the queries still unresolved, and
/// only a found proof that verifies against the observed peaks resolves one: any other answer, a
/// failed read, or an answer of the wrong length moves on to the next coprocessor. A query no
/// coprocessor answered is a [`ProofReadError`]; otherwise it keeps its last failure, except that
/// one coprocessor's terminal failure does not replace another's recoverable one. The worker loop
/// is the only retry layer.
pub async fn verify_proofs<P: HostProofReader, T: Sync>(
    reader: &P,
    batch: &[(LeafQuery, T)],
    verify: impl Fn(&T, &LeafProofOutcome) -> Result<(), HandleBindingFailure>,
) -> Result<Vec<Result<(), HandleBindingFailure>>, ProofReadError> {
    let mut results: Vec<Option<Result<(), HandleBindingFailure>>> = vec![None; batch.len()];
    let mut read_failures = Vec::new();
    for source in 0..reader.source_count() {
        let unresolved: Vec<usize> = (0..batch.len())
            .filter(|&position| results[position] != Some(Ok(())))
            .collect();
        if unresolved.is_empty() {
            break;
        }
        let queries: Vec<LeafQuery> = unresolved
            .iter()
            .map(|&position| batch[position].0)
            .collect();
        let outcomes = match reader
            .read_proofs(source, &queries)
            .await
            .and_then(|outcomes| {
                check_length(queries.len(), outcomes.len())?;
                Ok(outcomes)
            }) {
            Ok(outcomes) => outcomes,
            Err(error) => {
                read_failures.push(format!("coprocessor {source}: {error}"));
                continue;
            }
        };
        for (position, outcome) in unresolved.into_iter().zip(&outcomes) {
            let result = verify(&batch[position].1, outcome);
            let kept = &mut results[position];
            let keeps_recoverable = matches!(kept, Some(Err(earlier)) if earlier.is_recoverable())
                && matches!(&result, Err(later) if !later.is_recoverable());
            if !keeps_recoverable {
                *kept = Some(result);
            }
        }
    }
    results
        .into_iter()
        .map(|result| {
            result.ok_or_else(|| ProofReadError::Unavailable {
                reason: format!("no coprocessor answered: {}", read_failures.join("; ")),
            })
        })
        .collect()
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
