use connector_utils::monitoring::http::{
    DURATION_BUCKETS, HttpMetrics, ROUTES, UNMATCHED_ROUTE, route_label,
};
use kms_connector_api::{PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE};
use prometheus::{
    HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge,
    core::{Collector, Desc},
    proto::MetricFamily,
    register_histogram_vec, register_int_counter, register_int_counter_vec,
};
use std::sync::{Arc, LazyLock};
use tokio::sync::Semaphore;

/// Endpoint request counts by route, status, and code, request durations in seconds by route,
/// and active requests.
pub static HTTP: LazyLock<HttpMetrics> =
    LazyLock::new(|| HttpMetrics::register("kms_connector_endpoint"));

/// Decryption lookup counts by route and outcome: served, stored, retried, or already_in_progress.
///
/// Updated by `decrypt::handle` after the database transaction commits.
pub static LOOKUP: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_endpoint_decryption_lookup_counter",
        "Committed decryption lookups by outcome",
        &["route", "outcome"]
    )
    .expect("register lookup counter")
});
/// Time in seconds spent waiting for a decryption response and reading its database row.
///
/// Only waits ending with a response are recorded, so timeouts do not mask the response latency.
/// Timeouts, listener disruptions and cancellations are counted by the request counter.
pub static WAIT: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        HistogramOpts::new(
            "kms_connector_endpoint_decryption_wait_duration_seconds",
            "Time waiting for a response, for waits ending with a response"
        )
        .buckets(DURATION_BUCKETS.to_vec()),
        &["route"]
    )
    .expect("register wait histogram")
});
/// Successful PostgreSQL listener reconnections after a connection is lost.
pub static RECONNECTIONS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_endpoint_response_listener_reconnections_counter",
        "Re-established Postgres listener connections"
    )
    .expect("register reconnection counter")
});
/// HTTP requests whose wait for a decryption response failed because the listener was disrupted.
///
/// `wait_and_read_response` increments this once per affected request, including requests
/// waiting for the same decryption ID.
pub static WAITERS_FAILED: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_endpoint_response_listener_waiters_failed_counter",
        "HTTP requests failed by a disrupted response listener"
    )
    .expect("register disrupted waiter counter")
});

/// Registers endpoint metrics and creates zero-valued counters and histograms for known labels.
///
/// Called once at endpoint startup, through `Endpoint::register_metrics`, so metrics appear in
/// scrapes before any requests arrive. Forces the global `LazyLock` metrics to register at
/// startup rather than on first use.
/// Also registers a collector that reads used permits from `limiter` on each scrape;
/// `capacity` must be the semaphore's configured total number of permits.
/// Returns an error if the admission gauges cannot be created or registered, e.g. when called
/// twice in the same process.
pub fn init(limiter: Arc<Semaphore>, capacity: usize) -> prometheus::Result<()> {
    LazyLock::force(&RECONNECTIONS);
    LazyLock::force(&WAITERS_FAILED);
    for route in ROUTES.into_iter().filter(|&route| route != UNMATCHED_ROUTE) {
        HTTP.requests.with_label_values(&[route, "200", "success"]);
    }
    for route in [PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE].map(route_label) {
        for outcome in ["served", "stored", "retried", "already_in_progress"] {
            LOOKUP.with_label_values(&[route, outcome]);
        }
        WAIT.with_label_values(&[route]);
    }
    let limit = IntGauge::new(
        "kms_connector_endpoint_decryption_capacity",
        "Maximum admitted decryption requests",
    )?;
    limit.set(capacity as i64);
    prometheus::default_registry().register(Box::new(AdmissionCollector {
        limiter,
        capacity,
        in_flight: IntGauge::new(
            "kms_connector_endpoint_decryptions_in_flight",
            "Decryption requests holding admission permits",
        )?,
        limit,
    }))
}

/// Prometheus [`Collector`] that reads used semaphore permits on each scrape.
struct AdmissionCollector {
    /// Semaphore that limits the number of concurrent decryption requests.
    limiter: Arc<Semaphore>,
    /// Total number of permits configured for the semaphore.
    capacity: usize,
    /// Gauge reporting the number of decryption requests currently holding permits.
    in_flight: IntGauge,
    /// Gauge reporting the maximum number of concurrent decryption requests.
    limit: IntGauge,
}

impl Collector for AdmissionCollector {
    fn desc(&self) -> Vec<&Desc> {
        [self.in_flight.desc(), self.limit.desc()].concat()
    }
    fn collect(&self) -> Vec<MetricFamily> {
        self.in_flight.set(
            self.capacity
                .saturating_sub(self.limiter.available_permits()) as i64,
        );
        [self.in_flight.collect(), self.limit.collect()].concat()
    }
}
