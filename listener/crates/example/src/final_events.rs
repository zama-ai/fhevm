//! Final subset: finalized-only events + final catchup.
//!
//! This module drives the listener entirely through the **high-level contract
//! API** ([`ListenerConsumer::register_final_contracts`]) — compare with
//! `live_events.rs`, which hand-builds its `FilterCommand`.
//!
//! Finalized blocks never reorg, so these streams carry no `Reorged`
//! replays: payload flows are `BlockFlow::Final` and `BlockFlow::FinalCatchup`.
//! The listener's finality flow must be enabled (`finality_active: true`,
//! the default) or nothing is delivered and catchup requests are dropped.
//!
//! Flows started here (log tags in parentheses):
//! - `consume_final` on `{consumer_id}.final-event` (`FINAL`) — each block
//!   once it is final, per the listener's finality strategy (`finalized` tag
//!   or `head - finality_depth`).
//! - `consume_final_catchup` on `{consumer_id}.final-catchup-event`
//!   (`FINAL-CATCHUP`) — a bounded replay of finalized blocks, named by a
//!   `catchup_id` this process owns and persists. The id and range live in the
//!   same store as the live flow's, under a separate key — see
//!   [`crate::catchup_state`].

use alloy_primitives::Address;
use broker::BrokerError;
use consumer::{AckDecision, ListenerConsumer};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot, watch};
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::catchup_state::{self, Flow, Store};
use crate::stats::Stats;
use crate::transfer::log_transfers;

/// How far back to replay finalized blocks — used only on the very first boot,
/// when there is no persisted range to inherit. The listener clamps the range
/// to the finalized head anyway, so a request can never reach into the
/// unfinalized window.
const FINAL_CATCHUP_DEPTH: u64 = 100;

/// Handles of the two spawned consumers: `(final, final_catchup)`.
pub type FinalHandles = (
    JoinHandle<Result<(), BrokerError>>,
    JoinHandle<Result<(), BrokerError>>,
);

/// Set up the final subset and return the running consumer handles.
///
/// Order matters, same as the live subset: queues are declared *before* the
/// FINAL watcher is registered, so finalized events published between
/// registration and the first poll are not dropped.
pub async fn start(
    consumer: &ListenerConsumer,
    store: &Store,
    token: Address,
    stats: Arc<Stats>,
    active: Arc<watch::Sender<Option<Uuid>>>,
) -> anyhow::Result<FinalHandles> {
    // 1. Declare both delivery queues.
    consumer.ensure_final_consumer().await?;
    consumer.ensure_final_catchup_consumer().await?;

    // 2. Register the watcher — high-level style: the contract API builds and
    //    publishes the FINAL FilterCommand for us.
    consumer.register_final_contracts(&[token]).await?;
    info!(%token, "FINAL: registered FINAL watcher (register_final_contracts)");

    // 3. Final consumer — also signals the first finalized head so the final
    //    catchup request below knows the replay range.
    let (head_tx, head_rx) = oneshot::channel::<u64>();
    let head_tx: Arc<Mutex<Option<oneshot::Sender<u64>>>> = Arc::new(Mutex::new(Some(head_tx)));

    let final_handle = {
        let head_tx = head_tx.clone();
        let stats = stats.clone();
        tokio::spawn(consumer.consume_final(move |payload, _cancel| {
            let head_tx = head_tx.clone();
            let stats = stats.clone();
            async move {
                if let Some(tx) = head_tx.lock().await.take() {
                    let _ = tx.send(payload.block_number);
                }
                stats.record_final_delivered();
                log_transfers("FINAL", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // 4. Final catchup consumer — started before the request so the head of
    //    the replay range is not missed. Blocks stamped with an id other than
    //    the one we own belong to a catchup a previous boot retired; see the
    //    equivalent step in `live_events.rs`.
    let active_rx = active.subscribe();
    let final_catchup_handle = {
        let stats = stats.clone();
        tokio::spawn(consumer.consume_final_catchup(move |payload, _cancel| {
            let active_rx = active_rx.clone();
            let stats = stats.clone();
            async move {
                if is_stale(&active_rx, payload.catchup_id) {
                    stats.catchup(Flow::FinalCatchup).record_dropped_stale();
                    debug!(
                        block_number = payload.block_number,
                        catchup_id = ?payload.catchup_id,
                        "FINAL-CATCHUP: dropping block from a retired catchup"
                    );
                    return Ok(AckDecision::Ack);
                }
                stats.catchup(Flow::FinalCatchup).record_delivered();
                log_transfers("FINAL-CATCHUP", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // 5. Once the first finalized block arrives we know where the finalized
    //    head is, which is all that is missing to resolve the desired range.
    let requester = consumer.clone();
    let store = store.clone();
    tokio::spawn(async move {
        let Ok(head) = head_rx.await else {
            warn!("FINAL: consumer ended before any block — skipping final catchup request");
            return;
        };
        if let Err(e) = reconcile_final_catchup(&requester, &store, head, &active).await {
            warn!(error = %e, "FINAL-CATCHUP: reconciling the catchup request failed");
        }
    });

    Ok((final_handle, final_catchup_handle))
}

/// Resolve the desired range and bring the listener in line with it.
async fn reconcile_final_catchup(
    consumer: &ListenerConsumer,
    store: &Store,
    head: u64,
    active: &watch::Sender<Option<Uuid>>,
) -> anyhow::Result<()> {
    let desired =
        catchup_state::desired_range(store, Flow::FinalCatchup, head, FINAL_CATCHUP_DEPTH).await?;
    catchup_state::reconcile_and_track(consumer, store, Flow::FinalCatchup, desired, active).await
}

/// Whether a delivered block belongs to a catchup we no longer own. Unknown
/// ids are kept, not dropped — see the equivalent in `live_events.rs`.
fn is_stale(active: &watch::Receiver<Option<Uuid>>, delivered: Option<Uuid>) -> bool {
    match (*active.borrow(), delivered) {
        (Some(active), Some(delivered)) => active != delivered,
        _ => false,
    }
}

/// Unregister the FINAL watcher (best-effort — call after cancelling the flows).
pub async fn stop(consumer: &ListenerConsumer, token: Address) {
    if let Err(e) = consumer.unregister_final_contracts(&[token]).await {
        warn!(error = %e, "FINAL: unregister_final_contracts failed (filter may linger in DB)");
    }
}
