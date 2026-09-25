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
