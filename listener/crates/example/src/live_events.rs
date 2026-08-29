//! Live subset: head-of-chain events + historical catchup.
//!
//! This module deliberately drives the listener's control plane the
//! **low-level way**: it hand-builds a [`FilterCommand`] and publishes it via
//! [`ListenerConsumer::register_filter`], instead of using the high-level
//! contract API. Compare with `final_events.rs`, which shows the high-level
//! style — together they demonstrate both ways to interact with the listener.
//!
//! Flows started here (log tags in parentheses):
//! - `consume` on `{consumer_id}.new-event` (`LIVE`) — head-of-chain blocks,
//!   including `Reorged` replays when the chain reorganizes.
//! - `consume_catchup` on `{consumer_id}.catchup-event` (`LIVE-CATCHUP`) —
//!   a bounded historical replay, requested once the first live block tells
//!   us where the head is.

use alloy_primitives::Address;
use broker::BrokerError;
use consumer::{AckDecision, FilterCommand, ListenerConsumer};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::transfer::log_transfers;

/// How far back to backfill once we know the live head.
const CATCHUP_DEPTH: u64 = 2_000;

/// Build the raw WATCH command for the live flow: an address-level filter
/// pinned to `token`, with `filter_type: None` (defaults to a LIVE watcher).
fn live_filter(consumer: &ListenerConsumer, token: Address) -> FilterCommand {
    FilterCommand {
        consumer_id: consumer.consumer_id().to_string(),
        from: None,
        to: None,
        log_address: Some(token),
        filter_type: None,
    }
}

/// Handles of the two spawned consumers: `(live, live_catchup)`.
pub type LiveHandles = (
    JoinHandle<Result<(), BrokerError>>,
    JoinHandle<Result<(), BrokerError>>,
);

/// Set up the live subset and return the running consumer handles.
///
/// Order matters: queues are declared *before* the filter is registered, so
/// events published between registration and the first `consume` poll are not
/// dropped by the broker.
pub async fn start(consumer: &ListenerConsumer, token: Address) -> anyhow::Result<LiveHandles> {
    // 1. Declare both delivery queues.
    consumer.ensure_consumer().await?;
    consumer.ensure_catchup_consumer().await?;

    // 2. Register the watcher — low-level style: a hand-built FilterCommand.
    consumer
        .register_filter(&live_filter(consumer, token))
        .await?;
    info!(%token, "LIVE: registered WATCH filter (raw FilterCommand, log_address)");

    // 3. Live consumer — also signals the first live head so the catchup
    //    request below knows the replay range.
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
                log_transfers("LIVE", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // 4. Catchup consumer — started before the request so the head of the
    //    replay range is not missed.
    let catchup_handle = tokio::spawn(consumer.consume_catchup(
        move |payload, _cancel| async move {
            log_transfers("LIVE-CATCHUP", &payload, token);
            Ok(AckDecision::Ack)
        },
    ));

    // 5. Once the first live block arrives, request the backfill. Runs as its
    //    own task so start() returns immediately.
    let requester = consumer.clone();
    tokio::spawn(async move {
        let Ok(head) = head_rx.await else {
            warn!("LIVE: consumer ended before any block — skipping catchup request");
            return;
        };
        let start = head.saturating_sub(CATCHUP_DEPTH);
        info!(start, end = head, "LIVE-CATCHUP: requesting backfill");
        if let Err(e) = requester.request_catchup(start, head).await {
            warn!(error = %e, "LIVE-CATCHUP: request_catchup failed");
        }
    });

    Ok((live_handle, catchup_handle))
}

/// Unregister the live watcher (best-effort — call after cancelling the flows).
pub async fn stop(consumer: &ListenerConsumer, token: Address) {
    if let Err(e) = consumer
        .unregister_filter(&live_filter(consumer, token))
        .await
    {
        warn!(error = %e, "LIVE: unregister_filter failed (filter may linger in DB)");
    }
}
