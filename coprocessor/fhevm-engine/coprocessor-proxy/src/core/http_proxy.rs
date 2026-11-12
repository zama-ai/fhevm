//! The Pingora [`ProxyHttp`] implementation of [`Proxy`]: the request lifecycle of the proxy.

use crate::core::{Proxy, Route, RouteError, client_connections::first_head_received, match_route};
use alloy::primitives::Address;
use async_trait::async_trait;
use bytes::Bytes;
use coprocessor_api::{ErrorCode, ErrorResponse};
use fhevm_sig_auth::AuthError;
use http::{
    HeaderMap, HeaderValue, StatusCode,
    header::{
        ALLOW, AUTHORIZATION, CACHE_CONTROL, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, FORWARDED,
        HeaderName, PROXY_AUTHENTICATE, PROXY_AUTHORIZATION, RETRY_AFTER, TE, TRAILER, UPGRADE,
        VIA, WWW_AUTHENTICATE,
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
use std::{
    net::{IpAddr, Ipv6Addr},
    sync::{LazyLock, atomic::Ordering},
    time::{Duration, Instant},
};
use tracing::{error, info};

static REQUESTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_proxy_requests_counter",
        "Requests answered by the proxy, by route, status and authenticated signer",
        &["route", "status", "signer"]
    )
    .unwrap()
});

/// The signer the proxy authenticated, EIP-55 checksummed: prf-service's caller. Any inbound value
/// is dropped first.
pub const X_FHEVM_SIGNER: HeaderName = HeaderName::from_static(fhevm_sig_auth::SIGNER_HEADER);
// The `X-Forwarded-*` headers set by the proxy. Any inbound value is dropped first, once the client
// IP is read from the inbound `X-Forwarded-For`.
const X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_static("x-forwarded-proto");
// Client-supplied forwarding header the proxy never sets but must not forward either.
const X_FORWARDED_HOST: HeaderName = HeaderName::from_static("x-forwarded-host");
// Hop-by-hop headers without a constant in the `http` crate.
const KEEP_ALIVE: HeaderName = HeaderName::from_static("keep-alive");
const PROXY_CONNECTION: HeaderName = HeaderName::from_static("proxy-connection");

// Set on every answer the proxy writes. Pingora answers a few requests before any hook runs, and
// those lack it: an unparsable request head (`400`) and `CONNECT` (`405`).
const NO_STORE: &str = "no-store";

/// How long a request signed by an unknown signer waits for a poll of the KMS tx-senders that
/// started after it, before it is checked again.
pub const UNKNOWN_SIGNER_WAIT: Duration = Duration::from_secs(3);

/// The window [`Config::max_requests_per_signer`](crate::core::Config) and
/// [`Config::max_requests_per_ip`](crate::core::Config) count requests in. A refused request is told
/// to retry after it.
pub const REQUEST_RATE_INTERVAL: Duration = Duration::from_secs(1);

const NOT_ORIGIN_FORM: &str = "request target must be an origin-form UTF-8 path";
const NO_CLIENT_IP: &str = "X-Forwarded-For lacks the client IP appended by the front layer";

/// Per-request context.
pub struct RequestContext {
    started_at: Instant,
    route: Option<Route>,
    /// The sender authenticated by the `FhevmSig` header.
    sender: Option<Address>,
    /// See [`client_ip`].
    client_ip: Option<IpAddr>,
}

impl Proxy {
    /// The idle time a client connection is kept for, or `None` to close it after this response.
    /// While draining, the client's next connection goes through the Service, to a replica that
    /// is not stopping.
    fn keepalive(&self) -> Option<u64> {
        let draining = self.draining.load(Ordering::Relaxed);
        (!draining).then_some(self.config.client_idle_timeout.as_secs())
    }

    /// Answers `429`, telling the client to retry once the rate's interval has passed.
    async fn respond_rate_limited(
        &self,
        session: &mut Session,
        message: &str,
    ) -> pingora::Result<()> {
        let error = ErrorResponse::new(ErrorCode::ConcurrencyLimited, message);
        let retry_after = REQUEST_RATE_INTERVAL.as_secs().to_string();
        self.respond_with_error(session, error, Some((RETRY_AFTER, &retry_after)))
            .await
    }

