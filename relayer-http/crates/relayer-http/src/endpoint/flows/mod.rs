//! One file per route. Each owns its wire types, their conversion to the connector DTO, its validation and its
//! handler, which fills the request's `Summary` as it goes. Shared here: the body reader, the success envelope,
//! the clock and the SDK's integer encoding.

pub mod pad_decrypt;
pub mod public_decrypt;
pub mod user_decrypt;

use std::fmt::Write as _;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::error::Category;
use serde_path_to_error::Segment;
use tokio::time::timeout;
use tracing::debug;

use super::ApiError;
use super::error::tag;
use super::summary::Summary;
use super::validate::Invalid;
use crate::App;

/// Reads and parses the JSON body within `http.body_read_timeout`: a total bound (not per chunk) started when the
/// handler is entered, so a client trickling its body holds the request at most that long. Any failure is
/// `400 malformed` with one fixed text, which is also the logged `reason`: never the parser's own message, which
/// can quote the offending value. A wrong content type is refused before the body is read: the summary's
/// `body_len` stays 0 (not received), as for a body over `http.max_body_bytes` (`body exceeds <limit> bytes`).
pub async fn read_json<T: DeserializeOwned>(
    app: &App,
    request: Request,
    summary: &mut Summary,
) -> Result<T, ApiError> {
    if !is_json(request.headers().get(CONTENT_TYPE)) {
        return Err(ApiError::malformed(summary, "wrong content type"));
    }
    let limit = app.http.body_read_timeout;
    let reason = match timeout(limit, Bytes::from_request(request, &())).await {
        Ok(Ok(bytes)) => {
            summary.body_len = bytes.len();
            match parse(&bytes) {
                Ok(body) => return Ok(body),
                Err(reason) => reason,
            }
        }
        // Still `400 malformed`, as before the summary line existed; `body_len` stays 0, the body was not received.
        Ok(Err(rejection)) if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE => {
            format!("body exceeds {} bytes", app.http.max_body_bytes)
        }
        Ok(Err(_)) => "body unreadable".to_owned(),
        Err(_) => format!("request body not received within {limit:?}"),
    };
    Err(ApiError::malformed(summary, reason))
}

/// The request failed validation: `400 malformed` naming the field; the offending value goes to a DEBUG line, never
/// to the answer.
pub fn invalid(summary: &Summary, invalid: Invalid) -> ApiError {
    debug!(
        request_id = %summary.request_id,
        field = %invalid.field,
        issue = %invalid.issue,
        value = invalid.value.as_deref().unwrap_or("none"),
        "request invalid"
    );
    ApiError::malformed(summary, invalid.to_string())
}

/// `application/json` or `application/*+json`, as the `Json` extractor requires.
fn is_json(content_type: Option<&HeaderValue>) -> bool {
    let Some(Ok(value)) = content_type.map(HeaderValue::to_str) else {
        return false;
    };
    let essence = value
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    essence == "application/json"
        || (essence.starts_with("application/") && essence.ends_with("+json"))
}

/// The body as `T`, or the fixed text for what is wrong (see [`body_error`]). Data after the JSON value is
/// `invalid JSON`.
fn parse<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let body =
        serde_path_to_error::deserialize(&mut deserializer).map_err(|error| body_error(&error))?;
    deserializer.end().map_err(|_| "invalid JSON".to_owned())?;
    Ok(body)
}

/// The fixed text for a body that did not parse, built here and never from serde's text, which quotes the
/// offending value or key: `invalid JSON`; `invalid body: unknown field at <parent path>` (the key is the client's
/// and is never repeated); `invalid body: missing field <name> at <path>` (the name is one of ours); or
/// `invalid body: <field path>`. The `at <path>` is left out at the root.
fn body_error(error: &serde_path_to_error::Error<serde_json::Error>) -> String {
    let inner = error.inner();
    if inner.classify() != Category::Data {
        return "invalid JSON".to_owned();
    }
    let segments: Vec<&Segment> = error.path().iter().collect();
    let text = inner.to_string();
    if text.starts_with("unknown field") {
        // The path ends with the unknown key.
        let parent = segments.split_last().map(|(_, parent)| parent);
        return at("invalid body: unknown field", parent.unwrap_or_default());
    }
    if let Some(name) = missing_field(&text) {
        return at(&format!("invalid body: missing field {name}"), &segments);
    }
    match field_path(&segments).as_str() {
        "" => "invalid body".to_owned(),
        path => format!("invalid body: {path}"),
    }
}

/// `what`, followed by ` at <path>` unless the path is the root.
fn at(what: &str, segments: &[&Segment]) -> String {
    match field_path(segments).as_str() {
        "" => what.to_owned(),
        path => format!("{what} at {path}"),
    }
}

/// `payload.handles[0].ctHandle`; empty for the root.
fn field_path(segments: &[&Segment]) -> String {
    let mut path = String::new();
    for segment in segments {
        match segment {
            Segment::Seq { index } => {
                let _ = write!(path, "[{index}]");
            }
            Segment::Map { key } => {
                if !path.is_empty() {
                    path.push('.');
                }
                path.push_str(key);
            }
            _ => path.push('?'),
        }
    }
    path
}

