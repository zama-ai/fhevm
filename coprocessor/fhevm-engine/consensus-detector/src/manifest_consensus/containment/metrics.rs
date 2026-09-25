use std::sync::LazyLock;

use prometheus::{register_int_counter, IntCounter};

pub(super) static BARRIER_LOCK_TIMEOUT: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_containment_barrier_lock_timeout_total",
        "Guaranteed containment gave up waiting for running TFHE batches: a batch is stuck while ct64 drift is uncontained"
    )
    .unwrap()
});
