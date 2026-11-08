//! One HTTP exchange with one node: URL, auth header, body cap, decoding. No retries, no deadline (see `call.rs`).

use std::fmt;
use std::time::Duration;

use alloy::primitives::Address;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use kms_connector_api::{ErrorCode, ErrorResponse};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, RETRY_AFTER};
use serde::de::DeserializeOwned;
use url::Url;

use super::config::{ConfigError, EndpointConfig, KmsAggregatorConfig, MAX_TIMEOUT, RetryClass};

/// Response bodies above this are dropped. One user-decrypt share is ~1.4 KiB per handle, hex-encoded in JSON;
/// the 2048-bit budget allows 1024 handles, so the largest valid answer is ~2.8 MiB.
pub const MAX_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

/// One KMS node as the caller sees it.
#[derive(Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: Url,
    pub audience: Address,
}

impl From<&EndpointConfig> for Endpoint {
    fn from(cfg: &EndpointConfig) -> Self {
        Self {
            name: cfg.name.clone(),
            url: cfg.url.clone(),
            audience: cfg.audience,
        }
    }
}

impl fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Endpoint")
            .field("name", &self.name)
            .field("url", &self.url.as_str())
            .field("audience", &self.audience)
            .finish()
    }
}

/// Raw HTTP answer. Debug prints the status and the body length only.
pub struct HttpReply {
    pub status: u16,
    pub body: Bytes,
    /// The `Retry-After` header, when readable (see `retry_after`).
    pub retry_after: Option<Duration>,
}

impl fmt::Debug for HttpReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpReply")
            .field("status", &self.status)
            .field("body_len", &self.body.len())
            .finish()
    }
}

/// Why one attempt produced no usable response.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum AttemptError {
    /// The connector's `v1` error body, kept whole (code, message, retryable, decryption id).
    #[error("{} (http {status})", .error.code.as_str())]
    Api { status: u16, error: ErrorResponse },
    /// A non-2xx status without a parsable error body (empty 404, HTML 502, a 3xx: never followed).
    #[error("http {0}")]
    Status(u16),
    /// A 2xx whose body is unreadable: only the kind of failure (`syntax`, `schema`, `truncated`, `io`) and the body
    /// size are kept, never the parser's text, which echoes the input.
    #[error("bad body: {kind}, {len} bytes")]
    Body { kind: &'static str, len: usize },
    /// A body bigger than `MAX_RESPONSE_BYTES`: the declared `content-length`, or the bytes received when the
    /// stream was cut (a lower bound).
    #[error("response too large: {0} bytes (max {MAX_RESPONSE_BYTES})")]
    TooLarge(u64),
    /// Connection refused or reset, TLS failure, client-side timeout. Never contains the URL.
    #[error("transport: {0}")]
    Transport(String),
    /// Signing failed (e.g. AWS KMS unreachable); nothing was sent.
    #[error("signer: {0}")]
    Signer(String),
}

impl AttemptError {
    /// The node rejected our credential: never retried.
    pub fn is_auth(&self) -> bool {
        match self {
            Self::Api { error, .. } => error.code == ErrorCode::SenderAuthenticationFailed,
            Self::Status(status) => *status == 401,
            Self::Body { .. } | Self::TooLarge(_) | Self::Transport(_) | Self::Signer(_) => false,
        }
    }

    /// The connector's own table (`ErrorCode::retryable`); unknown codes follow the body's flag; a bare status is
    /// retried for 408, 429 and 5xx.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Api { error, .. } if error.code == ErrorCode::Unknown => error.retryable,
            Self::Api { error, .. } => error.code.retryable(),
            Self::Status(status) => matches!(status, 408 | 429) || (500..600).contains(status),
            Self::Transport(_) | Self::Signer(_) => true,
            Self::Body { .. } | Self::TooLarge(_) => false,
        }
    }

    /// Which retry budget a retryable failure consumes: a rate limit (`rate_limited`, a bare 429) has its own.
    pub fn retry_class(&self) -> RetryClass {
        match self {
            Self::Api { error, .. } if error.code == ErrorCode::RateLimited => {
                RetryClass::RateLimit
            }
            Self::Status(429) => RetryClass::RateLimit,
            _ => RetryClass::Transient,
        }
    }

    /// The connector code reported upward as the dominant error: transport and bare 5xx count as
    /// `upstream_transient`, a bare 401 as `sender_authentication_failed`, a bare 429 as `rate_limited` (an ingress
    /// or a future proxy rate limit answers without the connector body), a signing failure (ours, not the node's) as
    /// `unknown`.
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Api { error, .. } => error.code,
            Self::Status(401) => ErrorCode::SenderAuthenticationFailed,
            Self::Status(429) => ErrorCode::RateLimited,
            Self::Status(_) | Self::Transport(_) => ErrorCode::UpstreamTransient,
            Self::Body { .. } | Self::TooLarge(_) | Self::Signer(_) => ErrorCode::Unknown,
        }
    }
}

