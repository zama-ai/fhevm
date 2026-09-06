//! The failure taxonomy and its classification.
//!
//! Three actions exist for a rejected request, and the classification exists to pick one:
//! give up (terminal), try the same request again later (transient), or try again within the
//! ordinary attempt budget because the disagreement is expected to resolve itself
//! (retryable). Everything else about a failure is diagnostics.
//!
//! Every match here enumerates its variants. A new rule that forgets to say which action it
//! implies breaks the build, instead of inheriting whatever a catch-all arm happened to say —
//! and the two directions of that mistake are both expensive: clients that retry the
//! terminal forever, or bury the retryable.

use super::delegation::DelegationFailure;
use super::deployment::{DeploymentFailure, DeploymentIdentityError};
use super::encrypted_value_account::EncryptedValueAccountFailure;
use super::handle_binding::HandleBindingFailure;
use super::kms_pair::KmsPairFailure;
use super::pause::PauseFailure;
use super::proof::ProofReadError;
use super::request::RequestFormError;
use super::scope::ScopeFailure;
use super::snapshot::SnapshotError;
use super::watermark::{WatermarkFailure, WindowFailure};

/// What a client should do about a rejection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FailureClass {
    /// Nothing about this request will ever be authorized. Rebuilding the request may help;
    /// repeating it will not.
    Terminal,
    /// The request may be authorized from a later observation point, unchanged.
    Transient,
    /// A disagreement between observers that is expected to converge; retried within the
    /// ordinary attempt budget.
    Retryable,
}

/// Why one request was not authorized.
///
/// The variants follow the pipeline: form, signature, deployment, window, KMS pair, then the
/// state-dependent rules. Each carries the entry index where the rule is per handle,
/// because "some handle failed scope" is not an actionable diagnostic for a batch.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    /// The typed form of the request is wrong.
    #[error("request form: {0}")]
    Form(#[from] RequestFormError),
    /// The signature does not verify over the locally reconstructed envelope.
    #[error("permit signature does not verify over the reconstructed envelope")]
    SignatureMismatch,
    /// The permit's user pubkey is not a usable verifying key.
    #[error("permit names a user pubkey that is not a usable Ed25519 key")]
    UnusableUserPubkey,
    /// The permit was signed for another deployment.
    #[error("deployment: {0}")]
    Deployment(#[from] DeploymentFailure),
    /// The validity window rejects the permit.
    #[error("validity window: {0}")]
    Window(#[from] WindowFailure),
    /// The invalidation watermark rejects the permit, or could not be read.
    #[error("invalidation: {0}")]
    Watermark(#[from] WatermarkFailure),
    /// The signed KMS pair is not servable.
    #[error("KMS routing: {0}")]
    KmsPair(#[from] KmsPairFailure),
    /// Host state could not be observed as one point.
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    /// The host is paused, or its config singleton could not be read.
    #[error("host pause: {0}")]
    Pause(#[from] PauseFailure),
    /// One entry's encrypted value account could not be resolved.
    #[error("entry {index}: encrypted value account: {source}")]
    EncryptedValueAccount {
        /// Which entry.
        index: usize,
        /// Why.
        source: EncryptedValueAccountFailure,
    },
    /// The leaf record could not be read at all.
    #[error("leaf proofs: {0}")]
    ProofRead(#[from] ProofReadError),
    /// One entry's handle is not bound to its key.
    #[error("entry {index}: handle binding: {source}")]
    HandleBinding {
        /// Which entry.
        index: usize,
        /// Why.
        source: HandleBindingFailure,
    },
    /// One entry's encrypted value account is outside the signed scope.
    #[error("entry {index}: scope: {source}")]
    Scope {
        /// Which entry.
        index: usize,
        /// Why.
        source: ScopeFailure,
    },
    /// One delegated entry has no live delegation.
    #[error("entry {index}: delegation: {source}")]
    Delegation {
        /// Which entry.
        index: usize,
        /// Why.
        source: DelegationFailure,
    },
}

impl AuthorizationFailure {
    /// Which of the three actions this failure implies.
    ///
    /// Every arm delegates to the taxonomy that produced the failure, so each rule states the
    /// action its own outcomes imply once, next to the outcomes themselves.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Form(source) => source.class(),
            Self::SignatureMismatch | Self::UnusableUserPubkey => FailureClass::Terminal,
            Self::Deployment(source) => source.class(),
            Self::Window(source) => source.class(),
            Self::Watermark(source) => source.class(),
            Self::KmsPair(source) => source.class(),
            Self::Snapshot(source) => source.class(),
            Self::Pause(source) => source.class(),
            Self::EncryptedValueAccount { source, .. } => source.class(),
            Self::ProofRead(source) => source.class(),
            Self::HandleBinding { source, .. } => source.class(),
            Self::Scope { source, .. } => source.class(),
            Self::Delegation { source, .. } => source.class(),
        }
    }
}

impl RequestFormError {
    /// A request whose form is wrong is wrong forever: no observation changes its bytes. The
    /// client's move is to build a different request, which is what terminal means.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Permit(_)
            | Self::SignatureWidth { .. }
            | Self::EntryIdentityWidth { .. }
            | Self::EmptyHandles
            | Self::TooManyHandles { .. } => FailureClass::Terminal,
        }
    }
}

impl DeploymentFailure {
    /// A permit signed for another deployment is not a permit for this one, and no amount of
    /// waiting changes which deployment it names.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ProgramIdMismatch { .. }
            | Self::ChainIdMismatch { .. }
            | Self::MixedEmbeddedChainIds { .. }
            | Self::EmbeddedChainIdMismatch { .. } => FailureClass::Terminal,
        }
    }
}

