//! End-to-end tests of the proxy against a stub Connector endpoint.

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use alloy::transports::http::reqwest::{Client, StatusCode};
use http::uri::Authority;
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
    VersionResponse,
};
use proxy::core::{Config, Proxy, TlsConfig};
use serde_json::json;
use std::{
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

const API_KEY: &str = "test";
// sha256("test")
const API_KEY_DIGEST: &str = "0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
const MAX_BODY_BYTES: usize = 1024;

#[tokio::test]
async fn test_forwards_authenticated_requests_untouched() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .client
        .get(t.url(VERSION_ROUTE))
        .bearer_auth(API_KEY)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.json::<VersionResponse>().await.unwrap(),
        VersionResponse::default()
    );

    // Deliberately non-canonical JSON: the proxy must forward it byte-for-byte.
    let body = r#"{ "ctHandles":["0x01"] ,  "extraData":"0x00"}"#;
    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .header("content-type", "application/json")
        .header("x-forwarded-for", "9.9.9.9")
        .body(body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let echo: serde_json::Value = response.json().await.unwrap();
    assert_eq!(echo["body"], body);
    assert_eq!(echo["authorization"], serde_json::Value::Null);
    assert_eq!(echo["x-forwarded-for"], "127.0.0.1");
    assert_eq!(echo["x-forwarded-proto"], "https");

    let response = t
        .client
        .post(t.url(USER_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_rejects_unauthenticated_requests() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    for request in [
        t.client.post(t.url(PUBLIC_DECRYPTION_ROUTE)).body("{}"),
        t.client
            .post(t.url(PUBLIC_DECRYPTION_ROUTE))
            .bearer_auth("wrong")
            .body("{}"),
        t.client
            .post(t.url(USER_DECRYPTION_ROUTE))
            .header("authorization", format!("Basic {API_KEY}"))
            .body("{}"),
        t.client.get(t.url(VERSION_ROUTE)),
    ] {
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers()["www-authenticate"].to_str().unwrap(),
            "Bearer"
        );
        assert_eq!(
            response.headers()["content-type"].to_str().unwrap(),
            "application/json"
        );
        let error: ErrorResponse = response.json().await.unwrap();
        assert_eq!(error.code, ErrorCode::SenderAuthenticationFailed);
        assert!(!error.retryable);
        assert_eq!(error.decryption_id, None);
    }
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_default_deny_routing() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    for (request, expected, expected_allow) in [
        (t.client.get(t.url("/")), StatusCode::NOT_FOUND, None),
        (t.client.get(t.url("/healthz")), StatusCode::NOT_FOUND, None),
        (
            t.client.post(t.url("/v2/public-decrypt")),
            StatusCode::NOT_FOUND,
            None,
        ),
        (
            t.client.post(t.url("/v1/public-decrypt/extra")),
            StatusCode::NOT_FOUND,
            None,
        ),
        (
            t.client.get(t.url(PUBLIC_DECRYPTION_ROUTE)),
            StatusCode::METHOD_NOT_ALLOWED,
            Some("POST"),
        ),
        (
            t.client.post(t.url(VERSION_ROUTE)),
            StatusCode::METHOD_NOT_ALLOWED,
            Some("GET"),
        ),
    ] {
        // Even with valid credentials.
        let response = request.bearer_auth(API_KEY).send().await.unwrap();
        assert_eq!(response.status(), expected);
        assert_eq!(
            response
                .headers()
                .get("allow")
                .and_then(|v| v.to_str().ok()),
            expected_allow
        );
    }
    assert_eq!(t.endpoints[0].hits(), 0, "nothing must reach the endpoint");
}

#[tokio::test]
async fn test_rejects_oversized_bodies() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body(vec![b' '; MAX_BODY_BYTES + 1])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::Malformed);
    assert!(!error.retryable);

    // Chunked bodies carry no `Content-Length`, the cap is enforced while streaming.
    let status = send_chunked(&t, &[MAX_BODY_BYTES, 1]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let status = send_chunked(&t, &[MAX_BODY_BYTES / 2, MAX_BODY_BYTES / 2]).await;
    assert_eq!(status, StatusCode::OK);

    // Exactly at the limit is fine.
    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body(vec![b' '; MAX_BODY_BYTES])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_passes_endpoint_errors_through() {
    let t = TestProxy::start(vec![StubEndpoint::start().await]).await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .header("x-stub-overloaded", "1")
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.headers()["retry-after"].to_str().unwrap(), "2");
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::Overloaded);
    assert!(error.retryable);
}

