use prometheus::{IntCounter, register_int_counter};
use std::sync::LazyLock;

pub static EVENT_RECEIVED_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_gw_listener_event_received_counter",
        "Number of events received by the GatewayListener"
    )
    .unwrap()
});

pub static EVENT_RECEIVED_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_gw_listener_event_received_errors",
        "Number of errors encountered by the GatewayListener while receiving events"
    )
    .unwrap()
});

pub static EVENT_STORED_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_gw_listener_event_stored_counter",
        "Number of events stored in DB by the GatewayListener"
    )
    .unwrap()
});

pub static EVENT_STORAGE_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "kms_connector_gw_listener_event_storage_errors",
        "Number of errors encountered by the GatewayListener while trying to store events in DB"
    )
    .unwrap()
});
