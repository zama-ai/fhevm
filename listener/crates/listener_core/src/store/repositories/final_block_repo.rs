use alloy::primitives::B256;
use std::sync::Arc;
use uuid::Uuid;

use crate::store::client::PgClient;
use crate::store::error::SqlResult;
use crate::store::models::{FinalBlock, NewFinalBlock};

/// Repository for the `final_blocks` tip table.
///
/// Finalized blocks never reorg, so there is no canonical/uncle bookkeeping:
/// plain inserts, and exactly one final block per (chain_id, block_number)
/// (enforced by `idx_final_blocks_unique_chain_number`).
#[derive(Clone)]
pub struct FinalBlockRepository {
    client: Arc<PgClient>,
    chain_id: i64,
}

impl FinalBlockRepository {
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self { client, chain_id }
    }

    /// Insert a new final block into the database.
    pub async fn insert_block(&self, block: &NewFinalBlock) -> SqlResult<FinalBlock> {
        let mut conn = self.client.get_app_connection().await?;
        let id = Uuid::new_v4();

        let row = sqlx::query!(
            r#"
            INSERT INTO final_blocks (id, chain_id, block_number, block_hash, parent_hash)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, chain_id, block_number, block_hash, parent_hash, created_at
            "#,
            id,
            self.chain_id,
            block.block_number as i64,
            block.block_hash.as_slice(),
            block.parent_hash.as_slice()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(FinalBlock {
            id: row.id,
            chain_id: row.chain_id,
            block_number: row.block_number as u64,
            block_hash: B256::from_slice(&row.block_hash),
            parent_hash: B256::from_slice(&row.parent_hash),
            created_at: row.created_at,
        })
    }

    /// Get the latest final block (highest block_number for this chain).
    /// Returns None if no final block exists in the database.
    pub async fn get_latest_final_block(&self) -> SqlResult<Option<FinalBlock>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT id, chain_id, block_number, block_hash, parent_hash, created_at
            FROM final_blocks
            WHERE chain_id = $1
            ORDER BY block_number DESC
            LIMIT 1
            "#,
            self.chain_id
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row.map(|r| FinalBlock {
            id: r.id,
            chain_id: r.chain_id,
            block_number: r.block_number as u64,
            block_hash: B256::from_slice(&r.block_hash),
            parent_hash: B256::from_slice(&r.parent_hash),
            created_at: r.created_at,
        }))
    }

    /// Get the lowest final block number stored for this chain.
    ///
    /// Returns `None` if no final blocks exist.
    pub async fn get_min_block_number(&self) -> SqlResult<Option<i64>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT MIN(block_number) as "min_block_number: i64"
            FROM final_blocks
            WHERE chain_id = $1
            "#,
            self.chain_id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(row.min_block_number)
    }

    /// Keep only the N most recent final blocks, delete the rest.
    ///
    /// # Returns
    /// Number of final blocks deleted
    pub async fn delete_blocks_keeping_latest(&self, keep_count: i64) -> SqlResult<u64> {
        let mut conn = self.client.get_app_connection().await?;

        let result = sqlx::query!(
            r#"
            WITH ranked_blocks AS (
                SELECT id,
                       ROW_NUMBER() OVER (
                           ORDER BY block_number DESC, created_at DESC
                       ) as rn
                FROM final_blocks
                WHERE chain_id = $1
            )
            DELETE FROM final_blocks
            WHERE chain_id = $1 AND id IN (SELECT id FROM ranked_blocks WHERE rn > $2)
            "#,
            self.chain_id,
            keep_count
        )
        .execute(&mut *conn)
        .await?;

        Ok(result.rows_affected())
    }
}
