//! Full-block showcase of the `consumer` library.
//!
//! Subscribes to **every** block on the chain by registering a *full-block*
//! (wildcard) filter — no `from` / `to` / `log_address`. The listener then
//! publishes the complete `BlockPayload` (all transactions, all logs) to this
//! consumer, and we print each block so you can verify the feature works.
//!
//! This is a downstream-only binary: no RPC, no DB. The `listener_core`
//! service must be running and pointed at the same broker and `CHAIN_ID`.
//!
//! What it does, in order:
//!  1. Connects to the broker (Redis Streams or RabbitMQ via `BROKER_URL`).
//!  2. Declares the consumer queue (`full-block.new-event`) so no events are
//!     lost between filter registration and `consume`.
//!  3. Publishes a full-block WATCH filter (wildcard, no addresses).
//!  4. Spawns a live consumer and prints every block it receives.
//!  5. On Ctrl-C: cancels the flow and unregisters the filter.
//!
//! ```bash
//! BROKER_URL=redis://localhost:6379 CHAIN_ID=1 cargo run -p example --bin full_block
//! ```

use std::env;

use anyhow::Context;
use broker::{AckDecision, Broker};
use consumer::{BlockPayload, ListenerConsumer};
use tracing::{info, warn};

/// Logical name for this downstream — appears in the routing key
/// `full-block.new-event` and the WATCH command. Kept distinct from the
/// `token` example so the two can run side by side without colliding.
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

    info!(%broker_url, chain_id, consumer_id = CONSUMER_ID, "starting full-block showcase");

    let broker = Broker::from_url(&broker_url)
        .await
        .context("connecting to broker")?;
    let consumer = ListenerConsumer::new(&broker, chain_id, CONSUMER_ID);

    // ── 2. Declare the queue *before* publishing or consuming ──────────────
    //      Without this, a WATCH published while the queue is missing would
    //      generate events the broker drops on the floor.
    consumer.ensure_consumer().await?;

    // ── 3. Register the full-block (wildcard) filter ───────────────────────
    //      No address fields → the listener broadcasts the entire block.
    consumer.register_full_block().await?;
    info!("registered full-block WATCH filter (wildcard: no from/to/log_address)");

    // ── 4. Live consumer — print every block as it arrives ─────────────────
    let live_handle = tokio::spawn(consumer.consume(|payload, _cancel| async move {
        print_block(&payload);
        Ok(AckDecision::Ack)
    }));

    // ── 5. Wait for Ctrl-C, then shut the flow down cleanly ────────────────
    tokio::select! {
        _ = tokio::signal::ctrl_c() => info!("ctrl-c — shutting down"),
        r = live_handle             => warn!(?r, "live consumer exited unexpectedly"),
    }

    consumer.cancel();
    if let Err(e) = consumer.unregister_full_block().await {
        warn!(error = %e, "unregister_full_block failed (filter may linger in DB)");
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

/// Print a full block: header fields, then every transaction with its logs.
///
/// Uses `println!` so the dump is easy to read when verifying the feature,
/// kept separate from the structured `tracing` lifecycle logs above.
fn print_block(payload: &BlockPayload) {
    println!("════════════════════════════════════════════════════════════════");
    println!(
        "BLOCK #{} [{:?}]  (chain {})",
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
