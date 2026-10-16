//! The Pingora [`ProxyHttp`] implementation of [`Proxy`]: the request lifecycle of the proxy.

use crate::core::{ErrorCode, ErrorResponse, Proxy, Route, RouteError, match_route};
use alloy::primitives::{Address, B256, KECCAK256_EMPTY, Keccak256};
use async_trait::async_trait;
use bytes::Bytes;
use fhevm_sig_auth::AuthError;
use http::{
    HeaderValue, StatusCode,
    header::{
        ALLOW, AUTHORIZATION, CACHE_CONTROL, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, FORWARDED,
        HeaderName, PROXY_AUTHENTICATE, PROXY_AUTHORIZATION, TE, TRAILER, UPGRADE, VIA,
        WWW_AUTHENTICATE,
    },
};
use pingora::{
    Error, ErrorSource, ErrorType,
    http::{RequestHeader, ResponseHeader},
    prelude::HttpPeer,
    protocols::http::ServerSession,
    proxy::{FailToProxy, ProxyHttp, Session},
};
use prometheus::{IntCounterVec, register_int_counter_vec};
use std::{sync::LazyLock, time::Instant};
use tracing::{error, info};

static REQUESTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_proxy_requests_total",
        "Requests answered by the proxy, by route and status",
        &["route", "status"]
    )
    .unwrap()
});

// The `X-Forwarded-*` headers set by the proxy (any inbound value is dropped first).
const X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_static("x-forwarded-proto");
// Client-supplied forwarding header the proxy never sets but must not forward either.
const X_FORWARDED_HOST: HeaderName = HeaderName::from_static("x-forwarded-host");
// Hop-by-hop headers without a constant in the `http` crate.
const KEEP_ALIVE: HeaderName = HeaderName::from_static("keep-alive");
const PROXY_CONNECTION: HeaderName = HeaderName::from_static("proxy-connection");

const NO_STORE: &str = "no-store";

/// Error message for oversized request bodies.
const BODY_TOO_LARGE: &str = "request body too large";
const NOT_ORIGIN_FORM: &str = "request target must be an origin-form UTF-8 path";

/// Per-request context.
pub struct RequestContext {
    started_at: Instant,
    route: Option<Route>,
    endpoint: Option<String>,
    /// The sender authenticated by the `FhevmSig` header.
    sender: Option<Address>,
    /// The body digest signed in the `FhevmSig` header.
    body_digest: B256,
    /// The `keccak256` of the body received so far.
    body_hasher: Keccak256,
    /// The number of body bytes received.
    body_len: usize,
}

impl Proxy {
    /// Writes an error body as JSON. It must not be cached (RFC-038: only `200` answers may be).
    async fn respond_with_error(
        &self,
        session: &mut Session,
        error: ErrorResponse,
        allow: Option<&str>,
    ) -> pingora::Result<()> {
        let status = StatusCode::from_u16(error.code.http_status())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = serde_json::to_vec(&error).unwrap_or_default();
        let mut header = ResponseHeader::build(status, None)?;
        header.insert_header(CONTENT_TYPE, "application/json")?;
        header.insert_header(CONTENT_LENGTH, body.len().to_string())?;
        header.insert_header(CACHE_CONTROL, NO_STORE)?;
        if status == StatusCode::UNAUTHORIZED {
            header.insert_header(WWW_AUTHENTICATE, fhevm_sig_auth::SCHEME)?;
        }
        if let Some(methods) = allow {
            header.insert_header(ALLOW, methods)?;
        }
        session
            .write_response_header(Box::new(header), false)
            .await?;
        session
            .write_response_body(Some(Bytes::from(body)), true)
            .await
    }
}

/// Empty key for the `.select` call (unused by the function).
const SELECT_KEY: &[u8] = b"";
/// Maximum number of iterations for the `.select` call.
const MAX_SELECT_ITERATIONS: usize = 256;

#[async_trait]
impl ProxyHttp for Proxy {
    type CTX = RequestContext;

    fn new_ctx(&self) -> Self::CTX {
        RequestContext {
            started_at: Instant::now(),
            route: None,
            endpoint: None,
            sender: None,
            body_digest: B256::ZERO, // `ZERO` (matching no body) until it's parsed from header
            body_hasher: Keccak256::new(),
            body_len: 0,
        }
    }