#[tokio::test]
async fn test_load_balances_across_endpoints() {
    let endpoints = vec![StubEndpoint::start().await, StubEndpoint::start().await];
    let t = TestProxy::start(endpoints).await;

    for _ in 0..10 {
        let response = t
            .client
            .post(t.url(PUBLIC_DECRYPTION_ROUTE))
            .bearer_auth(API_KEY)
            .body("{}")
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
    assert_eq!(t.endpoints[0].hits() + t.endpoints[1].hits(), 10);
    assert!(t.endpoints[0].hits() > 0 && t.endpoints[1].hits() > 0);
}

#[tokio::test]
async fn test_upstream_timeout() {
    let endpoint = StubEndpoint::start().await;
    let t = TestProxy::start_with_config(
        vec![endpoint.addr.to_string().parse().unwrap()],
        vec![endpoint],
        Duration::from_millis(100),
    )
    .await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .header("x-stub-delay-ms", "500")
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::Timeout);
    assert!(error.retryable);
}

#[tokio::test]
async fn test_unreachable_endpoint() {
    // Bind a port and close it right away: nothing listens there.
    let dead_addr = free_addr();
    let t =
        TestProxy::start_with_addresses(vec![dead_addr.to_string().parse().unwrap()], vec![]).await;

    let response = t
        .client
        .post(t.url(PUBLIC_DECRYPTION_ROUTE))
        .bearer_auth(API_KEY)
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    let error: ErrorResponse = response.json().await.unwrap();
    assert_eq!(error.code, ErrorCode::UpstreamTransient);
    assert!(error.retryable);
}

// ---------------------------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------------------------

/// A stub Connector endpoint echoing what it receives.
struct StubEndpoint {
    addr: SocketAddr,
    hits: Arc<std::sync::atomic::AtomicUsize>,
}

impl StubEndpoint {
    async fn start() -> Self {
        let addr = free_addr();
        let hits = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let hits_clone = Arc::clone(&hits);
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(Arc::clone(&hits_clone)))
                .route(VERSION_ROUTE, web::get().to(version))
                .route(PUBLIC_DECRYPTION_ROUTE, web::post().to(echo))
                .route(USER_DECRYPTION_ROUTE, web::post().to(echo))
        })
        .workers(1)
        .bind(addr)
        .unwrap()
        .run();
        tokio::spawn(server);
        Self { addr, hits }
    }

    fn hits(&self) -> usize {
        self.hits.load(std::sync::atomic::Ordering::SeqCst)
    }
}

async fn version() -> HttpResponse {
    HttpResponse::Ok().json(VersionResponse::default())
}

async fn echo(
    req: HttpRequest,
    body: web::Bytes,
    hits: web::Data<Arc<std::sync::atomic::AtomicUsize>>,
) -> HttpResponse {
    hits.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    if req.headers().contains_key("x-stub-overloaded") {
        return HttpResponse::ServiceUnavailable()
            .insert_header(("Retry-After", "2"))
            .json(ErrorResponse::new(ErrorCode::Overloaded, "busy", None));
    }
    if let Some(delay_ms) = req
        .headers()
        .get("x-stub-delay-ms")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
    {
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }
    let header = |name: &str| req.headers().get(name).and_then(|v| v.to_str().ok());
    HttpResponse::Ok().json(json!({
        "body": String::from_utf8_lossy(&body),
        "authorization": header("authorization"),
        "x-forwarded-for": header("x-forwarded-for"),
        "x-forwarded-proto": header("x-forwarded-proto"),
    }))
}

/// Runs a `Proxy` for the lifetime of one test.
struct TestProxy {
    client: Client,
    addr: SocketAddr,
    tls: TestTls,
    endpoints: Vec<StubEndpoint>,
}

impl TestProxy {
    async fn start(endpoints: Vec<StubEndpoint>) -> Self {
        let addresses = endpoints
            .iter()
            .map(|u| u.addr.to_string().parse().unwrap())
            .collect();
        Self::start_with_addresses(addresses, endpoints).await
    }

    async fn start_with_addresses(
        endpoint_addresses: Vec<Authority>,
        endpoints: Vec<StubEndpoint>,
    ) -> Self {
        Self::start_with_config(endpoint_addresses, endpoints, Duration::from_secs(60)).await
    }

