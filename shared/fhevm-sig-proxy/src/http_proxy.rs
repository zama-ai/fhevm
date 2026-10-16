//! The Pingora [`ProxyHttp`] implementation of [`Proxy`]: the request lifecycle of the proxy.

use crate::{
    ErrorBody, Proxy, ProxyError,
    routing::{ParsedRoute, RouteError},
};
use alloy::primitives::{Address, B256, KECCAK256_EMPTY, Keccak256};
use async_trait::async_trait;
use bytes::Bytes;
use fhevm_sig_auth::AuthError;
use http::{
    HeaderValue, Method, StatusCode,
    header::{
        ALLOW, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, FORWARDED, HeaderName, VIA,
        WWW_AUTHENTICATE,
    },
};
use pingora::{
    Error, ErrorSource, ErrorType,
    http::{RequestHeader, ResponseHeader},
    prelude::HttpPeer,
    proxy::{FailToProxy, ProxyHttp, Session},
    upstreams::peer::HttpUpstreamRequestPolicy,
};
use prometheus::{IntCounterVec, register_int_counter_vec};
use std::{sync::LazyLock, time::Instant};
use tracing::{error, info};

// The `X-Forwarded-*` headers set by the proxy (any inbound value is dropped first).
const X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_static("x-forwarded-proto");
// Client-supplied forwarding header the proxy never sets but must not forward either.
const X_FORWARDED_HOST: HeaderName = HeaderName::from_static("x-forwarded-host");

/// The requests answered, by route template (`none` before a route matched) and status (`0` when
/// no response was written). Both label sets are bounded: no raw path, no signer.
static REQUESTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "fhevm_sig_proxy_requests_total",
        "Requests answered by the proxy, by route template and status",
        &["route", "status"]
    )
    .unwrap()
});

/// Per-request context.
pub struct RequestContext {
    started_at: Instant,
    /// The index of the matched route in [`Proxy::routes`].
    route: Option<usize>,
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
    /// Writes an error body as JSON, rendered by the deployment.
    async fn respond_with_error(
        &self,
        session: &mut Session,
        error: &ProxyError,
    ) -> pingora::Result<()> {
        let ErrorBody { status, json: body } = (self.config.error_body)(error);
        let mut header = ResponseHeader::build(status, None)?;
        header.insert_header(CONTENT_TYPE, "application/json")?;
        header.insert_header(CONTENT_LENGTH, body.len().to_string())?;
        if status == StatusCode::UNAUTHORIZED {
            header.insert_header(WWW_AUTHENTICATE, fhevm_sig_auth::SCHEME)?;
        }
        session
            .write_response_header(Box::new(header), false)
            .await?;
        session
            .write_response_body(Some(Bytes::from(body)), true)
            .await
    }

    /// Writes an empty response with the given status.
    async fn respond_empty(
        &self,
        session: &mut Session,
        status: StatusCode,
        allow: Option<&[Method]>,
    ) -> pingora::Result<()> {
        let mut header = ResponseHeader::build(status, None)?;
        header.insert_header(CONTENT_LENGTH, "0")?;
        if let Some(methods) = allow {
            let methods: Vec<_> = methods.iter().map(Method::as_str).collect();
            header.insert_header(ALLOW, methods.join(", "))?;
        }
        session.write_response_header(Box::new(header), true).await
    }

    /// The route `request_filter` matched: the later phases only run after it did.
    fn matched_route(&self, ctx: &RequestContext) -> pingora::Result<&ParsedRoute> {
        let index = ctx
            .route
            .ok_or_else(|| Error::explain(ErrorType::InternalError, "no route matched"))?;
        Ok(self.routes.get(index))
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
            self.respond_with_error(session, &ProxyError::NotOriginForm)
                .await?;
            return Ok(true);
        };

