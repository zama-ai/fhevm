//! The proof server binary drains on SIGTERM, the signal Kubernetes stops a pod with.

mod support;

use std::{
    process::{Child, Command, ExitStatus, Stdio},
    sync::Arc,
    time::{Duration, Instant},
};

use alloy::{primitives::U256, rpc::types::Block, sol_types::SolValue};
use axum::{routing::post, Json, Router};
use kms_context::KMS_CONTEXT_COUNTER_BASE;
use serde_json::{json, Value};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{watch, Notify},
    time::{sleep, timeout},
};

const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

/// SIGTERM while `/healthz` waits on its database read: the server keeps that request, answers
/// it once the read completes, and exits 0.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn sigterm_lets_an_in_flight_request_finish_and_exits_0() {
    let (instance, _pool) = support::record_db().await;
    let (database_url, database_open, database_held) =
        gated_database(instance.db_url()).await;
    let ethereum_rpc_url = protocol_config_without_kms_contexts().await;
    let port = free_port().await;
    let mut server = Server(
        Command::new(env!("CARGO_BIN_EXE_solana_merkle_proof_server"))
            .args(["--database-url", &database_url])
            .args(["--http-port", &port.to_string()])
            .args(["--ethereum-rpc-url", &ethereum_rpc_url])
            .args(["--protocol-config-address", ZERO_ADDRESS])
            .args(["--coprocessor-signer-address", ZERO_ADDRESS])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("start the proof server"),
    );
    wait_until_healthy(port, &mut server.0).await;

    // Closed for about a second, well under the server's 5 s pool acquire timeout, past which
    // `/healthz` would answer 503 instead of waiting.
    database_open.send_replace(false);
    let request = tokio::spawn(get(port, "/healthz"));
    timeout(Duration::from_secs(10), database_held.notified())
        .await
        .expect("the request reaches the database");
    let killed = Command::new("kill")
        .args(["-TERM", &server.0.id().to_string()])
        .status()
        .unwrap();
    assert!(killed.success());
    sleep(Duration::from_millis(500)).await;
    let early = server.0.try_wait().unwrap();
    assert!(early.is_none(), "exited before draining: {early:?}");

    database_open.send_replace(true);
    let response = request
        .await
        .unwrap()
        .expect("an answer to the in-flight request");
    assert!(response.starts_with("HTTP/1.1 200"), "{response:?}");
    let status = wait_for_exit(&mut server.0, Duration::from_secs(10)).await;
    assert_eq!(status.code(), Some(0), "{status}");
}

/// Kills the server if the test ends before the server exits.
struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

/// A database URL whose connections pass through a proxy that forwards only while the returned
/// sender holds `true`. The `Notify` fires when the proxy holds bytes at the closed gate.
async fn gated_database(
    db_url: &str,
) -> (String, watch::Sender<bool>, Arc<Notify>) {
    let upstream = upstream_of(db_url);
    let proxy = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = proxy.local_addr().unwrap();
    let (open, gate) = watch::channel(true);
    let held = Arc::new(Notify::new());
    let proxy_held = held.clone();
    let url = db_url.replacen(&upstream, &proxy_addr.to_string(), 1);
    tokio::spawn(async move {
        while let Ok((client, _)) = proxy.accept().await {
            let database = TcpStream::connect(&upstream).await.unwrap();
            let (client_read, client_write) = client.into_split();
            let (database_read, database_write) = database.into_split();
            tokio::spawn(forward(
                client_read,
                database_write,
                gate.clone(),
                proxy_held.clone(),
            ));
            tokio::spawn(forward(
                database_read,
                client_write,
                gate.clone(),
                proxy_held.clone(),
            ));
        }
    });
    (url, open, held)
}

fn upstream_of(db_url: &str) -> String {
    db_url
        .split('@')
        .nth(1)
        .and_then(|rest| rest.split('/').next())
        .expect("a host:port in the database URL")
        .to_owned()
}

async fn forward(
    mut from: impl AsyncReadExt + Unpin,
    mut to: impl AsyncWriteExt + Unpin,
    mut gate: watch::Receiver<bool>,
    held: Arc<Notify>,
) {
    let mut buffer = [0; 8192];
    loop {
        let Ok(read) = from.read(&mut buffer).await else {
            return;
        };
        if read == 0 {
            return;
        }
        if !*gate.borrow() {
            held.notify_one();
        }
        if gate.wait_for(|open| *open).await.is_err() {
            return;
        }
        if to.write_all(&buffer[..read]).await.is_err() {
            return;
        }
    }
}

/// An Ethereum JSON-RPC whose `ProtocolConfig` has created no KMS context, so the server loads
/// an empty tx-sender set and `/healthz` reaches the database.
async fn protocol_config_without_kms_contexts() -> String {
    async fn answer(Json(request): Json<Value>) -> Json<Value> {
        let result = match request["method"].as_str() {
            Some("eth_chainId") => json!("0x1"),
            Some("eth_getBlockByNumber") => {
                let mut block = Block::<()>::default();
                block.header.inner.number = 1;
                serde_json::to_value(block).unwrap()
            }
            Some("eth_call") => {
                let counter: U256 = KMS_CONTEXT_COUNTER_BASE;
                json!(format!("0x{}", hex::encode(counter.abi_encode())))
            }
            other => panic!("unexpected RPC method {other:?}"),
        };
        Json(json!({ "jsonrpc": "2.0", "id": request["id"], "result": result }))
    }
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, Router::new().route("/", post(answer)))
            .await
            .unwrap()
    });
    format!("http://{addr}")
}

async fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

async fn get(port: u16, path: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await?;
    stream
        .write_all(
            format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .await?;
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    Ok(response)
}

async fn wait_until_healthy(port: u16, server: &mut Child) {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        assert!(
            server.try_wait().unwrap().is_none(),
            "the proof server exited at startup"
        );
        if let Ok(response) = get(port, "/healthz").await {
            if response.starts_with("HTTP/1.1 200") {
                return;
            }
        }
        assert!(
            Instant::now() < deadline,
            "the proof server never became healthy"
        );
        sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_for_exit(server: &mut Child, limit: Duration) -> ExitStatus {
    let deadline = Instant::now() + limit;
    loop {
        if let Some(status) = server.try_wait().unwrap() {
            return status;
        }
        if Instant::now() >= deadline {
            server.kill().unwrap();
            panic!("the proof server did not exit within {limit:?} of SIGTERM");
        }
        sleep(Duration::from_millis(50)).await;
    }
}
