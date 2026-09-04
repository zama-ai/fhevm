use alloy::primitives::B256;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[cfg(test)]
use strum::IntoEnumIterator;
use strum::{EnumString, IntoStaticStr};

/// The `v1` error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, IntoStaticStr, EnumString)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed body, bad handle format, unsupported chain id (connector endpoint).
    Malformed,
    /// Sender authentication failure (connector proxy).
    SenderAuthenticationFailed,
    /// Per-sender rate limit exceeded (connector proxy).
    RateLimited,
    /// Endpoint replica at its in-flight cap (connector endpoint).
    Overloaded,
    /// ACL denied the requested handles (kms_worker row).
    AclDenied,
    /// The user's EIP-712 / ERC-1271 signature was rejected per RFC 016 (kms_worker row).
    UserSignatureRejected,
    /// Ciphertext material not found (kms_worker row).
    CiphertextNotFound,
    /// Coprocessor ciphertext-attestation consensus could not confirm the material.
    CoproConsensusFailed,
    /// The referenced KMS context/epoch is not currently valid/active on-chain (kms_worker row).
    KmsContextInvalid,
    /// The referenced KMS context/epoch has been destroyed (kms_worker row).
    KmsContextDestroyed,
    /// Irrecoverable rejection: malformed handle, handles resolving to different key ids,
    /// invalid `extra_data`, terminal KMS Core error (kms_worker row).
    Unprocessable,
    /// Transient KMS Core / RPC failure (kms_worker row).
    UpstreamTransient,
    /// The endpoint's request timed out before the response was available (connector endpoint).
    Timeout,
    /// Deserialization fallback for codes this crate version does not know.
    #[serde(other)]
    Unknown,
}

impl ErrorCode {
    /// HTTP status associated with this code.
    pub fn http_status(self) -> u16 {
        match self {
            Self::Malformed => 400,
            Self::SenderAuthenticationFailed => 401,
            Self::AclDenied | Self::UserSignatureRejected => 403,
            Self::CiphertextNotFound => 404,
            Self::KmsContextDestroyed => 410,
            Self::KmsContextInvalid => 412,
            Self::Unprocessable => 422,
            Self::RateLimited => 429,
            Self::CoproConsensusFailed | Self::UpstreamTransient => 502,
            Self::Overloaded => 503,
            Self::Timeout => 504,
            Self::Unknown => 500,
        }
    }

    /// The string used in JSON bodies for this error code.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Whether re-submitting the same request can result in a different outcome.
    pub fn retryable(self) -> bool {
        match self {
            Self::Malformed
            | Self::SenderAuthenticationFailed
            | Self::KmsContextDestroyed
            | Self::Unprocessable => false,
            Self::RateLimited
            | Self::Overloaded
            | Self::AclDenied
            | Self::UserSignatureRejected
            | Self::CiphertextNotFound
            | Self::CoproConsensusFailed
            | Self::KmsContextInvalid
            | Self::UpstreamTransient
            | Self::Timeout
            | Self::Unknown => true,
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

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

#[cfg(feature = "endpoint")]
use actix_web::{HttpResponse, http::StatusCode};

#[cfg(feature = "endpoint")]
impl actix_web::ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
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
    fn from_str_round_trips_wire_name() {
        for code in ErrorCode::iter() {
            assert_eq!(code.as_str().parse::<ErrorCode>().unwrap(), code);
        }
        assert!("not_a_code".parse::<ErrorCode>().is_err());
    }

    #[test]
    fn unknown_error_code_still_parses() {
        let json = r#"{"code":"random_unknown_code","message":"m","retryable":true}"#;
        let response: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, ErrorCode::Unknown);
        assert!(response.retryable);
    }
}
