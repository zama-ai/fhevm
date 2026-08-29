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
//!   (`FINAL-CATCHUP`) — a bounded replay of finalized blocks, requested once
//!   the first final block tells us where the finalized head is.

use alloy_primitives::Address;
use broker::BrokerError;
use consumer::{AckDecision, ListenerConsumer};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::transfer::log_transfers;

/// How far back to replay finalized blocks once we know the final head.
/// The listener clamps the range to the finalized head anyway, so a request
/// can never reach into the unfinalized window.
const FINAL_CATCHUP_DEPTH: u64 = 200;

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
pub async fn start(consumer: &ListenerConsumer, token: Address) -> anyhow::Result<FinalHandles> {
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
        tokio::spawn(consumer.consume_final(move |payload, _cancel| {
            let head_tx = head_tx.clone();
            async move {
                if let Some(tx) = head_tx.lock().await.take() {
                    let _ = tx.send(payload.block_number);
                }
                log_transfers("FINAL", &payload, token);
                Ok(AckDecision::Ack)
            }
        }))
    };

    // 4. Final catchup consumer — started before the request so the head of
    //    the replay range is not missed.
    let final_catchup_handle = tokio::spawn(consumer.consume_final_catchup(
        move |payload, _cancel| async move {
            log_transfers("FINAL-CATCHUP", &payload, token);
            Ok(AckDecision::Ack)
        },
    ));

    // 5. Once the first finalized block arrives, request the replay.
    let requester = consumer.clone();
    tokio::spawn(async move {
        let Ok(head) = head_rx.await else {
            warn!("FINAL: consumer ended before any block — skipping final catchup request");
            return;
        };
        let start = head.saturating_sub(FINAL_CATCHUP_DEPTH);
        info!(start, end = head, "FINAL-CATCHUP: requesting backfill");
        if let Err(e) = requester.request_final_catchup(start, head).await {
            warn!(error = %e, "FINAL-CATCHUP: request_final_catchup failed");
        }
    });

    Ok((final_handle, final_catchup_handle))
}

/// Unregister the FINAL watcher (best-effort — call after cancelling the flows).
pub async fn stop(consumer: &ListenerConsumer, token: Address) {
    if let Err(e) = consumer.unregister_final_contracts(&[token]).await {
        warn!(error = %e, "FINAL: unregister_final_contracts failed (filter may linger in DB)");
    }
}
