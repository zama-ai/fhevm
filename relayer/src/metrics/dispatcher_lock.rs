use prometheus::{register_gauge_with_registry, Gauge, Opts, Registry};
use std::sync::OnceLock;

/// Whether this pod holds the HA dispatch lock: 1 while held, 0 otherwise. A Grafana alert on
/// `sum(relayer_dispatcher_lock_held) == 0` across pods catches "no pod holds the dispatch
/// lock" - the state this metric exists for.
#[derive(Debug)]
struct DispatcherLockMetrics {
    held: Gauge,
}

static DISPATCHER_LOCK_METRICS: OnceLock<DispatcherLockMetrics> = OnceLock::new();

pub fn init_dispatcher_lock_metrics(registry: &Registry) {
    DISPATCHER_LOCK_METRICS.get_or_init(|| DispatcherLockMetrics {
        held: register_gauge_with_registry!(
            Opts::new(
                "relayer_dispatcher_lock_held",
                "1 if this pod holds the HA dispatch lock, 0 otherwise"
            ),
            registry,
        )
        .unwrap(),
    });
}

pub fn set_dispatcher_lock_held(held: bool) {
    // Absent outside the relayer process: the lock is exercised by tests that never start a
    // metrics server.
    if let Some(metrics) = DISPATCHER_LOCK_METRICS.get() {
        metrics.held.set(if held { 1.0 } else { 0.0 });
    }
}