/// The name in serde's ``missing field `name` ``: ours (a field of the schema), accepted only as an identifier.
fn missing_field(text: &str) -> Option<&str> {
    let name = text.strip_prefix("missing field `")?.split_once('`')?.0;
    (!name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        .then_some(name)
}

/// `{ "status": "succeeded", "requestId": "…", "result": … }`: the current relayer's success envelope.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply<T> {
    pub status: &'static str,
    pub request_id: String,
    pub result: T,
    /// The request, for the summary line.
    #[serde(skip)]
    pub summary: Summary,
}

impl<T> Reply<T> {
    pub fn succeeded(summary: Summary, result: T) -> Self {
        Self {
            status: "succeeded",
            request_id: summary.request_id.clone(),
            result,
            summary,
        }
    }
}

impl<T: Serialize> IntoResponse for Reply<T> {
    /// 200, `x-request-id`, JSON body, and the `Summary` in the extensions for `endpoint::record`.
    fn into_response(self) -> Response {
        let mut response = Json(&self).into_response();
        tag(&mut response, &self.request_id);
        response.extensions_mut().insert(self.summary);
        response
    }
}

/// The SDK sends these as decimal strings; numbers are accepted too.
pub fn u64_from_string_or_number<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Number(u64),
        Text(String),
    }
    match Raw::deserialize(deserializer)? {
        Raw::Number(value) => Ok(value),
        Raw::Text(text) => text
            .parse()
            .map_err(|_| serde::de::Error::custom("must be a decimal integer")),
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
    use crate::endpoint::flows::public_decrypt::PublicDecryptRequest;
    use crate::endpoint::flows::user_decrypt::UserDecryptRequest;
    use crate::metrics::FlowName;

    #[test]
    fn json_content_types() {
        let is = |value: &str| is_json(Some(&HeaderValue::from_str(value).unwrap()));
        for json in [
            "application/json",
            "Application/JSON; charset=utf-8",
            "application/vnd.api+json",
        ] {
            assert!(is(json), "{json}");
        }
        for other in [
            "text/plain",
            "application/jsonp",
            "application/x-ndjson",
            "",
        ] {
            assert!(!is(other), "{other}");
        }
        assert!(!is_json(None));
        assert!(!is_json(Some(&HeaderValue::from_bytes(b"\xff").unwrap())));
    }

    /// A public decrypt body through `parse`.
    fn public(body: &str) -> Result<PublicDecryptRequest, String> {
        parse(body.as_bytes())
    }

    #[test]
    fn parse_gives_the_value_or_a_fixed_text() {
        let user = |from: &str, to: &str| user_decrypt::tests::wire("1", "2").replace(from, to);
        let ok = r#"{"ciphertextHandles": [], "extraData": "0x00"}"#;
        assert!(public(ok).is_ok());
        assert!(public(&format!("  {ok}  ")).is_ok());
        for (body, text) in [
            ("{ not json", "invalid JSON"),
            ("", "invalid JSON"),
            (r#"{"ciphertextHandles": ["#, "invalid JSON"),
            (&format!("{ok} x"), "invalid JSON"),
            (&format!("{ok}{{}}"), "invalid JSON"),
            ("[1]", "invalid body: [0]"),
            (
                r#"{"ciphertextHandles": ["0x12"], "extraData": "0x00"}"#,
                "invalid body: ciphertextHandles[0]",
            ),
            (
                r#"{"ciphertextHandles": [], "extraData": "0x00", "SECRET-KEY": 1}"#,
                "invalid body: unknown field",
            ),
            (
                r#"{"ciphertextHandles": []}"#,
                "invalid body: missing field extraData",
            ),
        ] {
            assert_eq!(public(body).unwrap_err(), text, "{body}");
        }
        for (wire, text) in [
            (
                user(r#""publicKey""#, r#""SECRET-KEY": 1, "publicKey""#),
                "invalid body: unknown field at payload",
            ),
            (
                user(r#""ownerAddress""#, r#""SECRET-KEY""#),
                "invalid body: unknown field at payload.handles[0]",
            ),
            (
                user(r#""publicKey": "0x20002000","#, ""),
                "invalid body: missing field publicKey at payload",
            ),
        ] {
            let error = parse::<UserDecryptRequest>(wire.as_bytes()).unwrap_err();
            assert_eq!(error, text);
        }
    }

    #[test]
    fn missing_field_names_are_identifiers_only() {
        assert_eq!(
            missing_field("missing field `extraData` at line 1"),
            Some("extraData")
        );
        for text in [
            "missing field `a b`",
            "missing field ``",
            "invalid type: integer",
            "missing field `unterminated",
        ] {
            assert_eq!(missing_field(text), None, "{text}");
        }
    }

    #[tokio::test]
    async fn reply_is_the_relayer_envelope_with_the_header() {
        let mut summary = Summary::new(FlowName::None);
        summary.request_id = "req-9".to_owned();
        let response = Reply::succeeded(summary, json!({ "a": 1 })).into_response();
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
