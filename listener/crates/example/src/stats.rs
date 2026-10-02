//! Counters describing what this consumer actually received.
//!
//! Without these the example is unobservable: a correct run and a completely
//! broken one both produce no output when the watched contract happens to be
//! quiet, because [`crate::transfer::log_transfers`] only prints matching
//! `Transfer` events and the stale-block drop is a `debug!`.
//!
//! `dropped_stale` is the interesting one. It counts blocks belonging to a
//! catchup this consumer has since retired — the sub-ranges that were already
//! in flight when a cancel landed. A cancel stops the listener *fetching*; it
//! cannot recall work already claimed by a worker, so a non-zero value here
//! after a cancel is expected behaviour, not a fault.

use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;

use crate::catchup_state::Flow;

/// Delivery counters for one catchup flow.
#[derive(Debug, Default)]
pub struct CatchupCounters {
    delivered: AtomicU64,
    dropped_stale: AtomicU64,
}

impl CatchupCounters {
    pub fn record_delivered(&self) {
        self.delivered.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_dropped_stale(&self) {
        self.dropped_stale.fetch_add(1, Ordering::Relaxed);
    }

    fn snapshot(&self) -> CatchupCountersSnapshot {
        CatchupCountersSnapshot {
            delivered: self.delivered.load(Ordering::Relaxed),
            dropped_stale: self.dropped_stale.load(Ordering::Relaxed),
        }
    }
}

/// Everything the example counts, across all four flows.
///
/// The live and final counters exist to disambiguate a failure: "zero catchup
/// blocks delivered" means something very different depending on whether the
/// head-of-chain stream is also silent.
#[derive(Debug, Default)]
pub struct Stats {
    live_delivered: AtomicU64,
    final_delivered: AtomicU64,
    catchup: CatchupCounters,
    final_catchup: CatchupCounters,
}

impl Stats {
    pub fn record_live_delivered(&self) {
        self.live_delivered.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_final_delivered(&self) {
        self.final_delivered.fetch_add(1, Ordering::Relaxed);
    }

    /// The counters for one catchup flow.
    pub fn catchup(&self, flow: Flow) -> &CatchupCounters {
        match flow {
            Flow::Catchup => &self.catchup,
            Flow::FinalCatchup => &self.final_catchup,
        }
    }

    pub fn snapshot(&self) -> StatsSnapshot {
        StatsSnapshot {
            live_delivered: self.live_delivered.load(Ordering::Relaxed),
            final_delivered: self.final_delivered.load(Ordering::Relaxed),
            catchup: self.catchup.snapshot(),
            final_catchup: self.final_catchup.snapshot(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CatchupCountersSnapshot {
    pub delivered: u64,
    pub dropped_stale: u64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct StatsSnapshot {
    pub live_delivered: u64,
    pub final_delivered: u64,
    pub catchup: CatchupCountersSnapshot,
    pub final_catchup: CatchupCountersSnapshot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_are_per_flow() {
        let stats = Stats::default();
        stats.catchup(Flow::Catchup).record_delivered();
        stats.catchup(Flow::Catchup).record_delivered();
        stats.catchup(Flow::FinalCatchup).record_dropped_stale();

        let snap = stats.snapshot();
        assert_eq!(snap.catchup.delivered, 2);
        assert_eq!(snap.catchup.dropped_stale, 0);
        assert_eq!(snap.final_catchup.delivered, 0);
        assert_eq!(snap.final_catchup.dropped_stale, 1);
    }
}
