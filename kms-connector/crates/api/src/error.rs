use alloy::primitives::B256;
use serde::{Deserialize, Serialize};

/// The `v1` error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed body, bad handle format, unsupported chain id (connector endpoint).
    Malformed,
    /// Invalid `senderSignature` (connector endpoint).
    InvalidSenderSignature,
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
    /// Irrecoverable rejection: malformed handle, handles resolving to different key ids,
    /// terminal KMS Core error (kms_worker row).
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
            Self::InvalidSenderSignature => 401,
            Self::AclDenied | Self::UserSignatureRejected => 403,
            Self::CiphertextNotFound => 404,
            Self::Unprocessable => 422,
            Self::RateLimited => 429,
            Self::UpstreamTransient => 502,
            Self::Overloaded => 503,
            Self::Unknown => 500,
        }
    }

    /// Whether re-submitting the same request can result in a different outcome.
    pub fn retryable(self) -> bool {
        match self {
            Self::Malformed
            | Self::InvalidSenderSignature
            | Self::Unprocessable
            | Self::Unknown => false,
            Self::RateLimited
            | Self::Overloaded
            | Self::AclDenied
            | Self::UserSignatureRejected
            | Self::CiphertextNotFound
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
    fn unknown_error_code_still_parses() {
        let json = r#"{"code":"random_unknown_code","message":"m","retryable":true}"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, ErrorCode::Unknown);
        assert!(response.retryable);
    }
}
