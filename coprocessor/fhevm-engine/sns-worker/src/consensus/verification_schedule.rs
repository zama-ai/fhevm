use std::time::Duration;

use alloy_primitives::{B256, U256};
use sqlx::{Postgres, Transaction};

use crate::ExecutionError;

use super::{
    manifest_archive::AuthenticatedManifest, peer_downloader::bind_target_to_current_registry,
};

pub(super) async fn schedule_manifest_verification(
    trx: &mut Transaction<'_, Postgres>,
    local: &AuthenticatedManifest,
    verification_delay: Duration,
    retry_delay: Duration,
    retry_count: u32,
) -> Result<i64, ExecutionError> {
    let payload = &local.signed.payload;
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;
    let publication_block_number = i64_from_u256(
        "manifest publication block number",
        payload.publication_block_number,
    )?;
    let revision = i64::try_from(payload.revision)
        .map_err(|_| internal("manifest revision exceeds BIGINT"))?;
    let delay_micros = duration_micros("verification delay", verification_delay)?;
    let retry_delay_micros = duration_micros("verification retry delay", retry_delay)?;
    let max_attempts = retry_count
        .checked_add(1)
        .and_then(|attempts| i32::try_from(attempts).ok())
        .ok_or_else(|| internal("verification retry count exceeds INTEGER"))?;
    let context = payload.coprocessor_context_id.to_be_bytes::<32>();

    let inserted = sqlx::query!(
        r#"
        INSERT INTO block_consensus_verification_target (
            local_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision,
            local_manifest_digest,
            eligible_at,
            next_attempt_at,
            retry_delay_micros,
            max_attempts
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8,
            NOW() + $9::BIGINT * INTERVAL '1 microsecond',
            NOW() + $9::BIGINT * INTERVAL '1 microsecond',
            $10, $11
        )
        ON CONFLICT (
            local_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision
        ) DO NOTHING
        RETURNING id
        "#,
        payload.publisher.as_slice(),
        i16::from(u8::from(payload.version)),
        context.as_slice(),
        host_chain_id,
        publication_block_number,
        payload.publication_block_hash.as_slice(),
        revision,
        local.digest.as_slice(),
        delay_micros,
        retry_delay_micros,
        max_attempts,
    )
    .fetch_optional(trx.as_mut())
    .await?;

    let target_id = if let Some(row) = inserted {
        row.id
    } else {
        let row = sqlx::query!(
            r#"
            SELECT id, local_manifest_digest
              FROM block_consensus_verification_target
             WHERE local_publisher = $1
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND publication_block_number = $5
               AND publication_block_hash = $6
               AND revision = $7
            "#,
            payload.publisher.as_slice(),
            i16::from(u8::from(payload.version)),
            context.as_slice(),
            host_chain_id,
            publication_block_number,
            payload.publication_block_hash.as_slice(),
            revision,
        )
        .fetch_one(trx.as_mut())
        .await?;
        let stored_digest = b256(
            "stored verification target manifest digest",
            &row.local_manifest_digest,
        )?;
        if stored_digest != local.digest {
            return Err(internal(format!(
                "verification target for publisher {} revision {} has conflicting digest",
                payload.publisher, payload.revision,
            )));
        }
        row.id
    };

    bind_target_to_current_registry(trx, target_id).await?;
    Ok(target_id)
}

fn duration_micros(field: &str, duration: Duration) -> Result<i64, ExecutionError> {
    i64::try_from(duration.as_micros())
        .map_err(|_| internal(format!("{field} exceeds BIGINT microseconds")))
}

fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn b256(field: &str, value: &[u8]) -> Result<B256, ExecutionError> {
    let value: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(B256::from(value))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
