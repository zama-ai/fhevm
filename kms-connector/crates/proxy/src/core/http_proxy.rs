//! The Pingora [`ProxyHttp`] implementation of [`Proxy`]: the request lifecycle of the proxy.

use crate::core::{Proxy, Route, match_route};
use async_trait::async_trait;
use bytes::Bytes;
use http::{
    Method, StatusCode,
    header::{
        ALLOW, AUTHORIZATION, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, FORWARDED, HOST,
        HeaderName, PROXY_AUTHENTICATE, PROXY_AUTHORIZATION, TE, TRAILER, TRANSFER_ENCODING,
        UPGRADE, VIA, WWW_AUTHENTICATE,
    },
};
use kms_connector_api::{ErrorCode, ErrorResponse};
use pingora::{
    Error, ErrorSource, ErrorType,
    http::{RequestHeader, ResponseHeader},
    prelude::HttpPeer,
    proxy::{FailToProxy, ProxyHttp, Session},
};
use std::time::Instant;
use tracing::{error, info};

/// The `X-Forwarded-*` headers set by the proxy (any inbound value is dropped first).
const X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_static("x-forwarded-proto");
/// Hop-by-hop headers without a constant in the `http` crate.
const KEEP_ALIVE: HeaderName = HeaderName::from_static("keep-alive");
const PROXY_CONNECTION: HeaderName = HeaderName::from_static("proxy-connection");

/// Per-request context.
pub struct RequestContext {
    started_at: Instant,
    route: Option<Route>,
    endpoint: Option<String>,
    body_bytes: usize,
}

impl Proxy {
    /// Writes an error body as JSON with the given status.
    async fn respond_with_error(
        &self,
        session: &mut Session,
        status: StatusCode,
        error: ErrorResponse,
        extra_headers: &[(HeaderName, String)],
    ) -> pingora::Result<()> {
        let body = serde_json::to_vec(&error).unwrap_or_default();
        let fixed_headers = [
            (CONTENT_TYPE, "application/json".to_string()),
            (CONTENT_LENGTH, body.len().to_string()),
        ];
        let mut header =
            ResponseHeader::build(status, Some(fixed_headers.len() + extra_headers.len()))?;
        for (name, value) in fixed_headers.iter().chain(extra_headers) {
            header.insert_header(name.clone(), value.as_str())?;
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
        allow: Option<&Method>,
    ) -> pingora::Result<()> {
        let headers_len = if allow.is_some() { 2 } else { 1 };
        let mut header = ResponseHeader::build(status, Some(headers_len))?;
        header.insert_header(CONTENT_LENGTH, "0")?;
        if let Some(method) = allow {
            header.insert_header(ALLOW, method.as_str())?;
        }
        session.write_response_header(Box::new(header), true).await
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
            body_bytes: 0,
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

    // Route (404/405), authentication (401), body cap (400) then read the body.
    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        let req = session.req_header();

        let route = match match_route(&req.method, req.uri.path()) {
            Ok(route) => route,
            Err(e) => {
                self.respond_empty(session, e.http_status(), e.allow_header())
                    .await?;
                return Ok(true);
            }
        };
        ctx.route = Some(route);

        if !self.verifier.verify(req.headers.get(AUTHORIZATION)) {
            let error = ErrorResponse::new(
                ErrorCode::SenderAuthenticationFailed,
                "invalid or missing API key",
                None,
            );
            let headers = [(WWW_AUTHENTICATE, "Bearer".to_string())];
            self.respond_with_error(session, StatusCode::UNAUTHORIZED, error, &headers)
                .await?;
            return Ok(true);
        }

        let declared_length = req
            .headers
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<usize>().ok());
        if declared_length.is_some_and(|len| len > self.config.max_body_bytes) {
            let error = ErrorResponse::new(
                ErrorCode::Malformed,
                format!(
                    "request body exceeds the {} bytes limit",
                    self.config.max_body_bytes
                ),
                None,
            );
            self.respond_with_error(session, StatusCode::BAD_REQUEST, error, &[])
                .await?;
            return Ok(true);
        }

        Ok(false)
    }

