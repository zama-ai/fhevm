use prometheus::{register_int_counter, IntCounter};
use std::sync::LazyLock;

pub(crate) static PROTOCOL_CONFIG_EVENT_DECODE_FAIL_COUNTER: LazyLock<
    IntCounter,
> = LazyLock::new(|| {
    register_int_counter!(
            "coprocessor_host_listener_protocol_config_event_decode_fail_counter",
            "Number of ProtocolConfig logs that failed ABI decoding in host listener"
        )
        .unwrap()
});