    /// Writes an error body as JSON. It must not be cached (RFC-038: only `200` answers may be).
    async fn respond_with_error(
        &self,
        session: &mut Session,
        error: ErrorResponse,
        extra_header: Option<(HeaderName, &str)>,
    ) -> pingora::Result<()> {
        let status = error.code.http_status();
        let body = serde_json::to_vec(&error).unwrap_or_default();
        let mut header = ResponseHeader::build(status, None)?;
        header.insert_header(CONTENT_TYPE, "application/json")?;
        header.insert_header(CONTENT_LENGTH, body.len().to_string())?;
        header.insert_header(CACHE_CONTROL, NO_STORE)?;
        if status == StatusCode::UNAUTHORIZED.as_u16() {
            // A `HEAD` answer has no body, so the challenge carries the code too (RFC-038, as in
            // RFC 9449). The codes are fixed identifiers, so they need no escaping.
            let challenge = format!(
                "{} error=\"{}\"",
                fhevm_sig_auth::SCHEME,
                error.code.as_str()
            );
            header.insert_header(WWW_AUTHENTICATE, challenge)?;
        }
        if let Some((name, value)) = extra_header {
            header.insert_header(name, value)?;
        }
        session
            .write_response_header(Box::new(header), false)
            .await?;
        session
            .write_response_body(Some(Bytes::from(body)), true)
            .await
    }
}

#[async_trait]
impl ProxyHttp for Proxy {
    type CTX = RequestContext;

    fn new_ctx(&self) -> Self::CTX {
        RequestContext {
            started_at: Instant::now(),
            route: None,
            sender: None,
            client_ip: None,
        }
    }

