use fhevm_engine_common::telemetry::{register_histogram, MetricsConfig};
use prometheus::Histogram;
use std::sync::{LazyLock, OnceLock};

pub mod dfg;

pub static RERAND_LATENCY_HISTOGRAM_CONF: OnceLock<MetricsConfig> = OnceLock::new();
pub static RERAND_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        RERAND_LATENCY_HISTOGRAM_CONF.get(),
        "coprocessor_rerand_latency_seconds",
        "Re-randomization latencies per transaction in seconds",
    )
});

pub static FHE_LATENCY_HISTOGRAM_CONF: OnceLock<MetricsConfig> = OnceLock::new();
pub static FHE_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        FHE_LATENCY_HISTOGRAM_CONF.get(),
        "coprocessor_fhe_batch_latency_seconds",
        "The latency of FHE operations within a single transaction, in seconds",
    )
});
