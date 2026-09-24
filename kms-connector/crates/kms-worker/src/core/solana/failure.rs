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

/// An account the Connector read at an address only the host program can write, or a store the
/// host program owns, whose content the host program could never have written. The request is
/// judged on nothing else: it fails closed, even when another row would authorize it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, thiserror::Error)]
#[error("account {account_key:?} holds a record the host program could not have written")]
pub struct InvalidHostRecord {
    pub account_key: SolanaPubkeyBytes,
}

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
            Self::Invalidated { .. } | Self::InvalidHostRecord(_) => false,
        }
    }
}

impl EncryptedStoreFailure {
    /// An absent store may not have reached the observed commitment yet.
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Absent { .. } => true,
            Self::ForeignOwner { .. }
            | Self::NotAnEncryptedStore { .. }
            | Self::AddressMismatch { .. }
            | Self::InvalidHostRecord(_) => false,
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
    /// As on EVM, a delegation that is not live is an ACL denial a later attempt may clear.
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::NoLiveDelegation { .. } => true,
            Self::InvalidHostRecord(_) => false,
        }
    }
}
