//! Deduplication, completion tracking and cursor discipline for gateway chain events.
//!
//! Every listener hands its logs here instead of dispatching them itself, so one component
//! decides what is new, what is still being handled, and how far the block cursor may move.
//! The cursor is the only durable state, and one serves the whole relayer: it passes a
//! block once every event observed in it has been handled, so a process that dies
//! mid-handling re-reads those blocks at the next start.
//!
//! Only a producer that fetches whole block ranges can say "this is everything up to N",
//! and only it passes a [`RangeObserved`].

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::FixedBytes;
use moka::future::Cache;
use tokio::sync::Mutex;
use tracing::{debug, error, warn};

use crate::{
    core::event::{ApiCategory, ApiVersion, GatewayChainEventData, RelayerEvent, RelayerEventData},
    core::job_id::INTERNAL_EVENT_JOB_ID,
    logging::ListenerStep,
    metrics,
    orchestrator::Orchestrator,
    store::sql::repositories::chain_cursor_repo::ChainCursorRepository,
};

/// Chain coordinates of a gateway log.
///
/// `block_hash` is part of the key rather than `block_number` alone so a reorged block
/// yields a distinct entry: the same log index under a different block hash is a different
/// event and must be handled again.
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct EventKey {
    pub block_number: u64,
    pub block_hash: FixedBytes<32>,
    pub log_index: u64,
}

/// A routable gateway log, with the coordinates it is tracked under.
#[derive(Clone, Debug)]
pub struct ObservedEvent {
    pub key: EventKey,
    pub event: GatewayChainEventData,
}

/// A producer's attestation that it has observed every log up to and including `to_block`.
#[derive(Clone, Copy, Debug)]
pub struct RangeObserved {
    pub to_block: u64,
}

/// How far a claimed event has got.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    /// Claimed by a dispatch that has not returned yet.
    InFlight,
    /// Every subscribed handler has returned.
    Done,
}

/// What a caller may do with an event it has just observed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Claim {
    Dispatch,
    Skip,
    /// Claimed elsewhere, but dispatched again: see [`PhaseCache::claim`].
    Redispatch,
}

/// Two-phase dedup cache: `InFlight` on claim, `Done` on completion.
///
/// One phase is not enough. Inserting on observation loses completions, because a dropped
/// duplicate is not a handled event; inserting on completion alone lets two concurrent
/// deliveries both dispatch.
///
/// The cache holds a marker, never the work: a dispatched event lives in its own detached
/// task, which owns it to completion. Only a *present* entry causes a skip, so losing one to
/// the TTL or to `max_capacity` costs at most a duplicate dispatch - never a dropped event.
struct PhaseCache {
    entries: Cache<EventKey, Phase>,
}

impl PhaseCache {
    fn new(ttl: Duration, max_capacity: u64) -> Self {
        Self {
            entries: Cache::builder()
                .time_to_live(ttl)
                .max_capacity(max_capacity)
                .build(),
        }
    }

    /// Claim `key`, atomically against concurrent callers.
    ///
    /// `attested` callers gate a cursor on the event, so they cannot treat another task's
    /// unfinished dispatch as good enough: waiting on it would stall their read loop behind
    /// a racing event, so they dispatch again. Handling a gateway event twice is a cost, not
    /// a hazard - all five are responses, and the handlers' status guards absorb the repeat.
    async fn claim(&self, key: EventKey, attested: bool) -> Claim {
        let entry = self.entries.entry(key).or_insert(Phase::InFlight).await;
        if entry.is_fresh() {
            return Claim::Dispatch;
        }

        match entry.into_value() {
            Phase::Done => Claim::Skip,
            Phase::InFlight if attested => Claim::Redispatch,
            Phase::InFlight => Claim::Skip,
        }
    }

    /// Record that every handler subscribed to `key` has returned. This also restarts the
    /// entry's TTL, so the window in which a duplicate is recognized runs from completion.
    async fn complete(&self, key: EventKey) {
        self.entries.insert(key, Phase::Done).await;
    }
}

