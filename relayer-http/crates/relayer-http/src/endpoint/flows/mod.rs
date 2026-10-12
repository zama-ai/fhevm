//! One file per route. Each owns its wire types, their conversion to the connector DTO, its validation and its
//! handler, which logs every step through the request's `Log`. Shared here: the body reader, the success envelope and
//! the clock.

pub mod public_decrypt;
pub mod user_decrypt;

use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::extract::{FromRequest, Request};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json::value::RawValue;
use tokio::time::timeout;

use super::ApiError;
use super::error::tag;
use super::validate::Invalid;
use crate::App;
use crate::logging::Log;

/// Reads the JSON body within `http.body_read_timeout`: a total bound (not per chunk) started when the handler is
/// entered, so a client trickling its body holds the request at most that long. Any failure (timeout, invalid JSON,
/// wrong content type, body above `max_body_bytes`) is `request body rejected` then `400 malformed`.
pub async fn read_json(app: &App, request: Request, log: &Log) -> Result<Box<RawValue>, ApiError> {
    let limit = app.http.body_read_timeout;
    let reason = match timeout(limit, Json::<Box<RawValue>>::from_request(request, &())).await {
        Ok(Ok(Json(body))) => return Ok(body),
        Ok(Err(rejection)) => rejection.body_text(),
        Err(_) => format!("request body not received within {limit:?}"),
    };
    Err(rejected(log, reason))
}

/// Parses the typed body from the JSON [`read_json`] accepted. A failure (missing or unknown field, wrong type or
/// format) is axum's rejection text, for [`rejected`].
pub fn parse<T: DeserializeOwned>(body: &RawValue) -> Result<T, String> {
    Json::<T>::from_bytes(body.get().as_bytes())
        .map(|Json(value)| value)
        .map_err(|rejection| rejection.body_text())
}

/// `request body rejected` then `400 malformed`.
pub fn rejected(log: &Log, reason: String) -> ApiError {
    log.body_rejected(&reason);
    ApiError::malformed(log, reason)
}

/// A top-level field of an object body: the tag a route dispatches on.
pub fn field(body: &RawValue, name: &str) -> Option<Value> {
    serde_json::from_str::<serde_json::Map<String, Value>>(body.get())
        .ok()?
        .remove(name)
}

/// `400 malformed` for a validation rule, logged with its field.
pub fn invalid(log: &Log, error: Invalid) -> ApiError {
    log.validation_failed(&error.field, &error.issue);
    ApiError::malformed(log, error.to_string())
}

/// `{ "status": "succeeded", "requestId": "…", "result": … }`: the current relayer's success envelope.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply<T> {
    pub status: &'static str,
    pub request_id: String,
    pub result: T,
    /// The request's identifiers, for the one log line this reply produces.
    #[serde(skip)]
    pub log: Log,
}

impl<T> Reply<T> {
    pub fn succeeded(log: Log, result: T) -> Self {
        Self {
            status: "succeeded",
            request_id: log.request_id.clone(),
            result,
            log,
        }
    }
}

impl<T: Serialize> IntoResponse for Reply<T> {
    /// 200, `x-request-id`, JSON body; one log line per successful request, with the request's identifiers.
    fn into_response(self) -> Response {
        self.log.succeeded(200);
        let mut response = Json(&self).into_response();
        tag(&mut response, &self.request_id);
        response
    }
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
        let log = Log {
            request_id: "req-9".to_owned(),
            ..Log::new("test")
        };
        let response = Reply::succeeded(log, json!({ "a": 1 })).into_response();
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
    fn clock_is_after_2023() {
        assert!(now() > 1_700_000_000);
    }
}