/// Longest KMS node error message a log line carries, in characters: a node cannot flood the logs.
pub const MAX_LOGGED_NODE_MESSAGE: usize = 256;

impl AttemptError {
    /// What a log line says about the failure: the node's error code and truncated message, or the kind of
    /// failure. Never a body: a malformed 2xx body is reported by its kind and size only.
    pub fn digest(&self) -> String {
        match self {
            Self::Api { status, error } => format!(
                "{} (http {status}): {}",
                error.code.as_str(),
                error
                    .message
                    .chars()
                    .take(MAX_LOGGED_NODE_MESSAGE)
                    .collect::<String>()
            ),
            other => other.to_string(),
        }
    }
}

/// The transport seam: the real client and the test mock both produce raw bytes for `decode`.
#[async_trait]
pub trait ConnectorClient: Send + Sync + 'static {
    async fn post(
        &self,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
        authorization: &HeaderValue,
    ) -> Result<HttpReply, AttemptError>;
}

/// 2xx → DTO; otherwise the connector's error body when present, else the bare status.
pub fn decode<R: DeserializeOwned>(reply: HttpReply) -> Result<R, AttemptError> {
    if (200..300).contains(&reply.status) {
        // serde echoes the offending input in its message: keep the kind of failure only.
        return serde_json::from_slice(&reply.body).map_err(|e| AttemptError::Body {
            kind: match e.classify() {
                serde_json::error::Category::Io => "io",
                serde_json::error::Category::Syntax => "syntax",
                serde_json::error::Category::Data => "schema",
                serde_json::error::Category::Eof => "truncated",
            },
            len: reply.body.len(),
        });
    }
    match serde_json::from_slice::<ErrorResponse>(&reply.body) {
        Ok(error) => Err(AttemptError::Api {
            status: reply.status,
            error,
        }),
        Err(_) => Err(AttemptError::Status(reply.status)),
    }
}

/// One pooled reqwest client for every node.
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    /// `https_only` unless some endpoint is plain http (validated: loopback or `allow_insecure_http`); certificates
    /// verified unless `skip_tls_verification`.
    pub fn new(cfg: &KmsAggregatorConfig) -> Result<Self, ConfigError> {
        let https_only = cfg.endpoints.iter().all(|e| e.url.scheme() == "https");
        if cfg.skip_tls_verification {
            tracing::warn!(
                "kms_aggregator.skip_tls_verification is set: connector TLS certificates are not verified"
            );
        }
        let inner = reqwest::Client::builder()
            .connect_timeout(CONNECT_TIMEOUT.min(cfg.call.timeout))
            // Safety net for a socket that stops answering; the aggregator's deadline is the decision.
            .timeout(cfg.call.timeout)
            // Never forward the signed header to another host.
            .redirect(reqwest::redirect::Policy::none())
            .https_only(https_only)
            .danger_accept_invalid_certs(cfg.skip_tls_verification)
            .no_proxy()
            .pool_idle_timeout(Duration::from_secs(10))
            .tcp_keepalive(Duration::from_secs(15))
            .user_agent(concat!("zama-relayer-http/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| ConfigError(format!("http client: {}", with_causes(&e))))?;
        Ok(Self { inner })
    }
}

/// An error and its chain of causes, joined with `: ` (reqwest's own message is only "builder error").
fn with_causes(error: &dyn std::error::Error) -> String {
    let mut text = error.to_string();
    let mut cause = error.source();
    while let Some(inner) = cause {
        text.push_str(": ");
        text.push_str(&inner.to_string());
        cause = inner.source();
    }
    text
}

#[async_trait]
impl ConnectorClient for HttpClient {
    async fn post(
        &self,
        endpoint: &Endpoint,
        route: &str,
        body: Bytes,
        authorization: &HeaderValue,
    ) -> Result<HttpReply, AttemptError> {
        let mut response = self
            .inner
            .post(join_route(&endpoint.url, route))
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, authorization.clone())
            .body(body)
            .send()
            .await
            .map_err(transport)?;
        let status = response.status().as_u16();
        let retry_after = retry_after(response.headers());
        if let Some(len) = response
            .content_length()
            .filter(|len| *len > MAX_RESPONSE_BYTES as u64)
        {
            return Err(AttemptError::TooLarge(len));
        }
        let mut buf = BytesMut::new();
        while let Some(chunk) = response.chunk().await.map_err(transport)? {
            let received = buf.len().saturating_add(chunk.len());
            if received > MAX_RESPONSE_BYTES {
                return Err(AttemptError::TooLarge(received as u64));
            }
            buf.extend_from_slice(&chunk);
        }
        Ok(HttpReply {
            status,
            body: buf.freeze(),
            retry_after,
        })
    }
}

