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
use clap::Parser;
use coprocessor_proxy::{
    core::{Config, Proxy},
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
/// [`TestProxy::answer_code`] for a signer the proxy accepts: the signature checks out, then the
/// signed digest does not match the empty body.
pub const ACCEPTED: &str = "auth_body_mismatch";
/// [`TestProxy::answer_code`] for a signer still unknown after one read of the KMS tx-senders.
pub const UNKNOWN_SIGNER: &str = "auth_unknown_signer";

/// The test proxy's own coprocessor signer address, the audience of the headers it accepts.
pub const AUDIENCE: Address = Address::repeat_byte(0xa1);
pub const CHAIN_ID: u64 = 12345;
pub const PROTOCOL_CONFIG: Address = Address::repeat_byte(0xc0);
/// How often the test proxy checks the finalized block.
pub const POLL_INTERVAL: Duration = Duration::from_millis(200);

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

/// A path the stub prf-service answers `429` on.
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

/// The bytes the stub prf-service answers for `path`: deterministic, and not UTF-8.
pub fn stub_output(path: &str) -> Vec<u8> {
    keccak256(path.as_bytes()).repeat(4)
}

/// The `X-Prf-Attestation` the stub prf-service answers for `path`.
pub fn stub_attestation(path: &str) -> String {
    json!({"version": 1, "sns_ciphertext_digest": hex::encode(keccak256(stub_output(path)))})
        .to_string()
}

/// What the stub prf-service saw of the last request it answered.
#[derive(Clone, Debug, Default)]
pub struct SeenRequest {
    pub version: Option<Version>,
    pub headers: HeaderMap,
}

#[derive(Default)]
struct StubState {
    hits: AtomicUsize,
    last: Mutex<SeenRequest>,
    /// The client addresses of the connections the requests came on.
    connections: Mutex<HashSet<SocketAddr>>,
    release: tokio::sync::Notify,
}

/// A stub prf-service over HTTP/1.1.
/// Test headers can delay, hold or interrupt a response.
pub struct StubPrfService {
    pub addr: SocketAddr,
    state: Arc<StubState>,
    server: AbortHandle,
}

impl StubPrfService {
    pub async fn start() -> Self {
        let state = Arc::new(StubState::default());
        let app = Router::new()
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

    /// Releases one request carrying `x-stub-hold`.
    pub fn release_held_request(&self) {
        self.state.release.notify_one();
    }

    pub fn last_request(&self) -> SeenRequest {
        self.state.last.lock().unwrap().clone()
    }

    /// The number of distinct connections the requests came on.
    pub fn connections(&self) -> usize {
        self.state.connections.lock().unwrap().len()
    }

    /// Stops prf-service for good: nothing listens on `addr` anymore.
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
) -> Response {
    state.hits.fetch_add(1, Ordering::SeqCst);
    state.connections.lock().unwrap().insert(client);
    *state.last.lock().unwrap() = SeenRequest {
        version: Some(version),
        headers: headers.clone(),
    };
    if headers.contains_key("x-stub-hold") {
        state.release.notified().await;
    }
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
    let output = stub_output(&path);
    if headers.contains_key("x-stub-fail-mid-body") {
        // The header and the first 32 bytes, then, once the proxy has passed them on, the stream
        // is reset.
        use futures_util::StreamExt;
        let first: axum::body::Bytes = output[..32].to_vec().into();
        let chunks = futures_util::stream::once(async { Ok(first) }).chain(
            futures_util::stream::once(async {
                tokio::time::sleep(Duration::from_millis(200)).await;
                Err(std::io::Error::other("prf-service failed"))
            }),
        );
        return (
            [("content-length", output.len().to_string())],
            axum::body::Body::from_stream(chunks),
        )
            .into_response();
    }
    (
        [
            ("content-type", "application/octet-stream".to_string()),
            ("x-prf-attestation", stub_attestation(&path)),
        ],
        output,
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

struct RpcState {
    contexts: Vec<KmsContext>,
    /// The number of the finalized block, the only block the contexts are known at.
    finalized: u64,
    /// How many times the finalized block was asked for: one per poll of the proxy.
    finalized_polls: usize,
    failing: bool,
    /// How long each answer is held after it is computed.
    reply_delay: Duration,
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
            finalized: 100,
            finalized_polls: 0,
            failing: false,
            reply_delay: Duration::ZERO,
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

    /// Replaces the contexts, as of a new finalized block.
    pub fn set_contexts(&self, contexts: Vec<KmsContext>) {
        let mut state = self.state.lock().unwrap();
        state.contexts = contexts;
        state.finalized += 1;
    }

    pub fn set_failing(&self, failing: bool) {
        self.state.lock().unwrap().failing = failing;
    }

    /// Holds every later answer for `delay` after computing it: the answer is the state at the
    /// time of the call.
    pub fn set_reply_delay(&self, delay: Duration) {
        self.state.lock().unwrap().reply_delay = delay;
    }

    /// How many times the finalized block was asked for: one per poll of the proxy.
    pub fn finalized_polls(&self) -> usize {
        self.state.lock().unwrap().finalized_polls
    }
}

/// Context ids as `ProtocolConfig` allocates them: the type tag 0x07 in the top byte.
fn context_id(index: usize) -> U256 {
    (U256::from(7) << 248) + U256::from(index + 1)
}

async fn rpc(State(state): State<Arc<Mutex<RpcState>>>, Json(call): Json<Value>) -> Response {
    let (reply, delay) = {
        let mut state = state.lock().unwrap();
        (answer(&mut state, &call), state.reply_delay)
    };
    tokio::time::sleep(delay).await;
    reply
}

fn answer(state: &mut RpcState, call: &Value) -> Response {
    if state.failing {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let id = &call["id"];
    if call["method"] == "eth_getBlockByNumber" && call["params"][0] == "finalized" {
        state.finalized_polls += 1;
        let mut block: Block = Block::default();
        block.header.inner.number = state.finalized;
        return Json(json!({"jsonrpc": "2.0", "id": id, "result": block})).into_response();
    }
    assert_eq!(call["method"], "eth_call", "unexpected RPC {call}");
    let tx = &call["params"][0];
    let to: Option<Address> = tx["to"].as_str().and_then(|to| to.parse().ok());
    // Only `ProtocolConfig` at the finalized block's number knows the contexts.
    if to != Some(PROTOCOL_CONFIG) || call["params"][1] != format!("{:#x}", state.finalized) {
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

/// Runs a `Proxy` for the lifetime of one test.
pub struct TestProxy {
    pub client: Client,
    pub addr: SocketAddr,
    pub tls: TestTls,
    pub prf_service: StubPrfService,
    pub rpc: FakeRpc,
    /// The healthcheck state of the proxy, to serve with `healthz_server::HttpServer`.
    pub state: health::State,
}

impl TestProxy {
    pub async fn start(prf_service: StubPrfService) -> Self {
        Self::start_with_config(prf_service, |_| {}).await
    }

    /// Starts the proxy with the first KMS context holding [`sender`] only, and waits until it
    /// has read it.
    pub async fn start_with_config(
        prf_service: StubPrfService,
        customize: impl FnOnce(&mut Config),
    ) -> Self {
        let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
        let t = Self::launch(rpc, prf_service, customize).await;
        t.wait_until_accepted(&sender()).await;
        t
    }

    /// Starts the proxy reading `rpc` and forwarding to `prf_service`, without waiting for its
    /// first read.
    pub async fn launch(
        rpc: FakeRpc,
        prf_service: StubPrfService,
        customize: impl FnOnce(&mut Config),
    ) -> Self {
        let addr = free_addr();
        let tls = TestTls::generate(addr);
        let address = prf_service.addr.to_string().parse().unwrap();
        let mut config = Self::config(addr, &tls, &rpc, address);
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
            prf_service,
            rpc,
            state,
        }
    }

    pub fn config(
        addr: SocketAddr,
        tls: &TestTls,
        rpc: &FakeRpc,
        prf_service_address: Authority,
    ) -> Config {
        let mut config = Config::parse_from([
            "coprocessor-proxy".to_string(),
            format!("--bind-address={addr}"),
            format!("--tls-cert-path={}", tls.cert_path.display()),
            format!("--tls-key-path={}", tls.key_path.display()),
            format!("--ethereum-chain-id={CHAIN_ID}"),
            format!("--protocol-config-address={PROTOCOL_CONFIG}"),
            format!("--ethereum-rpc-url={}", rpc.url),
            format!("--coprocessor-signer-address={AUDIENCE}"),
            format!("--prf-service-address={prf_service_address}"),
            "--prf-service-connect-timeout=500ms".to_string(),
            "--healthcheck-timeout=2s".to_string(),
        ]);
        config.kms_registry_refresh_interval = POLL_INTERVAL;
        config
    }

    pub fn url(&self, route: &str) -> String {
        format!("https://{}{route}", self.addr)
    }

    /// A `GET` of `route`, signed by [`sender`].
    pub async fn get(&self, route: &str) -> RequestBuilder {
        let auth = auth_header(&sender(), route, b"").await;
        self.client
            .get(self.url(route))
            .header("authorization", auth)
    }

    /// The code of the proxy's answer to a request signed by `signer` for a body it does not send:
    /// [`ACCEPTED`] once the proxy accepts `signer`, [`UNKNOWN_SIGNER`] before. The request never
    /// reaches prf-service.
    pub async fn answer_code(&self, signer: &PrivateKeySigner) -> String {
        let path = prf_path(1);
        let response = self
            .client
            .get(self.url(&path))
            .header("authorization", auth_header(signer, &path, b"x").await)
            .send()
            .await
            .unwrap();
        let status = response.status();
        let error: Value = response.json().await.unwrap();
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{error}");
        error["code"].as_str().unwrap().to_string()
    }

    /// Waits until the proxy accepts requests signed by `signer`.
    pub async fn wait_until_accepted(&self, signer: &PrivateKeySigner) {
        self.wait_for_code(signer, ACCEPTED).await;
    }

    /// Waits until a request signed by `signer` gets [`UNKNOWN_SIGNER`].
    pub async fn wait_until_refused(&self, signer: &PrivateKeySigner) {
        self.wait_for_code(signer, UNKNOWN_SIGNER).await;
    }

    async fn wait_for_code(&self, signer: &PrivateKeySigner, expected: &str) {
        let deadline = Instant::now() + 50 * POLL_INTERVAL;
        loop {
            let code = self.answer_code(signer).await;
            if code == expected {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "{}: {code}, not {expected}",
                signer.address()
            );
            tokio::time::sleep(POLL_INTERVAL / 2).await;
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
