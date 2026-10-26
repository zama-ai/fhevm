//! Helpers shared by the proxy integration tests.
#![allow(dead_code)]

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, dev::ServerHandle, web};
use alloy::{
    primitives::Address,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::{Client, RequestBuilder},
};
use connector_utils::config::{ContractConfig, contract::default_protocol_config_contract_config};
use http::uri::{Authority, PathAndQuery};
use kms_connector_api::{
    ErrorCode, ErrorResponse, PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE,
    VersionResponse,
};
use proxy::{
    core::{Config, Proxy, TlsConfig},
    monitoring::health::State,
};
use serde_json::json;
use std::{
    collections::HashSet,
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

pub const MAX_BODY_BYTES: usize = 1024;
/// The private key of the only allowed sender.
pub const SENDER_KEY: &str = "0x0101010101010101010101010101010101010101010101010101010101010101";
/// The test proxy's own address, the audience of the headers it accepts.
pub const AUDIENCE: Address = Address::repeat_byte(0xa1);
pub const CHAIN_ID: u64 = 1;
pub const PROTOCOL_CONFIG: Address = Address::repeat_byte(0xc0);

/// The only allowed sender.
pub fn sender() -> PrivateKeySigner {
    SENDER_KEY.parse().unwrap()
}

pub fn now() -> u64 {
    fhevm_sig_auth::unix_now()
}

/// An `Authorization` value signed by `signer` for the test proxy, expiring at `expires`.
pub async fn auth_header_expiring(
    signer: &PrivateKeySigner,
    path: &str,
    body: &[u8],
    expires: u64,
) -> String {
    let domain = fhevm_sig_auth::domain(CHAIN_ID, PROTOCOL_CONFIG);
    let path = PathAndQuery::try_from(path).unwrap();
    fhevm_sig_auth::sign(signer, &domain, AUDIENCE, &path, body, expires)
        .await
        .unwrap()
        .to_string()
}

/// An `Authorization` value signed by `signer` for the test proxy, valid for one minute.
pub async fn auth_header(signer: &PrivateKeySigner, path: &str, body: &[u8]) -> String {
    auth_header_expiring(signer, path, body, now() + 60).await
}

/// A stub Connector endpoint echoing what it receives.
pub struct StubEndpoint {
    pub addr: SocketAddr,
    pub hits: Arc<std::sync::atomic::AtomicUsize>,
    handle: ServerHandle,
}

impl StubEndpoint {
    pub async fn start() -> Self {
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
        let handle = server.handle();
        tokio::spawn(server);
        Self { addr, hits, handle }
    }

    pub fn hits(&self) -> usize {
        self.hits.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Stops the endpoint for good: nothing listens on `addr` anymore.
    pub async fn stop(&self) {
        self.handle.stop(false).await;
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

/// How often the test proxy probes its endpoints in the background.
pub const ENDPOINT_HEALTHCHECK_FREQUENCY: Duration = Duration::from_millis(200);

/// Runs a `Proxy` for the lifetime of one test.
pub struct TestProxy {
    pub client: Client,
    pub addr: SocketAddr,
    pub tls: TestTls,
    pub endpoints: Vec<StubEndpoint>,
    /// The healthcheck state of the proxy, to serve with `start_monitoring_server`.
    pub state: State,
}

impl TestProxy {
    pub async fn start(endpoints: Vec<StubEndpoint>) -> Self {
        let addresses = endpoints
            .iter()
            .map(|u| u.addr.to_string().parse().unwrap())
            .collect();
        Self::start_with_addresses(addresses, endpoints).await
    }

    pub async fn start_with_addresses(
        endpoint_addresses: Vec<Authority>,
        endpoints: Vec<StubEndpoint>,
    ) -> Self {
        Self::start_with_config(endpoint_addresses, endpoints, |_| {}).await
    }

    pub async fn start_with_config(
        endpoint_addresses: Vec<Authority>,
        endpoints: Vec<StubEndpoint>,
        customize: impl FnOnce(&mut Config),
    ) -> Self {
        let addr = free_addr();
        let tls = TestTls::generate(addr);
        let mut config = Self::config(addr, &tls, endpoint_addresses);
        customize(&mut config);
        let (proxy, state) = Proxy::from_config(config).unwrap();
        proxy.register_metrics();
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
            state,
        }
    }

    pub fn config(addr: SocketAddr, tls: &TestTls, endpoint_addresses: Vec<Authority>) -> Config {
        Config {
            bind_address: addr,
            tls_config: TlsConfig {
                cert_path: tls.cert_path.clone(),
                key_path: tls.key_path.clone(),
            },
            ethereum_chain_id: CHAIN_ID,
            protocol_config_contract: ContractConfig {
                address: PROTOCOL_CONFIG,
                ..default_protocol_config_contract_config()
            },
            allowed_senders: HashSet::from([sender().address()]),
            tx_sender_address: AUDIENCE,
            endpoint_addresses,
            endpoint_connect_timeout: Duration::from_millis(500),
            endpoint_healthcheck_frequency: ENDPOINT_HEALTHCHECK_FREQUENCY,
            max_body_bytes: MAX_BODY_BYTES,
            healthcheck_timeout: Duration::from_secs(2),
            ..Config::default()
        }
    }

    pub fn url(&self, route: &str) -> String {
        format!("https://{}{route}", self.addr)
    }

    /// A `POST` of `body` to `route`, signed by the allowed sender.
    pub async fn post(&self, route: &str, body: impl Into<Vec<u8>>) -> RequestBuilder {
        let body = body.into();
        let auth = auth_header(&sender(), route, &body).await;
        self.client
            .post(self.url(route))
            .header("authorization", auth)
            .body(body)
    }

    /// A `GET` of `route`, signed by the allowed sender.
    pub async fn get(&self, route: &str) -> RequestBuilder {
        let auth = auth_header(&sender(), route, b"").await;
        self.client
            .get(self.url(route))
            .header("authorization", auth)
    }
}

/// A self-signed certificate generated for one test proxy instance.
pub struct TestTls {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub cert_der: rustls::pki_types::CertificateDer<'static>,
}

impl TestTls {
    pub fn generate(addr: SocketAddr) -> Self {
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
    pub async fn connect(
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

pub fn free_addr() -> SocketAddr {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
}

pub async fn wait_for_listener(addr: SocketAddr) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while tokio::net::TcpStream::connect(addr).await.is_err() {
        assert!(Instant::now() < deadline, "{addr} did not come up");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
