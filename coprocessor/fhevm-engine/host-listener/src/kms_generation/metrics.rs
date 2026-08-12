use prometheus::{register_int_counter, IntCounter};
use std::sync::LazyLock;

pub(crate) static ACTIVATE_CRS_SUCCESS_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_activate_crs_success_counter",
            "Number of successful activate CRS requests in host listener"
        )
        .unwrap()
    });

pub(crate) static ACTIVATE_CRS_FAIL_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_activate_crs_fail_counter",
            "Number of failed activate CRS requests in host listener"
        )
        .unwrap()
    });

pub(crate) static CRS_DIGEST_MISMATCH_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_crs_digest_mismatch_counter",
            "Number of CRS digest mismatches in host listener"
        )
        .unwrap()
    });

pub(crate) static ACTIVATE_KEY_SUCCESS_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_activate_key_success_counter",
            "Number of successful activate key requests in host listener"
        )
        .unwrap()
    });

pub(crate) static ACTIVATE_KEY_FAIL_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_activate_key_fail_counter",
            "Number of failed activate key requests in host listener"
        )
        .unwrap()
    });

pub(crate) static KEY_DIGEST_MISMATCH_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_key_digest_mismatch_counter",
            "Number of key digest mismatches in host listener"
        )
        .unwrap()
    });

pub(crate) static KMS_EVENT_DECODE_FAIL_COUNTER: LazyLock<IntCounter> =
    LazyLock::new(|| {
        register_int_counter!(
            "coprocessor_host_listener_kms_event_decode_fail_counter",
            "Number of KMSGeneration logs that failed ABI decoding in host listener"
        )
        .unwrap()
    });
