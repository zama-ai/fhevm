//! Exercise listener binaries in an isolated component harness through actual HTTP/WebSocket transports.
//! The proxy can make one header unavailable, drop a connection, and send a
//! single large frame; all successful RPC results come from a real Anvil node.
mod common;

use alloy::transports::http::reqwest::Client;
use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, B256, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol_types::SolEvent,
};
use axum::{extract::State, routing::post, Json, Router};
use futures_util::{SinkExt, StreamExt};
use host_listener::contracts::TfheContract::TrivialEncrypt;
use serde_json::{json, Value};
use serial_test::serial;
use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::{
    process::{Child, Command},
    task::JoinHandle,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Clone)]
struct Rpc {
    upstream: String,
    client: Client,
    headers: Arc<Mutex<Vec<u64>>>,
    unavailable: Arc<AtomicU64>,
    rejected: Arc<AtomicUsize>,
    drop_logs: Arc<AtomicBool>,
    large_frames: Arc<AtomicUsize>,
}
impl Rpc {
    async fn response(&self, request: Value) -> Value {
        let id = request["id"].clone();
        match request["method"].as_str() {
            Some("eth_subscribe") => {
                return json!({"jsonrpc":"2.0","id":id,"result":"0x1"})
            }
            Some("eth_unsubscribe") => {
                return json!({"jsonrpc":"2.0","id":id,"result":true})
            }
            Some("eth_getBlockByNumber") => {
                if let Some(n) = request["params"][0]
                    .as_str()
                    .and_then(|s| s.strip_prefix("0x"))
                    .and_then(|s| u64::from_str_radix(s, 16).ok())
                {
                    self.headers.lock().unwrap().push(n);
                    if n == self.unavailable.load(Ordering::SeqCst) {
                        self.rejected.fetch_add(1, Ordering::SeqCst);
                        return json!({"jsonrpc":"2.0","id":id,"result":null});
                    }
                }
            }
            _ => {}
        }
        self.client
            .post(&self.upstream)
            .json(&request)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
struct Proxy {
    rpc: Rpc,
    http: String,
    ws: String,
    tasks: Vec<JoinHandle<()>>,
}
impl Drop for Proxy {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}
impl Proxy {
    async fn new(anvil: &AnvilInstance, large: bool) -> anyhow::Result<Self> {
        let rpc = Rpc {
            upstream: anvil.endpoint(),
            client: Client::new(),
            headers: Default::default(),
            unavailable: Arc::new(AtomicU64::new(u64::MAX)),
            rejected: Default::default(),
            drop_logs: Default::default(),
            large_frames: Default::default(),
        };
        let http = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let ws = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let http_url = format!("http://{}", http.local_addr()?);
        let ws_url = format!("ws://{}", ws.local_addr()?);
        let app = Router::new()
            .route(
                "/",
                post(|State(rpc): State<Rpc>, Json(request): Json<Value>| async move {
                    Json(rpc.response(request).await)
                }),
            )
            .with_state(rpc.clone());
        let http_task = tokio::spawn(async move {
            axum::serve(http, app).await.unwrap();
        });
        let ws_rpc = rpc.clone();
        let ws_task = tokio::spawn(async move {
            let mut connections = tokio::task::JoinSet::new();
            loop {
                let (socket, _) = ws.accept().await.unwrap();
                let rpc = ws_rpc.clone();
                connections.spawn(async move {
                    let Ok(mut stream) = accept_async(socket).await else {
                        return;
                    };
                    while let Some(Ok(message)) = stream.next().await {
                        let Message::Text(text) = message else {
                            continue;
                        };
                        let request: Value =
                            serde_json::from_str(&text).unwrap();
                        let logs = request["method"] == "eth_getLogs";
                        if logs && rpc.drop_logs.swap(false, Ordering::SeqCst) {
                            let _ = stream.close(None).await;
                            break;
                        }
                        let mut text = rpc.response(request).await.to_string();
                        if large && logs {
                            // Trailing JSON whitespace keeps a real RPC response valid.
                            // Tungstenite writes this message as one unfragmented frame.
                            text.extend(std::iter::repeat_n(
                                ' ',
                                17 * 1024 * 1024,
                            ));
                            rpc.large_frames.fetch_add(1, Ordering::SeqCst);
                        }
                        if stream
                            .send(Message::Text(text.into()))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                });
            }
        });
        Ok(Self {
            rpc,
            http: http_url,
            ws: ws_url,
            tasks: vec![http_task, ws_task],
        })
    }
}

async fn until<F, Fut>(mut predicate: F) -> anyhow::Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = anyhow::Result<bool>>,
{
    tokio::time::timeout(Duration::from_secs(90), async {
        loop {
            if predicate().await? {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await?
}

struct Fixture {
    _db: DBInstance,
    pool: sqlx::PgPool,
    anvil: AnvilInstance,
    emitter: Address,
    handle: B256,
}
impl Fixture {
    async fn new(event_height: u64) -> anyhow::Result<Self> {
        let db = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let pool = sqlx::PgPool::connect(db.db_url()).await?;
        let anvil = Anvil::new().chain_id(42).spawn();
        let signer: PrivateKeySigner = anvil.first_key().clone().into();
        let caller = signer.address();
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::new(signer))
            .connect_http(anvil.endpoint_url());
        let emitter = common::RawLog::deploy(provider.clone()).await?;
        let _: Value = provider
            .raw_request(
                "anvil_mine".into(),
                [format!("0x{:x}", event_height - 2)],
            )
            .await?;
        let mut handle = [0; 32];
        handle[0] = 9;
        handle[21] = 0xff;
        handle[22..30].copy_from_slice(&42u64.to_be_bytes());
        handle[30] = 4;
        let handle = B256::from(handle);
        let receipt = provider
            .send_transaction(common::emit_log_request(
                &emitter,
                TrivialEncrypt {
                    caller,
                    pt: U256::from(7),
                    toType: 4,
                    result: handle,
                }
                .encode_log_data(),
            ))
            .await?
            .get_receipt()
            .await?;
        assert_eq!(receipt.block_number, Some(event_height));
        Ok(Self {
            _db: db,
            pool,
            anvil,
            emitter: *emitter.address(),
            handle,
        })
    }
    async fn header(&self, height: u64, wrong: bool) -> anyhow::Result<()> {
        let provider =
            ProviderBuilder::new().connect_http(self.anvil.endpoint_url());
        let header = provider
            .get_block_by_number(height.into())
            .await?
            .unwrap()
            .header;
        let hash = if wrong {
            B256::repeat_byte(0xad)
        } else {
            header.hash
        };
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id,block_number,block_hash,parent_hash,block_status) VALUES (42,$1,$2,$3,'finalized')")
            .bind(height as i64).bind(hash.as_slice()).bind(header.parent_hash.as_slice()).execute(&self.pool).await?;
        Ok(())
    }
    fn start(
        &self,
        proxy: &Proxy,
        poller: bool,
        from: u64,
        health: u16,
    ) -> anyhow::Result<Child> {
        let bin = if poller {
            env!("CARGO_BIN_EXE_host_listener_poller")
        } else {
            env!("CARGO_BIN_EXE_host_listener")
        };
        let mut cmd = Command::new(bin);
        cmd.kill_on_drop(true).args([
            "--url",
            if poller { &proxy.http } else { &proxy.ws },
            "--database-url",
            self._db.db_url(),
            "--tfhe-contract-address",
            &self.emitter.to_string(),
            "--acl-contract-address",
            &Address::ZERO.to_string(),
            "--health-port",
            &health.to_string(),
            "--disable-synthetic-ops",
            "--log-level",
            "warn",
        ]);
        if poller {
            cmd.args([
                "--seed-start-block",
                &from.to_string(),
                "--replay-window",
                "1",
                "--finality-lag",
                "0",
                "--poll-interval-ms",
                "50",
                "--retry-interval-ms",
                "50",
                "--max-http-retries",
                "0",
            ]);
        } else {
            cmd.args([
                "--start-at-block",
                &from.to_string(),
                "--catchup-margin",
                "0",
                "--catchup-finalization-in-blocks",
                "0",
                "--initial-block-time",
                "1",
                "--timeout-request-websocket",
                "2",
            ]);
        }
        Ok(cmd.spawn()?)
    }
    async fn delivered(&self) -> anyhow::Result<bool> {
        Ok(sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM computations WHERE output_handle=$1)",
        )
        .bind(self.handle.as_slice())
        .fetch_one(&self.pool)
        .await?)
    }
}

async fn restart_during_gap(poller: bool) -> anyhow::Result<()> {
    let f = Fixture::new(100).await?;
    f.header(1, false).await?;
    // A legacy cursor has crossed empty history without retaining its headers.
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id,last_caught_up_block) VALUES (42,80)").execute(&f.pool).await?;
    let proxy = Proxy::new(&f.anvil, false).await?;
    proxy.rpc.unavailable.store(35, Ordering::SeqCst);
    let mut child = f.start(&proxy, poller, 80, 0)?;
    until(|| async { Ok(proxy.rpc.rejected.load(Ordering::SeqCst) > 0) })
        .await?;
    assert!(
        child.try_wait()?.is_none(),
        "listener exited instead of retaining the delivery"
    );
    assert!(
        !f.delivered().await?,
        "original delivery advanced across an unfilled gap"
    );
    let tip: i64 = sqlx::query_scalar("SELECT MAX(block_number) FROM host_chain_blocks_valid WHERE block_status='finalized'").fetch_one(&f.pool).await?;
    assert_eq!(
        tip, 34,
        "the first bounded batch must have committed before restart"
    );
    if poller {
        let cursor: i64 = sqlx::query_scalar("SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42").fetch_one(&f.pool).await?;
        assert_eq!(cursor, 80);
    }
    child.kill().await?;
    child.wait().await?;
    proxy.rpc.unavailable.store(u64::MAX, Ordering::SeqCst);
    let mut restarted = f.start(&proxy, poller, 80, 0)?;
    until(|| f.delivered()).await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id=42 AND block_status='finalized'").fetch_one(&f.pool).await?;
    assert_eq!(count, 100, "restart must retain every recovered link");
    assert!(!fhevm_engine_common::chain_recovery::is_halted(&f.pool).await?);
    if poller {
        until(|| async {
            Ok(!sqlx::query_scalar::<_, bool>(
                "SELECT pending FROM listener_replay_state WHERE chain_id=42",
            )
            .fetch_one(&f.pool)
            .await?)
        })
        .await?;
    }
    assert!(restarted.try_wait()?.is_none());
    restarted.kill().await?;
    restarted.wait().await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn poller_restart_resumes_more_than_one_batch_of_empty_history(
) -> anyhow::Result<()> {
    restart_during_gap(true).await
}
#[tokio::test]
#[serial(db)]
async fn websocket_restart_resumes_more_than_one_batch_of_empty_history(
) -> anyhow::Result<()> {
    restart_during_gap(false).await
}

#[tokio::test]
#[serial(db)]
async fn websocket_reconnect_accepts_a_single_frame_above_sixteen_mib(
) -> anyhow::Result<()> {
    let f = Fixture::new(3).await?;
    let proxy = Proxy::new(&f.anvil, true).await?;
    proxy.rpc.drop_logs.store(true, Ordering::SeqCst);
    let mut child = f.start(&proxy, false, 1, 0)?;
    until(|| f.delivered()).await?;
    assert!(
        !proxy.rpc.drop_logs.load(Ordering::SeqCst),
        "connection drop was not exercised"
    );
    assert!(proxy.rpc.large_frames.load(Ordering::SeqCst) > 0);
    assert!(child.try_wait()?.is_none());
    child.kill().await?;
    child.wait().await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn websocket_startup_audit_does_not_fetch_ancient_segments(
) -> anyhow::Result<()> {
    let f = Fixture::new(1101).await?;
    f.header(1, true).await?; // Deliberately unverifiable, outside the adopted window.
    f.header(1100, false).await?;
    let proxy = Proxy::new(&f.anvil, false).await?;
    let mut child = f.start(&proxy, false, 1101, 0)?;
    until(|| f.delivered()).await?;
    assert!(!fhevm_engine_common::chain_recovery::is_halted(&f.pool).await?);
    let requests = proxy.rpc.headers.lock().unwrap().clone();
    assert!(
        requests.contains(&1100),
        "startup verification was not exercised"
    );
    assert!(
        requests.iter().all(|n| *n >= 101),
        "audit escaped its window: {requests:?}"
    );
    child.kill().await?;
    child.wait().await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn poller_gap_backfill_detects_a_fork_outside_the_startup_audit(
) -> anyhow::Result<()> {
    let f = Fixture::new(1101).await?;
    f.header(1, true).await?;
    f.header(1100, false).await?;
    let proxy = Proxy::new(&f.anvil, false).await?;
    // The bounded audit intentionally trusts old history. Traversing it later
    // must still stop at the contradictory parent instead of skipping the gap.
    let socket = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = socket.local_addr()?.port();
    drop(socket);
    let mut child = f.start(&proxy, true, 80, port)?;
    until(|| async {
        Ok(fhevm_engine_common::chain_recovery::is_halted(&f.pool).await?)
    })
    .await?;
    // Let the containment observer settle; an immediate probe could race with
    // a process that is about to exit or lose its heartbeat.
    tokio::time::sleep(Duration::from_secs(6)).await;
    let client = Client::new();
    until(|| async {
        Ok(client
            .get(format!("http://127.0.0.1:{port}/liveness"))
            .send()
            .await
            .is_ok_and(|r| r.status().is_success()))
    })
    .await?;
    assert!(!client
        .get(format!("http://127.0.0.1:{port}/healthz"))
        .send()
        .await?
        .status()
        .is_success());
    assert!(
        child.try_wait()?.is_none(),
        "contained listener must remain alive"
    );
    assert!(!f.delivered().await?);
    assert!(proxy.rpc.headers.lock().unwrap().contains(&2));
    child.kill().await?;
    child.wait().await?;
    Ok(())
}

/// The original finalized delivery remains pending while its missing empty
/// predecessor arrives through the independent final-catchup broker queue.
#[tokio::test]
#[serial(db)]
async fn broker_final_catchup_queue_repairs_a_blocked_final_delivery(
) -> anyhow::Result<()> {
    use broker::{AsyncHandlerPayloadClassified, Topic};
    use consumer::{Broker, ListenerConsumer};
    use fhevm_engine_common::CONSENSUS_PROTOCOL_VERSION;
    use primitives::{
        event::{
            BlockFlow, BlockPayload, CatchupPayload, IndexedLog,
            TransactionPayload,
        },
        routing,
    };
    use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage};
    let f = Fixture::new(3).await?;
    f.header(1, false).await?;
    // CI owns a disposable broker. Local runs may supply a dedicated instance.
    let (url, _container) =
        if let Ok(url) = std::env::var("FHEVM_TEST_BROKER_URL") {
            (url, None)
        } else {
            let container = GenericImage::new("redis", "7-alpine")
                .with_exposed_port(6379.into())
                .with_wait_for(WaitFor::message_on_stdout(
                    "Ready to accept connections",
                ))
                .start()
                .await?;
            (
                format!(
                    "redis://{}:{}",
                    container.get_host().await?,
                    container.get_host_port_ipv4(6379).await?
                ),
                Some(container),
            )
        };
    let broker = Broker::from_url(&url).await?;
    let service = format!(
        "recovery-integration-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos()
    );
    // Match the binary's stable routing identity and versioned groups. The
    // service name only controls telemetry. Seed the groups before publishing
    // so startup cannot skip the finalized delivery already in the stream.
    let consumer_id = "host-listener-consumer.42".to_owned();
    let client = ListenerConsumer::new(&broker, 42, &consumer_id)
        .with_group_suffix(format!("v{CONSENSUS_PROTOCOL_VERSION}"));
    client.ensure_consumer().await?;
    client.ensure_final_consumer().await?;
    client.ensure_final_catchup_consumer().await?;
    let provider = ProviderBuilder::new().connect_http(f.anvil.endpoint_url());
    let mut payloads = vec![];
    for n in [2, 3] {
        let header = provider
            .get_block_by_number(n.into())
            .await?
            .unwrap()
            .header;
        let logs = provider
            .get_logs(
                &alloy::rpc::types::Filter::new().at_block_hash(header.hash),
            )
            .await?;
        let transactions = logs
            .into_iter()
            .map(|log| TransactionPayload {
                from: Address::ZERO,
                to: Some(f.emitter),
                hash: log.transaction_hash.unwrap(),
                transaction_index: log.transaction_index.unwrap(),
                value: U256::ZERO,
                data: Default::default(),
                logs: vec![IndexedLog {
                    log_index: log.log_index.unwrap(),
                    address: log.address(),
                    topics: log.topics().to_vec(),
                    data: log.data().data.clone(),
                }],
            })
            .collect();
        payloads.push(BlockPayload {
            flow: if n == 2 {
                BlockFlow::FinalCatchup
            } else {
                BlockFlow::Final
            },
            chain_id: 42,
            block_number: n,
            block_hash: header.hash,
            parent_hash: header.parent_hash,
            timestamp: header.timestamp,
            transactions,
            catchup_id: None,
            last_in_catchup: false,
        });
    }
    let publisher = broker.publisher("").await?;
    let requests = Arc::new(AtomicUsize::new(0));
    let release = Arc::new(tokio::sync::Semaphore::new(0));
    let cancel = tokio_util::sync::CancellationToken::new();
    let _cancel_on_drop = cancel.clone().drop_guard();
    let request_consumer = broker
        .consumer(&Topic::namespaced("chain-id-42", routing::FINAL_CATCHUP))
        .group(format!("{service}-requests"))
        .with_cancellation(cancel.clone())
        .build()?;
    request_consumer.ensure_topology().await?;
    let reply = payloads[0].clone();
    let reply_publisher = publisher.clone();
    let expected_consumer = consumer_id.clone();
    let requested = requests.clone();
    let released = release.clone();
    let response_task = tokio::spawn(request_consumer.run(
        AsyncHandlerPayloadClassified::new(move |request: CatchupPayload| {
            let reply = reply.clone();
            let publisher = reply_publisher.clone();
            let expected = expected_consumer.clone();
            let requested = requested.clone();
            let released = released.clone();
            async move {
                assert_eq!(request.consumer_id, expected);
                assert_eq!((request.block_start, request.block_end), (2, 2));
                requested.fetch_add(1, Ordering::SeqCst);
                let permit = released.acquire().await.unwrap();
                permit.forget();
                publisher
                    .publish(
                        &routing::consumer_final_catchup_event_routing(
                            expected,
                        ),
                        &reply,
                    )
                    .await
                    .map_err(consumer::HandlerError::transient)?;
                Ok(())
            }
        }),
    ));
    let mut child = Command::new(env!("CARGO_BIN_EXE_host_listener_consumer"))
        .kill_on_drop(true)
        .args([
            "--url",
            &url,
            "--chain-id",
            "42",
            "--database-url",
            f._db.db_url(),
            "--tfhe-contract-address",
            &f.emitter.to_string(),
            "--acl-contract-address",
            &Address::ZERO.to_string(),
            "--service-name",
            &service,
            "--health-port",
            "0",
            "--disable-synthetic-ops",
            "--log-level",
            "warn",
        ])
        .spawn()?;
    publisher
        .publish_to_topic(&client.final_consumer_topic(), &payloads[1])
        .await?;
    until(|| async { Ok(requests.load(Ordering::SeqCst) > 0) }).await?;
    assert!(
        !f.delivered().await?,
        "finalized delivery was acknowledged before its predecessor"
    );
    release.add_permits(10);
    until(|| async {
        Ok(sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM host_chain_blocks_valid WHERE chain_id=42 AND block_number=2 AND block_status='finalized')",
        )
        .fetch_one(&f.pool)
        .await?)
    })
    .await?;
    until(|| f.delivered()).await?;
    let heights: Vec<i64> = sqlx::query_scalar("SELECT block_number FROM host_chain_blocks_valid WHERE chain_id=42 AND block_status='finalized' ORDER BY block_number").fetch_all(&f.pool).await?;
    assert_eq!(heights, [1, 2, 3]);
    assert!(child.try_wait()?.is_none());
    child.kill().await?;
    child.wait().await?;
    cancel.cancel();
    tokio::time::timeout(Duration::from_secs(5), response_task).await???;
    Ok(())
}
