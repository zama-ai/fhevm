use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use prometheus::{register_int_counter, register_int_gauge, IntCounter, IntGauge};
use sqlx::PgPool;
use tracing::error;

use crate::manifest_consensus::ManifestWorkGate;

pub(crate) static MANIFEST_PUBLICATION_SUCCESS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_manifest_publication_success_total",
        "Number of locally archived and published consensus manifests"
    )
    .unwrap()
});

pub(crate) static MANIFEST_PUBLICATION_FAILURE: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_manifest_publication_failure_total",
        "Number of failed local manifest publication attempts"
    )
    .unwrap()
});

pub(crate) static MANIFEST_WORK_SELECTION_TIMEOUT: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_manifest_work_selection_timeout_total",
        "Number of manifest work-selection queries cancelled by their database time limit"
    )
    .unwrap()
});

static PENDING_MANIFEST_WORK: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_manifest_pending_work",
        "Number of block rows still requiring sealing or manifest publication"
    )
    .unwrap()
});

static MANIFEST_PUBLICATION_RETRY_EXHAUSTED: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_manifest_publication_retry_exhausted",
        "Number of manifests whose local publication retry limit is exhausted"
    )
    .unwrap()
});

static LATEST_MANIFEST_PUBLICATION_UNIX_SECONDS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_manifest_latest_publication_unixtime",
        "Unix timestamp of the latest local manifest publication"
    )
    .unwrap()
});

pub(crate) fn spawn_publication_gauge_updates(
    period: Duration,
    pool: PgPool,
    work_gate: Arc<ManifestWorkGate>,
) {
    tokio::spawn(async move {
        loop {
            match work_gate.pinned_generation() {
                Some(generation) => {
                    if let Err(error) = update_publication_gauges(&pool, generation).await {
                        error!(%error, "Failed to update manifest publication gauges");
                    }
                }
                None => reset_publication_gauges(),
            }
            tokio::time::sleep(period).await;
        }
    });
}

fn reset_publication_gauges() {
    PENDING_MANIFEST_WORK.set(0);
    MANIFEST_PUBLICATION_RETRY_EXHAUSTED.set(0);
    LATEST_MANIFEST_PUBLICATION_UNIX_SECONDS.set(0);
}

async fn update_publication_gauges(pool: &PgPool, generation: i64) -> Result<(), sqlx::Error> {
    let totals = sqlx::query!(
        r#"
        SELECT
            (SELECT COUNT(*)::BIGINT
              FROM block_manifest_state
              WHERE generation = $1
                AND (block_content_digest IS NULL
                 OR (manifest_required AND manifest_published = FALSE))) AS "pending_work!",
            (SELECT COUNT(*)::BIGINT
              FROM block_manifest_state
              WHERE generation = $1
                AND manifest_required
                AND NOT manifest_published
                AND publication_error_count > 0
                AND publication_next_retry_at IS NULL) AS "publication_retry_exhausted!",
            COALESCE((
                SELECT EXTRACT(EPOCH FROM MAX(manifest_published_at))::BIGINT
                  FROM block_manifest_state
                 WHERE generation = $1
                   AND manifest_published
            ), 0) AS "latest_publication!"
        "#,
        generation,
    )
    .fetch_one(pool)
    .await?;
    PENDING_MANIFEST_WORK.set(totals.pending_work);
    MANIFEST_PUBLICATION_RETRY_EXHAUSTED.set(totals.publication_retry_exhausted);
    LATEST_MANIFEST_PUBLICATION_UNIX_SECONDS.set(totals.latest_publication);
    Ok(())
}
