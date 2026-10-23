use crate::core::LOOKUP_OUTCOME_LABELS;
use connector_utils::monitoring::http::{DURATION_BUCKETS, HttpMetrics};
use kms_connector_api::{PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE, VERSION_ROUTE};
use prometheus::{
    HistogramOpts, HistogramVec, IntCounterVec, IntGauge,
    core::{Collector, Desc},
    proto::MetricFamily,
    register_histogram_vec, register_int_counter_vec,
};
use std::sync::{Arc, LazyLock};
use tokio::sync::Semaphore;

/// Endpoint request counts by route, status, and code, request durations in seconds by route,
/// and active requests.
pub static HTTP_METRICS: LazyLock<HttpMetrics> =
    LazyLock::new(|| HttpMetrics::register("kms_connector_endpoint"));

/// Decryption lookup counts by route and outcome: served, stored, retried, or already_in_progress.
pub static DECRYPTION_LOOKUP_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_endpoint_decryption_lookup_counter",
        "Committed decryption lookups by outcome",
        &["route", "outcome"]
    )
    .expect("Failed to register kms_connector_endpoint_decryption_lookup_counter metric")
});
/// Time in seconds spent waiting for a decryption response and reading its database row.
///
/// Only waits ending with a response are recorded, so timeouts do not mask the response latency.
pub static DECRYPTION_WAIT_LATENCY_HISTOGRAM: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        HistogramOpts::new(
            "kms_connector_endpoint_decryption_wait_latency_seconds",
            "Time waiting for a response, for waits ending with a response"
        )
        .buckets(DURATION_BUCKETS.to_vec()),
        &["route"]
    )
    .expect("Failed to register kms_connector_endpoint_decryption_wait_latency_seconds metric")
});

/// Registers endpoint metrics and creates zero-valued counters and histograms for known labels.
///
/// Should be called once at endpoint startup, through `Endpoint::register_metrics`, so metrics appear in
/// scrapes before any requests arrive.
/// Panics if the admission gauges cannot be registered, e.g. when called twice in the same process.
pub(crate) fn init(decryption_limiter: Arc<Semaphore>, capacity: usize) {
    for route in [
        PUBLIC_DECRYPTION_ROUTE,
        USER_DECRYPTION_ROUTE,
        VERSION_ROUTE,
    ] {
        HTTP_METRICS
            .requests
            .with_label_values(&[route, "200", "success"]);
    }
    for route in [PUBLIC_DECRYPTION_ROUTE, USER_DECRYPTION_ROUTE] {
        for outcome in LOOKUP_OUTCOME_LABELS {
            DECRYPTION_LOOKUP_COUNTER.with_label_values(&[route, outcome]);
        }
        DECRYPTION_WAIT_LATENCY_HISTOGRAM.with_label_values(&[route]);
    }
    let limit = IntGauge::new(
        "kms_connector_endpoint_max_in_flight_decryptions",
        "Configured max_in_flight_decryptions",
    )
    .expect("Failed to create kms_connector_endpoint_max_in_flight_decryptions metric");
    limit.set(capacity as i64);
    let in_flight = IntGauge::new(
        "kms_connector_endpoint_decryptions_in_flight",
        "Decryption requests holding admission permits",
    )
    .expect("Failed to create kms_connector_endpoint_decryptions_in_flight metric");
    prometheus::default_registry()
        .register(Box::new(AdmissionCollector {
            decryption_limiter,
            permits: capacity,
            in_flight,
            max_in_flight_decryptions: limit,
        }))
        .expect("Failed to register kms_connector_endpoint_decryptions_in_flight and kms_connector_endpoint_max_in_flight_decryptions metrics");
}

/// Prometheus [`Collector`] that reads used semaphore permits on each scrape.
struct AdmissionCollector {
    /// Semaphore that limits the number of concurrent decryption requests.
    decryption_limiter: Arc<Semaphore>,
    /// Total number of permits configured for the semaphore.
    permits: usize,
    /// Gauge reporting the number of decryption requests currently holding permits.
    in_flight: IntGauge,
    /// Gauge reporting the configured `max_in_flight_decryptions`.
    max_in_flight_decryptions: IntGauge,
}

impl Collector for AdmissionCollector {
    fn desc(&self) -> Vec<&Desc> {
        [self.in_flight.desc(), self.max_in_flight_decryptions.desc()].concat()
    }
    fn collect(&self) -> Vec<MetricFamily> {
        self.in_flight.set(
            self.permits
                .saturating_sub(self.decryption_limiter.available_permits()) as i64,
        );
        [
            self.in_flight.collect(),
            self.max_in_flight_decryptions.collect(),
        ]
        .concat()
    }
}
