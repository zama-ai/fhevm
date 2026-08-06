//! Handle binding: current membership, or an inclusion proof for a replaced handle.
//!
//! The mode is chosen per entry by the presence of an access proof, and one request freely
//! mixes both. In either mode the subject whose access is established is the entry's owner —
//! the signer in the direct branch, the delegator in the delegated one.
//!
//! ## Verify first, classify second
//!
//! An inclusion proof is verified against the live peaks of the snapshot before anything is
//! compared for age. An append merges only some peaks, so a proof built against an older leaf
//! count very often still verifies — and when it does it MUST be accepted. Rejecting on age
//! would fail valid proofs after every append, which is a denial of service dressed as
//! strictness.
//!
//! Only once inclusion has actually failed does `proofLeafCount` say anything, and all it
//! says is which of two actions the client should take: rebuild the proof, or retry later.
//! It is a request field, it is not authoritative, and it never decides whether a proof is
//! accepted.
//!
//! That last sentence is a property of the signatures here, not a discipline to remember:
//! [`check_handle_binding`] is not given the claimed count, so no implementation of it can
//! consult one. The count reaches [`classify_inclusion_failure`] only, and only the pipeline
//! calls it — after the binding rule has already said no.

use super::encrypted_value_account::ResolvedEncryptedValueAccount;
use super::request::AccessEvidence;
use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use zama_solana_acl::{AclError, MmrProof, authorize_current, authorize_historical};

/// Establishes that `subject` may decrypt `handle` under this encrypted value account.
///
/// Takes the resolved encrypted value account — never raw account bytes — so the membership set and
/// the MMR commitments it reads are the validated ones.
pub fn check_handle_binding(
    encrypted_value_account: &ResolvedEncryptedValueAccount,
    handle: HandleBytes,
    subject: SolanaPubkeyBytes,
    access: &AccessEvidence,
) -> Result<(), HandleBindingFailure> {
    match access {
        AccessEvidence::Current => check_current(encrypted_value_account, handle, subject),
        AccessEvidence::Historical(proof) => {
            check_historical(encrypted_value_account, handle, subject, proof)
        }
    }
}

/// Current mode: the named handle is the live one and the subject is in the live subject set.
fn check_current(
    encrypted_value_account: &ResolvedEncryptedValueAccount,
    handle: HandleBytes,
    subject: SolanaPubkeyBytes,
) -> Result<(), HandleBindingFailure> {
    authorize_current(encrypted_value_account.encrypted_value(), handle, subject).map_err(|error| {
        match error {
            AclError::HandleMismatch => HandleBindingFailure::NotCurrentHandle {
                requested: handle,
                current: encrypted_value_account.encrypted_value().current_handle,
            },
            AclError::SubjectMissing => HandleBindingFailure::NotAMember { subject },
            // Current mode consults no MMR and no proof, so nothing else the shared rule can say
            // reaches here. Enumerated rather than caught, so a new outcome breaks the build.
            AclError::MmrInconsistent
            | AclError::MmrPeakCapacityExceeded
            | AclError::SubjectCapacityExceeded
            | AclError::BadDiscriminator
            | AclError::BadAccountData
            | AclError::HistoricalProofInvalid
            | AclError::PublicDecryptProofInvalid => HandleBindingFailure::MmrStateInconsistent,
        }
    })
}

