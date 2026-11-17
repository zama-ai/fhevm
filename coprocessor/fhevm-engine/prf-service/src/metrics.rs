use prometheus::{
    exponential_buckets, register_histogram_vec, register_int_counter_vec, register_int_gauge,
    HistogramVec, IntCounterVec, IntGauge,
};
use std::sync::LazyLock;

pub(crate) static REQUESTS_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_prf_service_requests_total",
        "Number of PRF output requests, by method and status code",
        &["method", "status"]
    )
    .unwrap()
});

pub(crate) static REQUEST_DURATION: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        "coprocessor_prf_service_request_duration_seconds",
        "Duration of PRF output requests in seconds, by method and status code",
        &["method", "status"],
        // 5 ms to 164 s: above the 120 s default response timeout of coprocessor-proxy.
        exponential_buckets(0.005, 2.0, 16).unwrap()
    )
    .unwrap()
});

pub(crate) static BODY_REFUSALS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_prf_service_body_refusals_total",
        "Requests refused by the body budget: `connector` (429) or `pod` (503)",
        &["reason"]
    )
    .unwrap()
});

pub(crate) static BODY_BYTES_HELD: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_prf_service_body_bytes_held",
        "Bytes of response bodies in memory, as the body budget counts them"
    )
    .unwrap()
});
