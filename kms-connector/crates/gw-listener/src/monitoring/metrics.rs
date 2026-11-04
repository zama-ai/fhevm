use prometheus::{IntCounterVec, IntGaugeVec, register_int_counter_vec, register_int_gauge_vec};
use std::sync::LazyLock;

pub static EVENT_RECEIVED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_gw_listener_event_received_counter",
        "Number of events received by the GatewayListener",
        &["event_type"]
    )
    .expect("Failed to register kms_connector_gw_listener_event_received_counter metric")
});

pub static EVENT_LISTENING_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_gw_listener_event_listening_errors",
        "Number of errors encountered by the GatewayListener while listening for events",
        &["contract"]
    )
    .expect("Failed to register kms_connector_gw_listener_event_listening_errors metric")
});

pub static RPC_HEAD_BLOCK: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "kms_connector_gw_listener_rpc_head_block",
        "Latest head reported by the chain's RPC node (finalized block for Ethereum)",
        &["chain"]
    )
    .expect("Failed to register kms_connector_gw_listener_rpc_head_block metric")
});

pub static LAST_POLLED_BLOCK: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "kms_connector_gw_listener_last_polled_block",
        "End of the last block range polled by the GatewayListener and stored in DB",
        &["chain"]
    )
    .expect("Failed to register kms_connector_gw_listener_last_polled_block metric")
});
