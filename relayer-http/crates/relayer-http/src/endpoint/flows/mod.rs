//! One file per route. Each owns its wire types, their conversion to the connector DTO, its validation and its
//! handler. Shared here: the success envelope, the request id and the clock.

pub mod public_decrypt;
pub mod user_decrypt;

use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::info;

use super::error::tag;

/// `{ "status": "succeeded", "requestId": "…", "result": … }`: the current relayer's success envelope.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply<T> {
    pub status: &'static str,
    pub request_id: String,
    pub result: T,
}

impl<T> Reply<T> {
    pub fn succeeded(request_id: String, result: T) -> Self {
        Self {
            status: "succeeded",
            request_id,
            result,
        }
    }
}

impl<T: Serialize> IntoResponse for Reply<T> {
    /// 200, `x-request-id`, JSON body; one log line per successful request.
    fn into_response(self) -> Response {
        info!(request_id = %self.request_id, status = 200u16, "request succeeded");
        let mut response = Json(&self).into_response();
        tag(&mut response, &self.request_id);
        response
    }
}

/// The relayer's correlation id for one HTTP request (UUIDv7): logged everywhere, returned in the body and in
/// `x-request-id`, passed to the aggregator for its logs. Nothing keys on it.
pub fn request_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

/// Seconds since the Unix epoch; 0 if the clock is before it (never in practice).
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    #[tokio::test]
    async fn reply_is_the_relayer_envelope_with_the_header() {
        let response = Reply::succeeded("req-9".to_owned(), json!({ "a": 1 })).into_response();
        assert_eq!(response.status(), 200);
        assert_eq!(response.headers()["x-request-id"], "req-9");
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            body,
            json!({ "status": "succeeded", "requestId": "req-9", "result": { "a": 1 } })
        );
    }

    #[test]
    fn request_ids_are_unique_uuids() {
        let (a, b) = (request_id(), request_id());
        assert_ne!(a, b);
        assert_eq!(a.len(), 36);
        assert!(now() > 1_700_000_000);
    }
}
