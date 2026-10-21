#![cfg(unix)]

mod common;

use alloy::transports::http::reqwest::{Client, StatusCode};
use common::{
    AUDIENCE, CHAIN_ID, FakeRpc, KmsContext, PROTOCOL_CONFIG, StubEndpoint, TestTls, auth_header,
    free_addr, prf_path, sender, stub_output,
};
use std::{fs, process::Stdio, time::Duration};
use tokio::process::Command;

#[tokio::test]
async fn sigterm_drains_a_held_request_and_exits_cleanly() {
    let endpoint = StubEndpoint::start().await;
    let rpc = FakeRpc::start(vec![KmsContext::active(&[sender().address()])]).await;
    let addr = free_addr();
    let health = free_addr();
    let metrics = free_addr();
    let tls = TestTls::generate(addr);
    let log_path = tls.cert_path.with_extension("log");
    let log = fs::File::create(&log_path).unwrap();
    let mut proxy = Command::new(env!("CARGO_BIN_EXE_coprocessor-proxy"))
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
            "COPROCESSOR_PROXY_ENDPOINT_ADDRESSES",
            endpoint.addr.to_string(),
        )
        .env("COPROCESSOR_PROXY_ENDPOINT_HEALTHCHECK_FREQUENCY", "50ms")
        .env("COPROCESSOR_PROXY_ENDPOINT_RESPONSE_TIMEOUT", "20s")
        .env("COPROCESSOR_PROXY_SHUTDOWN_GRACE_PERIOD", "5s")
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
    let client = Client::builder()
        .tls_danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        assert!(
            proxy.try_wait().unwrap().is_none(),
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
    let path = prf_path(1);
    let request = client
        .get(format!("https://{addr}{path}"))
        .header("authorization", auth_header(&sender(), &path, b"").await)
        .header("x-stub-hold", "1");
    let reply = tokio::spawn(async move { request.send().await.unwrap() });
    tokio::time::timeout(Duration::from_secs(5), async {
        while endpoint.hits() == 0 {
            assert!(
                !reply.is_finished(),
                "request ended before reaching the endpoint"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("request reached the endpoint");

    assert!(
        Command::new("kill")
            .args(["-TERM", &proxy.id().unwrap().to_string()])
            .status()
            .await
            .unwrap()
            .success()
    );
    // Observe the real signal handler before releasing the request.
    tokio::time::timeout(Duration::from_secs(3), async {
        while !fs::read_to_string(&log_path)
            .unwrap()
            .contains("SIGTERM received, gracefully exiting")
        {
            assert!(
                proxy.try_wait().unwrap().is_none(),
                "proxy exited before draining"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("SIGTERM handled within the drain grace");
    assert!(
        !reply.is_finished(),
        "request must remain held across SIGTERM"
    );
    endpoint.release_held_request();
    let response = reply.await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.bytes().await.unwrap(), stub_output(&path));
    let exit = tokio::time::timeout(Duration::from_secs(12), proxy.wait())
        .await
        .expect("proxy exited after draining")
        .unwrap();
    assert!(
        exit.success(),
        "{exit}: {}",
        fs::read_to_string(&log_path).unwrap()
    );
}
