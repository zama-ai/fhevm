use prometheus::{
    register_histogram_vec, register_int_counter_vec, HistogramVec,
    IntCounterVec,
};
use std::sync::LazyLock;

pub(crate) static BLOCKS_PROCESSED: LazyLock<IntCounterVec> = LazyLock::new(
    || {
        register_int_counter_vec!(
            "host_consumer_blocks_processed",
            "Number of blocks processed successfully by the host-listener consumer",
            &["chain_id"]
        )
        .expect("host_consumer_blocks_processed metric must register")
    },
);

pub(crate) static DB_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "host_consumer_db_errors",
        "Number of database errors encountered by the host-listener consumer",
        &["chain_id"]
    )
    .expect("host_consumer_db_errors metric must register")
});

pub(crate) static BLOCKS_MISSING: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "host_consumer_blocks_processed",
            "Number of blocks not received (gap) by the host-listener consumer",
            &["chain_id"]
        )
        .unwrap()
    });

pub(crate) static BLOCKS_DUPLICATED: LazyLock<IntCounterVec> = LazyLock::new(
    || {
        register_int_counter_vec!(
            "host_consumer_blocks_duplicated",
            "Number of blocks received several time by the host-listener consumer",
            &["chain_id"]
        )
        .unwrap()
    },
);

pub(crate) static LEGACY_INSERT_DELAY_SECONDS: LazyLock<HistogramVec> =
    LazyLock::new(|| {
        register_histogram_vec!(
            "host_consumer_legacy_insert_delay_seconds",
            "Delay in seconds between legacy host-listener block insertion and host-listener consumer block insertion, clamped to zero when the consumer is first",
            &["chain_id"],
            vec![0.0, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0]
        )
        .expect("host_consumer_legacy_insert_delay_seconds metric must register")
    });

pub(crate) fn inc_blocks_processed(chain_id: &str, count: u64) {
    BLOCKS_PROCESSED
        .with_label_values(&[chain_id])
        .inc_by(count);
}

pub(crate) fn inc_db_errors(chain_id: &str, count: u64) {
    DB_ERRORS.with_label_values(&[chain_id]).inc_by(count);
}

pub(crate) fn inc_blocks_missing(chain_id: &str, count: u64) {
    BLOCKS_MISSING.with_label_values(&[chain_id]).inc_by(count);
}

pub(crate) fn inc_gap_missing(chain_id: &str, count: u64) {
    BLOCKS_MISSING.with_label_values(&[chain_id]).inc_by(count);
}

pub(crate) fn inc_blocks_duplicated(chain_id: &str, count: u64) {
    BLOCKS_DUPLICATED
        .with_label_values(&[chain_id])
        .inc_by(count);
}

pub(crate) fn observe_legacy_insert_delay_seconds(
    chain_id: &str,
    delay_seconds: f64,
) {
    LEGACY_INSERT_DELAY_SECONDS
        .with_label_values(&[chain_id])
        .observe(delay_seconds);
}
