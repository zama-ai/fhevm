//! Full-block showcase of the `consumer` library — live AND final wildcard.
//!
//! Subscribes to **every** block on the chain twice, by registering both
//! full-block (wildcard) filters — no `from` / `to` / `log_address`:
//!  - the LIVE wildcard (`register_full_block`) delivers each block at the
//!    head of the chain on `full-block.new-event` (flow `LIVE`, plus
//!    `REORGED` replays on reorganizations);
//!  - the FINAL wildcard (`register_final_full_block`) delivers each block
//!    once it is final on `full-block.final-event` (flow `FINAL`, never
//!    reorged). Requires the listener's `finality_active: true` (default).
//!
//! Each printed block dump is banner-tagged `LIVE` / `FINAL` and includes the
//! payload's own `flow` field, so the two pipelines are easy to tell apart —
//! you should see every block twice: once at the head, once ~finality later.
//!
//! This is a downstream-only binary: no RPC, no DB. The `listener_core`
//! service must be running and pointed at the same broker and `CHAIN_ID`.
//!
//! ```bash
//! BROKER_URL=redis://localhost:6379 CHAIN_ID=1 cargo run -p example --bin full_block
//! ```

use std::env;

use anyhow::Context;
use broker::{AckDecision, Broker};
use consumer::{BlockPayload, ListenerConsumer};
use tracing::{info, warn};

/// Logical name for this downstream — prefix of the delivery queues
/// `full-block.new-event` / `full-block.final-event`. Kept distinct from the
/// `token` example so the two binaries can run side by side without colliding.
const CONSUMER_ID: &str = "full-block";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    // ── 1. Wire-up: broker + ListenerConsumer ──────────────────────────────
    let broker_url =
        env::var("BROKER_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let chain_id: u64 = env::var("CHAIN_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    info!(%broker_url, chain_id, consumer_id = CONSUMER_ID,
        "starting full-block showcase (live + final wildcard)");

    let broker = Broker::from_url(&broker_url)
        .await
        .context("connecting to broker")?;
    let consumer = ListenerConsumer::new(&broker, chain_id, CONSUMER_ID);

    // ── 2. Declare both queues *before* publishing or consuming ────────────
    //      Without this, a WATCH published while a queue is missing would
    //      generate events the broker drops on the floor.
    consumer.ensure_consumer().await?;
    consumer.ensure_final_consumer().await?;

    // ── 3. Register both full-block (wildcard) filters ─────────────────────
    //      No address fields → the listener broadcasts the entire block,
    //      once per flow: at the head (LIVE) and once finalized (FINAL).
    consumer.register_full_block().await?;
    consumer.register_final_full_block().await?;
    info!("registered LIVE and FINAL full-block WATCH filters (wildcards)");

    // ── 4. One consumer per flow — print every block as it arrives ─────────
    let live_handle = tokio::spawn(consumer.consume(|payload, _cancel| async move {
        print_block("LIVE", &payload);
        Ok(AckDecision::Ack)
    }));
    let final_handle = tokio::spawn(consumer.consume_final(|payload, _cancel| async move {
        print_block("FINAL", &payload);
        Ok(AckDecision::Ack)
    }));

    // ── 5. Wait for Ctrl-C, then shut both flows down cleanly ──────────────
    tokio::select! {
        _ = tokio::signal::ctrl_c() => info!("ctrl-c — shutting down"),
        r = live_handle             => warn!(?r, "LIVE consumer exited unexpectedly"),
        r = final_handle            => warn!(?r, "FINAL consumer exited unexpectedly"),
    }

    consumer.cancel();
    if let Err(e) = consumer.unregister_full_block().await {
        warn!(error = %e, "unregister_full_block failed (filter may linger in DB)");
    }
    if let Err(e) = consumer.unregister_final_full_block().await {
        warn!(error = %e, "unregister_final_full_block failed (filter may linger in DB)");
    }
    info!("bye");
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();
}

/// Print a full block: flow banner, header fields, then every transaction
/// with its logs.
///
/// Uses `println!` so the dump is easy to read when verifying the feature,
/// kept separate from the structured `tracing` lifecycle logs above. `tag`
/// is the subscription that delivered the block (`LIVE` or `FINAL`); the
/// payload's own `flow` field is printed too (`LIVE`/`REORGED` vs `FINAL`).
fn print_block(tag: &str, payload: &BlockPayload) {
    println!("════════════════════════════════════════════════════════════════");
    println!(
        "[{tag}] BLOCK #{} [{:?}]  (chain {})",
        payload.block_number, payload.flow, payload.chain_id
    );
    println!("  hash        : {}", payload.block_hash);
    println!("  parent hash : {}", payload.parent_hash);
    println!("  timestamp   : {}", payload.timestamp);
    println!("  transactions: {}", payload.transactions.len());

    for tx in &payload.transactions {
        let to = match tx.to {
            Some(addr) => addr.to_string(),
            None => "<contract-creation>".to_string(),
        };
        println!("  ──────────────────────────────────────────────────────────");
        println!("  TX #{}  {}", tx.transaction_index, tx.hash);
        println!("    from : {}", tx.from);
        println!("    to   : {}", to);
        println!("    logs : {}", tx.logs.len());

        for log in &tx.logs {
            println!(
                "      LOG #{}  addr={}  topics={}",
                log.log_index,
                log.address,
                log.topics.len()
            );
            for (i, topic) in log.topics.iter().enumerate() {
                println!("        topic[{i}] : {topic}");
            }
            println!("        data     : {}", log.data);
        }
    }
    println!("════════════════════════════════════════════════════════════════");
}
