//! Black-box harness: a `Proxy` behind real TLS, in front of recording stub endpoints.

#![allow(dead_code)]

use alloy::{
    primitives::{Address, U256, hex},
    signers::local::PrivateKeySigner,
    sol_types::{SolCall, SolInterface},
    transports::http::reqwest::{self, Client, Url},
};
use bytes::Bytes;
use fhevm_host_bindings::i_protocol_config_base::IProtocolConfigBase::{
    IProtocolConfigBaseCalls, KmsNode, getCurrentKmsContextIdCounterCall,
    getKmsNodesForContextCall, isValidKmsContextCall,
};
use fhevm_sig_proxy::{
    ErrorBody, Proxy, ProxyConfig, ProxyError, Route, SignerSource, TlsConfig, Upstream, health,
};
use http::{HeaderMap, Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::{
    collections::HashSet,
    convert::Infallible,
    net::{SocketAddr, TcpListener as StdListener},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU16, AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::net::TcpListener;

pub const CHAIN_ID: u64 = 1;
pub const PROTOCOL_CONFIG: Address = Address::repeat_byte(0xc0);
/// The test proxy's own address, the audience of the headers it accepts.
pub const AUDIENCE: Address = Address::repeat_byte(0xa1);
/// How often the test proxy probes its endpoints in the background.
pub const ENDPOINT_HEALTHCHECK_FREQUENCY: Duration = Duration::from_millis(200);

/// The only allowed sender.
pub fn sender() -> PrivateKeySigner {
    "0x0101010101010101010101010101010101010101010101010101010101010101"
        .parse()
        .unwrap()
}

/// An `Authorization` value signed by the allowed sender for `path` and `body`.
pub async fn authorization(path: &str, body: &[u8]) -> String {
    let domain = fhevm_sig_auth::domain(CHAIN_ID, PROTOCOL_CONFIG);
    fhevm_sig_auth::sign_valid_for(
        &sender(),
        &domain,
        AUDIENCE,
        &path.parse().unwrap(),
        body,
        Duration::from_secs(60),
    )
    .await
    .unwrap()
    .to_string()
}

/// A body naming the error, with the status a deployment would give it.
pub fn error_body(error: &ProxyError) -> ErrorBody {
    let status = match error {
        ProxyError::NotOriginForm | ProxyError::NonCanonicalPath | ProxyError::BodyTooLarge => {
            StatusCode::BAD_REQUEST
        }
        ProxyError::Auth(_) => StatusCode::UNAUTHORIZED,
        ProxyError::EndpointUnavailable => StatusCode::BAD_GATEWAY,
        ProxyError::EndpointTimeout => StatusCode::GATEWAY_TIMEOUT,
    };
    ErrorBody {
        status,
        json: format!("{error:?}").into_bytes(),
    }
}

/// A request the stub read in full.
#[derive(Clone, Debug)]
pub struct Recorded {
    pub method: Method,
    pub target: String,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// What a stub received, without the background healthchecks.
#[derive(Default)]
struct Log {
    /// The target of each request whose head arrived.
    heads: Vec<String>,
    /// The requests read in full.
    requests: Vec<Recorded>,
}

/// A stub endpoint recording what it receives. It answers `200` with an empty body, after
/// `hold`.
pub struct Stub {
    pub addr: SocketAddr,
    log: Arc<Mutex<Log>>,
    hold: Arc<Mutex<Duration>>,
}

impl Stub {
    pub async fn start() -> Self {
        let log = Arc::new(Mutex::new(Log::default()));
        let hold = Arc::new(Mutex::new(Duration::ZERO));
        let (recorder, delay) = (log.clone(), hold.clone());
        let addr = serve(move |request| {
            let (recorder, delay) = (recorder.clone(), delay.clone());
            async move {
                let (parts, body) = request.into_parts();
                let target = parts.uri.to_string();
                if target == HEALTHCHECK_PATH {
                    return Response::new(Full::default());
                }
                recorder.lock().unwrap().heads.push(target.clone());
                // A request cut short is not recorded.
                if let Ok(body) = body.collect().await {
                    recorder.lock().unwrap().requests.push(Recorded {
                        method: parts.method,
                        target,
                        headers: parts.headers,
                        body: body.to_bytes(),
                    });
                }
                let hold = *delay.lock().unwrap();
                tokio::time::sleep(hold).await;
                Response::new(Full::default())
            }
        })
        .await;
        Self { addr, log, hold }
    }

    /// The targets of the requests whose head reached the stub, complete or not.
    pub fn heads(&self) -> Vec<String> {
        self.log.lock().unwrap().heads.clone()
    }

    /// The requests the stub read in full.
    pub fn requests(&self) -> Vec<Recorded> {
        self.log.lock().unwrap().requests.clone()
    }

    pub fn hold(&self, hold: Duration) {
        *self.hold.lock().unwrap() = hold;
    }
}

pub const HEALTHCHECK_PATH: &str = "/healthz";

/// Serves `handler` over plain HTTP/1.1 on a free local port.
async fn serve<F, Fut>(handler: F) -> SocketAddr
where
    F: Fn(Request<Incoming>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Response<Full<Bytes>>> + Send,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let handler = handler.clone();
            let service = service_fn(move |request| {
                let handler = handler.clone();
                async move { Ok::<_, Infallible>(handler(request).await) }
            });
            tokio::spawn(http1::Builder::new().serve_connection(TokioIo::new(stream), service));
        }
    });
    addr
}

