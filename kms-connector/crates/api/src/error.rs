use alloy::primitives::B256;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use strum::IntoEnumIterator;
use strum::IntoStaticStr;

/// The `v1` error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, IntoStaticStr)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed body, bad handle format, unsupported chain id (connector endpoint).
    Malformed,
    /// Invalid user EIP-712 signature on user-decrypt, pure crypto check (connector endpoint).
    InvalidUserSignature,
    /// Sender authentication failure (connector proxy).
    SenderAuthenticationFailed,
    /// Per-sender rate limit exceeded (connector proxy).
    RateLimited,
    /// Endpoint replica at its in-flight cap; comes with `Retry-After` (connector endpoint).
    Overloaded,
    /// ACL denied the requested handles (kms_worker row).
    AclDenied,
    /// The user's EIP-712 / ERC-1271 signature was rejected per RFC 016 (kms_worker row).
    UserSignatureRejected,
    /// Ciphertext material not found (kms_worker row).
    CiphertextNotFound,
    /// Coprocessor ciphertext-attestation consensus could not confirm the material.
    CoproConsensusFailed,
    /// Irrecoverable rejection: malformed handle, handles resolving to different key ids,
    /// invalid `extra_data`, terminal KMS Core error (kms_worker row).
    Unprocessable,
    /// Transient KMS Core / RPC failure (kms_worker row).
    UpstreamTransient,
    /// Deserialization fallback for codes this crate version does not know.
    #[serde(other)]
    Unknown,
}

impl ErrorCode {
    /// HTTP status associated with this code.
    pub fn http_status(self) -> u16 {
        match self {
            Self::Malformed => 400,
            Self::InvalidUserSignature | Self::SenderAuthenticationFailed => 401,
            Self::AclDenied | Self::UserSignatureRejected => 403,
            Self::CiphertextNotFound => 404,
            Self::Unprocessable => 422,
            Self::RateLimited => 429,
            Self::CoproConsensusFailed | Self::UpstreamTransient => 502,
            Self::Overloaded => 503,
            Self::Unknown => 500,
        }
    }

    /// The canonical wire name of this code — the string used in JSON bodies and in the
    /// `error_code` column of the response tables.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Whether re-submitting the same request can result in a different outcome.
    pub fn retryable(self) -> bool {
        match self {
            Self::Malformed
            | Self::InvalidUserSignature
            | Self::SenderAuthenticationFailed
            | Self::Unprocessable
            | Self::Unknown => false,
            Self::RateLimited
            | Self::Overloaded
            | Self::AclDenied
            | Self::UserSignatureRejected
            | Self::CiphertextNotFound
            | Self::CoproConsensusFailed
            | Self::UpstreamTransient => true,
        }
    }
}

/// The `v1` error body.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
    // Present once the request was well-formed enough for the id to be derived.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub decryption_id: Option<B256>,
}

impl ErrorResponse {
    pub fn new(code: ErrorCode, message: impl Into<String>, decryption_id: Option<B256>) -> Self {
        Self {
            code,
            message: message.into(),
            retryable: code.retryable(),
            decryption_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_matches_serde_wire_name() {
        for code in ErrorCode::iter() {
            let serialized = serde_json::to_string(&code).unwrap();
            assert_eq!(serialized, format!("\"{}\"", code.as_str()));
        }
    }

    #[test]
    fn unknown_error_code_still_parses() {
        let json = r#"{"code":"random_unknown_code","message":"m","retryable":true}"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, ErrorCode::Unknown);
        assert!(response.retryable);
    }
}
