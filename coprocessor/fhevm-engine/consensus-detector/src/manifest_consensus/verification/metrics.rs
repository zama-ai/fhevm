use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
    time::Duration,
};

use prometheus::{register_int_counter_vec, register_int_gauge_vec, IntCounterVec, IntGaugeVec};
use sqlx::PgPool;
use tracing::error;

pub(crate) static PEER_MANIFEST_ARCHIVED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_peer_archived_total",
        "Number of peer manifest objects durably archived",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(crate) static PEER_MANIFEST_DOWNLOAD_FAILURE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_peer_download_failure_total",
        "Number of peer manifest listing, download, or authentication failures",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(crate) static VERIFICATION_OUTCOMES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_total",
        "Number of completed manifest verification attempts by outcome",
        &["consensus_epoch", "outcome"]
    )
    .unwrap()
});

pub(crate) static VERIFICATION_FAILURE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_failure_total",
        "Number of manifest verification runs aborted before producing an outcome",
        &["consensus_epoch"]
    )
    .unwrap()
});

pub(crate) static DRIFT_LOCALIZATION_INCOMPLETE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_drift_localization_incomplete_total",
        "Number of drift evaluations whose handle inventory could not be fully localized",
        &["consensus_epoch"]
    )
    .unwrap()
});

static UNRESOLVED_DRIFT_HANDLES: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_verification_drift_handles_unresolved",
        "Number of unresolved handle-level drift findings",
        &["consensus_epoch"]
    )
    .unwrap()
});

static OLDEST_DUE_VERIFICATION_AGE_SECONDS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_verification_oldest_due_age_seconds",
        "Age in seconds of the oldest overdue pending or expired-claim verification task",
        &["consensus_epoch"]
    )
    .unwrap()
});

static VERIFICATION_TARGETS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_verification_tasks",
        "Number of manifest verification tasks by durable state",
        &["consensus_epoch", "state"]
    )
    .unwrap()
});

const TARGET_STATES: [&str; 4] = ["pending", "claimed", "consensus", "retry_exhausted"];

static SEEN_CONSENSUS_EPOCHS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub(crate) fn spawn_verification_gauge_updates(period: Duration, pool: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = update_verification_gauges(&pool).await {
                error!(%error, "Failed to update manifest verification gauges");
            }
            tokio::time::sleep(period).await;
        }
    });
}

fn remove_consensus_epoch_gauges(consensus_epoch: &str) {
    let _ = UNRESOLVED_DRIFT_HANDLES.remove_label_values(&[consensus_epoch]);
    let _ = OLDEST_DUE_VERIFICATION_AGE_SECONDS.remove_label_values(&[consensus_epoch]);
    for state in TARGET_STATES {
        let _ = VERIFICATION_TARGETS.remove_label_values(&[consensus_epoch, state]);
    }
}

async fn update_verification_gauges(pool: &PgPool) -> Result<(), sqlx::Error> {
    let unresolved = sqlx::query!(
        r#"
        SELECT consensus_epoch,
               COUNT(*)::BIGINT AS "unresolved_drift!"
          FROM drifted_handle
         WHERE status = 'unresolved' AND healed_at IS NULL
         GROUP BY consensus_epoch
        "#,
    )
    .fetch_all(pool)
    .await?;
    let overdue = sqlx::query!(
        r#"
        SELECT consensus_epoch,
               GREATEST(
                   0,
                   EXTRACT(EPOCH FROM NOW() - MIN(
                       CASE state
                           WHEN 'pending' THEN next_attempt_at
                           WHEN 'claimed' THEN claim_expires_at
                       END
                   ))::BIGINT
               ) AS "oldest_due_age!"
          FROM block_manifest_verification_task
         WHERE (state = 'pending' AND next_attempt_at <= NOW())
            OR (state = 'claimed' AND claim_expires_at <= NOW())
         GROUP BY consensus_epoch
        "#,
    )
    .fetch_all(pool)
    .await?;
    let tasks = sqlx::query!(
        r#"
        SELECT consensus_epoch, state, COUNT(*)::BIGINT AS "count!"
          FROM block_manifest_verification_task
         GROUP BY consensus_epoch, state
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut live = HashSet::new();
    live.extend(unresolved.iter().map(|row| row.consensus_epoch.clone()));
    live.extend(overdue.iter().map(|row| row.consensus_epoch.clone()));
    live.extend(tasks.iter().map(|row| row.consensus_epoch.clone()));

    for consensus_epoch in &live {
        UNRESOLVED_DRIFT_HANDLES
            .with_label_values(&[consensus_epoch])
            .set(0);
        OLDEST_DUE_VERIFICATION_AGE_SECONDS
            .with_label_values(&[consensus_epoch])
            .set(0);
        for state in TARGET_STATES {
            VERIFICATION_TARGETS
                .with_label_values(&[consensus_epoch, state])
                .set(0);
        }
    }
    for row in &unresolved {
        UNRESOLVED_DRIFT_HANDLES
            .with_label_values(&[&row.consensus_epoch])
            .set(row.unresolved_drift);
    }
    for row in &overdue {
        OLDEST_DUE_VERIFICATION_AGE_SECONDS
            .with_label_values(&[&row.consensus_epoch])
            .set(row.oldest_due_age);
    }
    for row in &tasks {
        VERIFICATION_TARGETS
            .with_label_values(&[&row.consensus_epoch, &row.state])
            .set(row.count);
    }

    let mut seen = SEEN_CONSENSUS_EPOCHS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    for consensus_epoch in seen.difference(&live) {
        remove_consensus_epoch_gauges(consensus_epoch);
    }
    *seen = live;
    Ok(())
}

#[cfg(test)]
#[path = "metrics_tests.rs"]
mod tests;
