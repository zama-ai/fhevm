//! The errors the proxy answers itself, in the RFC-033 JSON shape `{code, message, retryable}`.

use http::StatusCode;
use serde::{Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    Malformed,
    NotFound,
    MethodNotAllowed,
    /// A failed `FhevmSig` check, with the `auth_*` code of the failing step (RFC-038).
    Auth {
        code: &'static str,
        retryable: bool,
    },
    /// The signer sent more requests than its rate limit allows.
    ConcurrencyLimited,
    UpstreamTransient,
    Timeout,
}

impl ErrorCode {
    pub fn http_status(self) -> StatusCode {
        match self {
            Self::Malformed => StatusCode::BAD_REQUEST,
            Self::Auth { .. } => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Self::ConcurrencyLimited => StatusCode::TOO_MANY_REQUESTS,
            Self::UpstreamTransient => StatusCode::BAD_GATEWAY,
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Malformed => "malformed",
            Self::NotFound => "not_found",
            Self::MethodNotAllowed => "method_not_allowed",
            Self::Auth { code, .. } => code,
            Self::ConcurrencyLimited => "concurrency_limited",
            Self::UpstreamTransient => "upstream_transient",
            Self::Timeout => "timeout",
        }
    }

    pub fn retryable(self) -> bool {
        match self {
            Self::Auth { retryable, .. } => retryable,
            Self::ConcurrencyLimited | Self::UpstreamTransient | Self::Timeout => true,
            Self::Malformed | Self::NotFound | Self::MethodNotAllowed => false,
        }
    }
}

impl Serialize for ErrorCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
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

    #[test]
    fn serializes_to_the_rfc_033_shape() {
        let error = ErrorResponse::new(ErrorCode::NotFound, "unknown route");
        assert_eq!(
            serde_json::to_value(&error).unwrap(),
            json!({"code": "not_found", "message": "unknown route", "retryable": false})
        );
    }

    #[test]
    fn only_rate_limits_upstream_failures_and_retryable_auth_failures_are_retryable() {
        for (code, status, retryable) in [
            (ErrorCode::Malformed, 400, false),
            (ErrorCode::NotFound, 404, false),
            (ErrorCode::MethodNotAllowed, 405, false),
            (
                ErrorCode::Auth {
                    code: "auth_missing",
                    retryable: false,
                },
                401,
                false,
            ),
            (ErrorCode::ConcurrencyLimited, 429, true),
            (ErrorCode::UpstreamTransient, 502, true),
            (ErrorCode::Timeout, 504, true),
        ] {
            assert_eq!(
                (code.http_status().as_u16(), code.retryable()),
                (status, retryable)
            );
        }
    }
}