/// `Retry-After` as delta-seconds (the form the connector proxy sends), at most `MAX_TIMEOUT`. Any other value
/// (an HTTP date, garbage, a huge number) is ignored: the configured floor applies. The raw header is never logged;
/// the parsed wait is what the call reports as `retry_after_ms`.
fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?;
    let seconds = value
        .to_str()
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .map(Duration::from_secs)
        .filter(|d| *d <= MAX_TIMEOUT);
    if seconds.is_none() {
        tracing::debug!("retry-after header ignored: not delta-seconds within {MAX_TIMEOUT:?}");
    }
    seconds
}

/// `https://host:8443` + `/v1/user-decrypt` → `https://host:8443/v1/user-decrypt` (a base path is kept).
fn join_route(base: &Url, route: &str) -> Url {
    let mut url = base.clone();
    let path = format!(
        "{}/{}",
        url.path().trim_end_matches('/'),
        route.trim_start_matches('/')
    );
    url.set_path(&path);
    url
}

/// reqwest errors without their URL, classified for the logs.
fn transport(e: reqwest::Error) -> AttemptError {
    let e = e.without_url();
    let kind = if e.is_timeout() {
        "timeout"
    } else if e.is_connect() {
        "connect"
    } else {
        "request"
    };
    AttemptError::Transport(format!("{kind}: {e}"))
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Address, B256};
    use kms_connector_api::UserDecryptionResponse;
    use wiremock::matchers::{body_bytes, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::kms_aggregator::config::tests::valid;
    use crate::kms_aggregator::mock::{self, Fixed};
    use crate::metrics::connector_codes;

    /// Route and body of the user-decrypt fixture.
    const USER: (&str, &[u8]) = (
        kms_connector_api::USER_DECRYPTION_ROUTE,
        mock::USER_REQUEST_JSON.as_bytes(),
    );

    fn api(code: ErrorCode, retryable: bool) -> AttemptError {
        AttemptError::Api {
            status: code.http_status(),
            error: ErrorResponse {
                code,
                message: "m".into(),
                retryable,
                decryption_id: None,
            },
        }
    }

    #[test]
    fn decode_every_error_code() {
        for code in connector_codes() {
            let reply = mock::error_reply(USER.0, USER.1, code);
            let err = decode::<UserDecryptionResponse>(reply).unwrap_err();
            let AttemptError::Api { status, error } = &err else {
                panic!("{code:?}: {err}");
            };
            assert_eq!(*status, code.http_status());
            assert_eq!(error.code, code);
            assert_eq!(error.retryable, code.retryable());
            assert_eq!(
                error.decryption_id.is_none(),
                matches!(
                    code,
                    ErrorCode::Malformed | ErrorCode::UnsupportedAttestationType
                ),
                "{code:?}"
            );
        }
    }

    #[test]
    fn decode_success_and_garbage() {
        let reply = mock::ok_reply(USER.0, USER.1, 3, Fixed::Ok);
        let response = decode::<UserDecryptionResponse>(reply).unwrap();
        assert_eq!(response.signature.len(), 65);

        let garbage = HttpReply {
            status: 200,
            body: Bytes::from(format!("\"{}\"", "x".repeat(10_000))),
            retry_after: None,
        };
        // A string where an object is expected: the schema is wrong, and the text keeps none of the 10 000 bytes.
        let error = decode::<UserDecryptionResponse>(garbage).unwrap_err();
        assert_eq!(
            error,
            AttemptError::Body {
                kind: "schema",
                len: 10_002
            }
        );
        assert_eq!(error.to_string(), "bad body: schema, 10002 bytes");
        for (body, kind) in [("{ not json", "syntax"), ("{\"a\":", "truncated")] {
            let reply = HttpReply {
                status: 200,
                body: Bytes::from(body),
                retry_after: None,
            };
            let AttemptError::Body { kind: got, .. } =
                decode::<UserDecryptionResponse>(reply).unwrap_err()
            else {
                panic!("expected Body");
            };
            assert_eq!(got, kind, "{body}");
        }
    }

    #[test]
    fn decode_without_error_body_is_the_bare_status() {
        for (status, body) in [(404, ""), (502, "<html>bad gateway</html>"), (307, "")] {
            let reply = HttpReply {
                status,
                body: Bytes::from(body),
                retry_after: None,
            };
            assert_eq!(
                decode::<UserDecryptionResponse>(reply).unwrap_err(),
                AttemptError::Status(status)
            );
        }
    }

    #[test]
    fn retry_class_is_rate_limit_for_429_only() {
        for code in connector_codes() {
            let expected = if code == ErrorCode::RateLimited {
                RetryClass::RateLimit
            } else {
                RetryClass::Transient
            };
            assert_eq!(api(code, true).retry_class(), expected, "{code:?}");
        }
        assert_eq!(
            AttemptError::Status(429).retry_class(),
            RetryClass::RateLimit
        );
        for status in [401, 408, 500, 503] {
            assert_eq!(
                AttemptError::Status(status).retry_class(),
                RetryClass::Transient
            );
        }
        assert_eq!(
            AttemptError::Transport("x".into()).retry_class(),
            RetryClass::Transient
        );
        assert_eq!(
            AttemptError::Signer("x".into()).retry_class(),
            RetryClass::Transient
        );
    }

    #[test]
    fn retry_after_reads_delta_seconds_only() {
        let parse = |value: &str| {
            let mut headers = HeaderMap::new();
            headers.insert(RETRY_AFTER, HeaderValue::from_str(value).unwrap());
            retry_after(&headers)
        };
        assert_eq!(parse("3"), Some(Duration::from_secs(3)));
        assert_eq!(parse(" 1 "), Some(Duration::from_secs(1)));
        assert_eq!(parse("0"), Some(Duration::ZERO));
        assert_eq!(parse("60"), Some(MAX_TIMEOUT));
        for ignored in [
            "61",
            "-1",
            "abc",
            "",
            "1.5",
            "Wed, 21 Oct 2015 07:28:00 GMT",
        ] {
            assert_eq!(parse(ignored), None, "{ignored:?}");
        }
        assert_eq!(retry_after(&HeaderMap::new()), None);
    }

    #[test]
    fn is_auth_cases() {
        assert!(api(ErrorCode::SenderAuthenticationFailed, false).is_auth());
        assert!(AttemptError::Status(401).is_auth());
        // An expired header is re-signed and retried, not a wrong key.
        assert!(!api(ErrorCode::AuthExpired, true).is_auth());
        assert!(!AttemptError::Signer("x".into()).is_auth());
        assert!(!api(ErrorCode::AclDenied, true).is_auth());
        assert!(!AttemptError::Status(403).is_auth());
        assert!(!AttemptError::Transport("x".into()).is_auth());
        assert!(
            !AttemptError::Body {
                kind: "syntax",
                len: 1
            }
            .is_auth()
        );
        assert!(!AttemptError::TooLarge(1).is_auth());
    }

    #[test]
    fn is_retryable_follows_the_connector_table() {
        for code in connector_codes() {
            // The body flag is ignored for known codes and followed for `unknown`.
            let flipped = !code.retryable();
            let expected = if code == ErrorCode::Unknown {
                flipped
            } else {
                code.retryable()
            };
            assert_eq!(api(code, flipped).is_retryable(), expected, "{code:?}");
        }
        assert!(AttemptError::Status(408).is_retryable());
        assert!(AttemptError::Status(429).is_retryable());
        assert!(AttemptError::Status(503).is_retryable());
        assert!(!AttemptError::Status(404).is_retryable());
        assert!(!AttemptError::Status(307).is_retryable());
        assert!(AttemptError::Transport("x".into()).is_retryable());
        assert!(AttemptError::Signer("x".into()).is_retryable());
        assert!(
            !AttemptError::Body {
                kind: "syntax",
                len: 1
            }
            .is_retryable()
        );
        assert!(!AttemptError::TooLarge(1).is_retryable());
    }

    #[test]
    fn code_mapping() {
        assert_eq!(api(ErrorCode::AclDenied, true).code(), ErrorCode::AclDenied);
        assert_eq!(
            AttemptError::Status(401).code(),
            ErrorCode::SenderAuthenticationFailed
        );
        assert_eq!(AttemptError::Status(429).code(), ErrorCode::RateLimited);
        assert_eq!(
            AttemptError::Status(502).code(),
            ErrorCode::UpstreamTransient
        );
        assert_eq!(
            AttemptError::Transport("x".into()).code(),
            ErrorCode::UpstreamTransient
        );
        assert_eq!(
            AttemptError::Body {
                kind: "syntax",
                len: 1
            }
            .code(),
            ErrorCode::Unknown
        );
        assert_eq!(AttemptError::TooLarge(1).code(), ErrorCode::Unknown);
        assert_eq!(AttemptError::Signer("x".into()).code(), ErrorCode::Unknown);
    }

    #[test]
    fn join_route_keeps_a_base_path() {
        let cases = [
            (
                "https://kms.example.net:8443",
                "https://kms.example.net:8443/v1/user-decrypt",
            ),
            (
                "https://kms.example.net/connector",
                "https://kms.example.net/connector/v1/user-decrypt",
            ),
            (
                "https://kms.example.net/connector/",
                "https://kms.example.net/connector/v1/user-decrypt",
            ),
            (
                "http://localhost:3002",
                "http://localhost:3002/v1/user-decrypt",
            ),
        ];
        for (base, expected) in cases {
            let url = join_route(&Url::parse(base).unwrap(), "/v1/user-decrypt");
            assert_eq!(url.as_str(), expected);
        }
    }

    #[test]
    fn endpoint_from_config() {
        let endpoint = Endpoint::from(&valid(1).endpoints[0]);
        assert_eq!(endpoint.name, "kms_00");
        assert_eq!(endpoint.audience, Address::repeat_byte(1));
        let printed = format!("{endpoint:?}");
        assert!(printed.contains("kms_00") && printed.contains("kms-00.example.net"));
    }

    #[test]
    fn http_reply_debug_hides_the_body() {
        let reply = HttpReply {
            status: 200,
            body: Bytes::from_static(b"secret share bytes"),
            retry_after: None,
        };
        assert_eq!(
            format!("{reply:?}"),
            "HttpReply { status: 200, body_len: 18 }"
        );
    }

    #[test]
    fn with_causes_joins_the_chain() {
        #[derive(Debug)]
        struct Outer(std::io::Error);
        impl std::fmt::Display for Outer {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("builder error")
            }
        }
        impl std::error::Error for Outer {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(&self.0)
            }
        }
        let outer = Outer(std::io::Error::other("no CA certificates"));
        assert_eq!(with_causes(&outer), "builder error: no CA certificates");
        assert_eq!(with_causes(&std::io::Error::other("alone")), "alone");
    }

    #[test]
    fn http_client_builds_from_config() {
        HttpClient::new(&valid(3)).unwrap();
        let mut cfg = valid(1);
        cfg.endpoints[0].url = Url::parse("http://localhost:3002").unwrap();
        HttpClient::new(&cfg).unwrap();
    }

    // ------------------------------------------------------------ wire tests (real sockets, no paused time)

    fn client() -> HttpClient {
        let mut cfg = valid(1);
        cfg.endpoints[0].url = Url::parse("http://localhost:1").unwrap();
        cfg.call.timeout = Duration::from_secs(2);
        HttpClient::new(&cfg).unwrap()
    }

    /// A stand-in `Authorization` value: the client sends whatever it is given.
    const AUTH: &str = "FhevmSig expires=1, sig=0x00";

    fn endpoint(server: &MockServer) -> Endpoint {
        Endpoint {
            name: "node".into(),
            url: Url::parse(&server.uri()).unwrap(),
            audience: Address::repeat_byte(1),
        }
    }

    async fn post(server: &MockServer) -> Result<HttpReply, AttemptError> {
        client()
            .post(
                &endpoint(server),
                kms_connector_api::USER_DECRYPTION_ROUTE,
                Bytes::from_static(mock::USER_REQUEST_JSON.as_bytes()),
                &HeaderValue::from_static(AUTH),
            )
            .await
    }

    #[tokio::test]
    async fn wire_sends_auth_header_content_type_and_exact_body() {
        let server = MockServer::start().await;
        let reply = mock::ok_reply(
            kms_connector_api::USER_DECRYPTION_ROUTE,
            mock::USER_REQUEST_JSON.as_bytes(),
            0,
            Fixed::Ok,
        );
        Mock::given(method("POST"))
            .and(path("/v1/user-decrypt"))
            // Exact bytes: wiremock's `header` matcher splits values on commas.
            .and(|request: &wiremock::Request| {
                request.headers.get("authorization").map(|v| v.as_bytes()) == Some(AUTH.as_bytes())
            })
            .and(header("content-type", "application/json"))
            .and(body_bytes(mock::USER_REQUEST_JSON.as_bytes().to_vec()))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(reply.body.to_vec()))
            .expect(1)
            .mount(&server)
            .await;
        let reply = post(&server).await.unwrap();
        let response = decode::<UserDecryptionResponse>(reply).unwrap();
        assert_eq!(response.decryption_id, B256::from(mock::USER_REQUEST_ID));
    }

    #[tokio::test]
    async fn wire_error_body_is_decoded() {
        let server = MockServer::start().await;
        let error = mock::error_reply(
            kms_connector_api::USER_DECRYPTION_ROUTE,
            mock::USER_REQUEST_JSON.as_bytes(),
            ErrorCode::AclDenied,
        );
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(403).set_body_bytes(error.body.to_vec()))
            .mount(&server)
            .await;
        let err = decode::<UserDecryptionResponse>(post(&server).await.unwrap()).unwrap_err();
        assert_eq!(err.code(), ErrorCode::AclDenied);
        assert!(err.is_retryable());
    }

    #[tokio::test]
    async fn wire_retry_after_is_read_on_a_429_and_ignored_elsewhere() {
        let server = MockServer::start().await;
        let error = mock::error_reply(
            kms_connector_api::USER_DECRYPTION_ROUTE,
            mock::USER_REQUEST_JSON.as_bytes(),
            ErrorCode::RateLimited,
        );
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "2")
                    .set_body_bytes(error.body.to_vec()),
            )
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).insert_header("retry-after", "2"))
            .mount(&server)
            .await;
        let limited = post(&server).await.unwrap();
        assert_eq!(
            (limited.status, limited.retry_after),
            (429, Some(Duration::from_secs(2)))
        );
        let err = decode::<UserDecryptionResponse>(limited).unwrap_err();
        assert_eq!(err.retry_class(), RetryClass::RateLimit);
        // The header is read whatever the status; what to do with it is the caller's decision.
        let ok = post(&server).await.unwrap();
        assert_eq!(
            (ok.status, ok.retry_after),
            (200, Some(Duration::from_secs(2)))
        );
    }

    #[tokio::test]
    async fn wire_redirects_are_not_followed() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(307).insert_header("location", "http://localhost:1/"),
            )
            .mount(&server)
            .await;
        assert_eq!(post(&server).await.unwrap().status, 307);
    }

    #[tokio::test]
    async fn wire_oversized_body_is_rejected() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200).set_body_bytes(vec![b'x'; MAX_RESPONSE_BYTES + 1]),
            )
            .mount(&server)
            .await;
        let err = post(&server).await.unwrap_err();
        assert_eq!(err, AttemptError::TooLarge(MAX_RESPONSE_BYTES as u64 + 1));
        assert_eq!(
            err.to_string(),
            "response too large: 4194305 bytes (max 4194304)"
        );
    }

    #[tokio::test]
    async fn wire_connection_refused_is_transport() {
        // Port 1 is never listening.
        let ep = Endpoint {
            name: "node".into(),
            url: Url::parse("http://127.0.0.1:1").unwrap(),
            audience: Address::repeat_byte(1),
        };
        let err = client()
            .post(
                &ep,
                kms_connector_api::USER_DECRYPTION_ROUTE,
                Bytes::new(),
                &HeaderValue::from_static(AUTH),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AttemptError::Transport(_)), "{err}");
        assert!(err.is_retryable());
    }

    /// An https server on loopback with a self-signed certificate; answers one request with 200 and an empty body.
    async fn self_signed_server() -> Url {
        use tokio::io::AsyncWriteExt;
        use tokio_rustls::rustls::{self, pki_types::pem::PemObject};
        let cert = rustls::pki_types::CertificateDer::from_pem_slice(include_bytes!(
            "testdata/self_signed.crt"
        ));
        let key = rustls::pki_types::PrivateKeyDer::from_pem_slice(include_bytes!(
            "testdata/self_signed.key"
        ));
        let tls = rustls::ServerConfig::builder_with_provider(
            rustls::crypto::aws_lc_rs::default_provider().into(),
        )
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(vec![cert.unwrap()], key.unwrap())
        .unwrap();
        let acceptor = tokio_rustls::TlsAcceptor::from(std::sync::Arc::new(tls));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            let (tcp, _) = listener.accept().await.unwrap();
            // A rejected handshake ends here: the client closed the connection.
            if let Ok(mut stream) = acceptor.accept(tcp).await {
                let reply = b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\n\r\n";
                stream.write_all(reply).await.unwrap();
                stream.shutdown().await.unwrap();
            }
        });
        Url::parse(&format!("https://localhost:{port}")).unwrap()
    }

    /// One post to a fresh self-signed server, with `skip_tls_verification` set as given.
    async fn post_self_signed(skip_tls_verification: bool) -> Result<HttpReply, AttemptError> {
        let mut cfg = valid(1);
        cfg.endpoints[0].url = self_signed_server().await;
        cfg.call.timeout = Duration::from_secs(2);
        cfg.skip_tls_verification = skip_tls_verification;
        HttpClient::new(&cfg)
            .unwrap()
            .post(
                &Endpoint::from(&cfg.endpoints[0]),
                kms_connector_api::USER_DECRYPTION_ROUTE,
                Bytes::new(),
                &HeaderValue::from_static(AUTH),
            )
            .await
    }

    #[tokio::test]
    async fn wire_self_signed_certificate_rejected_by_default() {
        let err = post_self_signed(false).await.unwrap_err();
        assert!(err.to_string().contains("connect:"), "{err}");
    }

    #[tokio::test]
    async fn wire_self_signed_certificate_accepted_when_opted_in() {
        assert_eq!(post_self_signed(true).await.unwrap().status, 200);
    }

    #[test]
    fn digest_keeps_the_nodes_code_and_a_truncated_message_never_a_body() {
        let api = AttemptError::Api {
            status: 403,
            error: ErrorResponse::new(ErrorCode::AclDenied, "x".repeat(1000), None),
        };
        let digest = api.digest();
        assert!(digest.starts_with("acl_denied (http 403): xxx"));
        assert_eq!(
            digest.chars().count(),
            "acl_denied (http 403): ".len() + MAX_LOGGED_NODE_MESSAGE
        );
        assert_eq!(
            AttemptError::Body {
                kind: "syntax",
                len: 12
            }
            .digest(),
            "bad body: syntax, 12 bytes"
        );
        assert_eq!(AttemptError::Status(502).digest(), "http 502");
    }
}
