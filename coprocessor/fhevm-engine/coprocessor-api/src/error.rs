//! The error answers of the coprocessor interface: one [`ErrorCode`] per failure, sent in an
//! [`ErrorResponse`] body.

use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

/// The error codes, one per row of the
/// [PRF endpoint spec's status table](https://github.com/zama-ai/tech-spec/blob/main/rfcs/038-coprocessor-prf-output-endpoint.md#status-codes).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoStaticStr)]
#[cfg_attr(test, derive(strum::EnumIter))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ErrorCode {
    /// The request target is not in origin form.
    Malformed,
    /// A non-canonical or out-of-range path parameter.
    NonCanonicalPath,
    // The `FhevmSig` checks, one code per failing step.
    AuthMissing,
    AuthMalformed,
    AuthQueryString,
    /// Retryable after re-signing.
    AuthExpired,
    AuthTooLongLived,
    AuthInvalidSignature,
    /// Not retryable, although a connector retries it: the signer list of a coprocessor can be late.
    AuthUnknownSigner,
    AuthBodyMismatch,
    /// The path matches no route.
    NotFound,
    /// The `prf` is not a supported instance on this coprocessor.
    UnknownPrf,
    MethodNotAllowed,
    /// A per-connector or per-client-IP rate limit is reached; `Retry-After` is set.
    ConcurrencyLimited,
    /// The PRF evaluation failed for good.
    PrfEvaluationFailed,
    /// The proxy cannot reach prf-service.
    UpstreamTransient,
    /// The coprocessor is temporarily unable to serve.
    Unavailable,
    /// prf-service did not answer in time.
    Timeout,
    /// Deserialization fallback for a code this crate version does not know. No server sends it.
    /// [`ErrorCode::retryable`] is false for it, so a new permanent error is not retried: a client
    /// acts on the HTTP status and the body's `retryable` flag, which the server sets.
    #[serde(other)]
    Unknown,
}

impl ErrorCode {
    /// The HTTP status answered with this code.
    pub fn http_status(self) -> u16 {
        match self {
            Self::Malformed | Self::NonCanonicalPath => 400,
            Self::AuthMissing
            | Self::AuthMalformed
            | Self::AuthQueryString
            | Self::AuthExpired
            | Self::AuthTooLongLived
            | Self::AuthInvalidSignature
            | Self::AuthUnknownSigner
            | Self::AuthBodyMismatch => 401,
            Self::NotFound | Self::UnknownPrf => 404,
            Self::MethodNotAllowed => 405,
            Self::ConcurrencyLimited => 429,
            Self::PrfEvaluationFailed | Self::Unknown => 500,
            Self::UpstreamTransient => 502,
            Self::Unavailable => 503,
            Self::Timeout => 504,
        }
    }

    /// The string of this code in error bodies.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Whether sending the same request again can get a different answer.
    pub fn retryable(self) -> bool {
        match self {
            Self::AuthExpired
            | Self::ConcurrencyLimited
            | Self::UpstreamTransient
            | Self::Unavailable
            | Self::Timeout => true,
            Self::Malformed
            | Self::NonCanonicalPath
            | Self::AuthMissing
            | Self::AuthMalformed
            | Self::AuthQueryString
            | Self::AuthTooLongLived
            | Self::AuthInvalidSignature
            | Self::AuthUnknownSigner
            | Self::AuthBodyMismatch
            | Self::NotFound
            | Self::UnknownPrf
            | Self::MethodNotAllowed
            | Self::PrfEvaluationFailed
            | Self::Unknown => false,
        }
    }
}

/// An error body: `{code, message, retryable}`, the shape of the
/// [KMS connector endpoint's errors](https://github.com/zama-ai/tech-spec/blob/main/rfcs/033-kms-connector-http-endpoint.md#error-contract),
/// so a KMS connector reads both kinds of error the same way.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
}

impl ErrorResponse {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            retryable: code.retryable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    #[derive(Deserialize)]
    struct Vectors {
        errors: Vec<ErrorVector>,
    }

    #[derive(Deserialize)]
    struct ErrorVector {
        code: String,
        status: u16,
        retryable: bool,
    }

    #[test]
    fn every_code_has_the_status_and_retryable_flag_of_the_vectors() {
        let vectors = serde_json::from_str::<Vectors>(include_str!("../tests/vectors.json"))
            .unwrap()
            .errors;
        let mut tested = HashSet::new();
        for vector in vectors {
            let code: ErrorCode = serde_json::from_value(json!(vector.code)).unwrap();
            assert_eq!(code.as_str(), vector.code);
            assert_eq!(
                (code.http_status(), code.retryable()),
                (vector.status, vector.retryable),
                "{}",
                vector.code
            );
            assert!(tested.insert(code), "{} has two vectors", vector.code);
        }
        let known: HashSet<_> = ErrorCode::iter()
            .filter(|code| *code != ErrorCode::Unknown)
            .collect();
        assert_eq!(tested, known);
    }

    #[test]
    fn serializes_the_body_shape() {
        let error = ErrorResponse::new(ErrorCode::AuthExpired, "FhevmSig expired");
        assert_eq!(
            serde_json::to_value(&error).unwrap(),
            json!({"code": "auth_expired", "message": "FhevmSig expired", "retryable": true})
        );
    }

    #[test]
    fn reads_an_unknown_code_as_unknown() {
        let error: ErrorResponse = serde_json::from_value(
            json!({"code": "added_later", "message": "", "retryable": false}),
        )
        .unwrap();
        assert_eq!(error.code, ErrorCode::Unknown);
        assert!(!error.code.retryable());
    }
}
