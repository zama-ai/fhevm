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
        &["generation"]
    )
    .unwrap()
});

pub(crate) static PEER_MANIFEST_DOWNLOAD_FAILURE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_peer_download_failure_total",
        "Number of peer manifest listing, download, or authentication failures",
        &["generation"]
    )
    .unwrap()
});

pub(crate) static VERIFICATION_OUTCOMES: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_total",
        "Number of completed manifest verification attempts by outcome",
        &["generation", "outcome"]
    )
    .unwrap()
});

pub(crate) static VERIFICATION_FAILURE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_failure_total",
        "Number of manifest verification runs aborted before producing an outcome",
        &["generation"]
    )
    .unwrap()
});

pub(crate) static DRIFT_LOCALIZATION_INCOMPLETE: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_manifest_verification_drift_localization_incomplete_total",
        "Number of drift evaluations whose handle inventory could not be fully localized",
        &["generation"]
    )
    .unwrap()
});

static UNRESOLVED_DRIFT_HANDLES: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_verification_drift_handles_unresolved",
        "Number of unresolved handle-level drift findings",
        &["generation"]
    )
    .unwrap()
});

static OLDEST_DUE_VERIFICATION_AGE_SECONDS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_verification_oldest_due_age_seconds",
        "Age in seconds of the oldest overdue pending or expired-claim verification task",
        &["generation"]
    )
    .unwrap()
});

static VERIFICATION_TARGETS: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "coprocessor_manifest_verification_tasks",
        "Number of manifest verification tasks by durable state",
        &["generation", "state"]
    )
    .unwrap()
});

const TARGET_STATES: [&str; 4] = ["pending", "claimed", "consensus", "retry_exhausted"];

static SEEN_GENERATIONS: LazyLock<Mutex<HashSet<String>>> =
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

fn remove_generation_gauges(generation: &str) {
    let _ = UNRESOLVED_DRIFT_HANDLES.remove_label_values(&[generation]);
    let _ = OLDEST_DUE_VERIFICATION_AGE_SECONDS.remove_label_values(&[generation]);
    for state in TARGET_STATES {
        let _ = VERIFICATION_TARGETS.remove_label_values(&[generation, state]);
    }
}

async fn update_verification_gauges(pool: &PgPool) -> Result<(), sqlx::Error> {
    let unresolved = sqlx::query!(
        r#"
        SELECT generation,
               COUNT(*)::BIGINT AS "unresolved_drift!"
          FROM drifted_handle
         WHERE status = 'unresolved' AND healed_at IS NULL
         GROUP BY generation
        "#,
    )
    .fetch_all(pool)
    .await?;
    let overdue = sqlx::query!(
        r#"
        SELECT generation,
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
         GROUP BY generation
        "#,
    )
    .fetch_all(pool)
    .await?;
    let tasks = sqlx::query!(
        r#"
        SELECT generation, state, COUNT(*)::BIGINT AS "count!"
          FROM block_manifest_verification_task
         GROUP BY generation, state
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut live = HashSet::new();
    live.extend(unresolved.iter().map(|row| row.generation.clone()));
    live.extend(overdue.iter().map(|row| row.generation.clone()));
    live.extend(tasks.iter().map(|row| row.generation.clone()));

    for generation in &live {
        UNRESOLVED_DRIFT_HANDLES
            .with_label_values(&[generation])
            .set(0);
        OLDEST_DUE_VERIFICATION_AGE_SECONDS
            .with_label_values(&[generation])
            .set(0);
        for state in TARGET_STATES {
            VERIFICATION_TARGETS
                .with_label_values(&[generation, state])
                .set(0);
        }
    }
    for row in &unresolved {
        UNRESOLVED_DRIFT_HANDLES
            .with_label_values(&[&row.generation])
            .set(row.unresolved_drift);
    }
    for row in &overdue {
        OLDEST_DUE_VERIFICATION_AGE_SECONDS
            .with_label_values(&[&row.generation])
            .set(row.oldest_due_age);
    }
    for row in &tasks {
        VERIFICATION_TARGETS
            .with_label_values(&[&row.generation, &row.state])
            .set(row.count);
    }

    let mut seen = SEEN_GENERATIONS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    for generation in seen.difference(&live) {
        remove_generation_gauges(generation);
    }
    *seen = live;
    Ok(())
}

#[cfg(test)]
#[path = "metrics_tests.rs"]
mod tests;