    async fn early_request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        first_head_received(session);
        // Without a front layer the connection is the client's own, and it keeps it only once a
        // request is authenticated (see `request_filter`): otherwise an unsigned request would
        // free it from the first head's deadline. Behind a front layer the connection is the front
        // layer's and carries every client's requests, so closing it would cost a TLS handshake
        // for each refused request.
        let kept = self.config.forwarded_hops > 0;
        session.set_keepalive(if kept { self.keepalive() } else { None });
        Ok(())
    }

    // Check the target, route, absence of a body and signature before any prf-service connection.
    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        let body_empty = session.is_body_empty();
        ctx.client_ip = client_ip(session, self.config.forwarded_hops);
        let req = session.req_header();

        // Pingora forwards the raw target, but its URI renders an absolute-form or non-UTF-8 target
        // lossily: only a target identical to what reaches prf-service can be authenticated.
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
                self.respond_with_error(
                    session,
                    route_error_response(&e),
                    e.allow_header().as_deref().map(|methods| (ALLOW, methods)),
                )
                .await?;
                return Ok(true);
            }
        };
        ctx.route = Some(route);

        // Before the signature, so a body never wakes a read of the KMS tx-senders. The signed body
        // digest must be that of an empty body, so a body is a body mismatch (RFC-038).
        if !body_empty {
            self.respond_with_error(session, auth_error_response(&AuthError::BodyMismatch), None)
                .await?;
            return Ok(true);
        }

        // Before the signature, so a flood from one client IP costs no signature check, nor a read
        // of the KMS tx-senders. After the routing, so the readiness self-check, sent without
        // `X-Forwarded-For` to a path no route matches, gets its `404`.
        // The front layers are not the `forwarded_hops` configured: the deployment's fault, not the
        // client's, and retryable once it is fixed.
        let Some(client_ip) = ctx.client_ip else {
            let error = ErrorResponse::new(ErrorCode::Unavailable, NO_CLIENT_IP);
            self.respond_with_error(session, error, None).await?;
            return Ok(true);
        };
        let requests = self.ip_requests.observe(&rate_key(client_ip), 1);
        if requests > self.config.max_requests_per_ip.get() as isize {
            self.respond_rate_limited(session, "client IP rate limit")
                .await?;
            return Ok(true);
        }

        let authorization = req.headers.get(AUTHORIZATION).map(HeaderValue::as_bytes);
        let verified = match self.verifier.verify(authorization, target, b"") {
            // The signer can be a node of a KMS context whose activation is finalized but not read
            // yet, so it is checked again after one read before being refused.
            Err(AuthError::UnknownSigner(_)) => {
                self.kms_tx_senders
                    .refresh_after_miss(UNKNOWN_SIGNER_WAIT)
                    .await;
                self.verifier.verify(authorization, target, b"")
            }
            verified => verified,
        };
        let sender = match verified {
            Ok(signed) => signed,
            Err(e) => {
                if let AuthError::UnknownSigner(signer) = &e {
                    info!(%signer, "Unknown signer after waiting for a registry refresh");
                }
                self.respond_with_error(session, auth_error_response(&e), None)
                    .await?;
                return Ok(true);
            }
        };
        ctx.sender = Some(sender);
        session.set_keepalive(self.keepalive());

        // A request for an output already stored still costs a signature check and a database
        // read, so each signer's request rate is limited too (RFC-038).
        let requests = self.signer_requests.observe(&sender, 1);
        if requests > self.config.max_requests_per_signer.get() as isize {
            self.respond_rate_limited(session, "signer rate limit")
                .await?;
            return Ok(true);
        }
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let config = &self.config;
        let mut peer = HttpPeer::new(self.prf_service, false, String::new());
        peer.options.connection_timeout = Some(config.prf_service_connect_timeout);
        peer.options.total_connection_timeout = Some(config.prf_service_connect_timeout);
        peer.options.read_timeout = Some(config.prf_service_response_timeout);
        peer.options.write_timeout = Some(config.prf_service_response_timeout);
        peer.options.idle_timeout = Some(config.prf_service_idle_timeout);
        Ok(Box::new(peer))
    }

    // Replaces the `FhevmSig` credential with the signer it authenticated, in `X-Fhevm-Signer`:
    // prf-service trusts this check and applies its per-connector limits to that signer. Also strips
    // the forwarding and hop-by-hop headers. The body is never touched.
    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        sanitize_headers(upstream_request);
        if let Some(sender) = ctx.sender {
            upstream_request.insert_header(X_FHEVM_SIGNER, sender.to_checksum(None))?;
        }
        if let Some(client_ip) = ctx.client_ip {
            upstream_request.insert_header(X_FORWARDED_FOR, client_ip.to_string())?;
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
        // Tell the client this failed connection cannot be reused.
        session.set_keepalive(None);
        // Once the response has started, a JSON error would corrupt its body. Close instead.
        if session
            .response_written()
            .is_some_and(|written| !written.status.is_informational())
        {
            return FailToProxy {
                error_code: 0,
                can_reuse_downstream: false,
            };
        }
        // Keep upstream details in server logs; clients get a fixed error message.
        let error_code = if *e.esource() == ErrorSource::Upstream {
            // prf-service that cannot be connected to is unavailable, a timeout or not.
            let (code, message) = match e.etype() {
                ErrorType::ReadTimedout => (ErrorCode::Timeout, "prf-service timed out"),
                _ => (ErrorCode::UpstreamTransient, "prf-service unavailable"),
            };
            let error = ErrorResponse::new(code, message);
            let status = error.code.http_status();
            if let Err(e) = self.respond_with_error(session, error, None).await {
                error!("Failed to send error response to downstream: {e}");
            }
            status
        } else {
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
        let client = ctx.client_ip.map(|ip| ip.to_string()).unwrap_or_default();
        // The connection's peer: the front layer, which tells which one sent a request without its
        // client IP.
        let peer = session
            .client_addr()
            .map(ToString::to_string)
            .unwrap_or_default();

        let route = ctx.route.map_or("none", Route::as_str);
        let sender = ctx.sender.map(|s| s.to_checksum(None)).unwrap_or_default();
        REQUESTS
            .with_label_values(&[route, &status.to_string(), &sender])
            .inc();

        info!(
            path = req.uri.path(),
            status,
            client,
            peer,
            sender,
            duration_ms = ctx.started_at.elapsed().as_millis() as u64,
            error = e.map(|e| e.to_string()).unwrap_or_default(),
            "Proxied request"
        );
    }
}

