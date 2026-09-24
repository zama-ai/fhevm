//! Why a Solana request was not authorized, and how the worker records it: one exhaustive
//! [`FailureClass`] per failure.

use super::SolanaPubkeyBytes;
use super::delegation::DelegationFailure;
use super::encrypted_store::EncryptedStoreFailure;
use super::handle_binding::HandleBindingFailure;
use super::proof::ProofReadError;
use super::public_decrypt::PublicDecryptFailure;
use super::scope::ScopeFailure;
use super::snapshot::SnapshotError;
use super::watermark::{WatermarkFailure, WindowFailure};
use crate::core::event_processor::RequestCheckKind;
use kms_connector_api::ErrorCode;
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

/// How the worker records a failure: the check family for the metric, the code a caller sees, and
/// whether a later attempt may succeed. The codes are the ones EVM uses for the same outcomes: a
/// bad, expired or revoked permit is a rejected signature, a host or coprocessor that cannot be
/// read now is transient, and host state that grants no access is an ACL denial. A host record the
/// host program could not have written, or a broken proof record, is not the user's permission
/// state, so it is unprocessable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FailureClass {
    pub check: RequestCheckKind,
    pub code: ErrorCode,
    pub recoverable: bool,
}

const fn class(check: RequestCheckKind, code: ErrorCode, recoverable: bool) -> FailureClass {
    FailureClass {
        check,
        code,
        recoverable,
    }
}

const SIGNATURE_REJECTED: FailureClass = class(
    RequestCheckKind::Signature,
    ErrorCode::UserSignatureRejected,
    false,
);
const UPSTREAM_TRANSIENT: FailureClass = class(
    RequestCheckKind::Network,
    ErrorCode::UpstreamTransient,
    true,
);
/// Access not granted yet: a later attempt may see the grant, as an EVM ACL denial is retried.
const ACL_DENIED: FailureClass = class(RequestCheckKind::Acl, ErrorCode::AclDenied, true);
/// Access that no later attempt can grant: the request itself names the wrong thing.
const ACL_REFUSED: FailureClass = class(RequestCheckKind::Acl, ErrorCode::AclDenied, false);
const UNPROCESSABLE: FailureClass = class(RequestCheckKind::Acl, ErrorCode::Unprocessable, false);

impl AuthorizationFailure {
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Signature(_) | Self::ProgramIdMismatch { .. } => SIGNATURE_REJECTED,
            Self::Window(source) => source.class(),
            Self::Snapshot(source) => source.class(),
            Self::Watermark(source) => source.class(),
            Self::EncryptedStore { source, .. } => source.class(),
            Self::Scope { source, .. } => source.class(),
            Self::ProofRead(source) => source.class(),
            Self::HandleBinding { source, .. } => source.class(),
            Self::Delegation { source, .. } => source.class(),
        }
    }

    pub fn is_recoverable(&self) -> bool {
        self.class().recoverable
    }
}

impl PublicDecryptFailure {
    pub fn class(&self) -> FailureClass {
        match self {
            Self::MalformedExtraData => UNPROCESSABLE,
            Self::Snapshot(source) => source.class(),
            Self::EncryptedStore(source) => source.class(),
            Self::ProofRead(source) => source.class(),
            Self::HandleBinding(source) => source.class(),
        }
    }

    pub fn is_recoverable(&self) -> bool {
        self.class().recoverable
    }
}

impl WindowFailure {
    /// A permit that starts later becomes usable; one that expired never does again.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::NotYetValid { .. } => FailureClass {
                recoverable: true,
                ..SIGNATURE_REJECTED
            },
            Self::Expired { .. } => SIGNATURE_REJECTED,
        }
    }
}

impl SnapshotError {
    /// Every host read failure is the node's.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Unavailable { .. }
            | Self::ResponseLengthMismatch { .. }
            | Self::MalformedClock
            | Self::NodeBehind => UPSTREAM_TRANSIENT,
        }
    }
}

impl ProofReadError {
    /// Every proof read failure is the coprocessors'.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Unavailable { .. } | Self::ResponseLengthMismatch { .. } => UPSTREAM_TRANSIENT,
        }
    }
}

impl InvalidHostRecord {
    pub fn class(&self) -> FailureClass {
        UNPROCESSABLE
    }
}

impl WatermarkFailure {
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Invalidated { .. } => SIGNATURE_REJECTED,
            Self::InvalidHostRecord(source) => source.class(),
        }
    }
}

impl EncryptedStoreFailure {
    /// An absent store may not have reached the observed commitment yet; any other store the
    /// request names is the request's mistake.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::Absent { .. } => ACL_DENIED,
            Self::ForeignOwner { .. }
            | Self::NotAnEncryptedStore { .. }
            | Self::AddressMismatch { .. } => ACL_REFUSED,
            Self::InvalidHostRecord(source) => source.class(),
        }
    }
}

impl ScopeFailure {
    pub fn class(&self) -> FailureClass {
        match self {
            Self::ScopeNotAllowed { .. } => ACL_REFUSED,
        }
    }
}

impl HandleBindingFailure {
    /// A proof record behind the chain may catch up, and so may the node this connector reads: a
    /// missing leaf is recoverable, as an EVM ACL denial is. A record whose history has a gap
    /// cannot serve this store until it is rebuilt.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::NoLeaf { .. }
            | Self::ProofRecordBehind { .. }
            | Self::AccountUnknownToProofRecord
            | Self::LeafIndexOutOfRange { .. }
            | Self::ProofDoesNotVerify { .. } => ACL_DENIED,
            Self::HistoryIncomplete => UNPROCESSABLE,
        }
    }

    pub fn is_recoverable(&self) -> bool {
        self.class().recoverable
    }
}

impl DelegationFailure {
    /// As on EVM, a delegation that is not live is an ACL denial a later attempt may clear.
    pub fn class(&self) -> FailureClass {
        match self {
            Self::NoLiveDelegation { .. } => ACL_DENIED,
            Self::InvalidHostRecord(source) => source.class(),
        }
    }
}
