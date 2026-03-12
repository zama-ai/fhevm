use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
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

pub(crate) static DRIFT_DETECTED_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_drift_detected_counter",
        "Number of detected drifts from peer submissions or consensus mismatch"
    )
    .unwrap()
});

pub(crate) static CONSENSUS_TIMEOUT_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_consensus_timeout_counter",
        "Number of handles that timed out without a consensus event"
    )
    .unwrap()
});

pub(crate) static MISSING_SUBMISSION_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_gw_listener_missing_submission_counter",
        "Number of handles where consensus was reached but some coprocessors never submitted"
    )
    .unwrap()
});

pub(crate) static CONSENSUS_LATENCY_BLOCKS_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    // Use this distribution to tune `--drift-no-consensus-timeout-blocks`.
    // In steady state, that timeout should sit above the normal tail of
    // `consensus_block - first_seen_block` so we alert on truly stalled handles
    // without retaining healthy ones for too long.
    register_histogram!(
        "coprocessor_gw_listener_consensus_latency_blocks",
        "Block distance between first observed submission and consensus",
        vec![1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0, 55.0, 89.0, 144.0]
    )
    .unwrap()
});

pub(crate) static POST_CONSENSUS_COMPLETION_BLOCKS_HISTOGRAM: LazyLock<Histogram> =
    LazyLock::new(|| {
        // Use this distribution to tune `--drift-post-consensus-grace-blocks`.
        // In steady state, that grace window should sit above the normal tail of
        // `all_submissions_seen_block - consensus_block` so lagging-but-healthy
        // coprocessors do not page, while truly missing submissions age out.
        register_histogram!(
            "coprocessor_gw_listener_post_consensus_completion_blocks",
            "Block distance between consensus and seeing all expected submissions",
            vec![0.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0]
        )
        .unwrap()
    });
