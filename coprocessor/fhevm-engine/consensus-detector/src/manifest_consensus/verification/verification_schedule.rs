use std::time::Duration;

use sqlx::{Postgres, Transaction};

use crate::manifest_consensus::ExecutionError;

use super::verification_queue::bind_task_to_current_registry;

pub(super) async fn schedule_manifest_verification(
    trx: &mut Transaction<'_, Postgres>,
    local_manifest_id: i64,
    verification_delay: Duration,
    retry_delay: Duration,
    retry_count: u32,
) -> Result<i64, ExecutionError> {
    let delay_micros = duration_micros("verification delay", verification_delay)?;
    let retry_delay_secs = duration_secs(retry_delay)?;
    let max_attempts = retry_count
        .checked_add(1)
        .and_then(|attempts| i32::try_from(attempts).ok())
        .ok_or_else(|| internal("verification retry count exceeds INTEGER"))?;
    let inserted = sqlx::query!(
        r#"
        INSERT INTO block_manifest_verification_task (
            consensus_epoch,
            local_manifest_id,
            eligible_at,
            next_attempt_at,
            retry_delay_secs,
            max_attempts
        )
        SELECT manifest.consensus_epoch,
            manifest.id,
            NOW() + $2::BIGINT * INTERVAL '1 microsecond',
            NOW() + $2::BIGINT * INTERVAL '1 microsecond',
            $3, $4
          FROM block_manifest manifest
         WHERE manifest.id = $1
        ON CONFLICT (local_manifest_id, consensus_epoch) DO NOTHING
        RETURNING id
        "#,
        local_manifest_id,
        delay_micros,
        retry_delay_secs,
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
               AND consensus_epoch = (SELECT consensus_epoch FROM block_manifest WHERE id = $1)
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

// Round up fractional seconds so a configured retry delay is never shortened.
fn duration_secs(duration: Duration) -> Result<i64, ExecutionError> {
    let seconds = duration
        .as_secs()
        .saturating_add(u64::from(duration.subsec_nanos() != 0));
    i64::try_from(seconds).map_err(|_| internal("verification retry delay exceeds BIGINT seconds"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_seconds_preserve_whole_delays_and_round_up_fractions() {
        assert_eq!(duration_secs(Duration::ZERO).unwrap(), 0);
        assert_eq!(duration_secs(Duration::from_secs(60)).unwrap(), 60);
        assert_eq!(duration_secs(Duration::from_millis(1500)).unwrap(), 2);
        assert_eq!(duration_secs(Duration::from_nanos(1)).unwrap(), 1);
        assert!(duration_secs(Duration::MAX).is_err());
    }
}
