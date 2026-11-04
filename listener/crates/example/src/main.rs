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

mod catchup_state;
mod control;
mod final_events;
mod live_events;
mod stats;
mod transfer;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use alloy_primitives::Address;
use anyhow::Context;
use broker::Broker;
use consumer::ListenerConsumer;
use tokio::sync::watch;
use tracing::{info, warn};

use crate::catchup_state::{Flow, Store};
use crate::stats::Stats;

/// Zama ERC-20 deployment on Ethereum mainnet. Override with `TOKEN_ADDRESS`
/// to point at a contract on a local chain.
const DEFAULT_TOKEN_ADDRESS: &str = "0xA12CC123ba206d4031D1c7f6223D1C2Ec249f4f3";
/// Logical name for this downstream — prefix of the four delivery queues
/// `token.{new,catchup,final,final-catchup}-event`. Override with
/// `CONSUMER_ID`; two instances with different ids are two independent
/// consumers, each owning its own catchup requests.
const DEFAULT_CONSUMER_ID: &str = "token";
/// Where the control plane listens. Loopback by default — the endpoint is
/// unauthenticated, so exposing it is an explicit choice.
const DEFAULT_CONTROL_ADDR: &str = "127.0.0.1:8088";

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
    let token: Address = env::var("TOKEN_ADDRESS")
        .unwrap_or_else(|_| DEFAULT_TOKEN_ADDRESS.to_string())
        .parse()
        .context("invalid TOKEN_ADDRESS")?;
    let consumer_id = env::var("CONSUMER_ID").unwrap_or_else(|_| DEFAULT_CONSUMER_ID.to_string());
    let control_addr: SocketAddr = env::var("CONTROL_ADDR")
        .unwrap_or_else(|_| DEFAULT_CONTROL_ADDR.to_string())
        .parse()
        .context("invalid CONTROL_ADDR")?;

    info!(%broker_url, chain_id, %token, %consumer_id,
        "starting Zama-token showcase (live + final)");

    let broker = Broker::from_url(&broker_url)
        .await
        .context("connecting to broker")?;
    let consumer = ListenerConsumer::new(&broker, chain_id, &consumer_id);

    // ── Durable catchup bookkeeping. The listener never retires a catchup on
    //    our behalf, so the ids and ranges we have asked for have to outlive
    //    this process — see `catchup_state` ─────────────────────────────────
    let store = Store::new(catchup_state::default_path());
    let stats = Arc::new(Stats::default());

    // ── The id each catchup handler should accept blocks for. Seeded from
    //    the store so a restart keeps dropping blocks from a catchup a
    //    previous boot retired, and shared with the control plane so a
    //    runtime cancel updates the handler as well as the listener ────────
    let catchup_active = Arc::new(watch::channel(store.current_id(Flow::Catchup).await?).0);
    let final_catchup_active =
        Arc::new(watch::channel(store.current_id(Flow::FinalCatchup).await?).0);

    // ── Start both subsets — each declares its queues, registers its
    //    watcher, and spawns its consumers ──────────────────────────────────
    let (live, live_catchup) = live_events::start(
        &consumer,
        &store,
        token,
        stats.clone(),
        catchup_active.clone(),
    )
    .await?;
    let (finality, final_catchup) = final_events::start(
        &consumer,
        &store,
        token,
        stats.clone(),
        final_catchup_active.clone(),
    )
    .await?;

    // ── Runtime control plane: inspect counters, request a different range,
    //    or cancel — without a redeploy ──────────────────────────────────────
    let control = control::Control::new(
        consumer.clone(),
        store.clone(),
        stats,
        catchup_active,
        final_catchup_active,
    );
    let control_handle = tokio::spawn(control::serve(control, control_addr));

    // ── Run until Ctrl-C or an unexpected consumer exit ────────────────────
    tokio::select! {
        _ = tokio::signal::ctrl_c() => info!("ctrl-c — shutting down"),
        r = live          => warn!(?r, "LIVE consumer exited unexpectedly"),
        r = live_catchup  => warn!(?r, "LIVE-CATCHUP consumer exited unexpectedly"),
        r = finality      => warn!(?r, "FINAL consumer exited unexpectedly"),
        r = final_catchup => warn!(?r, "FINAL-CATCHUP consumer exited unexpectedly"),
        r = control_handle => warn!(?r, "control endpoint exited unexpectedly"),
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
