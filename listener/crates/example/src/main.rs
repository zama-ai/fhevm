//! End-to-end showcase of the `consumer` library.
//!
//! Working example: Zama ERC-20 on **Ethereum mainnet**
//! (`0xA12CC123ba206d4031D1c7f6223D1C2Ec249f4f3`). The listener service
//! does the chain work; this binary is *only* a downstream — no RPC, no DB.
//!
//! What it does, in order:
//!  1. Connects to the broker (Redis Streams or RabbitMQ via `BROKER_URL`).
//!  2. Declares both consumer queues (`token.new-event`, `token.catchup-event`)
//!     so messages are not lost between filter registration and `consume`.
//!  3. Publishes a `WATCH` filter pinned to the Zama token contract address
//!     (`log_address` filter — the listener does address-level filtering
//!     server-side; topic-level filtering is on us, see `transfer.rs`).
//!  4. Spawns a live consumer.
//!  5. Once the first live block arrives, requests a catch-up over
//!     `[head - 2_000, head]` and consumes the replayed events on the
//!     dedicated catchup queue.
//!  6. On Ctrl-C: cancels both flows and unregisters the filter.
//!
//! ```bash
//! # The listener_core service must be running and pointed at the same
//! # broker and CHAIN_ID as this binary.
//! BROKER_URL=redis://localhost:6379 CHAIN_ID=1 cargo run -p example
//! ```

mod transfer;

use std::env;
use std::sync::Arc;

use alloy_primitives::Address;
use anyhow::Context;
use broker::{AckDecision, Broker};
use consumer::{BlockPayload, FilterCommand, ListenerConsumer};
use tokio::sync::{Mutex, oneshot};
use tracing::{info, warn};

use crate::transfer::{TRANSFER_TOPIC0, decode_transfer};

/// Zama ERC-20 deployment on Ethereum mainnet.
const TOKEN_ADDRESS: &str = "0xA12CC123ba206d4031D1c7f6223D1C2Ec249f4f3";
/// Logical name for this downstream — appears in the routing keys
/// `token.new-event` / `token.catchup-event` and the WATCH command.
const CONSUMER_ID: &str = "token";
/// How far back to backfill once we know the live head.
const CATCHUP_DEPTH: u64 = 2_000;

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
    let token: Address = TOKEN_ADDRESS.parse().context("invalid TOKEN_ADDRESS")?;

    info!(%broker_url, chain_id, %token, consumer_id = CONSUMER_ID,
        "starting Zama-token Transfer showcase");

    let broker = Broker::redis(&broker_url)
        .await
        .context("connecting to broker")?;
    let consumer = ListenerConsumer::new(&broker, chain_id, CONSUMER_ID);

    // ── 2. Declare the queues *before* publishing or consuming ────────────
    //      Without this, a WATCH published while the queue is missing would
    //      generate events the broker drops on the floor.
    consumer.ensure_consumer().await?;
    consumer.ensure_catchup_consumer().await?;

    // ── 3. Register an address-level filter for the Zama token ────────────
    let filter = FilterCommand {
        consumer_id: CONSUMER_ID.to_string(),
        from: None,
        to: None,
        log_address: Some(token),
    };
    consumer.register_filter(&filter).await?;
    info!(%token, "registered WATCH filter on log_address");

    // ── 4. Live consumer — also signals the live head to the main task ────
    let (head_tx, head_rx) = oneshot::channel::<u64>();
    let head_tx: Arc<Mutex<Option<oneshot::Sender<u64>>>> = Arc::new(Mutex::new(Some(head_tx)));

    let live_handle = {
        let head_tx = head_tx.clone();
        tokio::spawn(consumer.consume(move |payload, _cancel| {
            let head_tx = head_tx.clone();
            async move {
                if let Some(tx) = head_tx.lock().await.take() {
                    let _ = tx.send(payload.block_number);
                }
                handle_block("LIVE", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // ── 5. Catchup consumer — started before the request so we don't miss
    //      the head of the replay range.
    let catchup_handle = tokio::spawn(consumer.consume_catchup(
        move |payload, _cancel| async move {
            handle_block("CATCHUP", &payload, token);
            Ok(AckDecision::Ack)
        },
    ));

    // Block until the listener publishes its first live block.
    let head = head_rx
        .await
        .context("live consumer ended before any block")?;
    let start = head.saturating_sub(CATCHUP_DEPTH);
    info!(
        start,
        end = head,
        "requesting catch-up over the last 2k blocks"
    );
    consumer.request_catchup(start, head).await?;

    // ── 6. Wait for Ctrl-C, then shut both flows down cleanly ─────────────
    tokio::select! {
        _ = tokio::signal::ctrl_c() => info!("ctrl-c — shutting down"),
        r = live_handle             => warn!(?r, "live consumer exited unexpectedly"),
        r = catchup_handle          => warn!(?r, "catchup consumer exited unexpectedly"),
    }

    consumer.cancel(); // parent token: cancels both live and catchup
    if let Err(e) = consumer.unregister_filter(&filter).await {
        warn!(error = %e, "unregister_filter failed (filter may linger in DB)");
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

/// Iterate a `BlockPayload`, keep only logs from the Zama token where
/// `topics[0] == Transfer`, decode them, and log a human-readable line.
fn handle_block(tag: &str, payload: &BlockPayload, token: Address) {
    info!(
        flow = tag,
        block = payload.block_number,
        txs = payload.transactions.len(),
        "block"
    );
    for tx in &payload.transactions {
        for log in &tx.logs {
            if log.address != token {
                continue;
            }
            if log.topics.first() != Some(&TRANSFER_TOPIC0) {
                continue;
            }
            match decode_transfer(log) {
                Some(t) => info!(
                    flow = tag,
                    block = payload.block_number,
                    tx = %tx.hash,
                    log_index = log.log_index,
                    from = %t.from,
                    to = %t.to,
                    value = %t.value,
                    "Transfer"
                ),
                None => warn!(
                    flow = tag,
                    tx = %tx.hash,
                    "log matched topic0 but failed to decode"
                ),
            }
        }
    }
}
