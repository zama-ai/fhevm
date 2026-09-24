//! Why a validated JSON payload did not become a domain request.
//!
//! `parse_and_validate` runs three stages: deserialize, validate, convert. The first two report
//! the caller's mistakes field by field. The third used to report every failure as the relayer's
//! own — a 500 with an error log — on the reasoning that a payload the validator accepted has no
//! caller mistakes left. That holds for a conversion that only moves fields. It does not hold for
//! one that also checks a wire form the validator cannot see: the Solana permit's widths, ranges
//! and ordering are checked by the connector's typed decode, at conversion. This error tells the
//! two apart, so each is answered as what it is.

use zama_solana_permit::{IdentityField, PermitError};

/// Why a payload that passed field validation did not convert.
#[derive(Debug, thiserror::Error)]
pub enum RequestConversionError {
    /// The payload is not a well-formed request of its kind. The caller's fault: answered as a
    /// field-keyed `validation_failed` (400), the same shape a validator refusal has.
    #[error("{field}: {issue}")]
    Malformed { field: String, issue: String },
    /// The relayer could not build the request from a well-formed payload. Its own defect:
    /// answered as a server error (500) and logged at error level.
    #[error("{0}")]
    Internal(String),
}

impl RequestConversionError {
    pub fn malformed(field: impl Into<String>, issue: impl Into<String>) -> Self {
        Self::Malformed {
            field: field.into(),
            issue: issue.into(),
        }
    }
}

/// Conversions that only move already-validated fields keep `anyhow::Error`: a failure there
/// was never the caller's mistake, and stays the relayer's own until the conversion is typed.
impl From<anyhow::Error> for RequestConversionError {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal(error.to_string())
    }
}

/// Same for the v2 conversion that reports with a bare `String`.
impl From<String> for RequestConversionError {
    fn from(message: String) -> Self {
        Self::Internal(message)
    }
}

/// The payload field a permit-form refusal is about, named as the client sent it.
///
/// Total over `PermitError` on purpose: the typed decode never yields the two verification
/// outcomes, but a new variant has to be placed here rather than fall through.
pub fn permit_error_field(error: &PermitError) -> &'static str {
    match error {
        PermitError::IdentityWidth {
            field: IdentityField::UserPubkey,
            ..
        } => "userPubkey",
        PermitError::IdentityWidth {
            field: IdentityField::VerifyingProgramId,
            ..
        } => "verifyingProgramId",
        PermitError::ScopeWidth { .. }
        | PermitError::TooManyScopes { .. }
        | PermitError::ScopesNotAscending { .. }
        | PermitError::DuplicateScope { .. } => "allowedScopes",
        PermitError::DurationOutOfRange { .. } => "requestValidity.durationSeconds",
        PermitError::StartTimestampOutOfRange { .. } => "requestValidity.startTimestamp",
        PermitError::TransportKeyLength { .. } => "transportKey",
        PermitError::UnknownKmsRoutingVersion { .. } | PermitError::KmsRoutingLength { .. } => {
            "extraData"
        }
        PermitError::SignatureMismatch => "signature",
        PermitError::UnusableUserPubkey => "userPubkey",
    }
}