        let index = match self.routes.find(&req.method, target.path()) {
            Ok(index) => index,
            Err(RouteError::NotFound) => {
                self.respond_empty(session, StatusCode::NOT_FOUND, None)
                    .await?;
                return Ok(true);
            }
            // RFC 9110 §15.5.6: a `405` lists the methods the route accepts in `Allow`.
            Err(RouteError::MethodNotAllowed(methods)) => {
                self.respond_empty(session, StatusCode::METHOD_NOT_ALLOWED, Some(&methods))
                    .await?;
                return Ok(true);
            }
            Err(RouteError::NonCanonical) => {
                self.respond_with_error(session, &ProxyError::NonCanonicalPath)
                    .await?;
                return Ok(true);
            }
        };
        ctx.route = Some(index);
        let max_body_bytes = self.routes.get(index).route.max_body_bytes;

        let authorization = req.headers.get(AUTHORIZATION).map(HeaderValue::as_bytes);
        let (sender, body_digest) = match self.verifier.verify_header(authorization, target) {
            Ok(signed) => signed,
            Err(e) => {
                self.respond_with_error(session, &ProxyError::Auth(e))
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
        if declared_length.is_some_and(|len| len > max_body_bytes) {
            self.respond_with_error(session, &ProxyError::BodyTooLarge)
                .await?;
            return Ok(true);
        }

        if session.is_body_empty() && KECCAK256_EMPTY != body_digest {
            self.respond_with_error(session, &ProxyError::Auth(AuthError::BodyMismatch))
                .await?;
            return Ok(true);
        }
        ctx.body_digest = body_digest;

        Ok(false)
    }

    // Streams the body, within the cap, while hashing it. It is checked against the signed
    // digest before its last chunk is written, so the endpoint never receives a whole body that
    // does not match: it gets a truncated request and a closed connection instead.
    async fn request_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<Bytes>,
        end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        if let Some(chunk) = body {
            ctx.body_len += chunk.len();
            if ctx.body_len > self.matched_route(ctx)?.route.max_body_bytes {
                return abort_with(self.config.error_body, ProxyError::BodyTooLarge);
            }
            ctx.body_hasher.update(chunk);
        }
        if end_of_stream && ctx.body_hasher.clone().finalize() != ctx.body_digest {
            return abort_with(
                self.config.error_body,
                ProxyError::Auth(AuthError::BodyMismatch),
            );
        }
        Ok(())
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let route = self.matched_route(ctx)?;
        let response_timeout = route.route.endpoint_response_timeout;
        let Some(backend) =
            self.balancers[route.upstream].select(SELECT_KEY, MAX_SELECT_ITERATIONS)
        else {
            return abort_with(self.config.error_body, ProxyError::EndpointUnavailable);
        };
        ctx.endpoint = Some(backend.addr.to_string());

        let mut peer = HttpPeer::new(backend.addr, false, String::new());
        peer.options.connection_timeout = Some(self.config.endpoint_connect_timeout);
        peer.options.total_connection_timeout = Some(self.config.endpoint_connect_timeout);
        peer.options.read_timeout = Some(response_timeout);
        peer.options.write_timeout = Some(response_timeout);
        peer.options.idle_timeout = Some(self.config.endpoint_idle_timeout);
        // Pingora strips the hop-by-hop and `Connection`-nominated headers by default but rebuilds
        // a WebSocket handshake; the endpoints serve no upgrade.
        peer.options.http_upstream_request_policy = HttpUpstreamRequestPolicy::deny_upgrades();
        Ok(Box::new(peer))
    }

