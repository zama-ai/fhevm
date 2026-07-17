use crate::{ExecutionError, HandleItem};
use fhevm_engine_common::utils::to_hex;
use sqlx::{Postgres, Transaction};
use tracing::debug;

pub(crate) async fn ensure_block_consensus_row(
    trx: &mut Transaction<'_, Postgres>,
    task: &HandleItem,
    consensus_enabled: bool,
) -> Result<(), ExecutionError> {
    if !consensus_enabled {
        return Ok(());
    }

    if task.producer_block_hash.is_empty() {
        return Ok(());
    }

    let block_number = task.block_number.ok_or_else(|| {
        ExecutionError::InternalError(format!(
            "cannot create block consensus row without block number for producer {}",
            to_hex(&task.producer_block_hash),
        ))
    })?;

    let row = sqlx::query!(
        r#"
        WITH source AS (
            SELECT b.chain_id,
                   b.block_number,
                   b.block_hash,
                   b.parent_hash
              FROM host_chain_blocks_valid b
             WHERE b.chain_id = $1
               AND b.block_number = $2
               AND b.block_hash = $3
               AND b.parent_hash IS NOT NULL
               AND OCTET_LENGTH(b.parent_hash) = 32
               AND b.block_status <> 'orphaned'
        ),
        inserted AS (
            INSERT INTO block_consensus (
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash
            )
            SELECT chain_id,
                   block_number,
                   block_hash,
                   parent_hash
              FROM source
            ON CONFLICT (host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT EXISTS(SELECT 1 FROM source) AS "source_exists!",
               EXISTS(SELECT 1 FROM inserted) AS "inserted!"
        "#,
        task.host_chain_id.as_i64(),
        block_number,
        &task.producer_block_hash,
    )
    .fetch_one(trx.as_mut())
    .await?;

    if !row.source_exists {
        return Err(ExecutionError::InternalError(format!(
            "cannot create block consensus row without live parent metadata for chain {} block {} producer {}",
            task.host_chain_id.as_i64(),
            block_number,
            to_hex(&task.producer_block_hash),
        )));
    }

    if row.inserted {
        debug!(
            host_chain_id = task.host_chain_id.as_i64(),
            block_number,
            block_hash = %to_hex(&task.producer_block_hash),
            "Created block consensus row"
        );
    }

    Ok(())
}
