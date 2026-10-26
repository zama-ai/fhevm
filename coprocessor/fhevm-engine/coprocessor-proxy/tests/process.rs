//! Tests that run the proxy binary: its shutdown and its logs.
#![cfg(unix)]

mod common;

use alloy::transports::http::reqwest::{Client, RequestBuilder, Response, StatusCode};
use common::{
    AUDIENCE, CHAIN_ID, FakeRpc, KmsContext, PROTOCOL_CONFIG, StubPrfService, TestTls, auth_header,
    free_addr, prf_path, sender, stub_output,
};
use std::{
    fs,
    net::SocketAddr,
    path::PathBuf,
    process::{ExitStatus, Stdio},
    time::Duration,
};
use tokio::process::{Child, Command};

/// What Pingora logs once it stops accepting connections and starts the shutdown grace period.
const PINGORA_HANDLED_SIGTERM: &str = "SIGTERM received, gracefully exiting";

/// The proxy binary, run until it is ready to serve.
struct ProxyProcess {
    child: Child,
    addr: SocketAddr,
    log_path: PathBuf,
    _tls: TestTls,
}

impl ProxyProcess {
    async fn start(prf_service: &StubPrfService, rpc: &FakeRpc, env: &[(&str, &str)]) -> Self {
        let addr = free_addr();
        let health = free_addr();
        let metrics = free_addr();
        let tls = TestTls::generate(addr);
        let log_path = tls.cert_path.with_extension("log");
        let log = fs::File::create(&log_path).unwrap();
        let mut child = Command::new(env!("CARGO_BIN_EXE_coprocessor-proxy"))
            .kill_on_drop(true)
            .arg("start")
            .env("COPROCESSOR_PROXY_BIND_ADDRESS", addr.to_string())
            .env("COPROCESSOR_PROXY_TLS_CONFIG__CERT_PATH", &tls.cert_path)
            .env("COPROCESSOR_PROXY_TLS_CONFIG__KEY_PATH", &tls.key_path)
            .env("COPROCESSOR_PROXY_ETHEREUM_CHAIN_ID", CHAIN_ID.to_string())
            .env(
                "COPROCESSOR_PROXY_PROTOCOL_CONFIG_ADDRESS",
                PROTOCOL_CONFIG.to_string(),
            )
            .env("COPROCESSOR_PROXY_ETHEREUM_RPC_URL", rpc.url.as_str())
            .env(
                "COPROCESSOR_PROXY_COPROCESSOR_SIGNER_ADDRESS",
                AUDIENCE.to_string(),
            )
            .env(
                "COPROCESSOR_PROXY_PRF_SERVICE_ADDRESS",
                prf_service.addr.to_string(),
            )
            .envs(env.iter().copied())
            .env(
                "COPROCESSOR_PROXY_HEALTHCHECK_PORT",
                health.port().to_string(),
            )
            .env("COPROCESSOR_PROXY_METRICS_ENDPOINT", metrics.to_string())
            .env_remove("OTEL_EXPORTER_OTLP_ENDPOINT")
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .spawn()
            .unwrap();
        let client = client();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        loop {
            assert!(
                child.try_wait().unwrap().is_none(),
                "{}",
                fs::read_to_string(&log_path).unwrap()
            );
            if let Ok(response) = client.get(format!("http://{health}/healthz")).send().await
                && response.status() == StatusCode::OK
                && response.json::<serde_json::Value>().await.unwrap()["details"] == ""
            {
                break;
            }
            assert!(
                tokio::time::Instant::now() < deadline,
                "proxy never became ready"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        Self {
            child,
            addr,
            log_path,
            _tls: tls,
        }
    }

    async fn prf_request(&self, client: &Client, path: &str) -> RequestBuilder {
        client
            .get(format!("https://{}{path}", self.addr))
            .header("authorization", auth_header(&sender(), path, b"").await)
    }

    fn log(&self) -> String {
        fs::read_to_string(&self.log_path).unwrap()
    }

    async fn sigterm(&self) {
        assert!(
            Command::new("kill")
                .args(["-TERM", &self.child.id().unwrap().to_string()])
                .status()
                .await
                .unwrap()
                .success()
        );
    }

    /// Waits, while the proxy runs, until its log contains `line`.
    async fn wait_for_log(&mut self, line: &str, within: Duration) {
        tokio::time::timeout(within, async {
            while !self.log().contains(line) {
                assert!(
                    self.child.try_wait().unwrap().is_none(),
                    "proxy exited before logging {line:?}"
                );
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap_or_else(|_| panic!("proxy logged {line:?} within {within:?}"));
    }

    async fn wait_for_exit(&mut self, within: Duration) -> ExitStatus {
        tokio::time::timeout(within, self.child.wait())
            .await
            .expect("proxy exited after draining")
            .unwrap()
    }
}

/// A client with its own connection pool.
fn client() -> Client {
    Client::builder()
        .tls_danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap()
}

fn connection_header(response: &Response) -> &str {
    response.headers()["connection"].to_str().unwrap()
}

#[tokio::test]
async fn sigterm_drains_a_held_request_and_exits_cleanly() {
    let prf_service = StubPrfService::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    let mut proxy = ProxyProcess::start(
        &prf_service,
        &rpc,
        &[
            ("COPROCESSOR_PROXY_PRF_SERVICE_RESPONSE_TIMEOUT", "20s"),
            ("COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD", "5s"),
        ],
    )
    .await;
    let path = prf_path(1);
    let request = proxy
        .prf_request(&client(), &path)
        .await
        .header("x-stub-hold", "1");
    let reply = tokio::spawn(async move { request.send().await.unwrap() });
    tokio::time::timeout(Duration::from_secs(5), async {
        while prf_service.hits() == 0 {
            assert!(
                !reply.is_finished(),
                "request ended before reaching prf-service"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("request reached prf-service");

    proxy.sigterm().await;
    // Observe the real signal handler before releasing the request.
    proxy
        .wait_for_log(PINGORA_HANDLED_SIGTERM, Duration::from_secs(3))
        .await;
    assert!(
        !reply.is_finished(),
        "request must remain held across SIGTERM"
    );
    prf_service.release_held_request();
    let response = reply.await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.bytes().await.unwrap(), stub_output(&path));
    let exit = proxy.wait_for_exit(Duration::from_secs(12)).await;
    assert!(exit.success(), "{exit}: {}", proxy.log());
}

#[tokio::test]
async fn sigterm_keeps_serving_with_connection_close_for_the_shutdown_delay() {
    let prf_service = StubPrfService::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    let mut proxy = ProxyProcess::start(
        &prf_service,
        &rpc,
        &[
            ("COPROCESSOR_PROXY_SHUTDOWN_DELAY", "3s"),
            ("COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD", "1s"),
        ],
    )
    .await;
    let path = prf_path(1);
    let kept_alive = client();
    let response = proxy.prf_request(&kept_alive, &path).await.send().await;
    let response = response.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(connection_header(&response), "keep-alive");
    response.bytes().await.unwrap();

    proxy.sigterm().await;
    // A client that keeps reusing its connection is told to close it once the delay starts.
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let response = proxy.prf_request(&kept_alive, &path).await.send().await;
            let response = response.expect("the proxy answers during the delay");
            assert_eq!(response.status(), StatusCode::OK);
            if connection_header(&response) == "close" {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the proxy answered with `connection: close` once SIGTERM was received");
    // Nodes may still route new connections to a stopping pod: the proxy accepts and serves them.
    let response = proxy.prf_request(&client(), &path).await.send().await;
    let response = response.expect("the proxy accepts a new connection during the delay");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(connection_header(&response), "close");
    assert_eq!(response.bytes().await.unwrap(), stub_output(&path));
    assert!(
        !proxy.log().contains(PINGORA_HANDLED_SIGTERM),
        "Pingora handled SIGTERM before the delay ended"
    );

    proxy
        .wait_for_log(PINGORA_HANDLED_SIGTERM, Duration::from_secs(5))
        .await;
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            match proxy.prf_request(&client(), &path).await.send().await {
                Err(err) if err.is_connect() => break,
                _ => tokio::time::sleep(Duration::from_millis(10)).await,
            }
        }
    })
    .await
    .expect("the proxy refused new connections after the delay");
    let exit = proxy.wait_for_exit(Duration::from_secs(10)).await;
    assert!(exit.success(), "{exit}: {}", proxy.log());
}

#[tokio::test]
async fn logs_never_carry_the_signed_authorization() {
    // Pingora writes request headers at DEBUG, and the bytes it sends at TRACE.
    for level in ["DEBUG", "TRACE"] {
        let prf_service = StubPrfService::start().await;
        let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
        let mut proxy = ProxyProcess::start(
            &prf_service,
            &rpc,
            &[
                ("COPROCESSOR_PROXY_LOG_LEVEL", level),
                ("COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD", "1s"),
            ],
        )
        .await;
        let path = prf_path(1);
        let authorization = auth_header(&sender(), &path, b"").await;
        let response = client()
            .get(format!("https://{}{path}", proxy.addr))
            .header("authorization", &authorization)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.bytes().await.unwrap(), stub_output(&path));
        proxy.sigterm().await;
        let exit = proxy.wait_for_exit(Duration::from_secs(10)).await;
        assert!(exit.success(), "{exit}: {}", proxy.log());

        let log = proxy.log();
        assert!(log.contains(&format!(r#""level":"{level}""#)), "{log}");
        let signature = authorization.split("sig=").nth(1).unwrap();
        assert!(
            !log.contains(signature),
            "{level} log has the signature: {log}"
        );
        assert!(
            !log.contains(&authorization),
            "{level} log has the header: {log}"
        );
    }
}
