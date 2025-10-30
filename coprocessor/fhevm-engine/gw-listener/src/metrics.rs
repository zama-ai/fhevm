use prometheus::{register_int_counter, IntCounter};
use std::sync::LazyLock;

pub(crate) static VERIFY_PROOF_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_verify_proof_success_counter",
        "Number of successful verify request events in GW listener"
    )
    .unwrap()
});

pub(crate) static VERIFY_PROOF_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_verify_proof_fail_counter",
        "Number of failed verify request events in GW listener"
    )
    .unwrap()
});

pub(crate) static GET_BLOCK_NUM_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_get_block_num_success_counter",
        "Number of successful get block num requests in GW listener"
    )
    .unwrap()
});

pub(crate) static GET_BLOCK_NUM_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_get_block_num_fail_counter",
        "Number of failed get block num requests in GW listener"
    )
    .unwrap()
});

pub(crate) static GET_LOGS_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_get_logs_success_counter",
        "Number of successful get logs requests in GW listener"
    )
    .unwrap()
});

pub(crate) static GET_LOGS_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_get_logs_fail_counter",
        "Number of failed get logs requests in GW listener"
    )
    .unwrap()
});

pub(crate) static ACTIVATE_CRS_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_activate_crs_success_counter",
        "Number of successful activate CRS requests in GW listener"
    )
    .unwrap()
});

pub(crate) static ACTIVATE_CRS_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_activate_crs_fail_counter",
        "Number of failed activate CRS requests in GW listener"
    )
    .unwrap()
});

pub(crate) static CRS_DIGEST_MISMATCH_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_crs_digest_mismatch_counter",
        "Number of CRS digest mismatches in GW listener"
    )
    .unwrap()
});

pub(crate) static ACTIVATE_KEY_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_activate_key_success_counter",
        "Number of successful activate key requests in GW listener"
    )
    .unwrap()
});

pub(crate) static ACTIVATE_KEY_FAIL_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_activate_key_fail_counter",
        "Number of failed activate key requests in GW listener"
    )
    .unwrap()
});

pub(crate) static KEY_DIGEST_MISMATCH_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_key_digest_mismatch_counter",
        "Number of key digest mismatches in GW listener"
    )
    .unwrap()
});
