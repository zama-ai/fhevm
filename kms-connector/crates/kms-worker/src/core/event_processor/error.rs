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
            Erc1271Error::WrongMagic(..) | Erc1271Error::Rejected(..) => {
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

    /// Tags an already-classified [`ProcessingError`] with the check family that produced it,
    /// preserving its recoverable/irrecoverable variant. Used at the request-check boundary for
    /// lower-level checks that return a bare `ProcessingError` (e.g. the Solana ACL verifier).
    /// The natural inverse of [`RequestCheckError::record`].
    pub fn from_processing(kind: RequestCheckKind, source: ProcessingError) -> Self {
        Self { kind, source }
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

    /// Whether this failure is worth retrying. A recoverable/aborted source is transient; an
    /// irrecoverable one is terminal. Read by the Solana pipeline's KMS-pair adapter to map a
    /// context-servability outcome onto the pipeline's terminal/transient taxonomy.
    pub fn is_recoverable(&self) -> bool {
        !matches!(self.source.kind, ProcessingErrorKind::Irrecoverable)
    }
}

impl From<Erc1271Error> for RequestCheckError {
    fn from(err: Erc1271Error) -> Self {
        let kind = match &err {
            Erc1271Error::Transport(_) => RequestCheckKind::Network,
            Erc1271Error::EmptySigOnEoa(_)
            | Erc1271Error::EoaMismatchNoCode(_)
            | Erc1271Error::Rejected(..)
            | Erc1271Error::WrongMagic(..) => RequestCheckKind::Signature,
        };
        Self::new(kind, err.into())
    }
}