    async fn early_request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        session.set_read_timeout(Some(self.config.request_read_timeout));
        Ok(())
    }

    // Request target (400), route (404/405), `FhevmSig` header (401), body cap (400), all before
    // any endpoint connection. A request without a body is fully authenticated here.
    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        let req = session.req_header();

        // Pingora forwards the raw target, but its URI renders an absolute-form or non-UTF-8 target
        // lossily: only a target identical to what reaches the endpoint can be authenticated.
        let Some(target) = req
            .uri
            .path_and_query()
            .filter(|target| target.as_str().as_bytes() == req.raw_path())
        else {
            let error = ErrorResponse::new(ErrorCode::Malformed, NOT_ORIGIN_FORM);
            self.respond_with_error(session, error, None).await?;
            return Ok(true);
        };

        let route = match match_route(&req.method, target.path()) {
            Ok(route) => route,
            Err(e) => {
                self.respond_with_error(session, route_error_response(&e), e.allow_header())
                    .await?;
                return Ok(true);
            }
        };
        ctx.route = Some(route);

        let authorization = req.headers.get(AUTHORIZATION).map(HeaderValue::as_bytes);
        let (sender, body_digest) = match self.verifier.verify_header(authorization, target) {
            Ok(signed) => signed,
            Err(e) => {
                self.respond_with_error(session, auth_error_response(&e), None)
                    .await?;
                return Ok(true);
            }
        };
        ctx.sender = Some(sender);

        let declared_length = req
            .headers
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<usize>().ok());
        if declared_length.is_some_and(|len| len > self.config.max_body_bytes) {
            let error = ErrorResponse::new(ErrorCode::Malformed, BODY_TOO_LARGE);
            self.respond_with_error(session, error, None).await?;
            return Ok(true);
        }

        if session.is_body_empty() && KECCAK256_EMPTY != body_digest {
            self.respond_with_error(session, auth_error_response(&AuthError::BodyMismatch), None)
                .await?;
            return Ok(true);
        }
        ctx.body_digest = body_digest;

        Ok(false)
    }

    // Streams the body, within the cap, while hashing it. It is checked against the signed
    // digest before its last chunk is written, so the endpoint never receives a whole body that
    // does not match: it gets a truncated request and a closed connection instead.
    //
    // Both errors are the client's. Pingora's HTTP/2 upstream path ends the request only on a
    // downstream error: any other one keeps waiting for the endpoint's response.
    async fn request_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<Bytes>,
        end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        if let Some(chunk) = body {
            ctx.body_len += chunk.len();
            if ctx.body_len > self.config.max_body_bytes {
                return abort_with(ErrorResponse::new(ErrorCode::Malformed, BODY_TOO_LARGE))
                    .map_err(Error::into_down);
            }
            ctx.body_hasher.update(chunk);
        }
        if end_of_stream && ctx.body_hasher.clone().finalize() != ctx.body_digest {
            return abort_with(auth_error_response(&AuthError::BodyMismatch))
                .map_err(Error::into_down);
        }
        Ok(())
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let Some(backend) = self
            .endpoint_balancer
            .select(SELECT_KEY, MAX_SELECT_ITERATIONS)
        else {
            return abort_with(ErrorResponse::new(
                ErrorCode::UpstreamTransient,
                "endpoint unavailable",
            ));
        };
        ctx.endpoint = Some(backend.addr.to_string());

        let mut peer = HttpPeer::new(backend.addr, false, String::new());
        peer.options.connection_timeout = Some(self.config.endpoint_connect_timeout);
        peer.options.total_connection_timeout = Some(self.config.endpoint_connect_timeout);
        peer.options.read_timeout = Some(self.config.endpoint_response_timeout);
        peer.options.write_timeout = Some(self.config.endpoint_response_timeout);
        peer.options.idle_timeout = Some(self.config.endpoint_idle_timeout);
        // HTTP/2 prior knowledge (h2c), so the requests an endpoint holds share few connections.
        peer.options.set_http_version(2, 2);
        peer.options.max_h2_streams = self.config.endpoint_max_h2_streams;
        Ok(Box::new(peer))
    }

    // Strips the forwarding and hop-by-hop headers before forwarding. The `FhevmSig` header is
    // kept: the endpoint checks it again. The body is never touched.
    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        sanitize_headers(upstream_request);
        if let Some(addr) = session.client_addr().and_then(|a| a.as_inet()) {
            upstream_request.insert_header(X_FORWARDED_FOR, addr.ip().to_string())?;
        }
        upstream_request.insert_header(X_FORWARDED_PROTO, "https")?;
        Ok(())
    }

    async fn fail_to_proxy(
        &self,
        session: &mut Session,
        e: &Error,
        _ctx: &mut Self::CTX,
    ) -> FailToProxy {
        // The connection is closed after the answer (`can_reuse_downstream: false`), so the answer
        // says `Connection: close`, as Pingora's own `write_error_response` does: a client must not
        // send its next request on it.
        session.set_keepalive(None);
        let error_code = match e.root_cause().downcast_ref::<ErrorResponse>() {
            Some(error) => {
                let status = StatusCode::from_u16(error.code.http_status())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                if let Err(e) = self.respond_with_error(session, error.clone(), None).await {
                    error!("Failed to send error response to downstream: {e}");
                }
                status.as_u16()
            }
            // An upstream failure (unreachable endpoint, timeout...) gets an `ErrorResponse` body.
            // The detailed Pingora error (which includes the endpoint's address) is only logged
            // server-side by `logging()`; the client, e.g. the relayer, gets a fixed message.
            None if *e.esource() == ErrorSource::Upstream => {
                let (code, message) = match e.etype() {
                    ErrorType::ConnectTimedout | ErrorType::ReadTimedout => {
                        (ErrorCode::Timeout, "endpoint timed out")
                    }
                    _ => (ErrorCode::UpstreamTransient, "endpoint unavailable"),
                };
                let error = ErrorResponse::new(code, message);
                let status = StatusCode::from_u16(error.code.http_status())
                    .unwrap_or(StatusCode::BAD_GATEWAY);
                if let Err(e) = self.respond_with_error(session, error, None).await {
                    error!("Failed to send error response to downstream: {e}");
                }
                status.as_u16()
            }
            None => {
                let code = default_error_code(e);
                if code > 0 {
                    let mut header = ServerSession::generate_error(code);
                    let written = match header.insert_header(CACHE_CONTROL, NO_STORE) {
                        Ok(()) => session.write_error_response(header, Bytes::new()).await,
                        Err(e) => Err(e),
                    };
                    if let Err(e) = written {
                        error!("Failed to send error response to downstream: {e}");
                    }
                }
                code
            }
        };
        FailToProxy {
            error_code,
            can_reuse_downstream: false,
        }
    }

    // Structured access log. Never logs bodies nor credentials.
    async fn logging(&self, session: &mut Session, e: Option<&Error>, ctx: &mut Self::CTX) {
        let status = session
            .response_written()
            .map(|resp| resp.status.as_u16())
            .unwrap_or(0);
        let req = session.req_header();
        let client = session
            .client_addr()
            .map(ToString::to_string)
            .unwrap_or_default();

        let route = ctx.route.map_or("none", Route::as_str);
        REQUESTS
            .with_label_values(&[route, &status.to_string()])
            .inc();

        info!(
            path = req.uri.path(),
            status,
            client,
            endpoint = ctx.endpoint.as_deref().unwrap_or(""),
            sender = ctx.sender.map(|s| s.to_string()).unwrap_or_default(),
            duration_ms = ctx.started_at.elapsed().as_millis() as u64,
            error = e.map(|e| e.to_string()).unwrap_or_default(),
            "Proxied request"
        );
    }
}

