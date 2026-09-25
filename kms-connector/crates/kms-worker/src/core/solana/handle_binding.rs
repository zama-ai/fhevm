//! Handle binding: the leaf that grants a key access to a handle, proven against the peaks of the
//! observed encrypted store. The coprocessors' leaf record supplies the path; the chain decides.
//!
//! A proof is verified before its age is considered: an append merges only some peaks, so a proof
//! built against an older leaf count often still verifies and must be accepted. Only when the
//! record has no proof does its leaf count matter: a record at least as long as the observed store
//! holds no grant, a shorter one may still catch up.

use super::encrypted_store::ResolvedEncryptedStore;
use super::proof::{HostProofReader, LeafProofOutcome, LeafQuery, ProofReadError, check_length};
use alloy::primitives::B256;
use futures::stream::{FuturesUnordered, StreamExt};
use solana_pubkey::Pubkey;
use zama_solana_acl::{
    AclError, EncryptedStore, MmrProof, authorize_state_historical, authorize_state_public,
};

/// Asks every coprocessor for the whole batch at once, and returns one result per query in batch
/// order. Only a found proof that verifies against the observed peaks resolves a query; as soon as
/// every query is resolved the reads still running are dropped, so one slow coprocessor does not
/// hold a request another one can serve. Otherwise every coprocessor's answer is merged as it
/// arrives: a failed read or an answer of the wrong length says nothing about any leaf, and one
/// coprocessor's terminal failure does not replace another's recoverable one. A terminal failure
/// stands only if every coprocessor answered: a query that one of them could not answer, and no
/// other answered with a recoverable failure, is a [`ProofReadError`]. The worker loop is the only
/// retry layer.
pub async fn verify_proofs<P: HostProofReader, T: Sync>(
    reader: &P,
    batch: &[(LeafQuery, T)],
    verify: impl Fn(&T, &LeafProofOutcome) -> Result<(), HandleBindingFailure>,
) -> Result<Vec<Result<(), HandleBindingFailure>>, ProofReadError> {
    let queries: Vec<LeafQuery> = batch.iter().map(|(query, _)| *query).collect();
    let mut answers: FuturesUnordered<_> = (0..reader.source_count())
        .map(|source| {
            let queries = &queries;
            async move { (source, reader.read_proofs(source, queries).await) }
        })
        .collect();
    let mut results: Vec<Option<Result<(), HandleBindingFailure>>> = vec![None; batch.len()];
    let mut read_failures = Vec::new();
    while let Some((source, answer)) = answers.next().await {
        let outcomes = match answer.and_then(|outcomes| {
            check_length(queries.len(), outcomes.len())?;
            Ok(outcomes)
        }) {
            Ok(outcomes) => outcomes,
            Err(error) => {
                read_failures.push(format!("coprocessor {source}: {error}"));
                continue;
            }
        };
        for ((kept, (_, item)), outcome) in results.iter_mut().zip(batch).zip(&outcomes) {
            if *kept == Some(Ok(())) {
                continue;
            }
            let result = verify(item, outcome);
            let keeps_recoverable = matches!(kept, Some(Err(earlier)) if earlier.is_recoverable())
                && matches!(&result, Err(later) if !later.is_recoverable());
            if !keeps_recoverable {
                *kept = Some(result);
            }
        }
        if results.iter().all(|result| *result == Some(Ok(()))) {
            break;
        }
    }
    let unanswered = !read_failures.is_empty();
    results
        .into_iter()
        .map(|result| match result {
            Some(Err(failure)) if unanswered && !failure.is_recoverable() => {
                Err(ProofReadError::Unavailable {
                    reason: format!(
                        "{failure}, and not every coprocessor answered: {}",
                        read_failures.join("; ")
                    ),
                })
            }
            Some(result) => Ok(result),
            None => Err(ProofReadError::Unavailable {
                reason: format!("no coprocessor answered: {}", read_failures.join("; ")),
            }),
        })
        .collect()
}

/// Establishes that `owner_address` may decrypt `handle` under this encrypted store. Taking the
/// resolved store means the proof is checked against validated peaks.
pub fn check_handle_binding(
    encrypted_store: &ResolvedEncryptedStore,
    handle: B256,
    owner_address: Pubkey,
    outcome: &LeafProofOutcome,
) -> Result<(), HandleBindingFailure> {
    check_leaf(encrypted_store, outcome, |state, proof| {
        authorize_state_historical(
            encrypted_store.account_key().to_bytes(),
            state,
            handle.0,
            owner_address.to_bytes(),
            proof,
        )
    })
}

/// Establishes that `handle` was made public under this encrypted store.
pub fn check_public_binding(
    encrypted_store: &ResolvedEncryptedStore,
    handle: B256,
    outcome: &LeafProofOutcome,
) -> Result<(), HandleBindingFailure> {
    check_leaf(encrypted_store, outcome, |state, proof| {
        authorize_state_public(
            encrypted_store.account_key().to_bytes(),
            state,
            handle.0,
            proof,
        )
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

    // A leaf past the observed count means the record is ahead of this observation.
    let proof = MmrProof::for_leaf_count(leaf_index, siblings, live_leaf_count).ok_or(
        HandleBindingFailure::LeafIndexOutOfRange {
            leaf_index,
            leaf_count: live_leaf_count,
        },
    )?;
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
