//! Row age gauges for each `prfId`. Each lane sets them for all instances, to show a dead lane.

use crate::rows::OldestRows;
use prometheus::{register_gauge_vec, register_int_gauge, GaugeVec, IntGauge};
use std::sync::LazyLock;

static OLDEST_PENDING: LazyLock<GaugeVec> = LazyLock::new(|| {
    register_gauge_vec!(
        "coprocessor_pad_worker_oldest_pending_row_age_seconds",
        "Age of the oldest pending pads row",
        &["prf_instance"]
    )
    .unwrap()
});

static OLDEST_NOT_UPLOADED: LazyLock<GaugeVec> = LazyLock::new(|| {
    register_gauge_vec!(
        "coprocessor_pad_worker_oldest_not_uploaded_row_age_seconds",
        "Age of the oldest complete pads row without an S3 copy",
        &["prf_instance"]
    )
    .unwrap()
});

static STORED_CT64_BYTES: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_pad_worker_stored_ct64_bytes",
        "Bytes of the ct64 stored for Unmask, in all instances"
    )
    .unwrap()
});

pub fn set_stored_ct64_bytes(bytes: i64) {
    STORED_CT64_BYTES.set(bytes);
}

pub fn set_oldest_rows(rows: &[OldestRows]) {
    // An instance with no rows must not keep an old value.
    OLDEST_PENDING.reset();
    OLDEST_NOT_UPLOADED.reset();
    for row in rows {
        let prf = row.prf_instance.to_string();
        OLDEST_PENDING
            .with_label_values(&[&prf])
            .set(row.pending.unwrap_or(0.0));
        OLDEST_NOT_UPLOADED
            .with_label_values(&[&prf])
            .set(row.not_uploaded.unwrap_or(0.0));
    }
}