/// Converts an `AuthError` into an `ErrorResponse` with its `auth_*` code. Only an expired header is
/// worth retrying, after re-signing.
fn auth_error_response(error: &AuthError) -> ErrorResponse {
    let code = ErrorCode::Auth {
        code: error.code(),
        retryable: matches!(error, AuthError::Expired),
    };
    ErrorResponse::new(code, error.to_string())
}

/// Converts a `RouteError` into an `ErrorResponse`.
fn route_error_response(error: &RouteError) -> ErrorResponse {
    match error {
        RouteError::NotFound => ErrorResponse::new(ErrorCode::NotFound, "unknown route"),
        RouteError::MethodNotAllowed(methods) => ErrorResponse::new(
            ErrorCode::MethodNotAllowed,
            format!("method not allowed, expected {methods}"),
        ),
    }
}

/// Aborts the proxying with an `ErrorResponse`.
///
/// The [`ErrorResponse`] is kept as the cause so `fail_to_proxy` can send it back to the client.
fn abort_with<T>(error: ErrorResponse) -> pingora::Result<T> {
    let etype = ErrorType::HTTPStatus(error.code.http_status());
    Error::e_because(etype, error.to_string(), error)
}

/// The status Pingora's default `fail_to_proxy` would answer.
fn default_error_code(e: &Error) -> u16 {
    match (e.etype(), e.esource()) {
        (ErrorType::HTTPStatus(code), _) => *code,
        (_, ErrorSource::Upstream) => StatusCode::BAD_GATEWAY.as_u16(),
        (
            ErrorType::WriteError | ErrorType::ReadError | ErrorType::ConnectionClosed,
            ErrorSource::Downstream,
        ) => 0,
        (_, ErrorSource::Downstream) => StatusCode::BAD_REQUEST.as_u16(),
        (_, ErrorSource::Internal | ErrorSource::Unset) => {
            StatusCode::INTERNAL_SERVER_ERROR.as_u16()
        }
    }
}

