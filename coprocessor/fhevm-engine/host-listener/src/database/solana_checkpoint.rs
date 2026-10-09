//! The Solana listener's resume checkpoint: the last sealed block whose compute rows were
//! committed, written in that block's transaction. It only moves forward, so a replica that
//! replays a block another replica already applied leaves it in place.

use solana_host_follower::BlockCheckpoint;
use sqlx::Error as SqlxError;

use crate::database::tfhe_event_propagate::Transaction;

pub async fn store_checkpoint(
    tx: &mut Transaction<'_>,
    checkpoint: &BlockCheckpoint,
) -> Result<(), SqlxError> {
    let slot = i64::try_from(checkpoint.slot).map_err(|_| {
        SqlxError::Protocol("checkpoint slot exceeds PostgreSQL BIGINT".into())
    })?;
    sqlx::query!(
        r#"
        INSERT INTO solana_listener_checkpoint (singleton, slot, block_hash)
        VALUES (1, $1, $2)
        ON CONFLICT (singleton) DO UPDATE SET
            slot = EXCLUDED.slot,
            block_hash = EXCLUDED.block_hash,
            updated_at = NOW()
        WHERE solana_listener_checkpoint.slot < EXCLUDED.slot
        "#,
        slot,
        &checkpoint.block_hash[..],
    )
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn load_checkpoint(
    pool: &sqlx::PgPool,
) -> Result<Option<BlockCheckpoint>, SqlxError> {
    let row = sqlx::query!(
        "SELECT slot, block_hash FROM solana_listener_checkpoint WHERE singleton = 1"
    )
    .fetch_optional(pool)
    .await?;
    row.map(|row| {
        Ok(BlockCheckpoint {
            slot: u64::try_from(row.slot).map_err(|_| {
                SqlxError::Decode("checkpoint slot is negative".into())
            })?,
            block_hash: row.block_hash.as_slice().try_into().map_err(|_| {
                SqlxError::Decode(
                    "checkpoint block hash is not 32 bytes".into(),
                )
            })?,
        })
    })
    .transpose()
}
