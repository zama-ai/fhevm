//! Why a Solana request was not authorized, and whether a later attempt may succeed.

use super::SolanaPubkeyBytes;
use super::delegation::DelegationFailure;
use super::encrypted_store::EncryptedStoreFailure;
use super::handle_binding::HandleBindingFailure;
use super::proof::ProofReadError;
use super::scope::ScopeFailure;
use super::snapshot::SnapshotError;
use super::watermark::{WatermarkFailure, WindowFailure};
use zama_solana_permit::PermitError;

/// Per-entry rules carry the entry index: "some handle failed" is not actionable for a batch.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    #[error("signature: {0}")]
    Signature(PermitError),
    #[error("validity window: {0}")]
    Window(#[from] WindowFailure),
    #[error("permit names program {signed:?}, this host is {own:?}")]
    ProgramIdMismatch {
        signed: SolanaPubkeyBytes,
        own: SolanaPubkeyBytes,
    },
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    #[error("invalidation: {0}")]
    Watermark(#[from] WatermarkFailure),
    #[error("entry {index}: encrypted store: {source}")]
    EncryptedStore {
        index: usize,
        source: EncryptedStoreFailure,
    },
    #[error("entry {index}: scope: {source}")]
    Scope { index: usize, source: ScopeFailure },
    #[error("leaf proofs: {0}")]
    ProofRead(#[from] ProofReadError),
    #[error("entry {index}: handle binding: {source}")]
    HandleBinding {
        index: usize,
        source: HandleBindingFailure,
    },
    #[error("entry {index}: delegation: {source}")]
    Delegation {
        index: usize,
        source: DelegationFailure,
    },
}

impl AuthorizationFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Signature(_) | Self::ProgramIdMismatch { .. } => false,
            Self::Window(source) => source.is_recoverable(),
            Self::Snapshot(_) => true,
            Self::Watermark(source) => source.is_recoverable(),
            Self::EncryptedStore { source, .. } => source.is_recoverable(),
            Self::Scope { .. } => false,
            Self::ProofRead(_) => true,
            Self::HandleBinding { source, .. } => source.is_recoverable(),
            Self::Delegation { source, .. } => source.is_recoverable(),
        }
    }
}

impl WindowFailure {
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::NotYetValid { .. })
    }
}

impl WatermarkFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::UnreadAccount(_) => false,
            Self::Invalidated { .. }
            | Self::NotAnInvalidationRecord { .. }
            | Self::RecordNamesAnotherUser { .. }
            | Self::ForeignOwner { .. } => false,
        }
    }
}

impl EncryptedStoreFailure {
    /// An absent store may not have reached the observed commitment yet.
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Absent { .. } => true,
            Self::ForeignOwner { .. }
            | Self::WrongAccountType { .. }
            | Self::Malformed { .. }
            | Self::AddressMismatch { .. }
            | Self::SentinelProgram { .. } => false,
            Self::UnreadAccount(_) => false,
        }
    }
}

impl HandleBindingFailure {
    /// A proof record behind the chain may catch up, and so may the node this connector reads: a
    /// missing leaf is recoverable, as an EVM ACL denial is.
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::NoLeaf { .. }
            | Self::ProofRecordBehind { .. }
            | Self::AccountUnknownToProofRecord
            | Self::LeafIndexOutOfRange { .. }
            | Self::ProofDoesNotVerify { .. } => true,
            Self::HistoryIncomplete => false,
        }
    }
}

impl DelegationFailure {
    pub fn is_recoverable(&self) -> bool {
        match self {
            // As on EVM, a delegation that is not live is an ACL denial a later attempt may clear.
            Self::Absent { .. } | Self::NotLive { .. } => true,
            Self::ForeignOwner { .. }
            | Self::NotADelegationRecord { .. }
            | Self::TupleMismatch { .. } => false,
            Self::NoLiveDelegation { exact, wildcard } => {
                exact.is_recoverable() || wildcard.is_recoverable()
            }
            Self::UnreadAccount(_) => false,
        }
    }
}
