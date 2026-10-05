//! Handle binding: the leaf that grants a key access to a handle, proven against the peaks of the
//! observed encrypted store. The coprocessors' leaf record supplies the path; the chain decides.
//!
//! A proof is verified before its age is considered: an append merges only some peaks, so a proof
//! built against an older leaf count often still verifies and must be accepted. Only when the
//! record has no proof does its leaf count matter: a record at least as long as the observed store
//! holds no grant, a shorter one may still catch up.

use super::encrypted_store::ResolvedEncryptedStore;
use super::proof::{HostProofReader, LeafQuery, MerkleProofOutcome, ProofReadError, check_length};
use alloy::primitives::B256;
use futures::stream::{FuturesUnordered, StreamExt};
use solana_pubkey::Pubkey;
use std::time::Duration;
use zama_solana_acl::{
    AclError, EncryptedStore, MmrProof, authorize_state_historical, authorize_state_public,
};

/// How long a proof read may run before the next coprocessor is asked as well. A proof read takes
/// tens of milliseconds, so a coprocessor slower than this is most likely stalled or overloaded.
pub const HEDGE_DELAY: Duration = Duration::from_millis(250);

/// Asks the coprocessors for the whole batch one after another, in the reader's
/// [`HostProofReader::hedge_order`], and returns one result per query in batch order. The next
/// coprocessor is asked as soon as an answer leaves a query unresolved, or once [`HEDGE_DELAY`]
/// passes without one, so a lagging, failing or stalled coprocessor costs at most that delay. Only
/// a found proof that verifies against the observed peaks resolves a query; once every query is
/// resolved the reads still running are dropped. Otherwise a query keeps the last failure a
/// coprocessor answered; a failed read or an answer of the wrong length says nothing about any
/// leaf. A query no coprocessor answered is a [`ProofReadError`]. Every coprocessor receives the
/// same prepared batch. The worker loop is the only retry layer.
pub async fn verify_proofs<P: HostProofReader, T: Sync>(
    reader: &P,
    batch: &[(LeafQuery, T)],
    verify: impl Fn(&T, &MerkleProofOutcome) -> Result<(), HandleBindingFailure>,
) -> Result<Vec<Result<(), HandleBindingFailure>>, ProofReadError> {
    let queries: Vec<LeafQuery> = batch.iter().map(|(query, _)| *query).collect();
    let prepared = reader.prepare(&queries).await?;
    let read = |source: usize| {
        let prepared = &prepared;
        async move { (source, reader.read_proofs(source, prepared).await) }
    };
    let mut unasked = reader.hedge_order().into_iter();
    let mut answers = FuturesUnordered::new();
    answers.extend(unasked.next().map(read));
    let mut results: Vec<Option<Result<(), HandleBindingFailure>>> = vec![None; batch.len()];
    let mut read_failures = Vec::new();
    loop {
        let (source, answer) = tokio::select! {
            Some(answer) = answers.next() => answer,
            () = tokio::time::sleep(HEDGE_DELAY), if !unasked.as_slice().is_empty() => {
                answers.extend(unasked.next().map(read));
                continue;
            }
            else => break,
        };
        match answer.and_then(|outcomes| {
            check_length(queries.len(), outcomes.len())?;
            Ok(outcomes)
        }) {
            Ok(outcomes) => {
                for ((kept, (_, item)), outcome) in results.iter_mut().zip(batch).zip(&outcomes) {
                    if *kept != Some(Ok(())) {
                        *kept = Some(verify(item, outcome));
                    }
                }
            }
            Err(error) => read_failures.push(format!("coprocessor {source}: {error}")),
        }
        if results.iter().all(|result| *result == Some(Ok(()))) {
            break;
        }
        answers.extend(unasked.next().map(read));
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
    handle: B256,
    owner_address: Pubkey,
    outcome: &MerkleProofOutcome,
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
    outcome: &MerkleProofOutcome,
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
    outcome: &MerkleProofOutcome,
    verify: impl Fn(&EncryptedStore, &MmrProof) -> Result<(), AclError>,
) -> Result<(), HandleBindingFailure> {
    let state = encrypted_store.encrypted_store();
    let live_leaf_count = state.leaf_count;

    let (leaf_index, siblings, record_leaf_count) = match outcome {
        MerkleProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => (*leaf_index, siblings, *leaf_count),
        MerkleProofOutcome::NotFound { leaf_count } if *leaf_count >= live_leaf_count => {
            return Err(HandleBindingFailure::NoLeaf {
                record_leaf_count: *leaf_count,
                live_leaf_count,
            });
        }
        MerkleProofOutcome::NotFound { leaf_count } => {
            return Err(HandleBindingFailure::ProofRecordBehind {
                record_leaf_count: *leaf_count,
                live_leaf_count,
            });
        }
        MerkleProofOutcome::UnknownAccount => {
            return Err(HandleBindingFailure::AccountUnknownToProofRecord);
        }
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
    #[error(
        "Merkle proof does not verify against the observed peaks (record {record_leaf_count} \
         leaves, chain {live_leaf_count})"
    )]
    ProofDoesNotVerify {
        record_leaf_count: u64,
        live_leaf_count: u64,
    },
    #[error("leaf index {leaf_index} is not below the observed leaf count {leaf_count}")]
    LeafIndexOutOfRange { leaf_index: u64, leaf_count: u64 },
}
