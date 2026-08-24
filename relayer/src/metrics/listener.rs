use prometheus::{
    register_gauge_vec_with_registry, register_gauge_with_registry, Gauge, GaugeVec, Opts, Registry,
};
use std::sync::OnceLock;

#[derive(Debug)]
struct ListenerMetrics {
    /// Attested block ranges whose handlers have not all returned. Zero on a healthy
    /// listener; it climbs once per poll while a handler is stuck.
    pending_ranges: GaugeVec,
    /// Last block recorded as fully handled. One cursor serves the whole relayer, so this
    /// has no listener label; compare it against the chain head to see recovery drift.
    cursor_block: Gauge,
}

static LISTENER_METRICS: OnceLock<ListenerMetrics> = OnceLock::new();

pub fn init_listener_metrics(registry: &Registry) {
    LISTENER_METRICS.get_or_init(|| ListenerMetrics {
        pending_ranges: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_gateway_pending_ranges",
                "Gateway block ranges awaiting their event handlers"
            ),
            &["instance_id"],
            registry,
        )
        .unwrap(),
        cursor_block: register_gauge_with_registry!(
            Opts::new(
                "relayer_gateway_cursor_block",
                "Last gateway block recorded as fully handled"
            ),
            registry,
        )
        .unwrap(),
    });
}

pub fn set_listener_pending_ranges(instance_id: usize, pending: usize) {
    // Absent outside the relayer process: the listeners are exercised by tests that never
    // start a metrics server.
    if let Some(metrics) = LISTENER_METRICS.get() {
        metrics
            .pending_ranges
            .with_label_values(&[&instance_id.to_string()])
            .set(pending as f64);
    }
}

pub fn set_listener_cursor_block(block_number: u64) {
    if let Some(metrics) = LISTENER_METRICS.get() {
        metrics.cursor_block.set(block_number as f64);
    }
}