/// The client's IP, or `None` when the request does not carry it. Behind `forwarded_hops` front
/// layers, each appending the address it received the request from to `X-Forwarded-For`, it is the
/// entry `forwarded_hops` from the right: the entries further left are whatever the client sent.
/// With no front layer, it is the connection's peer address.
fn client_ip(session: &Session, forwarded_hops: usize) -> Option<IpAddr> {
    let ip = if forwarded_hops == 0 {
        session.client_addr()?.as_inet()?.ip()
    } else {
        forwarded_ip(&session.req_header().headers, forwarded_hops)?
    };
    // A client reaching a dual-stack listener over IPv4 appears as an IPv4-mapped IPv6 address.
    Some(ip.to_canonical())
}

/// The key a client counts under in the per-IP rates: its IPv4 address, or the /64 of its IPv6
/// address, which one host usually holds whole.
pub(super) fn rate_key(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V4(_) => ip,
        IpAddr::V6(ip) => IpAddr::V6(Ipv6Addr::from_bits(ip.to_bits() & !u128::from(u64::MAX))),
    }
}

/// The `X-Forwarded-For` entry `hops` from the right, over all the header's lines.
fn forwarded_ip(headers: &HeaderMap, hops: usize) -> Option<IpAddr> {
    let entry = headers
        .get_all(X_FORWARDED_FOR)
        .iter()
        .rev()
        .flat_map(|value| value.as_bytes().rsplit(|&byte| byte == b','))
        .nth(hops.checked_sub(1)?)?;
    std::str::from_utf8(entry).ok()?.trim().parse().ok()
}

/// Converts an `AuthError` into an `ErrorResponse` with the code of the failing step.
fn auth_error_response(error: &AuthError) -> ErrorResponse {
    let code = match error {
        AuthError::Missing => ErrorCode::AuthMissing,
        AuthError::Malformed(_) => ErrorCode::AuthMalformed,
        AuthError::QueryString => ErrorCode::AuthQueryString,
        AuthError::Expired => ErrorCode::AuthExpired,
        AuthError::TooLongLived => ErrorCode::AuthTooLongLived,
        AuthError::InvalidSignature(_) => ErrorCode::AuthInvalidSignature,
        AuthError::UnknownSigner(_) => ErrorCode::AuthUnknownSigner,
        AuthError::BodyMismatch => ErrorCode::AuthBodyMismatch,
    };
    ErrorResponse::new(code, error.to_string())
}

/// Converts a `RouteError` into an `ErrorResponse`.
fn route_error_response(error: &RouteError) -> ErrorResponse {
    match error {
        RouteError::NotFound => ErrorResponse::new(ErrorCode::NotFound, "unknown route"),
        RouteError::MethodNotAllowed(_) => ErrorResponse::new(
            ErrorCode::MethodNotAllowed,
            format!(
                "method not allowed, expected {}",
                error.allow_header().unwrap_or_default()
            ),
        ),
    }
}