    // Strips the forwarding information, and the credentials unless the endpoints read them. The
    // body is never touched.
    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        sanitize_headers(upstream_request, self.config.forward_authorization);
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
        let error = match e.root_cause().downcast_ref::<ProxyError>() {
            Some(error) => Some(error.clone()),
            // An upstream failure (unreachable endpoint, timeout...) gets an error body. The
            // detailed Pingora error (which includes the endpoint's address) is only logged
            // server-side by `logging()`; the client, e.g. the relayer, gets a fixed message.
            None if *e.esource() == ErrorSource::Upstream => Some(match e.etype() {
                ErrorType::ConnectTimedout | ErrorType::ReadTimedout => ProxyError::EndpointTimeout,
                _ => ProxyError::EndpointUnavailable,
            }),
            None => None,
        };
        let error_code = match error {
            Some(error) => {
                if let Err(e) = self.respond_with_error(session, &error).await {
                    error!("Failed to send error response to downstream: {e}");
                }
                (self.config.error_body)(&error).status.as_u16()
            }
            None => {
                let code = default_error_code(e);
                if code > 0
                    && let Err(e) = session.respond_error(code).await
                {
                    error!("Failed to send error response to downstream: {e}");
                }
                code
            }
        };
        FailToProxy {
            error_code,
            can_reuse_downstream: false,
        }
    }

    // Structured access log and request count. Never logs bodies nor credentials.
    async fn logging(&self, session: &mut Session, e: Option<&Error>, ctx: &mut Self::CTX) {
        let status = session
            .response_written()
            .map(|resp| resp.status.as_u16())
            .unwrap_or(0);
        let route = ctx
            .route
            .map_or("none", |index| self.routes.get(index).route.path.as_str());
        REQUESTS
            .with_label_values(&[route, &status.to_string()])
            .inc();
        let req = session.req_header();
        let client = session
            .client_addr()
            .map(ToString::to_string)
            .unwrap_or_default();

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

/// Aborts the proxying with a [`ProxyError`], with the status `error_body` renders it with.
///
/// The [`ProxyError`] is kept as the cause so `fail_to_proxy` can send it back to the client.
fn abort_with<T>(
    error_body: fn(&ProxyError) -> ErrorBody,
    error: ProxyError,
) -> pingora::Result<T> {
    let etype = ErrorType::HTTPStatus(error_body(&error).status.as_u16());
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

/// Client-supplied forwarding information, which must not reach the endpoint.
const STRIPPED_HEADERS: [HeaderName; 5] = [
    X_FORWARDED_FOR,
    X_FORWARDED_PROTO,
    X_FORWARDED_HOST,
    FORWARDED,
    VIA,
];

/// Removes the [`STRIPPED_HEADERS`] from the request, and `Authorization` unless
/// `forward_authorization`.
pub fn sanitize_headers(req: &mut RequestHeader, forward_authorization: bool) {
    for name in &STRIPPED_HEADERS {
        req.remove_header(name);
    }
    if !forward_authorization {
        req.remove_header(&AUTHORIZATION);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_error_is_recovered_from_the_pingora_error() {
        let bad_gateway = |_: &ProxyError| ErrorBody {
            status: StatusCode::BAD_GATEWAY,
            json: vec![],
        };
        let error = ProxyError::EndpointUnavailable;
        let e: Box<Error> = abort_with::<()>(bad_gateway, error.clone()).unwrap_err();
        assert_eq!(e.etype(), &ErrorType::HTTPStatus(502));
        assert_eq!(default_error_code(&e), 502);
        assert_eq!(e.root_cause().downcast_ref::<ProxyError>(), Some(&error));
    }

    fn request_with_all_headers() -> RequestHeader {
        let mut req = RequestHeader::build("POST", b"/v1/public-decrypt", None).unwrap();
        for (name, value) in [
            ("authorization", "Bearer secret"),
            ("x-forwarded-for", "1.2.3.4"),
            ("x-forwarded-proto", "https"),
            ("x-forwarded-host", "evil"),
            ("forwarded", "for=1.2.3.4"),
            ("via", "1.1 something"),
            ("content-type", "application/json"),
            ("content-length", "2"),
            ("host", "kms.example"),
            ("x-request-id", "abc"),
        ] {
            req.append_header(name, value).unwrap();
        }
        req
    }

    fn remaining(req: &RequestHeader) -> Vec<&str> {
        let mut names: Vec<_> = req.headers.keys().map(|k| k.as_str()).collect();
        names.sort();
        names
    }

    #[test]
    fn sanitize_strips_credentials_and_forwarding_headers() {
        let mut req = request_with_all_headers();
        sanitize_headers(&mut req, false);
        assert_eq!(
            remaining(&req),
            ["content-length", "content-type", "host", "x-request-id"]
        );
    }

    #[test]
    fn sanitize_keeps_the_credentials_the_endpoints_read() {
        let mut req = request_with_all_headers();
        sanitize_headers(&mut req, true);
        assert_eq!(
            remaining(&req),
            [
                "authorization",
                "content-length",
                "content-type",
                "host",
                "x-request-id"
            ]
        );
    }
}
