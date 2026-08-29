//! End-to-end showcase of the `consumer` library — live AND final flows.
//!
//! Working example: Zama ERC-20 on **Ethereum mainnet**
//! (`0xA12CC123ba206d4031D1c7f6223D1C2Ec249f4f3`). The listener service does
//! the chain work; this binary is *only* a downstream — no RPC, no DB.
//!
//! Two cleanly separated subsets watch the same token:
//!  - [`live_events`] — head-of-chain + historical catchup, driven by a
//!    hand-built `FilterCommand` (the low-level control-plane style).
//!    Log tags: `LIVE`, `LIVE-CATCHUP`.
//!  - [`final_events`] — finalized-only + final catchup, driven by the
//!    high-level contract API. Log tags: `FINAL`, `FINAL-CATCHUP`.
//!
//! Every log line carries a `flow` field with one of those tags, so the four
//! pipelines stay visually separated in the output.
//!
//! ```bash
//! # The listener_core service must be running and pointed at the same
//! # broker and CHAIN_ID as this binary (finality_active: true for the
//! # FINAL flows — that is the default).
//! BROKER_URL=redis://localhost:6379 CHAIN_ID=1 cargo run -p example
//! ```

mod final_events;
mod live_events;
mod transfer;

use std::env;

use alloy_primitives::Address;
use anyhow::Context;
use broker::Broker;
use consumer::ListenerConsumer;
use tracing::{info, warn};

/// Zama ERC-20 deployment on Ethereum mainnet.
const TOKEN_ADDRESS: &str = "0xA12CC123ba206d4031D1c7f6223D1C2Ec249f4f3";
/// Logical name for this downstream — prefix of the four delivery queues
/// `token.{new,catchup,final,final-catchup}-event`.
const CONSUMER_ID: &str = "token";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    // ── Wire-up: env config, broker, one ListenerConsumer ─────────────────
    let broker_url =
        env::var("BROKER_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let chain_id: u64 = env::var("CHAIN_ID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let token: Address = TOKEN_ADDRESS.parse().context("invalid TOKEN_ADDRESS")?;

    info!(%broker_url, chain_id, %token, consumer_id = CONSUMER_ID,
        "starting Zama-token showcase (live + final)");

    let broker = Broker::from_url(&broker_url)
        .await
        .context("connecting to broker")?;
    let consumer = ListenerConsumer::new(&broker, chain_id, CONSUMER_ID);

    // ── Start both subsets — each declares its queues, registers its
    //    watcher, and spawns its consumers ──────────────────────────────────
    let (live, live_catchup) = live_events::start(&consumer, token).await?;
    let (finality, final_catchup) = final_events::start(&consumer, token).await?;

    // ── Run until Ctrl-C or an unexpected consumer exit ────────────────────
    tokio::select! {
        _ = tokio::signal::ctrl_c() => info!("ctrl-c — shutting down"),
        r = live          => warn!(?r, "LIVE consumer exited unexpectedly"),
        r = live_catchup  => warn!(?r, "LIVE-CATCHUP consumer exited unexpectedly"),
        r = finality      => warn!(?r, "FINAL consumer exited unexpectedly"),
        r = final_catchup => warn!(?r, "FINAL-CATCHUP consumer exited unexpectedly"),
    }

    // ── Clean shutdown: parent token stops all four flows, then unregister ─
    consumer.cancel();
    live_events::stop(&consumer, token).await;
    final_events::stop(&consumer, token).await;
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
