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
//!   a bounded historical replay, named by a `catchup_id` this process owns
//!   and persists. See [`crate::catchup_state`] for why the id is durable and
//!   why the range is not derived from the head on every boot.

use alloy_primitives::Address;
use broker::BrokerError;
use consumer::{AckDecision, FilterCommand, ListenerConsumer};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot, watch};
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::catchup_state::{self, Flow, Store};
use crate::stats::Stats;
use crate::transfer::log_transfers;

/// How far back to backfill — used only on the very first boot, when there is
/// no persisted range to inherit.
const CATCHUP_DEPTH: u64 = 100;

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
pub async fn start(
    consumer: &ListenerConsumer,
    store: &Store,
    token: Address,
    stats: Arc<Stats>,
    active: Arc<watch::Sender<Option<Uuid>>>,
) -> anyhow::Result<LiveHandles> {
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
        let stats = stats.clone();
        tokio::spawn(consumer.consume(move |payload, _cancel| {
            let head_tx = head_tx.clone();
            let stats = stats.clone();
            async move {
                if let Some(tx) = head_tx.lock().await.take() {
                    let _ = tx.send(payload.block_number);
                }
                stats.record_live_delivered();
                log_transfers("LIVE", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // 4. Catchup consumer — started before the request so the head of the
    //    replay range is not missed.
    //
    //    `active_id` carries the id of the catchup we currently own. Blocks
    //    stamped with any other id belong to a catchup a previous boot
    //    retired: cancelling stops the listener fetching, but sub-ranges
    //    already in flight still land. Dropping them here is the consumer
    //    side of that guarantee.
    let active_rx = active.subscribe();
    let catchup_handle = {
        let stats = stats.clone();
        tokio::spawn(consumer.consume_catchup(move |payload, _cancel| {
            let active_rx = active_rx.clone();
            let stats = stats.clone();
            async move {
                if is_stale(&active_rx, payload.catchup_id) {
                    stats.catchup(Flow::Catchup).record_dropped_stale();
                    debug!(
                        block_number = payload.block_number,
                        catchup_id = ?payload.catchup_id,
                        "LIVE-CATCHUP: dropping block from a retired catchup"
                    );
                    return Ok(AckDecision::Ack);
                }
                stats.catchup(Flow::Catchup).record_delivered();
                log_transfers("LIVE-CATCHUP", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // 5. Once the first live block arrives we know where the chain is, which
    //    is all that is missing to resolve the desired range. Runs as its own
    //    task so start() returns immediately.
    let requester = consumer.clone();
    let store = store.clone();
    tokio::spawn(async move {
        let Ok(head) = head_rx.await else {
            warn!("LIVE: consumer ended before any block — skipping catchup request");
            return;
        };
        if let Err(e) = reconcile_catchup(&requester, &store, head, &active).await {
            warn!(error = %e, "LIVE-CATCHUP: reconciling the catchup request failed");
        }
    });

    Ok((live_handle, catchup_handle))
}

/// Resolve the desired range and bring the listener in line with it.
async fn reconcile_catchup(
    consumer: &ListenerConsumer,
    store: &Store,
    head: u64,
    active: &watch::Sender<Option<Uuid>>,
) -> anyhow::Result<()> {
    let desired = catchup_state::desired_range(store, Flow::Catchup, head, CATCHUP_DEPTH).await?;
    catchup_state::reconcile_and_track(consumer, store, Flow::Catchup, desired, active).await
}

/// Whether a delivered block belongs to a catchup we no longer own.
///
/// Unknown ids are kept, not dropped, in the two cases where we cannot judge:
/// a listener predating the id field sends `None`, and we have not finished
/// reconciling yet. Delivering a block twice is harmless; dropping one is not.
fn is_stale(active: &watch::Receiver<Option<Uuid>>, delivered: Option<Uuid>) -> bool {
    match (*active.borrow(), delivered) {
        (Some(active), Some(delivered)) => active != delivered,
        _ => false,
    }
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
