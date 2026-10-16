//! Helpers shared by the proxy integration tests.
#![allow(dead_code)]

use alloy::{
    hex,
    primitives::{Address, U256, keccak256},
    rpc::types::Block,
    signers::local::PrivateKeySigner,
    sol_types::{SolCall, SolInterface},
    transports::http::reqwest::{Client, RequestBuilder, StatusCode, Url},
};
use axum::{
    Json, Router,
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, Version},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use coprocessor_proxy::{
    core::{Config, Proxy, TlsConfig},
    monitoring::health,
};
use fhevm_host_bindings::i_protocol_config_base::IProtocolConfigBase::{
    IProtocolConfigBaseCalls, KmsNode, getCurrentKmsContextIdCounterCall,
    getKmsNodesForContextCall, isValidKmsContextCall,
};
use http::uri::{Authority, PathAndQuery};
use serde_json::{Value, json};
use std::{
    collections::HashSet,
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::task::AbortHandle;

/// The private key of the KMS tx-sender in the first KMS context.
pub const SENDER_KEY: &str = "0x0101010101010101010101010101010101010101010101010101010101010101";
/// The test proxy's own coprocessor signer address, the audience of the headers it accepts.
pub const AUDIENCE: Address = Address::repeat_byte(0xa1);
pub const CHAIN_ID: u64 = 12345;
pub const PROTOCOL_CONFIG: Address = Address::repeat_byte(0xc0);
/// How often the test proxy reads the KMS tx-senders.
pub const REFRESH_INTERVAL: Duration = Duration::from_millis(200);

/// The KMS tx-sender of the first KMS context.
pub fn sender() -> PrivateKeySigner {
    SENDER_KEY.parse().unwrap()
}

pub fn now() -> u64 {
    fhevm_sig_auth::unix_now()
}

/// The PRF route path of the label made of `label_byte`, for 64 pad bits.
pub fn prf_path(label_byte: u8) -> String {
    format!("/v1/prf/1/labels/{}/64", hex::encode([label_byte; 32]))
}

/// A path the stub endpoint answers `429` on.
pub fn rate_limited_path() -> String {
    prf_path(RATE_LIMITED_LABEL_BYTE)
}

const RATE_LIMITED_LABEL_BYTE: u8 = 0x29;

/// An `Authorization` value signed by `signer` for the test proxy, expiring at `expires`.
pub async fn auth_header_expiring(
    signer: &PrivateKeySigner,
    path: &str,
    body: &[u8],
    expires: u64,
) -> String {
    auth_header_for(signer, AUDIENCE, path, body, expires).await
}

/// An `Authorization` value signed by `signer` for `audience`, expiring at `expires`.
pub async fn auth_header_for(
    signer: &PrivateKeySigner,
    audience: Address,
    path: &str,
    body: &[u8],
    expires: u64,
) -> String {
    let domain = fhevm_sig_auth::domain(CHAIN_ID, PROTOCOL_CONFIG);
    let path = PathAndQuery::try_from(path).unwrap();
    fhevm_sig_auth::sign(signer, &domain, audience, &path, body, expires)
        .await
        .unwrap()
        .to_string()
}

/// An `Authorization` value signed by `signer` for the test proxy, valid for one minute.
pub async fn auth_header(signer: &PrivateKeySigner, path: &str, body: &[u8]) -> String {
    auth_header_expiring(signer, path, body, now() + 60).await
}

/// The bytes the stub endpoint answers for `path`: deterministic, and not UTF-8.
pub fn stub_output(path: &str) -> Vec<u8> {
    keccak256(path.as_bytes()).repeat(4)
}

/// The `X-Prf-Attestation` the stub endpoint answers for `path`.
pub fn stub_attestation(path: &str) -> String {
    json!({"version": 1, "sns_ciphertext_digest": hex::encode(keccak256(stub_output(path)))})
        .to_string()
}

/// What the stub endpoint saw of the last request it answered.
#[derive(Clone, Debug, Default)]
pub struct SeenRequest {
    pub version: Option<Version>,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

#[derive(Default)]
struct StubState {
    hits: AtomicUsize,
    last: Mutex<SeenRequest>,
    /// The client addresses of the connections the requests came on.
    connections: Mutex<HashSet<SocketAddr>>,
    /// The HTTP versions of the `/healthz` probes.
    probe_versions: Mutex<Vec<Version>>,
}

/// A stub prf-service endpoint, over HTTP/1.1 and HTTP/2 cleartext.
///
/// It holds a request for the milliseconds of an `x-stub-delay-ms` header, answers `429` with
/// `Retry-After` on [`rate_limited_path`], and [`stub_output`] with [`stub_attestation`] otherwise.
pub struct StubEndpoint {
    pub addr: SocketAddr,
    state: Arc<StubState>,
    server: AbortHandle,
}

impl StubEndpoint {
    pub async fn start() -> Self {
        let state = Arc::new(StubState::default());
        let app = Router::new()
            .route(
                "/healthz",
                get(
                    |State(state): State<Arc<StubState>>, version: Version| async move {
                        state.probe_versions.lock().unwrap().push(version);
                        "ok"
                    },
                ),
            )
            .route("/v1/prf/:prf/labels/:label/:pad_length", get(prf))
            .with_state(Arc::clone(&state));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        })
        .abort_handle();
        Self {
            addr,
            state,
            server,
        }
    }

    pub fn hits(&self) -> usize {
        self.state.hits.load(Ordering::SeqCst)
    }

    pub fn last_request(&self) -> SeenRequest {
        self.state.last.lock().unwrap().clone()
    }

    /// The HTTP versions of the `/healthz` probes received so far.
    pub fn probe_versions(&self) -> Vec<Version> {
        self.state.probe_versions.lock().unwrap().clone()
    }

    /// The number of distinct connections the requests came on.
    pub fn connections(&self) -> usize {
        self.state.connections.lock().unwrap().len()
    }

    /// Stops the endpoint for good: nothing listens on `addr` anymore.
    pub async fn stop(&self) {
        self.server.abort();
        while tokio::net::TcpStream::connect(self.addr).await.is_ok() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }
}

