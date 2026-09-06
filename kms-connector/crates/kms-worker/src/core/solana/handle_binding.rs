//! Handle binding: a sealed leaf, proven against the account's own peaks.
//!
//! Every decrypt permission on an encrypted value is a leaf of the account's MMR — an
//! `Allowed(key, handle)` leaf for a key, a `Public(handle)` leaf for everyone — and the account
//! itself holds only the peaks. Binding a handle to a key therefore means: the coprocessors' leaf
//! record says where the leaf is, and its sibling path hashes up to a peak the connector observed
//! on chain. The record supplies the path; the chain decides.
//!
//! ## Verify first, classify second
//!
//! A proof is verified against the observed peaks before its age is looked at. An append merges
//! only some peaks, so a proof built against an older leaf count very often still verifies — and
//! when it does it MUST be accepted. Rejecting on age would fail valid proofs after every append.
//!
//! Only once the record has *no* proof does its leaf count say anything, and all it says is which
//! of two things happened: the record has sealed at least as much history as the chain shows and
//! the leaf is not in it — there is no such permission — or the record is behind and may yet seal
//! it. The first is terminal, the second is retried.

use super::encrypted_value_account::ResolvedEncryptedValueAccount;
use super::proof::LeafProofOutcome;
use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use zama_solana_acl::{AclError, MmrProof, authorize_historical, authorize_public};

/// Establishes that `allowed_key` may decrypt `handle` under this encrypted value account.
///
/// Takes the resolved account — never raw account bytes — so the peaks it verifies against are
/// the validated ones, and the record's answer for exactly this `(account, handle, key)` leaf.
pub fn check_handle_binding(
    encrypted_value_account: &ResolvedEncryptedValueAccount,
    handle: HandleBytes,
    allowed_key: SolanaPubkeyBytes,
    outcome: &LeafProofOutcome,
) -> Result<(), HandleBindingFailure> {
    check_leaf(encrypted_value_account, outcome, |value, proof| {
        authorize_historical(
            encrypted_value_account.account_key(),
            value,
            handle,
            allowed_key,
            proof,
        )
    })
}

/// Establishes that `handle` was made public under this encrypted value account.
pub fn check_public_binding(
    encrypted_value_account: &ResolvedEncryptedValueAccount,
    handle: HandleBytes,
    outcome: &LeafProofOutcome,
) -> Result<(), HandleBindingFailure> {
    check_leaf(encrypted_value_account, outcome, |value, proof| {
        authorize_public(encrypted_value_account.account_key(), value, handle, proof)
    })
}

fn check_leaf(
    encrypted_value_account: &ResolvedEncryptedValueAccount,
    outcome: &LeafProofOutcome,
    verify: impl Fn(&zama_solana_acl::EncryptedValue, &MmrProof) -> Result<(), AclError>,
) -> Result<(), HandleBindingFailure> {
    let value = encrypted_value_account.encrypted_value();
    let live_leaf_count = value.leaf_count;

    // An MMR has one peak per set bit of its leaf count. A state that does not is the host
    // program's own inconsistency, and no proof can match it.
    if value.peaks.len() != live_leaf_count.count_ones() as usize {
        return Err(HandleBindingFailure::MmrStateInconsistent);
    }

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

    // A position the account does not have is refused before any hashing: there is nothing for
    // the proof to be a proof of, and the record is ahead of this observation.
    if leaf_index >= live_leaf_count {
        return Err(HandleBindingFailure::LeafIndexOutOfRange {
            leaf_index,
            leaf_count: live_leaf_count,
        });
    }

    // Verify first. Age is not a failure — an older proof whose peak survived still verifies,
    // and it must be accepted. Only a proof that genuinely does not verify is reported.
    let proof = MmrProof {
        leaf_index,
        siblings: siblings.clone(),
    };
    verify(value, &proof).map_err(|error| match error {
        AclError::HistoricalProofInvalid | AclError::PublicDecryptProofInvalid => {
            HandleBindingFailure::ProofDoesNotVerify {
                record_leaf_count,
                live_leaf_count,
            }
        }
        AclError::MmrInconsistent | AclError::MmrPeakCapacityExceeded => {
            HandleBindingFailure::MmrStateInconsistent
        }
        // Decoding outcomes cannot arise from proof verification. Enumerated rather than caught,
        // so a new outcome breaks the build.
        AclError::BadDiscriminator | AclError::BadAccountData => {
            HandleBindingFailure::MmrStateInconsistent
        }
    })
}

/// Why a handle was not bound.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum HandleBindingFailure {
    /// The record has sealed at least as much history as the chain shows, and no such leaf is in
    /// it: the permission was never granted.
    #[error(
        "no leaf for this key and handle in a record of {record_leaf_count} leaves against \
         {live_leaf_count} on chain"
    )]
    NoLeaf {
        /// How many leaves the record had sealed.
        record_leaf_count: u64,
        /// How many the account shows.
        live_leaf_count: u64,
    },
    /// The record has sealed less history than the chain shows and has no such leaf yet.
    #[error(
        "the leaf record is behind the chain ({record_leaf_count} leaves against \
         {live_leaf_count}) and has no such leaf yet"
    )]
    ProofRecordBehind {
        /// How many leaves the record had sealed.
        record_leaf_count: u64,
        /// How many the account shows.
        live_leaf_count: u64,
    },
    /// The record has never seen this account, which exists on chain.
    #[error("the leaf record does not know this encrypted value account")]
    AccountUnknownToProofRecord,
    /// The record's history for this account has a gap it cannot close.
    #[error("the leaf record's history for this encrypted value account is incomplete")]
    HistoryIncomplete,
    /// The proof did not verify against the observed peaks.
    #[error(
        "leaf proof does not verify against the observed peaks (record {record_leaf_count} \
         leaves, chain {live_leaf_count})"
    )]
    ProofDoesNotVerify {
        /// How many leaves the record had sealed when it built the proof.
        record_leaf_count: u64,
        /// How many the account shows.
        live_leaf_count: u64,
    },
    /// The proof names a leaf position the account does not have.
    #[error("leaf index {leaf_index} is not below the observed leaf count {leaf_count}")]
    LeafIndexOutOfRange {
        /// The position the proof claims.
        leaf_index: u64,
        /// The observed count.
        leaf_count: u64,
    },
    /// The account's own MMR state is internally inconsistent, which no retry can repair.
    #[error("encrypted value account MMR state is internally inconsistent")]
    MmrStateInconsistent,
}
