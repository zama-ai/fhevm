use std::sync::LazyLock;

use prometheus::{register_int_counter_vec, IntCounterVec};

pub(super) static BAD_TARGET_DIGEST: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_ct64_healing_bad_target_digest_total",
        "Quorum still names the pinned ct64 digest but no peer could supply matching bytes",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(super) static QUORUM_CHANGED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_ct64_healing_quorum_changed_total",
        "Peer ciphertext attestations now quorum on a digest other than the pinned target",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(super) static HEALED_UNCONTAINED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_ct64_healing_uncontained_total",
        "ct64 drift healed after the containment timeout without being contained; descendants rely on verification",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(super) const SUCCESS: &str = "success";
pub(super) const TRANSIENT_FAILURE: &str = "transient_failure";
pub(super) const TERMINAL_FAILURE: &str = "terminal_failure";

pub(super) static ATTEMPTS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_ct64_healing_attempts_total",
        "ct64 healing attempts by outcome: success installs the ct64, transient_failure is retried, terminal_failure cannot succeed without operator action",
        &["consensus_epoch", "outcome"]
    )
    .unwrap()
});
