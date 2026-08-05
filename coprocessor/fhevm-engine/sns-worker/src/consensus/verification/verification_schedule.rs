use std::time::Duration;

use sqlx::{Postgres, Transaction};

use crate::ExecutionError;

use super::peer_downloader::bind_task_to_current_registry;

pub(super) async fn schedule_manifest_verification(
    trx: &mut Transaction<'_, Postgres>,
    local_manifest_id: i64,
    verification_delay: Duration,
    retry_delay: Duration,
    retry_count: u32,
) -> Result<i64, ExecutionError> {
    let delay_micros = duration_micros("verification delay", verification_delay)?;
    let retry_delay_micros = duration_micros("verification retry delay", retry_delay)?;
    let max_attempts = retry_count
        .checked_add(1)
        .and_then(|attempts| i32::try_from(attempts).ok())
        .ok_or_else(|| internal("verification retry count exceeds INTEGER"))?;
    let inserted = sqlx::query!(
        r#"
        INSERT INTO block_manifest_verification_task (
            local_manifest_id,
            eligible_at,
            next_attempt_at,
            retry_delay_micros,
            max_attempts
        )
        VALUES (
            $1,
            NOW() + $2::BIGINT * INTERVAL '1 microsecond',
            NOW() + $2::BIGINT * INTERVAL '1 microsecond',
            $3, $4
        )
        ON CONFLICT (local_manifest_id) DO NOTHING
        RETURNING id
        "#,
        local_manifest_id,
        delay_micros,
        retry_delay_micros,
        max_attempts,
    )
    .fetch_optional(trx.as_mut())
    .await?;

    let task_id = if let Some(row) = inserted {
        row.id
    } else {
        let row = sqlx::query!(
            r#"
            SELECT id
              FROM block_manifest_verification_task
             WHERE local_manifest_id = $1
            "#,
            local_manifest_id,
        )
        .fetch_one(trx.as_mut())
        .await?;
        row.id
    };

    bind_task_to_current_registry(trx, task_id).await?;
    Ok(task_id)
}

fn duration_micros(field: &str, duration: Duration) -> Result<i64, ExecutionError> {
    i64::try_from(duration.as_micros())
        .map_err(|_| internal(format!("{field} exceeds BIGINT microseconds")))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
