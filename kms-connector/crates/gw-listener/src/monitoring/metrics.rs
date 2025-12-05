use prometheus::{IntCounterVec, register_int_counter_vec};
use std::sync::LazyLock;

pub static EVENT_RECEIVED_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_gw_listener_event_received_counter",
        "Number of events received by the GatewayListener",
        &["event_type"]
    )
    .unwrap()
});

pub static EVENT_RECEIVED_ERRORS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "kms_connector_gw_listener_event_received_errors",
        "Number of errors encountered by the GatewayListener while receiving events",
        &["event_type"]
    )
    .unwrap()
});
