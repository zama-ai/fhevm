use prometheus::{
    register_int_counter_vec, register_int_gauge_vec, IntCounterVec,
    IntGaugeVec,
};
use std::sync::LazyLock;

pub(crate) static BLOCKS_PROCESSED: LazyLock<IntCounterVec> = LazyLock::new(
    || {
        register_int_counter_vec!(
            "host_poller_blocks_processed",
            "Number of blocks processed successfully by the host-listener poller",
            &["chain_id"]
        )
        .unwrap()
    },
);

pub(crate) static DB_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "host_poller_db_errors",
        "Number of database errors encountered by the host-listener poller",
        &["chain_id"]
    )
    .unwrap()
});

pub(crate) fn inc_blocks_processed(chain_id: &str, count: u64) {
    BLOCKS_PROCESSED
        .with_label_values(&[chain_id])
        .inc_by(count);
}

pub(crate) fn inc_db_errors(chain_id: &str, count: u64) {
    DB_ERRORS.with_label_values(&[chain_id]).inc_by(count);
}

pub(crate) static RPC_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "host_poller_rpc_errors",
        "Number of HTTP/RPC errors encountered by the host-listener poller",
        &["chain_id"]
    )
    .unwrap()
});

pub(crate) fn inc_rpc_errors(chain_id: &str, count: u64) {
    RPC_ERRORS.with_label_values(&[chain_id]).inc_by(count);
}

pub(crate) static RECEIVED_BLOCK: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "host_poller_received_block_number",
        "Most recent block number fetched from the host chain RPC by the host-listener poller, recorded before ingest is attempted",
        &["chain_id"]
    )
    .expect("host_poller_received_block_number metric must register")
});

pub(crate) fn set_received_block(chain_id: &str, block_number: u64) {
    RECEIVED_BLOCK
        .with_label_values(&[chain_id])
        .set(block_number as i64);
}