/// One KMS context in the fake `ProtocolConfig`: Active or not, and its tx-senders.
#[derive(Clone)]
pub struct KmsContext {
    pub active: bool,
    pub tx_senders: Vec<Address>,
}

#[derive(Default)]
struct RpcState {
    contexts: Vec<KmsContext>,
    failing: bool,
}

/// An Ethereum JSON-RPC answering the `ProtocolConfig` calls of the signer reader from a list of
/// KMS contexts, or failing every call.
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
        let handler_state = state.clone();
        let addr = serve(move |request| {
            let state = handler_state.clone();
            async move { rpc(&state, request).await }
        })
        .await;
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

/// Context ids as `ProtocolConfig` allocates them: the type tag 0x07 in the top byte.
fn context_id(index: usize) -> U256 {
    (U256::from(7) << 248) + U256::from(index + 1)
}

async fn rpc(state: &Mutex<RpcState>, request: Request<Incoming>) -> Response<Full<Bytes>> {
    let body = request.into_body().collect().await.unwrap().to_bytes();
    let call: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let state = state.lock().unwrap();
    if state.failing {
        let mut response = Response::new(Full::default());
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return response;
    }
    assert_eq!(call["method"], "eth_call", "unexpected RPC {call}");
    let tx = &call["params"][0];
    let to: Option<Address> = tx["to"].as_str().and_then(|to| to.parse().ok());
    // Only `ProtocolConfig` at the finalized block knows the contexts.
    if to != Some(PROTOCOL_CONFIG) || call["params"][1] != "finalized" {
        let error = serde_json::json!({"jsonrpc": "2.0", "id": call["id"],
            "error": {"code": -32000, "message": format!("unexpected call {call}")}});
        return Response::new(Full::new(Bytes::from(error.to_string())));
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
    let id = &call["id"];
    let reply = match result {
        Some(result) => {
            serde_json::json!({"jsonrpc": "2.0", "id": id, "result": hex::encode_prefixed(result)})
        }
        None => serde_json::json!({"jsonrpc": "2.0", "id": id,
            "error": {"code": 3, "message": "execution reverted: InvalidKmsContext"}}),
    };
    Response::new(Full::new(Bytes::from(reply.to_string())))
}

/// Two upstreams `a` and `b`, with routes differing in methods, template, body cap and timeout:
/// - `POST /small` to `a`, no body, 300 ms;
/// - `POST /large` to `b`, up to 1024 bytes, 5 s;
/// - `GET|HEAD /items/{id:u16}` to `b`, no body, 5 s.
pub fn config(addr: SocketAddr, tls: &TestTls, a: &Stub, b: &Stub) -> ProxyConfig {
    let route =
        |methods: &[Method], path: &str, upstream: &str, max_body_bytes, timeout_ms| Route {
            methods: methods.to_vec(),
            path: path.to_owned(),
            upstream: upstream.to_owned(),
            max_body_bytes,
            endpoint_response_timeout: Duration::from_millis(timeout_ms),
        };
    let upstream = |name: &str, stub: &Stub| Upstream {
        name: name.to_owned(),
        endpoint_addresses: vec![stub.addr.to_string().parse().unwrap()],
        healthcheck_path: HEALTHCHECK_PATH.to_owned(),
    };
    ProxyConfig {
        bind_address: addr,
        tls_config: TlsConfig {
            cert_path: tls.cert_path.clone(),
            key_path: tls.key_path.clone(),
        },
        ethereum_chain_id: CHAIN_ID,
        protocol_config_address: PROTOCOL_CONFIG,
        signers: SignerSource::Static(HashSet::from([sender().address()])),
        audience: AUDIENCE,
        error_body,
        forward_authorization: false,
        routes: vec![
            route(&[Method::POST], "/small", "a", 0, 300),
            route(&[Method::POST], "/large", "b", 1024, 5000),
            route(
                &[Method::GET, Method::HEAD],
                "/items/{id:u16}",
                "b",
                0,
                5000,
            ),
        ],
        upstreams: vec![upstream("a", a), upstream("b", b)],
        endpoint_connect_timeout: Duration::from_millis(500),
        endpoint_healthcheck_frequency: ENDPOINT_HEALTHCHECK_FREQUENCY,
        endpoint_idle_timeout: Duration::from_secs(10),
        request_read_timeout: Duration::from_secs(5),
        shutdown_grace_period: Duration::from_secs(1),
        service_name: "test-proxy".to_owned(),
        healthcheck_timeout: Duration::from_secs(2),
    }
}

/// Runs a `Proxy` for the lifetime of one test.
pub struct TestProxy {
    pub addr: SocketAddr,
    pub client: Client,
    pub a: Stub,
    pub b: Stub,
    pub state: health::State,
    _tls: TestTls,
}

impl TestProxy {
    pub async fn start(customize: impl FnOnce(&mut ProxyConfig)) -> Self {
        let (a, b) = (Stub::start().await, Stub::start().await);
        let addr = SocketAddr::from(([127, 0, 0, 1], free_port()));
        let tls = TestTls::generate();
        let mut config = config(addr, &tls, &a, &b);
        customize(&mut config);
        let (proxy, state) = Proxy::from_config(config).unwrap();
        std::thread::spawn(move || proxy.run());
        wait_for_listener(addr).await;
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .tls_danger_accept_invalid_certs(true) // self-signed test certificate
            .build()
            .unwrap();
        Self {
            addr,
            client,
            a,
            b,
            state,
            _tls: tls,
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("https://{}{path}", self.addr)
    }

    /// A request of `path` with `body`, signed by the allowed sender.
    pub async fn signed(&self, method: Method, path: &str, body: &[u8]) -> reqwest::Response {
        let authorization = authorization(path, body).await;
        self.client
            .request(method, self.url(path))
            .header("authorization", authorization)
            .body(body.to_vec())
            .send()
            .await
            .unwrap()
    }
}

/// A self-signed certificate in its own temporary directory.
pub struct TestTls {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl TestTls {
    pub fn generate() -> Self {
        static NEXT_DIR: AtomicU32 = AtomicU32::new(0);
        let certified = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into()]).unwrap();
        let dir = std::env::temp_dir().join(format!(
            "fhevm-sig-proxy-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let (cert_path, key_path) = (dir.join("tls.crt"), dir.join("tls.key"));
        std::fs::write(&cert_path, certified.cert.pem()).unwrap();
        std::fs::write(&key_path, certified.signing_key.serialize_pem()).unwrap();
        Self {
            cert_path,
            key_path,
        }
    }
}

impl Drop for TestTls {
    fn drop(&mut self) {
        if let Some(dir) = self.cert_path.parent() {
            std::fs::remove_dir_all(dir).ok();
        }
    }
}

/// A port nothing listens on, below the ephemeral ranges (Linux 32768, macOS 49152) so that no
/// client socket takes it before the proxy binds it. Each test process counts through its own
/// block of 250 ports.
pub fn free_port() -> u16 {
    static NEXT: AtomicU16 = AtomicU16::new(0);
    let block = 20_000 + (std::process::id() % 40) as u16 * 250;
    loop {
        let port = block + NEXT.fetch_add(1, Ordering::Relaxed) % 250;
        if StdListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
}

pub async fn wait_for_listener(addr: SocketAddr) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while tokio::net::TcpStream::connect(addr).await.is_err() {
        assert!(Instant::now() < deadline, "{addr} did not come up");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
