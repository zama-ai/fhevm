use std::{sync::LazyLock, time::Duration};

use prometheus::{
    register_int_counter, register_int_counter_vec, register_int_gauge, register_int_gauge_vec,
    IntCounter, IntCounterVec, IntGauge, IntGaugeVec,
};
use sqlx::PgPool;
use tracing::error;

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

pub(crate) static PEER_MANIFEST_ARCHIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_peer_manifest_archived_total",
        "Number of peer manifest objects durably archived"
    )
    .unwrap()
});

pub(crate) static PEER_MANIFEST_DOWNLOAD_FAILURE: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_peer_manifest_download_failure_total",
        "Number of peer manifest listing, download, or authentication failures"
    )
    .unwrap()
});

pub(crate) static VERIFICATION_OUTCOMES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_sns_manifest_verification_total",
        "Number of completed manifest verification attempts by outcome",
        &["outcome"]
    )
    .unwrap()
});

pub(crate) static VERIFICATION_FAILURE: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_manifest_verification_failure_total",
        "Number of manifest verification runs aborted before producing an outcome"
    )
    .unwrap()
});

pub(crate) static DRIFT_LOCALIZATION_INCOMPLETE: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_drift_localization_incomplete_total",
        "Number of drift evaluations whose handle inventory could not be fully localized"
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

static UNRESOLVED_DRIFT_HANDLES: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_drift_handles_unresolved",
        "Number of unresolved handle-level drift findings"
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

static OLDEST_DUE_VERIFICATION_AGE_SECONDS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_manifest_oldest_due_verification_age_seconds",
        "Age in seconds of the oldest due verification target"
    )
    .unwrap()
});

static VERIFICATION_TARGETS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_sns_manifest_verification_targets",
        "Number of manifest verification targets by durable state",
        &["state"]
    )
    .unwrap()
});

const TARGET_STATES: [&str; 5] = [
    "waiting_registry",
    "pending",
    "leased",
    "complete",
    "exhausted",
];

pub(crate) fn spawn_consensus_gauge_updates(period: Duration, pool: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = update_consensus_gauges(&pool).await {
                error!(%error, "Failed to update consensus observability gauges");
            }
            tokio::time::sleep(period).await;
        }
    });
}

async fn update_consensus_gauges(pool: &PgPool) -> Result<(), sqlx::Error> {
    let totals = sqlx::query!(
        r#"
        SELECT
            (SELECT COUNT(*)::BIGINT
               FROM block_consensus
              WHERE block_content_digest IS NULL
                 OR (manifest_required AND manifest_published = FALSE)) AS "pending_work!",
            (SELECT COUNT(*)::BIGINT
               FROM block_consensus_drift_handle
              WHERE status = 'unresolved') AS "unresolved_drift!",
            COALESCE((
                SELECT EXTRACT(EPOCH FROM MAX(manifest_published_at))::BIGINT
                  FROM block_consensus
                 WHERE manifest_published
            ), 0) AS "latest_publication!",
            COALESCE((
                SELECT GREATEST(0, EXTRACT(EPOCH FROM NOW() - MIN(next_attempt_at))::BIGINT)
                  FROM block_consensus_verification_target
                 WHERE next_attempt_at <= NOW()
                   AND state IN ('waiting_registry', 'pending', 'leased')
            ), 0) AS "oldest_due_age!"
        "#,
    )
    .fetch_one(pool)
    .await?;
    PENDING_MANIFEST_WORK.set(totals.pending_work);
    UNRESOLVED_DRIFT_HANDLES.set(totals.unresolved_drift);
    LATEST_MANIFEST_PUBLICATION_UNIX_SECONDS.set(totals.latest_publication);
    OLDEST_DUE_VERIFICATION_AGE_SECONDS.set(totals.oldest_due_age);

    for state in TARGET_STATES {
        VERIFICATION_TARGETS.with_label_values(&[state]).set(0);
    }
    for row in sqlx::query!(
        "SELECT state, COUNT(*)::BIGINT AS \"count!\" FROM block_consensus_verification_target GROUP BY state"
    )
    .fetch_all(pool)
    .await?
    {
        VERIFICATION_TARGETS
            .with_label_values(&[&row.state])
            .set(row.count);
    }
    Ok(())
}
