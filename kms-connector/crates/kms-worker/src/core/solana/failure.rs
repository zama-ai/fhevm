//! Why a Solana request was not authorized, and how the worker records it: one exhaustive
//! [`FailureClass`] per failure.

use super::delegation::DelegationFailure;
use super::encrypted_store::EncryptedStoreFailure;
use super::handle_binding::HandleBindingFailure;
use super::proof::ProofReadError;
use super::public_decrypt::PublicDecryptFailure;
use super::snapshot::SnapshotError;
use super::watermark::{WatermarkFailure, WindowFailure};
use crate::core::event_processor::{RequestCheckError, RequestCheckKind};
use kms_connector_api::ErrorCode;
use solana_pubkey::Pubkey;
use zama_solana_permit::PermitError;

/// An account the Connector read at an address only the host program can write, or a store the
/// host program owns, whose content the host program could never have written. The request is
/// judged on nothing else: it fails closed, even when another row would authorize it.
#[derive(Clone, Copy, PartialEq, Eq, Debug, thiserror::Error)]
#[error("account {account_key} holds a record the host program could not have written")]
pub struct InvalidHostRecord {
    pub account_key: Pubkey,
}

/// Per-entry rules carry the entry index: "some handle failed" is not actionable for a batch.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    #[error("signature: {0}")]
    Signature(PermitError),
    #[error("validity window: {0}")]
    Window(#[from] WindowFailure),
    #[error("permit names program {signed}, this host is {own}")]
    ProgramIdMismatch { signed: Pubkey, own: Pubkey },
    #[error("host state: {0}")]
    Snapshot(#[from] SnapshotError),
    #[error("invalidation: {0}")]
    Watermark(#[from] WatermarkFailure),
    #[error("entry {index}: encrypted store: {source}")]
    EncryptedStore {
        index: usize,
        source: EncryptedStoreFailure,
    },
    #[error("entry {index}: application ({program}, {scope}) is outside the signed scope")]
    ScopeNotAllowed {
        index: usize,
        program: Pubkey,
        scope: Pubkey,
    },
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
            Self::ScopeNotAllowed { .. } => ACL_REFUSED,
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
            Self::Snapshot(source) => source.class(),
            Self::EncryptedStore { source, .. } => source.class(),
            Self::ProofRead(source) => source.class(),
            Self::HandleBinding { source, .. } => source.class(),
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

impl From<AuthorizationFailure> for RequestCheckError {
    fn from(failure: AuthorizationFailure) -> Self {
        solana_check_error(failure.class(), failure)
    }
}

impl From<PublicDecryptFailure> for RequestCheckError {
    fn from(failure: PublicDecryptFailure) -> Self {
        solana_check_error(failure.class(), failure)
    }
}

fn solana_check_error(
    FailureClass {
        check,
        code,
        recoverable,
    }: FailureClass,
    failure: impl std::error::Error + Send + Sync + 'static,
) -> RequestCheckError {
    if recoverable {
        RequestCheckError::recoverable(check, code, failure)
    } else {
        RequestCheckError::irrecoverable(check, code, failure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;
    use zama_solana_acl::DeadRow;

    const KEY: [u8; 32] = [7; 32];
    const INVALID: InvalidHostRecord = InvalidHostRecord {
        account_key: Pubkey::new_from_array(KEY),
    };

    /// Signature family, `user_signature_rejected`.
    const SIGNATURE: (RequestCheckKind, ErrorCode) = (
        RequestCheckKind::Signature,
        ErrorCode::UserSignatureRejected,
    );
    /// Network family, `upstream_transient`.
    const NETWORK: (RequestCheckKind, ErrorCode) =
        (RequestCheckKind::Network, ErrorCode::UpstreamTransient);
    /// ACL family, `acl_denied`.
    const DENIED: (RequestCheckKind, ErrorCode) = (RequestCheckKind::Acl, ErrorCode::AclDenied);
    /// ACL family, `unprocessable`.
    const UNPROCESSABLE: (RequestCheckKind, ErrorCode) =
        (RequestCheckKind::Acl, ErrorCode::Unprocessable);

    use ProcessingErrorKind::{Irrecoverable as TERMINAL, Recoverable as RETRY};

    fn store(source: EncryptedStoreFailure) -> AuthorizationFailure {
        AuthorizationFailure::EncryptedStore { index: 0, source }
    }

    fn binding(source: HandleBindingFailure) -> AuthorizationFailure {
        AuthorizationFailure::HandleBinding { index: 0, source }
    }

    fn delegation(source: DelegationFailure) -> AuthorizationFailure {
        AuthorizationFailure::Delegation { index: 0, source }
    }

    fn unavailable() -> ProofReadError {
        ProofReadError::Unavailable {
            reason: "down".into(),
        }
    }

    /// Every Solana user-decryption failure, written out with the family, code and recoverability
    /// it must be recorded with. The expectations are this table's, not the mapping's: a changed
    /// class fails here.
    #[test]
    fn every_solana_authorization_failure_is_recorded_as_written() {
        let cases = [
            (
                AuthorizationFailure::Signature(PermitError::SignatureMismatch),
                SIGNATURE,
                TERMINAL,
            ),
            (
                AuthorizationFailure::Window(WindowFailure::NotYetValid {
                    start_timestamp: 2,
                    now: 1,
                }),
                SIGNATURE,
                RETRY,
            ),
            (
                AuthorizationFailure::Window(WindowFailure::Expired { end: 1, now: 2 }),
                SIGNATURE,
                TERMINAL,
            ),
            (
                AuthorizationFailure::ProgramIdMismatch {
                    signed: Pubkey::new_from_array([1; 32]),
                    own: Pubkey::new_from_array([2; 32]),
                },
                SIGNATURE,
                TERMINAL,
            ),
            (
                AuthorizationFailure::Snapshot(SnapshotError::Unavailable {
                    reason: "down".into(),
                }),
                NETWORK,
                RETRY,
            ),
            (
                AuthorizationFailure::Snapshot(SnapshotError::ResponseLengthMismatch {
                    requested: 1,
                    returned: 0,
                }),
                NETWORK,
                RETRY,
            ),
            (
                AuthorizationFailure::Snapshot(SnapshotError::MalformedClock),
                NETWORK,
                RETRY,
            ),
            (
                AuthorizationFailure::Snapshot(SnapshotError::NodeBehind),
                NETWORK,
                RETRY,
            ),
            (
                AuthorizationFailure::Watermark(WatermarkFailure::Invalidated {
                    start_timestamp: 1,
                    watermark: 2,
                }),
                SIGNATURE,
                TERMINAL,
            ),
            (
                AuthorizationFailure::Watermark(WatermarkFailure::InvalidHostRecord(INVALID)),
                UNPROCESSABLE,
                TERMINAL,
            ),
            (
                store(EncryptedStoreFailure::Absent {
                    account_key: Pubkey::new_from_array(KEY),
                }),
                DENIED,
                RETRY,
            ),
            (
                store(EncryptedStoreFailure::ForeignOwner {
                    account_key: Pubkey::new_from_array(KEY),
                    owner: Pubkey::new_from_array([1; 32]),
                }),
                DENIED,
                TERMINAL,
            ),
            (
                store(EncryptedStoreFailure::NotAnEncryptedStore {
                    account_key: Pubkey::new_from_array(KEY),
                }),
                DENIED,
                TERMINAL,
            ),
            (
                store(EncryptedStoreFailure::AddressMismatch {
                    account_key: Pubkey::new_from_array(KEY),
                    derived: None,
                }),
                DENIED,
                TERMINAL,
            ),
            (
                store(EncryptedStoreFailure::InvalidHostRecord(INVALID)),
                UNPROCESSABLE,
                TERMINAL,
            ),
            (
                AuthorizationFailure::ScopeNotAllowed {
                    index: 0,
                    program: Pubkey::new_from_array([1; 32]),
                    scope: Pubkey::new_from_array([2; 32]),
                },
                DENIED,
                TERMINAL,
            ),
            (
                AuthorizationFailure::ProofRead(unavailable()),
                NETWORK,
                RETRY,
            ),
            (
                AuthorizationFailure::ProofRead(ProofReadError::ResponseLengthMismatch {
                    requested: 1,
                    returned: 0,
                }),
                NETWORK,
                RETRY,
            ),
            (
                binding(HandleBindingFailure::NoLeaf {
                    record_leaf_count: 1,
                    live_leaf_count: 1,
                }),
                DENIED,
                RETRY,
            ),
            (
                binding(HandleBindingFailure::ProofRecordBehind {
                    record_leaf_count: 0,
                    live_leaf_count: 1,
                }),
                DENIED,
                RETRY,
            ),
            (
                binding(HandleBindingFailure::AccountUnknownToProofRecord),
                DENIED,
                RETRY,
            ),
            (
                binding(HandleBindingFailure::LeafIndexOutOfRange {
                    leaf_index: 1,
                    leaf_count: 1,
                }),
                DENIED,
                RETRY,
            ),
            (
                binding(HandleBindingFailure::ProofDoesNotVerify {
                    record_leaf_count: 1,
                    live_leaf_count: 2,
                }),
                DENIED,
                RETRY,
            ),
            (
                binding(HandleBindingFailure::HistoryIncomplete),
                UNPROCESSABLE,
                TERMINAL,
            ),
            (
                delegation(DelegationFailure::NoLiveDelegation {
                    exact: DeadRow::NotLive { expires_at: 0 },
                    wildcard: DeadRow::Absent,
                    now: 1,
                }),
                DENIED,
                RETRY,
            ),
            (
                delegation(DelegationFailure::InvalidHostRecord(INVALID)),
                UNPROCESSABLE,
                TERMINAL,
            ),
        ];
        for (failure, (check, code), kind) in cases {
            let case = failure.to_string();
            let recoverable = kind == RETRY;
            assert_eq!(
                failure.class(),
                FailureClass {
                    check,
                    code,
                    recoverable
                },
                "{case}"
            );
            let recorded = RequestCheckError::from(failure).record();
            assert_eq!((recorded.code, recorded.kind), (code, kind), "{case}");
        }
    }

    #[test]
    fn every_solana_public_decrypt_failure_is_recorded_as_written() {
        let cases = [
            (
                PublicDecryptFailure::Snapshot(SnapshotError::NodeBehind),
                NETWORK,
                RETRY,
            ),
            (
                PublicDecryptFailure::EncryptedStore {
                    index: 0,
                    source: EncryptedStoreFailure::Absent {
                        account_key: Pubkey::new_from_array(KEY),
                    },
                },
                DENIED,
                RETRY,
            ),
            (
                PublicDecryptFailure::EncryptedStore {
                    index: 0,
                    source: EncryptedStoreFailure::InvalidHostRecord(INVALID),
                },
                UNPROCESSABLE,
                TERMINAL,
            ),
            (
                PublicDecryptFailure::ProofRead(unavailable()),
                NETWORK,
                RETRY,
            ),
            (
                PublicDecryptFailure::HandleBinding {
                    index: 0,
                    source: HandleBindingFailure::NoLeaf {
                        record_leaf_count: 1,
                        live_leaf_count: 1,
                    },
                },
                DENIED,
                RETRY,
            ),
            (
                PublicDecryptFailure::HandleBinding {
                    index: 0,
                    source: HandleBindingFailure::HistoryIncomplete,
                },
                UNPROCESSABLE,
                TERMINAL,
            ),
        ];
        for (failure, (check, code), kind) in cases {
            let case = failure.to_string();
            let recoverable = kind == RETRY;
            assert_eq!(
                failure.class(),
                FailureClass {
                    check,
                    code,
                    recoverable
                },
                "{case}"
            );
            let recorded = RequestCheckError::from(failure).record();
            assert_eq!((recorded.code, recorded.kind), (code, kind), "{case}");
        }
    }
}
