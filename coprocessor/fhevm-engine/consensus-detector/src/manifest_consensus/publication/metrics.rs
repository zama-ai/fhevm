use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
    time::Duration,
};

use prometheus::{register_int_counter_vec, register_int_gauge_vec, IntCounterVec, IntGaugeVec};
use sqlx::PgPool;
use tracing::error;

pub(crate) static MANIFEST_PUBLICATION_SUCCESS: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_publication_success_total",
        "Number of locally archived and published consensus manifests",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(crate) static MANIFEST_PUBLICATION_FAILURE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_publication_failure_total",
        "Number of failed local manifest publication attempts",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(crate) static MANIFEST_WORK_SELECTION_TIMEOUT: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_publication_work_selection_timeout_total",
        "Number of manifest work-selection queries cancelled by their database time limit",
        &["consensus_epoch"]
    )
    .unwrap()
});

static PENDING_MANIFEST_WORK: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_publication_pending_work",
        "Number of block rows still requiring sealing or manifest publication",
        &["consensus_epoch"]
    )
    .unwrap()
});

static MANIFEST_PUBLICATION_RETRY_EXHAUSTED: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_publication_retry_exhausted",
        "Number of manifests whose local publication retry limit is exhausted",
        &["consensus_epoch"]
    )
    .unwrap()
});

static SEEN_CONSENSUS_EPOCHS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub(crate) fn spawn_publication_gauge_updates(period: Duration, pool: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = update_publication_gauges(&pool).await {
                error!(%error, "Failed to update manifest publication gauges");
            }
            tokio::time::sleep(period).await;
        }
    });
}

async fn update_publication_gauges(pool: &PgPool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT consensus_epoch,
               COUNT(*) FILTER (
                   WHERE block_content_digest IS NULL
                      OR (manifest_required AND manifest_published = FALSE)
               )::BIGINT AS "pending_work!",
               COUNT(*) FILTER (
                   WHERE manifest_required
                     AND NOT manifest_published
                     AND publication_error_count > 0
                     AND publication_next_retry_at IS NULL
               )::BIGINT AS "publication_retry_exhausted!"
          FROM block_manifest_state
         GROUP BY consensus_epoch
        "#,
    )
    .fetch_all(pool)
    .await?;

    let live: HashSet<String> = rows.iter().map(|row| row.consensus_epoch.clone()).collect();
    for row in &rows {
        let consensus_epoch = row.consensus_epoch.as_str();
        PENDING_MANIFEST_WORK
            .with_label_values(&[consensus_epoch])
            .set(row.pending_work);
        MANIFEST_PUBLICATION_RETRY_EXHAUSTED
            .with_label_values(&[consensus_epoch])
            .set(row.publication_retry_exhausted);
    }

    let mut seen = SEEN_CONSENSUS_EPOCHS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    for consensus_epoch in seen.difference(&live) {
        let consensus_epoch = consensus_epoch.as_str();
        let _ = PENDING_MANIFEST_WORK.remove_label_values(&[consensus_epoch]);
        let _ = MANIFEST_PUBLICATION_RETRY_EXHAUSTED.remove_label_values(&[consensus_epoch]);
    }
    *seen = live;
    Ok(())
}
