use prometheus::{register_int_counter_vec, IntCounterVec};
use std::sync::LazyLock;

pub(crate) static REQUESTS_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_prf_service_requests_total",
        "Number of PRF output requests, by method and status code",
        &["method", "status"]
    )
    .unwrap()
});