/// One attested range, and the dispatches that must finish before the cursor may pass it.
struct PendingRange {
    to_block: u64,

    /// The range's unfinished dispatches. Each decrements it on its way out, and whichever
    /// takes it to zero releases the range - the listener moved on the moment it dispatched.
    outstanding: Arc<AtomicUsize>,
}

/// The attested ranges still waiting on their handlers, per producer: only the producer
/// that attested a range knows its order. The position they feed is shared, and lives in
/// the database.
#[derive(Default)]
struct CursorState {
    pending: HashMap<usize, VecDeque<PendingRange>>,
}

impl CursorState {
    fn push(&mut self, instance_id: usize, range: PendingRange) {
        self.pending
            .entry(instance_id)
            .or_default()
            .push_back(range);
    }

    /// Take `instance_id`'s longest completed prefix, returning the furthest block every
    /// observed event of which has been handled. Ranges that complete out of order wait for
    /// their predecessors.
    fn take_completed_prefix(&mut self, instance_id: usize) -> Option<u64> {
        let ranges = self.pending.get_mut(&instance_id)?;

        let mut furthest = None;
        while ranges
            .front()
            .is_some_and(|range| range.outstanding.load(Ordering::Acquire) == 0)
        {
            furthest = ranges.pop_front().map(|range| range.to_block);
        }

        furthest
    }

    fn pending_count(&self, instance_id: usize) -> usize {
        self.pending
            .get(&instance_id)
            .map(|ranges| ranges.len())
            .unwrap_or(0)
    }
}

/// The listeners' entry point: claims what is new, dispatches it, and advances the cursor
/// as attested ranges complete.
pub struct HandledEvents {
    phases: PhaseCache,
    cursor: Mutex<CursorState>,
    chain_cursor_repo: Arc<ChainCursorRepository>,
    orchestrator: Arc<Orchestrator>,
}

impl HandledEvents {
    pub fn new(
        dedup_ttl: Duration,
        dedup_max_capacity: u64,
        chain_cursor_repo: Arc<ChainCursorRepository>,
        orchestrator: Arc<Orchestrator>,
    ) -> Self {
        Self {
            phases: PhaseCache::new(dedup_ttl, dedup_max_capacity),
            cursor: Mutex::new(CursorState::default()),
            chain_cursor_repo,
            orchestrator,
        }
    }

    /// Claim, dispatch and account for one producer's batch.
    ///
    /// `range` is the producer's attestation that `events` is everything up to its end. With
    /// one, the cursor moves to that end once every event in the range is handled, including
    /// events another producer claimed first. Without one, the batch is deduplicated and
    /// dispatched but moves no cursor.
    pub async fn record_and_dispatch(
        self: &Arc<Self>,
        events: Vec<ObservedEvent>,
        range: Option<RangeObserved>,
        instance_id: usize,
    ) {
        let attested = range.is_some();
        let mut owned = Vec::with_capacity(events.len());

        for observed in events {
            match self.phases.claim(observed.key.clone(), attested).await {
                Claim::Dispatch => owned.push(observed),
                Claim::Redispatch => {
                    debug!(
                        step = %ListenerStep::EventRedispatched,
                        instance_id,
                        block_number = observed.key.block_number,
                        log_index = observed.key.log_index,
                        event = observed.event.event_name(),
                        "Event in flight elsewhere, dispatching again to complete the range"
                    );
                    owned.push(observed);
                }
                Claim::Skip => debug!(
                    step = %ListenerStep::EventDuplicate,
                    instance_id,
                    block_number = observed.key.block_number,
                    log_index = observed.key.log_index,
                    event = observed.event.event_name(),
                    "Duplicate event skipped"
                ),
            }
        }

        // Only an attested batch gates a cursor, so only it counts its dispatches down.
        let outstanding = match range {
            Some(range) => {
                let outstanding = Arc::new(AtomicUsize::new(owned.len()));

                // Queue the range before its dispatches can complete, so a handler that
                // returns at once still finds something to release.
                let pending = {
                    let mut cursor = self.cursor.lock().await;
                    cursor.push(
                        instance_id,
                        PendingRange {
                            to_block: range.to_block,
                            outstanding: outstanding.clone(),
                        },
                    );
                    cursor.pending_count(instance_id)
                };
                metrics::set_listener_pending_ranges(instance_id, pending);

                // A range whose events were all handled elsewhere - the common case for a
                // quiet chain - is complete on arrival.
                if owned.is_empty() {
                    self.advance_cursor(instance_id).await;
                }

                Some(outstanding)
            }
            None => None,
        };

        for observed in owned {
            let events = Arc::clone(self);
            let outstanding = outstanding.clone();
            // Detached, so handler latency does not hold up the listener's read loop.
            self.orchestrator.spawn_detached(async move {
                events.dispatch(observed, instance_id, outstanding).await;
            });
        }
    }