impl WindowFailure {
    /// The two halves of the window differ in kind, and it is the one classification in this
    /// file that a reader is likely to get backwards: time opens a window that has not opened,
    /// and never reopens one that has closed.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::NotYetValid { .. } => FailureClass::Transient,
            Self::Expired { .. } => FailureClass::Terminal,
        }
    }
}

impl WatermarkFailure {
    /// A revocation is permanent for the permits below it, and an account of the wrong shape at
    /// the invalidation address is host state no retry repairs.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Invalidated { .. }
            | Self::NotAnInvalidationRecord { .. }
            | Self::RecordNamesAnotherUser { .. }
            | Self::ForeignOwner { .. } => FailureClass::Terminal,
            Self::Snapshot(source) => source.class(),
        }
    }
}

impl KmsPairFailure {
    /// Two outcomes, because the inherited validation can tell exactly two apart. The
    /// uncomfortable half is deliberate: a destroyed *epoch* is indistinguishable from one that
    /// is merely not active yet, so it is retried within the attempt budget instead of failing
    /// fast. Only a destroyed *context* has a signal of its own.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ContextDestroyed => FailureClass::Terminal,
            Self::ContextUnknown | Self::PairNotServable | Self::Unavailable { .. } => {
                FailureClass::Transient
            }
        }
    }
}

impl SnapshotError {
    /// A read that produced nothing says nothing about the request, so it is transient. The
    /// exception is the unplanned key: that is a defect in this Connector's own key planning,
    /// and answering "try again" would hide it behind ordinary commitment lag forever.
    pub fn class(&self) -> FailureClass {
        match self {
            // A read that went backwards is transient for the same reason: the request is fine and
            // a retry that lands on a node which has caught up authorizes it. Rejecting it as
            // terminal would let one lagging endpoint decide the request.
            Self::Unavailable { .. }
            | Self::ResponseLengthMismatch { .. }
            | Self::DecidingReadOlderThanDiscovery { .. } => FailureClass::Transient,
            Self::KeyNotInSnapshot { .. } => FailureClass::Terminal,
        }
    }
}

impl PauseFailure {
    /// A pause is lifted by the operator, and the permit, the delegations and the handles all
    /// survive it untouched — the same request authorizes from a later observation, which is what
    /// transient means. An account of the wrong shape or the wrong owner at the singleton's
    /// address is host state no retry repairs.
    ///
    /// `Absent` is transient too, and that is a choice rather than an oversight: the singleton is
    /// written once at deployment and never closed, so its absence usually means this Connector
    /// is pointed at a program that has no host state — a misconfiguration that will retry
    /// forever. Terminal would surface that faster and is the wrong trade anyway, because the
    /// other way absence arises is a reader that has fallen behind, and killing valid requests
    /// over one lagging endpoint is the failure this file refuses everywhere else
    /// ([`DelegationFailure::Absent`], [`SnapshotError::DecidingReadOlderThanDiscovery`]). The
    /// misconfiguration is diagnosable without spending a request: every request fails on this
    /// one rule, which no ordinary lag produces.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Paused | Self::Absent { .. } => FailureClass::Transient,
            Self::ForeignOwner { .. } | Self::NotAHostConfig { .. } => FailureClass::Terminal,
            Self::Snapshot(source) => source.class(),
        }
    }
}

