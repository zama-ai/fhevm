use crate::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    public_decrypt::PublicDecryptFailure,
};
use crate::monitoring::metrics::REQUEST_CHECK_ERRORS;
use anyhow::anyhow;
use kms_connector_api::ErrorCode;
use thiserror::Error;
use tonic::Code;
use user_decryption_signature::Erc1271Error;

#[derive(Debug)]
pub struct ProcessingError {
    pub kind: ProcessingErrorKind,
    /// Caller-facing error code, stored in the error response row for HTTP-sourced decryption.
    /// Unused for non-decryption events and onchain-sourced decryption.
    pub code: ErrorCode,
    pub source: anyhow::Error,
}

/// Recoverability classification of a [`ProcessingError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingErrorKind {
    Recoverable,
    Irrecoverable,
    Aborted,
}

impl ProcessingError {
    pub fn recoverable(code: ErrorCode, source: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: ProcessingErrorKind::Recoverable,
            code,
            source: source.into(),
        }
    }

    pub fn irrecoverable(code: ErrorCode, source: impl Into<anyhow::Error>) -> Self {
        Self {
            kind: ProcessingErrorKind::Irrecoverable,
            code,
            source: source.into(),
        }
    }

    /// The KMS Core aborted the operation.
    pub fn aborted() -> Self {
        Self {
            kind: ProcessingErrorKind::Aborted,
            code: ErrorCode::Unprocessable,
            source: anyhow!("the KMS Core aborted the operation"),
        }
    }

    /// Generic transient infra failure (DB / RPC / transport).
    pub fn transient(source: impl Into<anyhow::Error>) -> Self {
        Self::recoverable(ErrorCode::UpstreamTransient, source)
    }

    /// Converts the GRPC status of a KMS Core send/poll into a `ProcessingError`.
    pub fn from_grpc_status(status: tonic::Status) -> Self {
        match status.code() {
            Code::Aborted => Self::aborted(),
            Code::DeadlineExceeded | Code::Unavailable | Code::ResourceExhausted => {
                Self::recoverable(
                    ErrorCode::UpstreamTransient,
                    anyhow!("KMS GRPC error: {status}"),
                )
            }
            _ => Self::irrecoverable(
                ErrorCode::Unprocessable,
                anyhow!("KMS GRPC error: {status}"),
            ),
        }
    }

    /// Wraps the inner error with additional context.
    pub fn context(mut self, ctx: String) -> Self {
        self.source = self.source.context(ctx);
        self
    }
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.kind {
            ProcessingErrorKind::Irrecoverable => "Processing failed with irrecoverable error",
            ProcessingErrorKind::Recoverable => "Processing failed",
            ProcessingErrorKind::Aborted => "Processing aborted",
        };
        write!(f, "{prefix}: {:#}", self.source)
    }
}

impl std::error::Error for ProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

// ERC-1271 (RFC-012) signature errors map onto `ProcessingError`. Missing code at an EOA is
// terminal, but smart-account validation can depend on mutable wallet state, so negative ERC-1271
// results (and transport blips) stay recoverable and are retried through the existing attempt and
// validity-window limits.
impl From<Erc1271Error> for ProcessingError {
    fn from(err: Erc1271Error) -> Self {
        match err {
            Erc1271Error::EoaMismatchNoCode(_) | Erc1271Error::EmptySigOnEoa(_) => {
                Self::irrecoverable(ErrorCode::UserSignatureRejected, err)
            }
            Erc1271Error::Transport(_) => Self::transient(err),
            Erc1271Error::WrongMagic(..)
            | Erc1271Error::Rejected(..)
            | Erc1271Error::EmptyRevert(_) => {
                Self::recoverable(ErrorCode::UserSignatureRejected, err)
            }
        }
    }
}

/// The family of request check that rejected a request.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestCheckKind {
    /// ACL authorization checks and related errors (malformed handles, missing config...).
    Acl,
    /// RFC-012/016 signature & request-validity checks (EIP-712/ERC-1271 signature, validity
    /// window, signature invalidation).
    Signature,
    /// RFC-023 off-chain ciphertext-attestation consensus check.
    CoproConsensus,
    /// KMS context/epoch validity check.
    KmsContext,
    /// Network error (on-chain call or DB query) encountered while running any check.
    Network,
}

impl RequestCheckKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Acl => "acl",
            Self::Signature => "signature",
            Self::CoproConsensus => "copro_consensus",
            Self::KmsContext => "kms_context",
            Self::Network => "network",
        }
    }

    /// Increments [`REQUEST_CHECK_ERRORS`] for this check family.
    pub fn inc_metric(self) {
        REQUEST_CHECK_ERRORS
            .with_label_values(&[self.as_str()])
            .inc();
    }
}