/// Historical mode: a sealed leaf naming this encrypted value account, position, handle and
/// subject, proven against the observed peaks.
fn check_historical(
    encrypted_value_account: &ResolvedEncryptedValueAccount,
    handle: HandleBytes,
    subject: SolanaPubkeyBytes,
    proof: &MmrProof,
) -> Result<(), HandleBindingFailure> {
    let value = encrypted_value_account.encrypted_value();

    // An MMR has one peak per set bit of its leaf count. A state that does not is the host
    // program's own inconsistency, and calling it a proof failure would tell the client to
    // rebuild a proof against a state no proof can match.
    if value.peaks.len() != value.leaf_count.count_ones() as usize {
        return Err(HandleBindingFailure::MmrStateInconsistent);
    }

    // A position the encrypted value account does not have is refused before any hashing: there is
    // nothing for the proof to be a proof of, and the distinction is worth naming in the failure.
    if proof.leaf_index >= value.leaf_count {
        return Err(HandleBindingFailure::LeafIndexOutOfRange {
            leaf_index: proof.leaf_index,
            leaf_count: value.leaf_count,
        });
    }

    // Verify first. Age is not a failure — an append merges only some peaks, so an older proof
    // whose peak survived still verifies, and it must be accepted. Only a proof that genuinely
    // does not verify is reported, and it carries the observed count and not the claimed one.
    authorize_historical(
        encrypted_value_account.account_key(),
        value,
        handle,
        subject,
        proof,
    )
    .map_err(|error| {
        match error {
            AclError::HistoricalProofInvalid => HandleBindingFailure::ProofDoesNotVerify {
                live_leaf_count: value.leaf_count,
            },
            AclError::MmrInconsistent | AclError::MmrPeakCapacityExceeded => {
                HandleBindingFailure::MmrStateInconsistent
            }
            // The current-mode and decoding outcomes cannot arise from proof verification.
            AclError::SubjectCapacityExceeded
            | AclError::BadDiscriminator
            | AclError::BadAccountData
            | AclError::HandleMismatch
            | AclError::SubjectMissing
            | AclError::PublicDecryptProofInvalid => HandleBindingFailure::MmrStateInconsistent,
        }
    })
}

/// Classifies an inclusion failure into the action the client should take.
///
/// The two counts are diagnostic input to this function and to nothing else: a proof that
/// verified is accepted whatever they say.
pub fn classify_inclusion_failure(proof_leaf_count: u64, live_leaf_count: u64) -> InclusionAction {
    if proof_leaf_count < live_leaf_count {
        // The proof was built against less history than was observed, and it did not survive
        // the appends in between: its peak was merged and its sibling path no longer exists.
        InclusionAction::RebuildProof
    } else {
        // Equal counts with disagreeing peaks is fork disagreement rather than staleness, and a
        // higher claimed count means the proof service is ahead of this observation. Both may
        // agree at a later one, with the very same proof.
        InclusionAction::RetryAtLaterSnapshot
    }
}

/// What a client should do about a proof that did not verify.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InclusionAction {
    /// The proof predates an append that merged its peak: it can never verify again, and a
    /// fresh proof is needed. Terminal for this request.
    RebuildProof,
    /// The proof service observed more state than this snapshot did — it is ahead, or the two
    /// are on disagreeing confirmed forks. The same proof may verify from a later
    /// observation.
    RetryAtLaterSnapshot,
}

/// Why a handle was not bound to its subject.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum HandleBindingFailure {
    /// Current mode: the encrypted value account's current handle is not the one named. The request
    /// is not silently redirected to whatever is live now — the same handle remains reachable as
    /// historical access, because replacing a handle seals a leaf for the then-subjects.
    #[error("encrypted value account's current handle is not the requested one")]
    NotCurrentHandle {
        /// The handle the request named.
        requested: HandleBytes,
        /// The handle the encrypted value account currently holds.
        current: HandleBytes,
    },
    /// Current mode: the subject is not a member of the encrypted value account's current subject
    /// set.
    #[error("subject {subject:?} is not a current member of the encrypted value account")]
    NotAMember {
        /// The subject whose access was being established.
        subject: SolanaPubkeyBytes,
    },
    /// Historical mode: the proof did not verify against the snapshot's live peaks.
    ///
    /// Carries the observed count, which is state, and not the claimed one, which is a request
    /// field: turning this into an action is the caller's job precisely because the caller is
    /// the layer that holds the request.
    #[error("historical access proof does not verify against the observed peaks")]
    ProofDoesNotVerify {
        /// The leaf count observed in the snapshot.
        live_leaf_count: u64,
    },
    /// Historical mode: the proof names a leaf position the encrypted value account does not have.
    #[error("leaf index {leaf_index} is not below the observed leaf count {leaf_count}")]
    LeafIndexOutOfRange {
        /// The position the proof claims.
        leaf_index: u64,
        /// The observed count.
        leaf_count: u64,
    },
    /// The encrypted value account's own MMR state is internally inconsistent, which no retry can
    /// repair.
    #[error("encrypted value account MMR state is internally inconsistent")]
    MmrStateInconsistent,
}
