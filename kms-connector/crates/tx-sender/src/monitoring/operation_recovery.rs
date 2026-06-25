use crate::monitoring::{DECRYPTION_TABLES, OPERATION_TABLES};
use sqlx::{Pool, Postgres, postgres::types::PgInterval};
use std::time::Duration;
use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// Spawns the background routine responsible for recovering stuck operations.
///
/// Operations are never deleted from the database. This routine only updates their status:
///
/// * operations stuck in the `under_process` status are unlocked back to `pending` so they can be
///   retried.
/// * decryptions too old to ever be processed successfully are marked as `failed` to stop retrying
///   them indefinitely. Only decryptions are allowed to expire this way.
///
/// # Arguments
///
/// * `period` - Duration to wait between two runs of the routine.
/// * `decryption_expiry` - Age after which `pending`/`under_process` decryption requests and
///   responses are marked as `failed` (computed from their `created_at`).
/// * `operation_under_process_timeout` - Age after which operations stuck in the `under_process`
///   status are unlocked back to `pending` (computed from their `updated_at`).
/// * `db_pool` - Connection pool to the database.
/// * `cancel_token` - Token used to stop the routine gracefully.
pub fn spawn_operation_recovery_routine(
    period: Duration,
    decryption_expiry: PgInterval,
    operation_under_process_timeout: PgInterval,
    db_pool: Pool<Postgres>,
    cancel_token: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        select! {
            _ = run_operation_recovery_routine(period, decryption_expiry, operation_under_process_timeout, db_pool) => {}
            _ = cancel_token.cancelled() => {}
        }
    })
}

async fn run_operation_recovery_routine(
    period: Duration,
    decryption_expiry: PgInterval,
    operation_under_process_timeout: PgInterval,
    db_pool: Pool<Postgres>,
) {
    loop {
        fail_expired_decryptions(&db_pool, &decryption_expiry).await;
        unlock_stuck_operations(&db_pool, &operation_under_process_timeout).await;
        tokio::time::sleep(period).await;
    }
}

/// Marks `pending`/`under_process` decryptions older than `expiry` (by `created_at`) as `failed`.
async fn fail_expired_decryptions(db_pool: &Pool<Postgres>, expiry: &PgInterval) {
    for table in DECRYPTION_TABLES {
        let query = format!(
            "UPDATE {table} SET status = 'failed' \
             WHERE status IN ('pending', 'under_process') AND NOW() - created_at > $1"
        );
        match sqlx::query(&query).bind(*expiry).execute(db_pool).await {
            Ok(result) => info!(
                "Marked {} expired rows in {table} as failed",
                result.rows_affected()
            ),
            Err(e) => error!("Failed to mark expired rows in {table} as failed: {e}"),
        }
    }
}

/// Unlocks operations stuck in `under_process` for longer than `operation_under_process_timeout`
/// (by `updated_at`) back to `pending` so they can be retried.
async fn unlock_stuck_operations(
    db_pool: &Pool<Postgres>,
    operation_under_process_timeout: &PgInterval,
) {
    for table in OPERATION_TABLES {
        let query = format!(
            "UPDATE {table} SET status = 'pending' \
             WHERE status = 'under_process' AND NOW() - updated_at > $1"
        );
        match sqlx::query(&query)
            .bind(*operation_under_process_timeout)
            .execute(db_pool)
            .await
        {
            Ok(result) => info!("Unlocked {} stuck rows in {table}", result.rows_affected()),
            Err(e) => error!("Failed to unlock stuck rows in {table}: {e}"),
        }
    }
}