/// Error returned by the request pre-flight checks (ACL, KMS context, ...).
///
/// It is just a [`ProcessingError`] tagged with the check family that produced it. The metric
/// increment is centralized in [`RequestCheckError::record`], called at each conversion boundary.
#[derive(Debug, Error)]
#[error("{source}")]
pub struct RequestCheckError {
    kind: RequestCheckKind,
    #[source]
    source: ProcessingError,
}

impl RequestCheckError {
    pub fn new(kind: RequestCheckKind, source: ProcessingError) -> Self {
        Self { kind, source }
    }

    pub fn recoverable(
        kind: RequestCheckKind,
        code: ErrorCode,
        source: impl Into<anyhow::Error>,
    ) -> Self {
        Self::new(kind, ProcessingError::recoverable(code, source))
    }

    pub fn irrecoverable(
        kind: RequestCheckKind,
        code: ErrorCode,
        source: impl Into<anyhow::Error>,
    ) -> Self {
        Self::new(kind, ProcessingError::irrecoverable(code, source))
    }

    pub fn network(err: impl Into<anyhow::Error>) -> Self {
        Self::new(RequestCheckKind::Network, ProcessingError::transient(err))
    }

    /// Wraps the inner error with additional context.
    pub fn context(mut self, ctx: String) -> Self {
        self.source = self.source.context(ctx);
        self
    }

    /// Records the error in [`REQUEST_CHECK_ERRORS`] and unwraps it into a [`ProcessingError`].
    pub fn record(self) -> ProcessingError {
        self.kind.inc_metric();
        self.source
    }
}

impl From<Erc1271Error> for RequestCheckError {
    fn from(err: Erc1271Error) -> Self {
        let kind = match &err {
            Erc1271Error::Transport(_) => RequestCheckKind::Network,
            Erc1271Error::EmptyRevert(_)
            | Erc1271Error::EmptySigOnEoa(_)
            | Erc1271Error::EoaMismatchNoCode(_)
            | Erc1271Error::Rejected(..)
            | Erc1271Error::WrongMagic(..) => RequestCheckKind::Signature,
        };
        Self::new(kind, err.into())
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
    use crate::core::solana::{
        delegation::{DeadRow, DelegationFailure},
        encrypted_store::EncryptedStoreFailure,
        failure::InvalidHostRecord,
        handle_binding::HandleBindingFailure,
        proof::ProofReadError,
        snapshot::SnapshotError,
        watermark::{WatermarkFailure, WindowFailure},
    };
    use solana_pubkey::Pubkey;
    use zama_solana_permit::PermitError;

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

    fn assert_recorded(
        error: RequestCheckError,
        (check, code): (RequestCheckKind, ErrorCode),
        kind: ProcessingErrorKind,
        case: &str,
    ) {
        let recorded_check = error.kind;
        let error = error.record();
        assert_eq!(
            (recorded_check, error.code, error.kind),
            (check, code, kind),
            "{case}"
        );
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
        for (failure, family_and_code, kind) in cases {
            let case = failure.to_string();
            assert_eq!(failure.is_recoverable(), kind == RETRY, "{case}");
            assert_recorded(failure.into(), family_and_code, kind, &case);
        }
    }

    #[test]
    fn every_solana_public_decrypt_failure_is_recorded_as_written() {
        let cases = [
            (
                PublicDecryptFailure::MalformedExtraData,
                UNPROCESSABLE,
                TERMINAL,
            ),
            (
                PublicDecryptFailure::Snapshot(SnapshotError::NodeBehind),
                NETWORK,
                RETRY,
            ),
            (
                PublicDecryptFailure::EncryptedStore(EncryptedStoreFailure::Absent {
                    account_key: Pubkey::new_from_array(KEY),
                }),
                DENIED,
                RETRY,
            ),
            (
                PublicDecryptFailure::EncryptedStore(EncryptedStoreFailure::InvalidHostRecord(
                    INVALID,
                )),
                UNPROCESSABLE,
                TERMINAL,
            ),
            (
                PublicDecryptFailure::ProofRead(unavailable()),
                NETWORK,
                RETRY,
            ),
            (
                PublicDecryptFailure::HandleBinding(HandleBindingFailure::NoLeaf {
                    record_leaf_count: 1,
                    live_leaf_count: 1,
                }),
                DENIED,
                RETRY,
            ),
            (
                PublicDecryptFailure::HandleBinding(HandleBindingFailure::HistoryIncomplete),
                UNPROCESSABLE,
                TERMINAL,
            ),
        ];
        for (failure, family_and_code, kind) in cases {
            let case = failure.to_string();
            assert_eq!(failure.is_recoverable(), kind == RETRY, "{case}");
            assert_recorded(failure.into(), family_and_code, kind, &case);
        }
    }
}
