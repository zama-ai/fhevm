use prometheus::{register_int_counter_vec, IntCounterVec};
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

pub(crate) fn inc_blocks_processed(chain_id: &str, count: u64) {
    BLOCKS_PROCESSED
        .with_label_values(&[chain_id])
        .inc_by(count);
}

pub(crate) fn inc_db_errors(chain_id: &str, count: u64) {
    DB_ERRORS.with_label_values(&[chain_id]).inc_by(count);
}