async fn prf(
    State(state): State<Arc<StubState>>,
    ConnectInfo(client): ConnectInfo<SocketAddr>,
    Path((prf, label, pad_length)): Path<(String, String, String)>,
    version: Version,
    headers: HeaderMap,
    // Read as prf-service does to check it against the signed digest: a body the proxy cuts
    // short is never handled.
    body: axum::body::Bytes,
) -> Response {
    state.hits.fetch_add(1, Ordering::SeqCst);
    state.connections.lock().unwrap().insert(client);
    *state.last.lock().unwrap() = SeenRequest {
        version: Some(version),
        headers: headers.clone(),
        body: body.to_vec(),
    };
    if let Some(delay_ms) = headers
        .get("x-stub-delay-ms")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
    {
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }
    let path = format!("/v1/prf/{prf}/labels/{label}/{pad_length}");
    if path == rate_limited_path() {
        let body = json!({"code": "concurrency_limited", "message": "busy", "retryable": true});
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [("retry-after", "2")],
            Json(body),
        )
            .into_response();
    }
    (
        [
            ("content-type", "application/octet-stream".to_string()),
            ("x-prf-attestation", stub_attestation(&path)),
        ],
        stub_output(&path),
    )
        .into_response()
}

/// One KMS context in the fake `ProtocolConfig`: Active or not, and its tx-senders.
#[derive(Clone)]
pub struct KmsContext {
    pub active: bool,
    pub tx_senders: Vec<Address>,
}

impl KmsContext {
    pub fn active(tx_senders: &[Address]) -> Self {
        Self {
            active: true,
            tx_senders: tx_senders.to_vec(),
        }
    }

    pub fn destroyed(tx_senders: &[Address]) -> Self {
        Self {
            active: false,
            tx_senders: tx_senders.to_vec(),
        }
    }
}

#[derive(Default)]
struct RpcState {
    contexts: Vec<KmsContext>,
    failing: bool,
}

/// An Ethereum JSON-RPC answering the `ProtocolConfig` reads of the proxy from a list of KMS
/// contexts, or failing every call.
pub struct FakeRpc {
    pub url: Url,
    state: Arc<Mutex<RpcState>>,
}