    async fn start_with_config(
        endpoint_addresses: Vec<Authority>,
        endpoints: Vec<StubEndpoint>,
        endpoint_response_timeout: Duration,
    ) -> Self {
        let addr = free_addr();
        let tls = TestTls::generate(addr);
        let config = Config {
            bind_address: addr,
            tls_config: TlsConfig {
                cert_path: tls.cert_path.clone(),
                key_path: tls.key_path.clone(),
            },
            api_key_digest: API_KEY_DIGEST.parse().unwrap(),
            endpoint_addresses,
            endpoint_connect_timeout: Duration::from_millis(500),
            endpoint_response_timeout,
            max_body_bytes: MAX_BODY_BYTES,
            ..Config::default()
        };
        let proxy = Proxy::from_config(config).unwrap();
        std::thread::spawn(move || proxy.run());

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .tls_danger_accept_invalid_certs(true) // self-signed test certificate
            .build()
            .unwrap();
        wait_for_listener(addr).await;

        Self {
            client,
            addr,
            tls,
            endpoints,
        }
    }

    fn url(&self, route: &str) -> String {
        format!("https://{}{route}", self.addr)
    }
}

/// A self-signed certificate generated for one test proxy instance.
struct TestTls {
    cert_path: PathBuf,
    key_path: PathBuf,
    cert_der: rustls::pki_types::CertificateDer<'static>,
}

impl TestTls {
    fn generate(addr: SocketAddr) -> Self {
        let certified_key =
            rcgen::generate_simple_self_signed(vec![addr.ip().to_string()]).unwrap();
        let dir = std::env::temp_dir().join(format!("proxy-tls-{}-{}", std::process::id(), addr));
        std::fs::create_dir_all(&dir).unwrap();
        let cert_path = dir.join("tls.crt");
        let key_path = dir.join("tls.key");
        std::fs::write(&cert_path, certified_key.cert.pem()).unwrap();
        std::fs::write(&key_path, certified_key.signing_key.serialize_pem()).unwrap();
        Self {
            cert_path,
            key_path,
            cert_der: certified_key.cert.der().clone(),
        }
    }

    /// Opens a TLS connection to `addr`, trusting only the generated certificate.
    async fn connect(
        &self,
        addr: SocketAddr,
    ) -> tokio_rustls::client::TlsStream<tokio::net::TcpStream> {
        let mut roots = rustls::RootCertStore::empty();
        roots.add(self.cert_der.clone()).unwrap();
        let config = rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::aws_lc_rs::default_provider(),
        ))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_root_certificates(roots)
        .with_no_client_auth();
        let server_name = rustls::pki_types::ServerName::from(addr.ip());
        let tcp = tokio::net::TcpStream::connect(addr).await.unwrap();
        tokio_rustls::TlsConnector::from(Arc::new(config))
            .connect(server_name, tcp)
            .await
            .unwrap()
    }
}

impl Drop for TestTls {
    fn drop(&mut self) {
        if let Some(dir) = self.cert_path.parent() {
            std::fs::remove_dir_all(dir).ok();
        }
    }
}

/// Sends a `Transfer-Encoding: chunked` request made of whitespace chunks of the given sizes to
/// the public decryption route, and returns the response status.
async fn send_chunked(t: &TestProxy, chunk_sizes: &[usize]) -> StatusCode {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let addr = t.addr;
    let mut stream = t.tls.connect(addr).await;
    let mut request = format!(
        "POST {PUBLIC_DECRYPTION_ROUTE} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: Bearer {API_KEY}\r\n\
         Content-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
    )
    .into_bytes();
    for size in chunk_sizes {
        request.extend_from_slice(format!("{size:x}\r\n").as_bytes());
        request.extend(std::iter::repeat_n(b' ', *size));
        request.extend_from_slice(b"\r\n");
    }
    request.extend_from_slice(b"0\r\n\r\n");
    stream.write_all(&request).await.unwrap();

    let mut response = Vec::new();
    // The server may close without a TLS `close_notify`, which surfaces as an EOF error.
    let _ = stream.read_to_end(&mut response).await;
    let response = String::from_utf8_lossy(&response);
    response
        .split_whitespace()
        .nth(1)
        .and_then(|status| StatusCode::from_bytes(status.as_bytes()).ok())
        .unwrap_or_else(|| panic!("unexpected response: {response}"))
}

fn free_addr() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
}

async fn wait_for_listener(addr: SocketAddr) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while tokio::net::TcpStream::connect(addr).await.is_err() {
        assert!(Instant::now() < deadline, "{addr} did not come up");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
