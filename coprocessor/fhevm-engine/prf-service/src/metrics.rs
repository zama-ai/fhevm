use prometheus::{
    exponential_buckets, register_histogram_vec, register_int_counter_vec, HistogramVec,
    IntCounterVec,
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
