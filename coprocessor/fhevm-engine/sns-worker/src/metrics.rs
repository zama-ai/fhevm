use std::sync::{LazyLock, OnceLock};

use fhevm_engine_common::telemetry::{register_histogram, MetricsConfig};
use prometheus::{register_int_counter, register_int_counter_vec, IntCounter, IntCounterVec};
use prometheus::{register_int_gauge, Histogram, IntGauge};
use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{error, info};

pub static SNS_LATENCY_OP_HISTOGRAM_CONF: OnceLock<MetricsConfig> = OnceLock::new();
pub(crate) static SNS_LATENCY_OP_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    register_histogram(
        SNS_LATENCY_OP_HISTOGRAM_CONF.get(),
        "coprocessor_sns_op_latency_seconds",
        "Squash_noise computation latencies in seconds",
    )
});

pub(crate) static TASK_EXECUTE_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_worker_task_execute_success_counter",
        "Number of successful task execute operations in sns-worker (including persistence to DB)"
    )
    .unwrap()
});

pub(crate) static TASK_EXECUTE_FAILURE_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_worker_task_execute_failure_counter",
        "Number of failed task execute operations in sns-worker (including persistence to DB)"
    )
    .unwrap()
});

pub(crate) static AWS_UPLOAD_SUCCESS_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_worker_aws_upload_success_counter",
        "Number of successful AWS uploads in sns-worker"
    )
    .unwrap()
});

pub(crate) static AWS_UPLOAD_FAILURE_COUNTER: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_sns_worker_aws_upload_failure_counter",
        "Number of failed AWS uploads in sns-worker"
    )
    .unwrap()
});

pub(crate) static S3_MIGRATION_SUCCESS_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_sns_worker_s3_migration_success_counter",
        "Number of ciphertext handles successfully migrated to the current S3 format",
        &["host_chain_id"]
    )
    .unwrap()
});

pub(crate) static S3_MIGRATION_INTERMEDIATE_ERROR_COUNTER: LazyLock<IntCounterVec> =
    LazyLock::new(|| {
        register_int_counter_vec!(
            "coprocessor_sns_worker_s3_migration_intermediate_error_counter",
            "Number of non-terminal S3 migration failures that remain eligible for retry",
            &["host_chain_id", "error_type"]
        )
        .unwrap()
    });

pub(crate) static S3_MIGRATION_FINAL_ERROR_COUNTER: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_sns_worker_s3_migration_final_error_counter",
        "Number of S3 migration failures that reached the configured retry limit",
        &["host_chain_id", "error_type"]
    )
    .unwrap()
});

pub(crate) static UNCOMPLETE_TASKS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_worker_uncomplete_tasks_gauge",
        "Number of uncomplete tasks in sns-worker"
    )
    .unwrap()
});

pub(crate) static UNCOMPLETE_AWS_UPLOADS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_sns_worker_uncomplete_aws_uploads_gauge",
        "Number of uncomplete AWS uploads in sns-worker"
    )
    .unwrap()
});

pub fn spawn_gauge_update_routine(period: std::time::Duration, db_pool: PgPool) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match sqlx::query_scalar!(
                "SELECT COUNT(*)::BIGINT FROM pbs_computations WHERE is_completed = FALSE",
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(count) => {
                    let count = count.unwrap_or(0);
                    info!(uncomplete_tasks = %count, "Fetched uncomplete tasks count");
                    UNCOMPLETE_TASKS.set(count);
                }
                Err(e) => {
                    error!(error = %e, "Failed to fetch uncomplete tasks count");
                }
            }

            match sqlx::query_scalar!(
                "
                SELECT COUNT(*)::BIGINT
                FROM ciphertext_digest d
                WHERE d.ciphertext IS NULL
                   OR (
                     d.ciphertext128 IS NULL
                     AND EXISTS (
                       SELECT 1
                       FROM ciphertexts128 c
                       WHERE c.handle = d.handle
                         AND c.ciphertext IS NOT NULL
                     )
                   )
                ",
            )
            .fetch_one(&db_pool)
            .await
            {
                Ok(count) => {
                    let count = count.unwrap_or(0);
                    info!(uncomplete_aws_uploads = %count, "Fetched uncomplete AWS uploads count");
                    UNCOMPLETE_AWS_UPLOADS.set(count);
                }
                Err(e) => {
                    error!(error = %e, "Failed to fetch uncomplete AWS uploads count");
                }
            }

            sleep(period).await;
        }
    })
}