/// The status Pingora's default `fail_to_proxy` would answer to an error that is not
/// prf-service's: those are answered with a JSON body instead.
fn default_error_code(e: &Error) -> u16 {
    match (e.etype(), e.esource()) {
        (ErrorType::HTTPStatus(code), _) => *code,
        (
            ErrorType::WriteError | ErrorType::ReadError | ErrorType::ConnectionClosed,
            ErrorSource::Downstream,
        ) => 0,
        (_, ErrorSource::Downstream) => StatusCode::BAD_REQUEST.as_u16(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
    }
}

/// Headers that must not be forwarded to prf-service: the client's credential, client-supplied
/// identity and forwarding information, and the HTTP/1.1 hop-by-hop headers (RFC 9110 §7.6.1).
const STRIPPED_HEADERS: [HeaderName; 15] = [
    AUTHORIZATION,
    X_FHEVM_SIGNER,
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
    fn answers_each_auth_failure_with_its_fhevm_sig_auth_code() {
        for error in [
            AuthError::Missing,
            AuthError::Malformed("bad"),
            AuthError::QueryString,
            AuthError::Expired,
            AuthError::TooLongLived,
            AuthError::InvalidSignature("bad".into()),
            AuthError::UnknownSigner(Address::ZERO),
            AuthError::BodyMismatch,
        ] {
            assert_eq!(auth_error_response(&error).code.as_str(), error.code());
        }
    }

    fn forwarded(lines: &[&str]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for line in lines {
            headers.append(X_FORWARDED_FOR, HeaderValue::from_str(line).unwrap());
        }
        headers
    }

    fn ip(value: &str) -> Option<IpAddr> {
        Some(value.parse().unwrap())
    }

    #[test]
    fn the_client_ip_is_the_entry_appended_by_the_outermost_front_layer() {
        // The client sent 6.6.6.6; the outer front layer appended 203.0.113.7, the inner 10.0.0.1.
        let headers = forwarded(&["6.6.6.6, 203.0.113.7,10.0.0.1"]);
        assert_eq!(forwarded_ip(&headers, 1), ip("10.0.0.1"));
        assert_eq!(forwarded_ip(&headers, 2), ip("203.0.113.7"));
        assert_eq!(forwarded_ip(&headers, 3), ip("6.6.6.6"));
        // A front layer can append its entry on a header line of its own.
        let headers = forwarded(&["6.6.6.6", "203.0.113.7", " 2001:db8::1 "]);
        assert_eq!(forwarded_ip(&headers, 1), ip("2001:db8::1"));
        assert_eq!(forwarded_ip(&headers, 2), ip("203.0.113.7"));
    }

    #[test]
    fn a_request_without_the_client_ip_has_none() {
        assert_eq!(forwarded_ip(&HeaderMap::new(), 1), None);
        assert_eq!(forwarded_ip(&forwarded(&["203.0.113.7"]), 2), None);
        assert_eq!(forwarded_ip(&forwarded(&["203.0.113.7"]), 0), None);
        assert_eq!(forwarded_ip(&forwarded(&["203.0.113.7, unknown"]), 1), None);
        assert_eq!(forwarded_ip(&forwarded(&["203.0.113.7:443"]), 1), None);
        assert_eq!(forwarded_ip(&forwarded(&["203.0.113.7,"]), 1), None);
        // Only the selected entry must parse.
        assert_eq!(
            forwarded_ip(&forwarded(&["garbage, 203.0.113.7"]), 1),
            ip("203.0.113.7")
        );
    }

    #[test]
    fn the_per_ip_rate_counts_an_ipv6_client_by_its_64() {
        let key = |value: &str| rate_key(value.parse().unwrap());
        assert_eq!(key("2001:db8:1:2::1"), key("2001:db8:1:2:ffff::9"));
        assert_eq!(key("2001:db8:1:2::1"), key("2001:db8:1:2::"));
        assert_ne!(key("2001:db8:1:2::1"), key("2001:db8:1:3::1"));
        assert_eq!(key("203.0.113.7"), key("203.0.113.7"));
        assert_ne!(key("203.0.113.7"), key("203.0.113.8"));
    }

    #[test]
    fn sanitize_strips_credential_identity_forwarding_and_hop_by_hop_headers() {
        let mut req = RequestHeader::build("GET", b"/v1/prf/0/labels/00/1", None).unwrap();
        for (name, value) in [
            ("authorization", "FhevmSig expires=1"),
            (
                "x-fhevm-signer",
                "0x0101010101010101010101010101010101010101",
            ),
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
        assert_eq!(remaining.len(), 4);
        for kept in ["content-type", "content-length", "host", "x-request-id"] {
            assert!(remaining.contains(&kept), "{kept} should be kept");
        }
    }
}