/// Headers that must not be forwarded to the endpoint: client-supplied forwarding information and
/// the HTTP/1.1 hop-by-hop headers (RFC 9110 §7.6.1).
const STRIPPED_HEADERS: [HeaderName; 13] = [
    X_FORWARDED_FOR,
    X_FORWARDED_PROTO,
    X_FORWARDED_HOST,
    FORWARDED,
    VIA,
    CONNECTION,
    KEEP_ALIVE,
    PROXY_CONNECTION,
    PROXY_AUTHENTICATE,
    PROXY_AUTHORIZATION,
    TE,
    TRAILER,
    UPGRADE,
];

/// Removes the [`STRIPPED_HEADERS`] from the request.
pub fn sanitize_headers(req: &mut RequestHeader) {
    for name in &STRIPPED_HEADERS {
        req.remove_header(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_response_is_recovered_from_the_pingora_error() {
        let error = ErrorResponse::new(ErrorCode::UpstreamTransient, "endpoint unavailable");
        let e: Box<Error> = abort_with::<()>(error.clone()).unwrap_err();
        assert_eq!(e.etype(), &ErrorType::HTTPStatus(502));
        assert_eq!(default_error_code(&e), 502);
        assert_eq!(e.root_cause().downcast_ref::<ErrorResponse>(), Some(&error));
    }

    #[test]
    fn only_expired_auth_rejection_is_retryable() {
        let expired = auth_error_response(&AuthError::Expired);
        assert_eq!(expired.code.as_str(), "auth_expired");
        assert!(expired.retryable);

        let missing = auth_error_response(&AuthError::Missing);
        assert_eq!(missing.code.as_str(), "auth_missing");
        assert!(!missing.retryable);
    }

    #[test]
    fn sanitize_strips_forwarding_and_hop_by_hop_headers() {
        let mut req = RequestHeader::build("GET", b"/v1/prf/0/labels/00/1", None).unwrap();
        for (name, value) in [
            ("authorization", "FhevmSig expires=1"),
            ("x-forwarded-for", "1.2.3.4"),
            ("x-forwarded-proto", "https"),
            ("x-forwarded-host", "evil"),
            ("forwarded", "for=1.2.3.4"),
            ("via", "1.1 something"),
            ("connection", "keep-alive"),
            ("keep-alive", "timeout=5"),
            ("te", "trailers"),
            ("content-type", "application/json"),
            ("content-length", "2"),
            ("host", "coprocessor.example"),
            ("x-request-id", "abc"),
        ] {
            req.append_header(name, value).unwrap();
        }

        sanitize_headers(&mut req);

        let remaining: Vec<_> = req.headers.keys().map(|k| k.as_str()).collect();
        assert_eq!(remaining.len(), 5);
        for kept in [
            "authorization",
            "content-type",
            "content-length",
            "host",
            "x-request-id",
        ] {
            assert!(remaining.contains(&kept), "{kept} should be kept");
        }
    }
}