impl EncryptedValueAccountFailure {
    /// Absence is the one outcome a later observation can change: the account may not have
    /// reached the observed commitment yet. Everything else is a statement about an account that
    /// exists and is not the encrypted value account it was claimed to be.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Absent { .. } => FailureClass::Transient,
            Self::ForeignOwner { .. }
            | Self::WrongAccountType { .. }
            | Self::Malformed { .. }
            | Self::AddressMismatch { .. }
            | Self::SentinelAuthority { .. } => FailureClass::Terminal,
            Self::Snapshot(source) => source.class(),
        }
    }
}

impl ProofReadError {
    /// A read that produced nothing says nothing about any leaf, so it is transient. The exception
    /// is the oversized batch: that is a defect in this Connector's own planning, and answering
    /// "try again" would hide it forever.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Unavailable { .. } | Self::ResponseLengthMismatch { .. } => {
                FailureClass::Transient
            }
            Self::TooManyQueries { .. } => FailureClass::Terminal,
        }
    }
}

impl HandleBindingFailure {
    /// Terminal are the outcomes that describe a permission that was never granted or a state
    /// that will not repair itself: a record with at least the chain's history and no leaf in it,
    /// a record whose history for the account is broken, and an inconsistent on-chain MMR.
    ///
    /// Retryable are the disagreements between the record and this observation that are expected
    /// to converge: a record behind the chain, a record that has not yet seen an account the chain
    /// has, a proof from a record ahead of this observation, and a proof that does not verify —
    /// the record and the reader can sit on different confirmed views for a moment, and the
    /// ordinary attempt budget decides.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ProofRecordBehind { .. }
            | Self::AccountUnknownToProofRecord
            | Self::LeafIndexOutOfRange { .. }
            | Self::ProofDoesNotVerify { .. } => FailureClass::Retryable,
            Self::NoLeaf { .. } | Self::HistoryIncomplete | Self::MmrStateInconsistent => {
                FailureClass::Terminal
            }
        }
    }
}

impl ScopeFailure {
    /// The signed scope is signed: an application outside it stays outside it.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ScopeNotAllowed { .. } => FailureClass::Terminal,
        }
    }
}

impl DelegationFailure {
    /// Absence is an outcome a later observation can change, exactly as it is for an encrypted
    /// value account: this connector and the relayer read through their own RPCs and can sit at
    /// different confirmed slots, so a grant that is confirmed elsewhere can be missing here for
    /// as long as this reader lags. Judging that terminally fails a delegated request permanently
    /// over ordinary replica lag, so it is transient and the ordinary attempt budget decides. The
    /// cost of the other reading — a grant that was never made spending its attempts — is the
    /// price of not killing a valid request.
    ///
    /// `NewerThanObservation` is transient for a stronger reason: it cannot be produced by a
    /// coherent node at all. `last_update_slot` is written on-chain from `Clock::get().slot`, and
    /// `observed_slot` is the response's own `context.slot`, so a bank at slot S cannot hold a
    /// write from a later slot. A record that claims one says the observation is incoherent —
    /// account data and context slot taken from different banks behind a proxy, or a broken node —
    /// not that the record is dead: every other check on it has already passed. A repeat against
    /// a coherent node authorizes, so terminal would let one bad response kill a valid request,
    /// which is the same reasoning as [`SnapshotError::DecidingReadOlderThanDiscovery`] above.
    ///
    /// Every other outcome is a statement about a record that was read: a revoked or expired
    /// grant will not come back, and a record of the wrong shape, the wrong owner or the wrong
    /// tuple is not this delegation at all.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Absent { .. } | Self::NewerThanObservation { .. } => FailureClass::Transient,
            Self::ForeignOwner { .. }
            | Self::NotADelegationRecord { .. }
            | Self::TupleMismatch { .. }
            | Self::Revoked
            | Self::Expired { .. } => FailureClass::Terminal,
            // The class of a pair is the more forgiving of its halves: if either row could still
            // authorize a repeat, that is the advice to give. Derived from the halves rather than
            // stated as terminal, so a future row-level outcome that is not terminal cannot be
            // swallowed by the pair that carries it.
            Self::NoLiveGrant { exact, wildcard } => match (exact.class(), wildcard.class()) {
                (FailureClass::Terminal, wildcard) => wildcard,
                (exact, _) => exact,
            },
            Self::Snapshot(source) => source.class(),
        }
    }
}

impl DeploymentIdentityError {
    /// Startup failures are terminal by construction: the process must not run.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ChainKindBitMissing { .. } => FailureClass::Terminal,
        }
    }
}
