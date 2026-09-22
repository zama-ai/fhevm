//! Authorization failure reasons and whether another observation may succeed.

use super::delegation::DelegationFailure;
use super::deployment::{DeploymentFailure, DeploymentIdentityError};
use super::encrypted_store::EncryptedStoreFailure;
use super::handle_binding::HandleBindingFailure;
use super::pause::PauseFailure;
use super::proof::ProofReadError;
use super::scope::ScopeFailure;
use super::snapshot::SnapshotError;
use super::watermark::{WatermarkFailure, WindowFailure};
use connector_utils::types::solana_request::RequestFormError;

/// Why one request was not authorized.
///
/// The variants follow the pipeline: form, signature, deployment, window, KMS pair, then the
/// state-dependent rules. Each carries the entry index where the rule is per handle,
/// because "some handle failed scope" is not an actionable diagnostic for a batch.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    /// Internal batch planning omitted an entry or its verification result.
    #[error("entry {index}: planned proof binding is missing")]
    MissingProofBinding {
        /// Which entry.
        index: usize,
    },
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
    /// Host state could not be observed as one point.
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    /// The host is paused, or its config singleton could not be read.
    #[error("host pause: {0}")]
    Pause(#[from] PauseFailure),
    /// One entry's encrypted store could not be resolved.
    #[error("entry {index}: encrypted store: {source}")]
    EncryptedStore {
        /// Which entry.
        index: usize,
        /// Why.
        source: EncryptedStoreFailure,
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
    /// One entry's encrypted store is outside the signed scope.
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
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Form(_) => false,
            Self::SignatureMismatch
            | Self::UnusableUserPubkey
            | Self::MissingProofBinding { .. } => false,
            Self::Deployment(source) => source.is_recoverable(),
            Self::Window(source) => source.is_recoverable(),
            Self::Watermark(source) => source.is_recoverable(),
            Self::Snapshot(source) => source.is_recoverable(),
            Self::Pause(source) => source.is_recoverable(),
            Self::EncryptedStore { source, .. } => source.is_recoverable(),
            Self::ProofRead(source) => source.is_recoverable(),
            Self::HandleBinding { source, .. } => source.is_recoverable(),
            Self::Scope { source, .. } => source.is_recoverable(),
            Self::Delegation { source, .. } => source.is_recoverable(),
        }
    }
}

impl DeploymentFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ProgramIdMismatch { .. } | Self::ChainIdMismatch { .. } => false,
        }
    }
}

impl WindowFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::NotYetValid { .. } => true,
            Self::Expired { .. } => false,
        }
    }
}

impl WatermarkFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Invalidated { .. }
            | Self::NotAnInvalidationRecord { .. }
            | Self::RecordNamesAnotherUser { .. }
            | Self::ForeignOwner { .. } => false,
            Self::Snapshot(source) => source.is_recoverable(),
        }
    }
}

impl SnapshotError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Unavailable { .. }
            | Self::ResponseLengthMismatch { .. }
            | Self::DecidingReadOlderThanDiscovery { .. } => true,
            Self::KeyNotInSnapshot { .. } => false,
        }
    }
}

impl PauseFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Paused | Self::Absent { .. } => true,
            Self::ForeignOwner { .. } | Self::NotAHostConfig { .. } => false,
            Self::Snapshot(source) => source.is_recoverable(),
        }
    }
}

impl EncryptedStoreFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Absent { .. } => true,
            Self::ForeignOwner { .. }
            | Self::WrongAccountType { .. }
            | Self::Malformed { .. }
            | Self::AddressMismatch { .. }
            | Self::SentinelAuthority { .. } => false,
            Self::Snapshot(source) => source.is_recoverable(),
        }
    }
}

impl ProofReadError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Unavailable { .. } | Self::ResponseLengthMismatch { .. } => true,
            Self::TooManyQueries { .. } | Self::RequestEncoding { .. } => false,
        }
    }
}

impl HandleBindingFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ProofRecordBehind { .. }
            | Self::AccountUnknownToProofRecord
            | Self::LeafIndexOutOfRange { .. }
            | Self::ProofDoesNotVerify { .. } => true,
            Self::NoLeaf { .. } | Self::HistoryIncomplete | Self::MmrStateInconsistent => false,
        }
    }
}

impl ScopeFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ScopeNotAllowed { .. } => false,
        }
    }
}

impl DelegationFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Absent { .. } | Self::NewerThanObservation { .. } => true,
            Self::ForeignOwner { .. }
            | Self::NotADelegationRecord { .. }
            | Self::TupleMismatch { .. }
            | Self::Revoked
            | Self::Expired { .. } => false,
            Self::NoLiveGrant { exact, wildcard } => {
                exact.is_recoverable() || wildcard.is_recoverable()
            }
            Self::Snapshot(source) => source.is_recoverable(),
        }
    }
}

impl DeploymentIdentityError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::ChainTypeByteInvalid { .. } => false,
        }
    }
}