    // Checks the body cap on chunked requests.
    async fn request_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<Bytes>,
        _end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()> {
        if let Some(chunk) = body {
            ctx.body_bytes += chunk.len();
            if ctx.body_bytes > self.config.max_body_bytes {
                return abort_with(ErrorResponse::new(
                    ErrorCode::Malformed,
                    format!(
                        "request body exceeds the {} bytes limit",
                        self.config.max_body_bytes
                    ),
                    None,
                ));
            }
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
                ErrorCode::Overloaded,
                "no endpoint available",
                None,
            ));
        };
        ctx.endpoint = Some(backend.addr.to_string());

        let mut peer = HttpPeer::new(backend.addr, false, String::new());
        peer.options.connection_timeout = Some(self.config.endpoint_connect_timeout);
        peer.options.total_connection_timeout = Some(self.config.endpoint_connect_timeout);
        peer.options.read_timeout = Some(self.config.endpoint_response_timeout);
        peer.options.write_timeout = Some(self.config.endpoint_response_timeout);
        peer.options.idle_timeout = Some(self.config.endpoint_idle_timeout);
        Ok(Box::new(peer))
    }

    // Strips the credentials before forwarding. The body is never touched.
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
        let error_code = match e.root_cause().downcast_ref::<ErrorResponse>() {
            Some(error) => {
                let status = StatusCode::from_u16(error.code.http_status())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                if let Err(e) = self
                    .respond_with_error(session, status, error.clone(), &[])
                    .await
                {
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
                let error = ErrorResponse::new(code, message.to_string(), None);
                let status = StatusCode::from_u16(error.code.http_status())
                    .unwrap_or(StatusCode::BAD_GATEWAY);
                if let Err(e) = self.respond_with_error(session, status, error, &[]).await {
                    error!("Failed to send error response to downstream: {e}");
                }
                status.as_u16()
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

        info!(
            path = req.uri.path(),
            status,
            client,
            endpoint = ctx.endpoint.as_deref().unwrap_or(""),
            duration_ms = ctx.started_at.elapsed().as_millis() as u64,
            error = e.map(|e| e.to_string()).unwrap_or_default(),
            "Proxied request"
        );
    }
}

/// Aborts the proxying with an kms-connector-api error.
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

/// Removes the headers that must never reach the endpoint: the proxy credentials, client-supplied
/// forwarding information and the HTTP/1.1 hop-by-hop headers (RFC 9110 §7.6.1).
pub fn sanitize_headers(req: &mut RequestHeader) {
    // Headers listed in `Connection` are hop-by-hop too.
    let connection_headers = req
        .headers
        .get_all(CONNECTION)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .filter_map(|name| HeaderName::from_bytes(name.trim().as_bytes()).ok())
        .filter(|name| ![HOST, CONTENT_LENGTH, CONTENT_TYPE, TRANSFER_ENCODING].contains(name));

    // These headers must be set by the proxy itself, not the client.
    let forwarded_headers = req
        .headers
        .keys()
        .filter(|name| name.as_str().starts_with("x-forwarded-"))
        .cloned();

    let to_remove: Vec<HeaderName> = [
        AUTHORIZATION,
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
    ]
    .into_iter()
    .chain(connection_headers)
    .chain(forwarded_headers)
    .collect();

    for name in to_remove {
        req.remove_header(&name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_response_is_recovered_from_the_pingora_error() {
        let error = ErrorResponse::new(ErrorCode::Overloaded, "no endpoint available", None);
        let e: Box<Error> = abort_with::<()>(error.clone()).unwrap_err();
        assert_eq!(e.etype(), &ErrorType::HTTPStatus(503));
        assert_eq!(default_error_code(&e), 503);
        assert_eq!(e.root_cause().downcast_ref::<ErrorResponse>(), Some(&error));
    }

    #[test]
    fn sanitize_strips_credentials_forwarding_and_hop_by_hop_headers() {
        let mut req = RequestHeader::build("POST", b"/v1/public-decrypt", None).unwrap();
        for (name, value) in [
            ("authorization", "Bearer secret"),
            ("x-forwarded-for", "1.2.3.4"),
            ("x-forwarded-proto", "https"),
            ("x-forwarded-host", "evil"),
            ("forwarded", "for=1.2.3.4"),
            ("via", "1.1 something"),
            ("connection", "keep-alive, x-custom-hop, content-length"),
            ("x-custom-hop", "1"),
            ("keep-alive", "timeout=5"),
            ("te", "trailers"),
            ("content-type", "application/json"),
            ("content-length", "2"),
            ("host", "kms.example"),
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