    /// Dispatch one event, mark it done, and release whatever range waited on it.
    ///
    /// A dispatch that fails still counts as done. Handlers own their retries and return
    /// nothing, so there is nothing here to retry against, and holding the cursor for one
    /// failed event would freeze recovery for every later event too.
    async fn dispatch(
        &self,
        observed: ObservedEvent,
        instance_id: usize,
        outstanding: Option<Arc<AtomicUsize>>,
    ) {
        let ObservedEvent { key, event } = observed;
        let event_name = event.event_name();

        match self
            .orchestrator
            .dispatch_event_and_wait(gateway_relayer_event(event))
            .await
        {
            Ok(()) => debug!(
                step = %ListenerStep::EventCompleted,
                instance_id,
                block_number = key.block_number,
                log_index = key.log_index,
                event = event_name,
                "Gateway event handled"
            ),
            Err(e) => error!(
                alert = true,
                instance_id,
                block_number = key.block_number,
                log_index = key.log_index,
                event = event_name,
                error = %e,
                "Gateway event handlers did not complete, advancing past it"
            ),
        }

        self.phases.complete(key).await;

        let Some(outstanding) = outstanding else {
            return;
        };

        if outstanding.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.advance_cursor(instance_id).await;
        }
    }

    /// Persist the furthest block whose every observed event is handled.
    ///
    /// The write runs outside the lock: two completions take disjoint prefixes, and the
    /// statement refuses a lower block, so nothing here needs the lock to order them. Holding
    /// it across the write would stall the listener's read loop behind one slow round trip.
    async fn advance_cursor(&self, instance_id: usize) {
        let to_block = {
            let mut cursor = self.cursor.lock().await;
            let Some(to_block) = cursor.take_completed_prefix(instance_id) else {
                return;
            };
            metrics::set_listener_pending_ranges(instance_id, cursor.pending_count(instance_id));
            to_block
        };

        match self.chain_cursor_repo.advance(to_block).await {
            Ok(true) => {
                metrics::set_listener_cursor_block(to_block);
                debug!(
                    step = %ListenerStep::BlockProgressUpdated,
                    instance_id,
                    block_number = to_block,
                    "Block progress updated"
                );
            }
            // A further block is already recorded - by another listener, or by a completion
            // here that took a later prefix and reached the write first.
            Ok(false) => {}
            // The next range to complete writes a higher block, so a failed write costs
            // recovery precision rather than events: a restart re-reads from the last
            // block that did land.
            Err(e) => warn!(
                step = %ListenerStep::BlockUpdateFailed,
                instance_id,
                block_number = to_block,
                error = %e,
                "Failed to update block progress"
            ),
        }
    }
}

