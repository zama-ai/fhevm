//! Black-box harness: a `Proxy` run in-process behind real TLS, in front of the recording stub
//! endpoints of the conformance toolbox.

#![allow(dead_code)]

use alloy::transports::http::reqwest::{self, Client};
use fhevm_sig_proxy::{
    ErrorBody, Proxy, ProxyConfig, ProxyError, Route, SignerSource, TlsConfig, Upstream,
    conformance::{
        AUDIENCE, CHAIN_ID, ENDPOINT_HEALTHCHECK_FREQUENCY, PROTOCOL_CONFIG, Stub, TestTls,
        authorization, free_port, sender, wait_for_listener,
    },
    health,
};
use http::{Method, StatusCode};
use std::{collections::HashSet, net::SocketAddr, time::Duration};

/// The path the test proxy probes its endpoints on.
const HEALTHCHECK_PATH: &str = "/healthz";

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