impl FakeRpc {
    pub async fn start(contexts: Vec<KmsContext>) -> Self {
        let state = Arc::new(Mutex::new(RpcState {
            contexts,
            failing: false,
        }));
        let app = Router::new()
            .route("/", post(rpc))
            .with_state(Arc::clone(&state));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        Self {
            url: format!("http://{addr}").parse().unwrap(),
            state,
        }
    }

    pub fn set_contexts(&self, contexts: Vec<KmsContext>) {
        self.state.lock().unwrap().contexts = contexts;
    }

    pub fn set_failing(&self, failing: bool) {
        self.state.lock().unwrap().failing = failing;
    }
}

/// The number of the finalized block, the only block the contexts are known at.
const FINALIZED: u64 = 100;

/// Context ids as `ProtocolConfig` allocates them: the type tag 0x07 in the top byte.
fn context_id(index: usize) -> U256 {
    (U256::from(7) << 248) + U256::from(index + 1)
}

async fn rpc(State(state): State<Arc<Mutex<RpcState>>>, Json(call): Json<Value>) -> Response {
    let state = state.lock().unwrap();
    if state.failing {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let id = &call["id"];
    if call["method"] == "eth_getBlockByNumber" && call["params"][0] == "finalized" {
        let mut block: Block = Block::default();
        block.header.inner.number = FINALIZED;
        return Json(json!({"jsonrpc": "2.0", "id": id, "result": block})).into_response();
    }
    assert_eq!(call["method"], "eth_call", "unexpected RPC {call}");
    let tx = &call["params"][0];
    let to: Option<Address> = tx["to"].as_str().and_then(|to| to.parse().ok());
    // Only `ProtocolConfig` at the finalized block's number knows the contexts.
    if to != Some(PROTOCOL_CONFIG) || call["params"][1] != format!("{FINALIZED:#x}") {
        return Json(json!({"jsonrpc": "2.0", "id": id,
            "error": {"code": -32000, "message": format!("unexpected call {call}")}}))
        .into_response();
    }
    let data = tx["input"].as_str().or(tx["data"].as_str()).unwrap();
    let context = |id: U256| {
        (0..state.contexts.len())
            .find(|&index| context_id(index) == id)
            .map(|index| &state.contexts[index])
    };
    let result = match IProtocolConfigBaseCalls::abi_decode(&hex::decode(data).unwrap()).unwrap() {
        IProtocolConfigBaseCalls::getCurrentKmsContextIdCounter(_) => {
            let counter = match state.contexts.len() {
                0 => U256::ZERO,
                n => context_id(n - 1),
            };
            Some(getCurrentKmsContextIdCounterCall::abi_encode_returns(
                &counter,
            ))
        }
        IProtocolConfigBaseCalls::isValidKmsContext(call) => {
            let active = context(call.kmsContextId).is_some_and(|c| c.active);
            Some(isValidKmsContextCall::abi_encode_returns(&active))
        }
        // As the contract: reverts unless the context is Active.
        IProtocolConfigBaseCalls::getKmsNodesForContext(call) => {
            context(call.kmsContextId).filter(|c| c.active).map(|c| {
                let nodes: Vec<KmsNode> = c
                    .tx_senders
                    .iter()
                    .map(|&tx_sender| KmsNode {
                        txSenderAddress: tx_sender,
                        ..Default::default()
                    })
                    .collect();
                getKmsNodesForContextCall::abi_encode_returns(&nodes)
            })
        }
        _ => panic!("unexpected ProtocolConfig call"),
    };
    let reply = match result {
        Some(result) => json!({"jsonrpc": "2.0", "id": id, "result": hex::encode_prefixed(result)}),
        None => json!({"jsonrpc": "2.0", "id": id,
            "error": {"code": 3, "message": "execution reverted: InvalidKmsContext"}}),
    };
    Json(reply).into_response()
}

/// How often the test proxy probes its endpoints in the background.
pub const ENDPOINT_HEALTHCHECK_FREQUENCY: Duration = Duration::from_millis(200);

/// Runs a `Proxy` for the lifetime of one test.
pub struct TestProxy {
    pub client: Client,
    pub addr: SocketAddr,
    pub tls: TestTls,
    pub endpoints: Vec<StubEndpoint>,
    pub rpc: FakeRpc,
    /// The healthcheck state of the proxy, to serve with `healthz_server::HttpServer`.
    pub state: health::State,
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

    /// Starts the proxy with the first KMS context holding [`sender`] only, and waits until it
    /// has read it.
    pub async fn start_with_config(
        endpoint_addresses: Vec<Authority>,
        endpoints: Vec<StubEndpoint>,
        customize: impl FnOnce(&mut Config),
    ) -> Self {
        let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
        let t = Self::launch(rpc, endpoint_addresses, endpoints, customize).await;
        t.wait_until_accepted(&sender()).await;
        t
    }

    /// Starts the proxy reading `rpc`, without waiting for its first read.
    pub async fn launch(
        rpc: FakeRpc,
        endpoint_addresses: Vec<Authority>,
        endpoints: Vec<StubEndpoint>,
        customize: impl FnOnce(&mut Config),
    ) -> Self {
        let addr = free_addr();
        let tls = TestTls::generate(addr);
        let mut config = Self::config(addr, &tls, &rpc, endpoint_addresses);
        customize(&mut config);
        let (proxy, state) = Proxy::from_config(config).unwrap();
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
            rpc,
            state,
        }
    }

    pub fn config(
        addr: SocketAddr,
        tls: &TestTls,
        rpc: &FakeRpc,
        endpoint_addresses: Vec<Authority>,
    ) -> Config {
        Config {
            bind_address: addr,
            tls_config: TlsConfig {
                cert_path: tls.cert_path.clone(),
                key_path: tls.key_path.clone(),
            },
            ethereum_chain_id: CHAIN_ID,
            protocol_config_address: PROTOCOL_CONFIG,
            ethereum_rpc_url: rpc.url.clone(),
            kms_registry_refresh_interval: REFRESH_INTERVAL,
            coprocessor_signer_address: AUDIENCE,
            endpoint_addresses,
            endpoint_connect_timeout: Duration::from_millis(500),
            endpoint_healthcheck_frequency: ENDPOINT_HEALTHCHECK_FREQUENCY,
            healthcheck_timeout: Duration::from_secs(2),
            ..Config::default()
        }
    }

    pub fn url(&self, route: &str) -> String {
        format!("https://{}{route}", self.addr)
    }

    /// A `GET` of `route`, signed by [`sender`].
    pub async fn get(&self, route: &str) -> RequestBuilder {
        self.get_signed_by(&sender(), route).await
    }

    /// A `GET` of `route`, signed by `signer`.
    pub async fn get_signed_by(&self, signer: &PrivateKeySigner, route: &str) -> RequestBuilder {
        let auth = auth_header(signer, route, b"").await;
        self.client
            .get(self.url(route))
            .header("authorization", auth)
    }

    /// The `auth_*` code of the proxy's answer to a request signed by `signer`, `None` when
    /// the request got through.
    pub async fn auth_rejection(&self, signer: &PrivateKeySigner) -> Option<String> {
        let response = self
            .get_signed_by(signer, &prf_path(1))
            .await
            .send()
            .await
            .unwrap();
        if response.status() != StatusCode::UNAUTHORIZED {
            return None;
        }
        let error: Value = response.json().await.unwrap();
        Some(error["code"].as_str().unwrap().to_string())
    }

    /// Waits until a request signed by `signer` gets through the authentication.
    pub async fn wait_until_accepted(&self, signer: &PrivateKeySigner) {
        let deadline = Instant::now() + 50 * REFRESH_INTERVAL;
        while let Some(code) = self.auth_rejection(signer).await {
            assert!(
                Instant::now() < deadline,
                "{} still refused: {code}",
                signer.address()
            );
            tokio::time::sleep(REFRESH_INTERVAL / 2).await;
        }
    }

    /// Waits until a request signed by `signer` is refused as `auth_unknown_signer`.
    pub async fn wait_until_refused(&self, signer: &PrivateKeySigner) {
        let deadline = Instant::now() + 50 * REFRESH_INTERVAL;
        while self.auth_rejection(signer).await.as_deref() != Some("auth_unknown_signer") {
            assert!(
                Instant::now() < deadline,
                "{} still accepted",
                signer.address()
            );
            tokio::time::sleep(REFRESH_INTERVAL / 2).await;
        }
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