/// Wraps a gateway event in the envelope the dispatcher routes on.
fn gateway_relayer_event(event: GatewayChainEventData) -> RelayerEvent {
    RelayerEvent::new(
        INTERNAL_EVENT_JOB_ID,
        ApiVersion {
            category: ApiCategory::PRODUCTION,
            number: 1,
        },
        RelayerEventData::GatewayChain(event),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TTL: Duration = Duration::from_secs(60);

    fn key(block_number: u64, log_index: u64) -> EventKey {
        EventKey {
            block_number,
            block_hash: FixedBytes::with_last_byte(block_number as u8),
            log_index,
        }
    }

    const ONLY: usize = 0;

    fn range(to_block: u64, outstanding: usize) -> PendingRange {
        PendingRange {
            to_block,
            outstanding: Arc::new(AtomicUsize::new(outstanding)),
        }
    }

    #[tokio::test]
    async fn first_claim_dispatches_and_the_second_does_not() {
        let cache = PhaseCache::new(TTL, 100);

        assert_eq!(cache.claim(key(1, 0), false).await, Claim::Dispatch);
        assert_eq!(cache.claim(key(1, 0), false).await, Claim::Skip);
        assert_eq!(
            cache.claim(key(1, 1), false).await,
            Claim::Dispatch,
            "a different log index is a different event"
        );
    }

    #[tokio::test]
    async fn a_handled_event_is_skipped_by_everyone() {
        let cache = PhaseCache::new(TTL, 100);

        cache.claim(key(2, 0), false).await;
        cache.complete(key(2, 0)).await;

        assert_eq!(cache.claim(key(2, 0), false).await, Claim::Skip);
        assert_eq!(
            cache.claim(key(2, 0), true).await,
            Claim::Skip,
            "a cursor may pass an event that is already handled"
        );
    }

    #[tokio::test]
    async fn an_in_flight_event_is_skipped_without_a_range_and_redispatched_with_one() {
        let cache = PhaseCache::new(TTL, 100);

        cache.claim(key(3, 0), false).await;

        assert_eq!(cache.claim(key(3, 0), false).await, Claim::Skip);
        assert_eq!(cache.claim(key(3, 0), true).await, Claim::Redispatch);
    }

    #[tokio::test]
    async fn a_claim_is_forgotten_after_its_ttl() {
        let cache = PhaseCache::new(Duration::from_millis(50), 100);

        assert_eq!(cache.claim(key(4, 0), false).await, Claim::Dispatch);
        tokio::time::sleep(Duration::from_millis(120)).await;

        assert_eq!(cache.claim(key(4, 0), false).await, Claim::Dispatch);
    }

    #[test]
    fn an_empty_range_completes_on_arrival() {
        let mut cursor = CursorState::default();
        cursor.push(ONLY, range(10, 0));

        assert_eq!(cursor.take_completed_prefix(ONLY), Some(10));
    }

    #[test]
    fn only_the_completed_prefix_advances_the_cursor() {
        let mut cursor = CursorState::default();
        let first = range(10, 1);
        let stalled = first.outstanding.clone();
        cursor.push(ONLY, first);
        cursor.push(ONLY, range(20, 0));
        cursor.push(ONLY, range(30, 0));

        assert_eq!(
            cursor.take_completed_prefix(ONLY),
            None,
            "later ranges wait for the one that is stuck"
        );

        stalled.store(0, Ordering::Release);

        assert_eq!(
            cursor.take_completed_prefix(ONLY),
            Some(30),
            "the whole prefix collapses into one cursor write"
        );
        assert_eq!(cursor.pending_count(ONLY), 0);
    }

    #[test]
    fn one_listener_advances_the_cursor_while_another_lags() {
        let mut cursor = CursorState::default();
        let lagging = range(90, 1);
        cursor.push(1, lagging);
        cursor.push(0, range(100, 0));

        assert_eq!(
            cursor.take_completed_prefix(0),
            Some(100),
            "a listener that saw the whole range carries the shared cursor"
        );
        assert_eq!(
            cursor.take_completed_prefix(1),
            None,
            "the lagging listener's own range is still outstanding"
        );
    }
}
