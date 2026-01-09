use std::sync::LazyLock;

use prometheus::{register_histogram_vec, HistogramVec};
use prometheus::{register_int_counter_vec, IntCounterVec};
use tracing::warn;

pub(crate) static LATENCY_TO_DB: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        "host_listener_latency_to_db_ms",
        "Latency to insert into DB (milli-seconds)",
        &["chain_id"]
    )
    .unwrap()
});

pub(crate) static LATENCY_FROM_HOST: LazyLock<HistogramVec> =
    LazyLock::new(|| {
        register_histogram_vec!(
            "host_listener_latency_from_host_ms",
            "Latency to fetch block from host (milli-seconds)",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static APPROX_LATENCY_TO_NOTIFY_BLOCK: LazyLock<HistogramVec> =
    LazyLock::new(|| {
        register_histogram_vec!(
            "host_listener_approx_latency_block_notify",
            "Latency to be notify new block from host (seconds)",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static REALTIME_BLOCKS_INSERTED: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "host_listener_realtime_blocks_inserted",
            "Number of blocks processed successfully by the host-listener",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static REALTIME_EVENTS_INSERTED: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "host_listener_realtime_blocks_inserted",
            "Number of blocks processed successfully by the host-listener",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static CATCHUP_BLOCKS_INSERTED: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "host_listener_catchup_blocks_inserted",
            "Number of blocks processed successfully by the host-listener",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static BLOCKS_NOT_INSERTED: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "host_listener_blocks_not_inserted",
            "Number of blocks not inserted successfully by the host-listener",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static EVENTS_MISSED: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "host_listener_blocks_missed",
            "Number of blocks seen missed by catchup on the host-listener",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) fn touch(chain_id: &str, metric: &LazyLock<IntCounterVec>) {
    metric.with_label_values(&[chain_id]).inc_by(1);
}

pub(crate) fn increment(
    chain_id: &str,
    metric: &LazyLock<IntCounterVec>,
    count: usize,
) {
    if count == 0 {
        return;
    }
    metric.with_label_values(&[chain_id]).inc_by(count as u64);
}

pub(crate) fn latency_metric(
    chain_id: &str,
    metric: &LazyLock<HistogramVec>,
) -> Result<prometheus::Histogram, prometheus::Error> {
    match metric.get_metric_with_label_values(&[chain_id]) {
        Ok(m) => Ok(m),
        Err(e) => {
            warn!("Failed to get metric for chain_id {}: {}", chain_id, e);
            Err(e)
        }
    }
}

pub(crate) fn latency_timer(
    chain_id: &str,
    metric: &LazyLock<HistogramVec>,
) -> Result<prometheus::HistogramTimer, prometheus::Error> {
    latency_metric(chain_id, metric).map(|m| m.start_timer())
}

pub(crate) fn observe_timer(
    timer: Result<prometheus::HistogramTimer, prometheus::Error>,
) {
    let Ok(t) = timer else {
        return;
    };
    t.observe_duration();
}

pub(crate) fn observe_approximate_latency(
    chain_id: &str,
    metric: &LazyLock<HistogramVec>,
    approximate_start_timestamp_in_seconds: u64,
) {
    let Ok(t) = metric.get_metric_with_label_values(&[chain_id]) else {
        return;
    };
    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(approximate_start_timestamp_in_seconds);
    let latency = current_timestamp
        .saturating_sub(approximate_start_timestamp_in_seconds);
    t.observe(latency as f64);
}
