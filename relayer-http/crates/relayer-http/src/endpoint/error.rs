//! The only error a handler returns: `{ "code", "message", "requestId" }`, logged once here, at the boundary.

use axum::Json;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use kms_connector_api::ErrorCode;
use serde::Serialize;
use tracing::{info, warn};

use crate::kms_aggregator::AggregationError;

/// Wire body of every error.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
    pub request_id: String,
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub body: ErrorBody,
}

impl ApiError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        request_id: &str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            status,
            body: ErrorBody {
                code,
                message: message.into(),
                request_id: request_id.to_owned(),
            },
        }
    }

    /// The request is wrong: unreadable or oversized body, invalid JSON, unknown field, wrong content type, or a
    /// validation rule. `message` names the field.
    pub fn malformed(request_id: &str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "malformed", request_id, message)
    }

    /// The aggregator's verdict. A dominant connector error is answered with the connector's own status and code.
    pub fn aggregation(request_id: &str, error: AggregationError) -> Self {
        match error {
            AggregationError::Timeout {
                counted, threshold, ..
            } => Self::new(
                StatusCode::GATEWAY_TIMEOUT,
                "timeout",
                request_id,
                format!("KMS nodes did not answer in time ({counted} of {threshold} responses)"),
            ),
            AggregationError::ThresholdNotReached {
                counted,
                threshold,
                rejected,
                dominant,
            } => {
                let code = dominant.unwrap_or(ErrorCode::UpstreamTransient);
                let status =
                    StatusCode::from_u16(code.http_status()).unwrap_or(StatusCode::BAD_GATEWAY);
                Self::new(
                    status,
                    code.as_str(),
                    request_id,
                    format!(
                        "KMS threshold not reached ({counted} of {threshold} responses, {rejected} rejected)"
                    ),
                )
            }
            AggregationError::Cancelled => Self::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "shutting_down",
                request_id,
                "relayer is shutting down",
            ),
            AggregationError::Internal(message) => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
                request_id,
                message,
            ),
        }
    }
}

impl IntoResponse for ApiError {
    /// One log line per failed request: 4xx are the client's (info), 5xx are ours or the nodes' (warn).
    fn into_response(self) -> Response {
        let (request_id, status, code) =
            (&self.body.request_id, self.status.as_u16(), self.body.code);
        if self.status.is_server_error() {
            warn!(request_id, status, code, "request failed");
        } else {
            info!(request_id, status, code, reason = %self.body.message, "request rejected");
        }
        let mut response = (self.status, Json(&self.body)).into_response();
        tag(&mut response, &self.body.request_id);
        response
    }
}

/// Router fallback: no such route.
pub async fn not_found() -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        "not_found",
        &super::flows::request_id(),
        "no such route",
    )
}

/// Router fallback: the route exists, the method does not.
pub async fn method_not_allowed() -> ApiError {
    ApiError::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "method_not_allowed",
        &super::flows::request_id(),
        "method not allowed on this route",
    )
}

/// `x-request-id: <request_id>` on every response. A UUID is always a valid header value; the `if let` keeps the
/// no-panic rule.
pub(crate) fn tag(response: &mut Response, request_id: &str) {
    if let Ok(value) = HeaderValue::from_str(request_id) {
        response.headers_mut().insert("x-request-id", value);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    async fn rendered(error: ApiError) -> (StatusCode, String, Value) {
        let response = error.into_response();
        let status = response.status();
        let header = response
            .headers()
            .get("x-request-id")
            .map(|v| v.to_str().unwrap().to_owned())
            .unwrap_or_default();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, header, serde_json::from_slice(&bytes).unwrap())
    }

    #[tokio::test]
    async fn body_has_exactly_code_message_and_request_id() {
        let (status, header, body) =
            rendered(ApiError::malformed("req-1", "handles: must not be empty")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(header, "req-1");
        assert_eq!(
            body,
            json!({ "code": "malformed", "message": "handles: must not be empty", "requestId": "req-1" })
        );
    }

    #[tokio::test]
    async fn aggregation_errors_map_to_status_and_code() {
        let cases = [
            (
                AggregationError::Timeout {
                    counted: 7,
                    threshold: 9,
                    rejected: 0,
                    dominant: None,
                },
                StatusCode::GATEWAY_TIMEOUT,
                "timeout",
            ),
            (
                AggregationError::ThresholdNotReached {
                    counted: 4,
                    threshold: 9,
                    rejected: 1,
                    dominant: Some(ErrorCode::AclDenied),
                },
                StatusCode::FORBIDDEN,
                "acl_denied",
            ),
            (
                AggregationError::ThresholdNotReached {
                    counted: 0,
                    threshold: 9,
                    rejected: 0,
                    dominant: Some(ErrorCode::CiphertextNotFound),
                },
                StatusCode::NOT_FOUND,
                "ciphertext_not_found",
            ),
            (
                AggregationError::ThresholdNotReached {
                    counted: 2,
                    threshold: 9,
                    rejected: 7,
                    dominant: None,
                },
                StatusCode::BAD_GATEWAY,
                "upstream_transient",
            ),
            (
                AggregationError::Cancelled,
                StatusCode::SERVICE_UNAVAILABLE,
                "shutting_down",
            ),
            (
                AggregationError::Internal("boom".into()),
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal",
            ),
        ];
        for (error, expected_status, expected_code) in cases {
            let (status, _, body) = rendered(ApiError::aggregation("req-2", error)).await;
            assert_eq!(
                (status, body["code"].as_str().unwrap()),
                (expected_status, expected_code)
            );
            assert_eq!(body["requestId"], "req-2");
        }
    }

    #[test]
    fn every_connector_code_keeps_its_own_status() {
        for code in [
            ErrorCode::Malformed,
            ErrorCode::SenderAuthenticationFailed,
            ErrorCode::RateLimited,
            ErrorCode::Overloaded,
            ErrorCode::AclDenied,
            ErrorCode::UserSignatureRejected,
            ErrorCode::UnsupportedAttestationType,
            ErrorCode::CiphertextNotFound,
            ErrorCode::CoproConsensusFailed,
            ErrorCode::KmsContextInvalid,
            ErrorCode::KmsContextDestroyed,
            ErrorCode::Unprocessable,
            ErrorCode::UpstreamTransient,
            ErrorCode::Timeout,
            ErrorCode::Unknown,
        ] {
            let error = ApiError::aggregation(
                "r",
                AggregationError::ThresholdNotReached {
                    counted: 0,
                    threshold: 1,
                    rejected: 0,
                    dominant: Some(code),
                },
            );
            assert_eq!(error.status.as_u16(), code.http_status(), "{code:?}");
            assert_eq!(error.body.code, code.as_str());
            assert!(error.body.message.contains("0 of 1"));
        }
    }
}
