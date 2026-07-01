use crate::monitoring::metrics::REQUEST_CHECK_ERRORS;
use anyhow::anyhow;
use thiserror::Error;
use tonic::Code;
use user_decryption_signature::Erc1271Error;

#[derive(Debug, Error)]
pub enum ProcessingError {
    #[error("Processing failed with irrecoverable error : {0}")]
    Irrecoverable(anyhow::Error),
    #[error("Processing failed: {0}")]
    Recoverable(anyhow::Error),
    /// Terminal, expected outcome: the KMS Core reported the an operation as aborted.
    #[error("Processing stopped: the operation was aborted on the KMS Core")]
    Aborted,
}

impl ProcessingError {
    /// Converts GRPC status of the polling of a KMS Response into a `ProcessingError`.
    pub fn from_response_status(value: tonic::Status) -> Self {
        let anyhow_error = anyhow!("KMS GRPC error: {value}");
        match value.code() {
            Code::DeadlineExceeded | Code::Unavailable | Code::ResourceExhausted => {
                Self::Recoverable(anyhow_error)
            }
            _ => Self::Irrecoverable(anyhow_error),
        }
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
                Self::Irrecoverable(anyhow::Error::new(err))
            }
            Erc1271Error::Transport(_)
            | Erc1271Error::WrongMagic(..)
            | Erc1271Error::Rejected(..) => Self::Recoverable(anyhow::Error::new(err)),
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
    pub fn recoverable(kind: RequestCheckKind, source: anyhow::Error) -> Self {
        Self {
            kind,
            source: ProcessingError::Recoverable(source),
        }
    }

    pub fn irrecoverable(kind: RequestCheckKind, source: anyhow::Error) -> Self {
        Self {
            kind,
            source: ProcessingError::Irrecoverable(source),
        }
    }

    pub fn network(err: impl Into<anyhow::Error>) -> Self {
        // Network errors are always considered as recoverable
        Self {
            kind: RequestCheckKind::Network,
            source: ProcessingError::Recoverable(err.into()),
        }
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
            Erc1271Error::EmptySigOnEoa(_)
            | Erc1271Error::EoaMismatchNoCode(_)
            | Erc1271Error::Rejected(..)
            | Erc1271Error::WrongMagic(..) => RequestCheckKind::Signature,
        };
        Self {
            kind,
            source: err.into(),
        }
    }
}
