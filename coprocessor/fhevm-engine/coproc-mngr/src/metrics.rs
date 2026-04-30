use prometheus::{
    register_int_counter, register_int_counter_vec, IntCounter, IntCounterVec,
};
use std::sync::LazyLock;

pub static INBOUND_EVENT_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coproc_mngr_inbound_event_total",
        "Total upgrade_events rows dispatched"
    )
    .unwrap()
});

pub static INBOUND_NOTIFICATION_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coproc_mngr_inbound_notification_total",
        "Total `event_upgrade` NOTIFYs received"
    )
    .unwrap()
});

pub static INBOUND_POLL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coproc_mngr_inbound_poll_total",
        "Total polling-fallback ticks"
    )
    .unwrap()
});

pub static UPGRADE_EVENT_SUCCESS_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coproc_mngr_event_success_total",
        "Successfully handled upgrade events, by stage label",
        &["stage"]
    )
    .unwrap()
});

pub static UPGRADE_EVENT_FAIL_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coproc_mngr_event_fail_total",
        "Upgrade event handler failures, by reason label",
        &["reason"]
    )
    .unwrap()
});
