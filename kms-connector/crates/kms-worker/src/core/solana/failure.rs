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
use super::handle_binding::{HandleBindingFailure, InclusionAction};
use super::kms_pair::KmsPairFailure;
use super::lineage::LineageFailure;
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
    /// One entry's lineage could not be resolved.
    #[error("entry {index}: lineage: {source}")]
    Lineage {
        /// Which entry.
        index: usize,
        /// Why.
        source: LineageFailure,
    },
    /// One entry's handle is not bound to its subject.
    #[error("entry {index}: handle binding: {source}")]
    HandleBinding {
        /// Which entry.
        index: usize,
        /// Why.
        source: HandleBindingFailure,
    },
    /// One entry's access proof did not verify, classified into an action.
    ///
    /// A variant of its own rather than a field of [`HandleBindingFailure`]: the classification
    /// needs the claimed leaf count, which is a request field, and the binding rule is
    /// deliberately not given request fields. This is the layer where both are in scope.
    #[error(
        "entry {index}: access proof does not verify (claimed count {proof_leaf_count}, observed \
         {live_leaf_count}, action {action:?})"
    )]
    InclusionFailed {
        /// Which entry.
        index: usize,
        /// What the client should do about it.
        action: InclusionAction,
        /// The count the request claimed the proof was built against.
        proof_leaf_count: u64,
        /// The count observed in the snapshot.
        live_leaf_count: u64,
    },
    /// One entry's lineage domain is outside the signed scope.
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
            Self::Lineage { source, .. } => source.class(),
            Self::HandleBinding { source, .. } => source.class(),
            Self::InclusionFailed { action, .. } => action.class(),
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
            | Self::AccessProofMalformed { .. }
            | Self::AccessProofTrailingBytes { .. }
            | Self::AccessProofTooManySiblings { .. }
            | Self::EmptyHandles => FailureClass::Terminal,
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

impl LineageFailure {
    /// Absence is the one outcome a later observation can change: the account may not have
    /// reached the observed commitment yet. Everything else is a statement about an account that
    /// exists and is not the lineage it was claimed to be.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Absent { .. } => FailureClass::Transient,
            Self::ForeignOwner { .. }
            | Self::WrongAccountType { .. }
            | Self::Malformed { .. }
            | Self::ValueKeyMismatch { .. } => FailureClass::Terminal,
            Self::Snapshot(source) => source.class(),
        }
    }
}

impl HandleBindingFailure {
    /// The two proof outcomes are retryable because both describe an observation that has not
    /// caught up with the proof: an unverified proof may verify from a later one, and a leaf
    /// position beyond the observed count may exist at a later one. Supersession and
    /// non-membership are decisions about state that will not un-happen, and an inconsistent
    /// MMR is the host program's own defect.
    ///
    /// Both proof outcomes normally reach a client as
    /// [`AuthorizationFailure::InclusionFailed`], which classifies them with the count the
    /// request claimed; these arms are what a direct caller of the rule gets.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ProofDoesNotVerify { .. } | Self::LeafIndexOutOfRange { .. } => {
                FailureClass::Retryable
            }
            Self::Superseded { .. } | Self::NotAMember { .. } | Self::MmrStateInconsistent => {
                FailureClass::Terminal
            }
        }
    }
}

impl ScopeFailure {
    /// The signed scope is signed: a domain outside it stays outside it.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::DomainNotAllowed { .. } => FailureClass::Terminal,
        }
    }
}

impl DelegationFailure {
    /// Delegation outcomes are terminal for the request that hit them, including the absence of a
    /// grant: what repairs that is the delegator granting one, not this request being repeated,
    /// and a request retried through its whole budget against a grant that was never made costs
    /// the attempt budget for nothing. A revoked or expired grant will not come back, and a
    /// record of the wrong shape or the wrong tuple is not this delegation at all.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Absent { .. }
            | Self::ForeignOwner { .. }
            | Self::NotADelegationRecord { .. }
            | Self::TupleMismatch { .. }
            | Self::Revoked
            | Self::Expired { .. }
            | Self::NewerThanObservation { .. } => FailureClass::Terminal,
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

impl InclusionAction {
    /// The class this action corresponds to. Stated once, so the two mappings cannot drift.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::RebuildProof => FailureClass::Terminal,
            Self::RetryAtLaterSnapshot => FailureClass::Retryable,
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
