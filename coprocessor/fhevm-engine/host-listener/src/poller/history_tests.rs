//! Exercise the production poll loop across an unavailable historical segment.
use super::recovery_tests::pause_test_config;
use super::*;
use alloy::providers::{ext::AnvilApi, Provider};
use fhevm_engine_common::chain_recovery::{
    is_halted, pending_host_replay, pending_replay, readiness_blocked,
    ListenerSource,
};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Forward a real node's responses except for one historical header. In
// particular, the current head, cursor, replay range, and new blocks remain
// available; an unavailable old segment must independently gate admission.
async fn proxy_request(
    mut socket: tokio::net::TcpStream,
    http: alloy::transports::http::reqwest::Client,
    upstream: String,
    deny_old: Arc<AtomicBool>,
    failures: Arc<AtomicUsize>,
) -> Result<()> {
    let mut request = vec![];
    let mut buffer = [0_u8; 8192];
    loop {
        let n = socket.read(&mut buffer).await?;
        anyhow::ensure!(n > 0, "RPC connection closed before request");
        request.extend_from_slice(&buffer[..n]);
        let Some(boundary) = request.windows(4).position(|w| w == b"\r\n\r\n")
        else {
            continue;
        };
        let Ok(body) =
            serde_json::from_slice::<Value>(&request[boundary + 4..])
        else {
            continue;
        };
        let response: Value = if body["method"] == "eth_getBlockByNumber"
            && body["params"][0] == "0x50"
            && deny_old.load(Ordering::SeqCst)
        {
            failures.fetch_add(1, Ordering::SeqCst);
            json!({"jsonrpc": "2.0", "id": body["id"], "result": null})
        } else {
            http.post(&upstream)
                .json(&body)
                .send()
                .await?
                .json()
                .await?
        };
        let body = response.to_string();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body
        );
        socket.write_all(response.as_bytes()).await?;
        return Ok(());
    }
}

async fn assert_poller_waits_for_history_verification(
    reorg: bool,
) -> Result<()> {
    let instance = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(instance.db_url()).await?;
    let anvil = alloy::node_bindings::Anvil::new().chain_id(42).spawn();
    let rpc = ProviderBuilder::new().connect_http(anvil.endpoint_url());
    rpc.anvil_mine(Some(74), Some(2)).await?;
    let snapshot = rpc.anvil_snapshot().await?;
    rpc.anvil_mine(Some(26), Some(2)).await?;
    let old80 = rpc.get_block_by_number(80.into()).await?.unwrap().header;
    if reorg {
        assert!(rpc.anvil_revert(snapshot).await?);
        rpc.anvil_set_next_block_timestamp(old80.timestamp + 1)
            .await?;
        rpc.anvil_mine(Some(166), Some(2)).await?;
        let canonical80 = rpc.get_block_by_number(80.into()).await?.unwrap();
        assert_ne!(old80.hash, canonical80.header.hash);
    } else {
        rpc.anvil_mine(Some(140), Some(2)).await?;
    }
    let tip200 = rpc.get_block_by_number(200.into()).await?.unwrap().header;
    // Disconnected history retained by an older listener. Height 80 is outside
    // the default 64-block replay window (137..=200) and was already below the
    // configured finality depth when first committed.
    for header in [old80, tip200] {
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid
                (chain_id, block_number, block_hash, parent_hash, block_status)
             VALUES (42, $1, $2, $3, 'finalized')",
        )
        .bind(header.number as i64)
        .bind(header.hash.as_slice())
        .bind(header.parent_hash.as_slice())
        .execute(&pool)
        .await?;
    }
    sqlx::query(
        "INSERT INTO host_listener_poller_state
             (chain_id, last_caught_up_block) VALUES (42, 200)",
    )
    .execute(&pool)
    .await?;

    let deny_old = Arc::new(AtomicBool::new(true));
    let failures = Arc::new(AtomicUsize::new(0));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let mut config = pause_test_config(instance.db_url.clone(), &anvil, 0);
    config.url = format!("http://{}", listener.local_addr()?);
    config.rpc_compute_units_per_second = 1_000_000;
    let upstream = anvil.endpoint();
    let proxy = async {
        let http = alloy::transports::http::reqwest::Client::new();
        let mut connections = tokio::task::JoinSet::new();
        loop {
            tokio::select! {
                accepted = listener.accept() => {
                    let (socket, _) = accepted?;
                    connections.spawn(proxy_request(
                        socket, http.clone(), upstream.clone(),
                        deny_old.clone(), failures.clone(),
                    ));
                }
                Some(result) = connections.join_next(), if !connections.is_empty() => {
                    if let Err(error) = result? {
                        break Err::<(), _>(error);
                    }
                }
            }
        }
    };
    let checks = async {
        // Wait for several actual attempts, not a presumed scheduling delay.
        // The poller must stay alive and retry without acknowledging any work.
        while failures.load(Ordering::SeqCst) < 3 {
            sleep(Duration::from_millis(10)).await;
        }
        let cursor: i64 = sqlx::query_scalar(
            "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42",
        )
        .fetch_one(&pool).await?;
        assert_eq!(
            cursor, 200,
            "failed verification must not advance the cursor"
        );
        let rows: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id=42",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            rows, 2,
            "no replay or newer finalized block may be admitted"
        );
        let replay = pending_host_replay(&pool, false, 42).await?.unwrap();
        assert_eq!(
            (replay.progress.from_block, replay.progress.to_block),
            (137, 200)
        );
        assert!(
            readiness_blocked(&pool).await?,
            "replay must remain pending"
        );
        assert!(
            !is_halted(&pool).await?,
            "RPC availability must not cause containment"
        );

        deny_old.store(false, Ordering::SeqCst);
        if reorg {
            while !is_halted(&pool).await? {
                sleep(Duration::from_millis(10)).await;
            }
            let cursor: i64 = sqlx::query_scalar(
                "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42",
            ).fetch_one(&pool).await?;
            assert_eq!(cursor, 200, "a confirmed conflict must stop admission");
            assert_eq!(
                pending_replay(&pool, ListenerSource::Host, false, 42).await?,
                Some(replay.progress),
                "containment must leave the replay obligation pending"
            );
            let rows: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id=42",
            ).fetch_one(&pool).await?;
            assert_eq!(rows, 2);
        } else {
            loop {
                let cursor: i64 = sqlx::query_scalar(
                    "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id=42",
                ).fetch_one(&pool).await?;
                if cursor == 220 && !readiness_blocked(&pool).await? {
                    break;
                }
                sleep(Duration::from_millis(10)).await;
            }
            let new_rows: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM host_chain_blocks_valid
                 WHERE chain_id=42 AND block_number > 200 AND block_status='finalized'",
            ).fetch_one(&pool).await?;
            assert_eq!(new_rows, 20);
            assert!(!is_halted(&pool).await?);
        }
        Ok::<_, anyhow::Error>(())
    };
    // Dropping these futures also cancels the poller's background services and
    // the proxy's connection tasks, including when an assertion fails.
    tokio::select! {
        result = run_poller(config) => panic!("poller exited instead of retrying: {result:?}"),
        result = proxy => panic!("RPC proxy stopped: {result:?}"),
        result = tokio::time::timeout(Duration::from_secs(20), checks) => result??,
    }
    Ok(())
}

#[tokio::test]
async fn poller_retries_unavailable_history_then_resumes_replay() -> Result<()>
{
    assert_poller_waits_for_history_verification(false).await
}

#[tokio::test]
async fn poller_contains_reorg_after_historical_rpc_recovers() -> Result<()> {
    assert_poller_waits_for_history_verification(true).await
}
