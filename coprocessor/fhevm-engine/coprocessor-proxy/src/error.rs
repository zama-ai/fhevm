//! The errors the proxy answers itself, in the RFC-033 JSON shape `{code, message, retryable}`.

use fhevm_sig_auth::AuthError;
use fhevm_sig_proxy::{ErrorBody, ProxyError};
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
    retryable: bool,
}

/// Renders a [`ProxyError`]. A failed `FhevmSig` check has the `auth_*` code of its step
/// (RFC-038). Only an expired header and the endpoint failures are retryable.
pub fn error_body(error: &ProxyError) -> ErrorBody {
    let (status, code) = match error {
        ProxyError::NotOriginForm | ProxyError::BodyTooLarge => {
            (StatusCode::BAD_REQUEST, "malformed")
        }
        ProxyError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
        ProxyError::MethodNotAllowed(_) => (StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed"),
        ProxyError::Auth(auth) => (StatusCode::UNAUTHORIZED, auth.code()),
        ProxyError::EndpointUnavailable => (StatusCode::BAD_GATEWAY, "upstream_transient"),
        ProxyError::EndpointTimeout => (StatusCode::GATEWAY_TIMEOUT, "timeout"),
    };
    let response = ErrorResponse {
        code,
        message: error.to_string(),
        retryable: status.is_server_error() || *error == ProxyError::Auth(AuthError::Expired),
    };
    ErrorBody {
        status,
        json: serde_json::to_vec(&response).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn rendered(error: ProxyError) -> (StatusCode, Value) {
        let ErrorBody { status, json } = error_body(&error);
        (status, serde_json::from_slice(&json).unwrap())
    }

    #[test]
    fn each_error_has_its_status_code_and_retryability() {
        for (error, status, code, retryable) in [
            (ProxyError::NotOriginForm, 400, "malformed", false),
            (ProxyError::BodyTooLarge, 400, "malformed", false),
            (ProxyError::NotFound, 404, "not_found", false),
            (
                ProxyError::MethodNotAllowed(vec![http::Method::GET]),
                405,
                "method_not_allowed",
                false,
            ),
            (
                ProxyError::Auth(AuthError::Missing),
                401,
                "auth_missing",
                false,
            ),
            (
                ProxyError::Auth(AuthError::Expired),
                401,
                "auth_expired",
                true,
            ),
            (
                ProxyError::EndpointUnavailable,
                502,
                "upstream_transient",
                true,
            ),
            (ProxyError::EndpointTimeout, 504, "timeout", true),
        ] {
            let message = error.to_string();
            assert_eq!(
                rendered(error),
                (
                    StatusCode::from_u16(status).unwrap(),
                    json!({"code": code, "message": message, "retryable": retryable})
                )
            );
        }
    }
}
