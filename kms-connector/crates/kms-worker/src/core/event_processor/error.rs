use crate::core::solana::{
    delegation::DelegationFailure, encrypted_store::EncryptedStoreFailure,
    failure::AuthorizationFailure, handle_binding::HandleBindingFailure,
    public_decrypt::PublicDecryptFailure, watermark::WatermarkFailure,
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
#[derive(Clone, Copy, Debug)]
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
    /// The codes EVM uses for the same outcomes: a bad, expired or revoked permit is a rejected
    /// signature, a host that cannot be read now is transient, and host state that grants no
    /// access is an ACL denial. A Connector bug or a broken proof record is not the user's
    /// permission state, so it is unprocessable.
    fn from(failure: AuthorizationFailure) -> Self {
        let (kind, code) = match &failure {
            AuthorizationFailure::Signature(_)
            | AuthorizationFailure::Window(_)
            | AuthorizationFailure::ProgramIdMismatch { .. }
            | AuthorizationFailure::Watermark(WatermarkFailure::Invalidated { .. }) => (
                RequestCheckKind::Signature,
                ErrorCode::UserSignatureRejected,
            ),
            AuthorizationFailure::Snapshot(_) | AuthorizationFailure::ProofRead(_) => {
                (RequestCheckKind::Network, ErrorCode::UpstreamTransient)
            }
            AuthorizationFailure::HandleBinding {
                source: HandleBindingFailure::HistoryIncomplete,
                ..
            }
            | AuthorizationFailure::Watermark(WatermarkFailure::InvalidHostRecord(_))
            | AuthorizationFailure::EncryptedStore {
                source: EncryptedStoreFailure::InvalidHostRecord(_),
                ..
            }
            | AuthorizationFailure::Delegation {
                source: DelegationFailure::InvalidHostRecord(_),
                ..
            } => (RequestCheckKind::Acl, ErrorCode::Unprocessable),
            AuthorizationFailure::EncryptedStore { .. }
            | AuthorizationFailure::Scope { .. }
            | AuthorizationFailure::HandleBinding { .. }
            | AuthorizationFailure::Delegation { .. } => {
                (RequestCheckKind::Acl, ErrorCode::AclDenied)
            }
        };
        solana_check_error(kind, code, failure.is_recoverable(), failure)
    }
}

impl From<PublicDecryptFailure> for RequestCheckError {
    fn from(failure: PublicDecryptFailure) -> Self {
        let (kind, code) = match &failure {
            PublicDecryptFailure::MalformedExtraData => {
                (RequestCheckKind::Acl, ErrorCode::Unprocessable)
            }
            PublicDecryptFailure::Snapshot(_) | PublicDecryptFailure::ProofRead(_) => {
                (RequestCheckKind::Network, ErrorCode::UpstreamTransient)
            }
            PublicDecryptFailure::HandleBinding(HandleBindingFailure::HistoryIncomplete)
            | PublicDecryptFailure::EncryptedStore(EncryptedStoreFailure::InvalidHostRecord(_)) => {
                (RequestCheckKind::Acl, ErrorCode::Unprocessable)
            }
            PublicDecryptFailure::EncryptedStore(_) | PublicDecryptFailure::HandleBinding(_) => {
                (RequestCheckKind::Acl, ErrorCode::AclDenied)
            }
        };
        solana_check_error(kind, code, failure.is_recoverable(), failure)
    }
}

fn solana_check_error(
    kind: RequestCheckKind,
    code: ErrorCode,
    recoverable: bool,
    failure: impl std::error::Error + Send + Sync + 'static,
) -> RequestCheckError {
    if recoverable {
        RequestCheckError::recoverable(kind, code, failure)
    } else {
        RequestCheckError::irrecoverable(kind, code, failure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::solana::proof::ProofReadError;
    use crate::core::solana::snapshot::SnapshotError;
    use rstest::rstest;

    fn coprocessors_down() -> ProofReadError {
        ProofReadError::Unavailable {
            reason: "down".into(),
        }
    }

    /// Each Solana outcome takes the code EVM uses for it. A Connector bug or a broken proof
    /// record is not the user's permission state: `acl_denied` would tell an HTTP caller to retry
    /// a request that fails the same way every time.
    #[rstest]
    #[case::program_id_mismatch(
        AuthorizationFailure::ProgramIdMismatch { signed: [1; 32], own: [2; 32] },
        ErrorCode::UserSignatureRejected,
        ProcessingErrorKind::Irrecoverable
    )]
    #[case::incomplete_history(
        AuthorizationFailure::HandleBinding { index: 0, source: HandleBindingFailure::HistoryIncomplete },
        ErrorCode::Unprocessable,
        ProcessingErrorKind::Irrecoverable
    )]
    #[case::unreadable_host(
        AuthorizationFailure::Snapshot(SnapshotError::Unavailable { reason: "down".into() }),
        ErrorCode::UpstreamTransient,
        ProcessingErrorKind::Recoverable
    )]
    #[case::unreachable_coprocessors(
        AuthorizationFailure::ProofRead(coprocessors_down()),
        ErrorCode::UpstreamTransient,
        ProcessingErrorKind::Recoverable
    )]
    #[case::missing_leaf(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::NoLeaf { record_leaf_count: 1, live_leaf_count: 1 },
        },
        ErrorCode::AclDenied,
        ProcessingErrorKind::Recoverable
    )]
    fn solana_failures_take_the_evm_code_for_their_outcome(
        #[case] failure: AuthorizationFailure,
        #[case] code: ErrorCode,
        #[case] kind: ProcessingErrorKind,
    ) {
        let error = RequestCheckError::from(failure).record();
        assert_eq!((error.code, error.kind), (code, kind));
    }

    #[rstest]
    #[case::incomplete_history(
        PublicDecryptFailure::HandleBinding(HandleBindingFailure::HistoryIncomplete),
        ErrorCode::Unprocessable,
        ProcessingErrorKind::Irrecoverable
    )]
    #[case::unreachable_coprocessors(
        PublicDecryptFailure::ProofRead(coprocessors_down()),
        ErrorCode::UpstreamTransient,
        ProcessingErrorKind::Recoverable
    )]
    #[case::missing_leaf(
        PublicDecryptFailure::HandleBinding(HandleBindingFailure::NoLeaf {
            record_leaf_count: 1,
            live_leaf_count: 1,
        }),
        ErrorCode::AclDenied,
        ProcessingErrorKind::Recoverable
    )]
    fn solana_public_decrypt_failures_take_the_evm_code_for_their_outcome(
        #[case] failure: PublicDecryptFailure,
        #[case] code: ErrorCode,
        #[case] kind: ProcessingErrorKind,
    ) {
        let error = RequestCheckError::from(failure).record();
        assert_eq!((error.code, error.kind), (code, kind));
    }
}
