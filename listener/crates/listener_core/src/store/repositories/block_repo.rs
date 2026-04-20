use alloy::primitives::B256;
use sqlx::Acquire;
use std::sync::Arc;
use uuid::Uuid;

use crate::store::client::PgClient;
use crate::store::error::SqlResult;
use crate::store::models::{Block, BlockStatus, NewDatabaseBlock, UpsertResult};

#[derive(Clone)]
pub struct BlockRepository {
    client: Arc<PgClient>,
    chain_id: i64,
}

impl BlockRepository {
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self { client, chain_id }
    }

    /// Insert a new block into the database.
    pub async fn insert_block(&self, block: &NewDatabaseBlock) -> SqlResult<Block> {
        let mut conn = self.client.get_app_connection().await?;
        let id = Uuid::new_v4();

        let row = sqlx::query!(
            r#"
            INSERT INTO blocks (id, chain_id, block_number, block_hash, parent_hash, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, chain_id, block_number, block_hash, parent_hash, status as "status: BlockStatus", created_at
            "#,
            id,
            self.chain_id,
            block.block_number as i64,
            block.block_hash.as_slice(),
            block.parent_hash.as_slice(),
            block.status as BlockStatus
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(Block {
            id: row.id,
            chain_id: row.chain_id,
            block_number: row.block_number as u64,
            block_hash: B256::from_slice(&row.block_hash),
            parent_hash: B256::from_slice(&row.parent_hash),
            status: row.status,
            created_at: row.created_at,
        })
    }

    /// Upsert a block as CANONICAL and mark other blocks at the same height as UNCLE.
    ///
    /// # Returns
    /// - `UpsertResult::Inserted` - A new block was inserted
    /// - `UpsertResult::Updated` - An existing block was updated (was UNCLE, now CANONICAL)
    /// - `UpsertResult::NoOp` - Block was already CANONICAL or FINALIZED, no changes made
    pub async fn upsert_block_canonical(
        &self,
        block: &NewDatabaseBlock,
    ) -> SqlResult<UpsertResult> {
        let mut conn = self.client.get_app_connection().await?;
        let mut tx = conn.begin().await?;

        // Step 1: Check if block exists and its current status
        let existing = sqlx::query!(
            r#"
            SELECT status as "status: BlockStatus" FROM blocks WHERE chain_id = $1 AND block_hash = $2
            "#,
            self.chain_id,
            block.block_hash.as_slice()
        )
        .fetch_optional(&mut *tx)
        .await?;

        // Early return: no status change needed for CANONICAL or FINALIZED
        if let Some(ref row) = existing
            && (row.status == BlockStatus::Canonical || row.status == BlockStatus::Finalized)
        {
            tx.commit().await?;
            return Ok(UpsertResult::NoOp);
        }

        // Step 2: Mark other canonical blocks at same height as UNCLE (before INSERT to
        // avoid violating the partial unique index on (chain_id, block_number) WHERE status = 'CANONICAL')
        sqlx::query!(
            r#"
            UPDATE blocks SET status = 'UNCLE'::block_status
            WHERE chain_id = $1 AND block_number = $2 AND block_hash != $3 AND status = 'CANONICAL'::block_status
            "#,
            self.chain_id,
            block.block_number as i64,
            block.block_hash.as_slice()
        )
        .execute(&mut *tx)
        .await?;

        // Step 3: Insert or update the block as CANONICAL
        let result = match existing {
            None => {
                let id = Uuid::new_v4();
                sqlx::query!(
                    r#"
                    INSERT INTO blocks (id, chain_id, block_number, block_hash, parent_hash, status)
                    VALUES ($1, $2, $3, $4, $5, 'CANONICAL'::block_status)
                    "#,
                    id,
                    self.chain_id,
                    block.block_number as i64,
                    block.block_hash.as_slice(),
                    block.parent_hash.as_slice()
                )
                .execute(&mut *tx)
                .await?;

                UpsertResult::Inserted
            }
            Some(_) => {
                sqlx::query!(
                    r#"
                    UPDATE blocks SET status = 'CANONICAL'::block_status WHERE chain_id = $1 AND block_hash = $2
                    "#,
                    self.chain_id,
                    block.block_hash.as_slice()
                )
                .execute(&mut *tx)
                .await?;

                UpsertResult::Updated
            }
        };

        tx.commit().await?;
        Ok(result)
    }

    /// Atomically upsert a batch of blocks as CANONICAL within a single DB transaction.
    ///
    /// Mirrors [`upsert_block_canonical`] semantics exactly, per block:
    /// 1. If the block already exists as CANONICAL or FINALIZED → `NoOp` (FINALIZED is never demoted).
    /// 2. Demote any other CANONICAL block at the same height to UNCLE.
    /// 3. Insert the new block or promote an existing UNCLE to CANONICAL.
    ///
    /// All-or-nothing: if any SQL operation fails mid-batch, the entire transaction
    /// is rolled back and no DB state has changed.
    ///
    /// # Arguments
    /// * `blocks` - Slice of blocks to upsert. Order matters only for logging;
    ///   ascending height order is conventional.
    ///
    /// # Returns
    /// A `Vec<UpsertResult>` parallel to the input, one entry per block.
    pub async fn batch_upsert_blocks_canonical(
        &self,
        blocks: &[NewDatabaseBlock],
    ) -> SqlResult<Vec<UpsertResult>> {
        if blocks.is_empty() {
            return Ok(vec![]);
        }

        let mut conn = self.client.get_app_connection().await?;
        let mut tx = conn.begin().await?;
        let mut results = Vec::with_capacity(blocks.len());

        for block in blocks {
            // Step 1: Check if block exists and its current status
            // NOTE: SQL string whitespace must match upsert_block_canonical exactly
            // for sqlx offline cache to reuse the same query hash.
            let existing = sqlx::query!(
            r#"
            SELECT status as "status: BlockStatus" FROM blocks WHERE chain_id = $1 AND block_hash = $2
            "#,
                self.chain_id,
                block.block_hash.as_slice()
            )
            .fetch_optional(&mut *tx)
            .await?;

            // Early continue: no status change needed for CANONICAL or FINALIZED
            // (mirrors upsert_block_canonical lines 77-83 exactly)
            if let Some(ref row) = existing
                && (row.status == BlockStatus::Canonical || row.status == BlockStatus::Finalized)
            {
                results.push(UpsertResult::NoOp);
                continue;
            }

            // Step 2: Mark other canonical blocks at same height as UNCLE
            sqlx::query!(
            r#"
            UPDATE blocks SET status = 'UNCLE'::block_status
            WHERE chain_id = $1 AND block_number = $2 AND block_hash != $3 AND status = 'CANONICAL'::block_status
            "#,
                self.chain_id,
                block.block_number as i64,
                block.block_hash.as_slice()
            )
            .execute(&mut *tx)
            .await?;

            // Step 3: Insert or update the block as CANONICAL
            match existing {
                None => {
                    let id = Uuid::new_v4();
                    sqlx::query!(
                        r#"
                    INSERT INTO blocks (id, chain_id, block_number, block_hash, parent_hash, status)
                    VALUES ($1, $2, $3, $4, $5, 'CANONICAL'::block_status)
                    "#,
                        id,
                        self.chain_id,
                        block.block_number as i64,
                        block.block_hash.as_slice(),
                        block.parent_hash.as_slice()
                    )
                    .execute(&mut *tx)
                    .await?;

                    results.push(UpsertResult::Inserted);
                }
                Some(_) => {
                    // Block exists as UNCLE → promote to CANONICAL
                    sqlx::query!(
                    r#"
                    UPDATE blocks SET status = 'CANONICAL'::block_status WHERE chain_id = $1 AND block_hash = $2
                    "#,
                        self.chain_id,
                        block.block_hash.as_slice()
                    )
                    .execute(&mut *tx)
                    .await?;

                    results.push(UpsertResult::Updated);
                }
            }
        }

        tx.commit().await?;
        Ok(results)
    }

    /// Get the canonical block at a specific block number.
    /// Returns None if no canonical block exists at that height.
    pub async fn get_canonical_block_by_number(
        &self,
        block_number: u64,
    ) -> SqlResult<Option<Block>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT id, chain_id, block_number, block_hash, parent_hash, status as "status: BlockStatus", created_at
            FROM blocks
            WHERE chain_id = $1 AND block_number = $2 AND status = 'CANONICAL'::block_status
            "#,
            self.chain_id,
            block_number as i64
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row.map(|r| Block {
            id: r.id,
            chain_id: r.chain_id,
            block_number: r.block_number as u64,
            block_hash: B256::from_slice(&r.block_hash),
            parent_hash: B256::from_slice(&r.parent_hash),
            status: r.status,
            created_at: r.created_at,
        }))
    }

    /// Delete blocks older than the specified number of seconds (finality cleanup).
    ///
    /// # Returns
    /// Number of blocks deleted
    pub async fn delete_blocks_before_timestamp(&self, seconds: i64) -> SqlResult<u64> {
        let mut conn = self.client.get_app_connection().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM blocks
            WHERE chain_id = $1 AND created_at < NOW() - make_interval(secs => $2)
            "#,
            self.chain_id,
            seconds as f64
        )
        .execute(&mut *conn)
        .await?;

        Ok(result.rows_affected())
    }

    /// Keep only the N most recent blocks, delete the rest.
    ///
    /// # Returns
    /// Number of blocks deleted
    pub async fn delete_blocks_keeping_latest(&self, keep_count: i64) -> SqlResult<u64> {
        let mut conn = self.client.get_app_connection().await?;

        let result = sqlx::query!(
            r#"
            WITH ranked_blocks AS (
                SELECT id,
                       ROW_NUMBER() OVER (
                           ORDER BY block_number DESC, created_at DESC
                       ) as rn
                FROM blocks
                WHERE chain_id = $1
            )
            DELETE FROM blocks
            WHERE chain_id = $1 AND id IN (SELECT id FROM ranked_blocks WHERE rn > $2)
            "#,
            self.chain_id,
            keep_count
        )
        .execute(&mut *conn)
        .await?;

        Ok(result.rows_affected())
    }

    /// Get the lowest block number stored for this chain.
    ///
    /// Returns `None` if no blocks exist.
    pub async fn get_min_block_number(&self) -> SqlResult<Option<i64>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT MIN(block_number) as "min_block_number: i64"
            FROM blocks
            WHERE chain_id = $1
            "#,
            self.chain_id
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(row.min_block_number)
    }

    /// Get the latest canonical block (highest block_number with status CANONICAL).
    /// Returns None if no canonical block exists in the database.
    pub async fn get_latest_canonical_block(&self) -> SqlResult<Option<Block>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query!(
            r#"
            SELECT id, chain_id, block_number, block_hash, parent_hash, status as "status: BlockStatus", created_at
            FROM blocks
            WHERE chain_id = $1 AND status = 'CANONICAL'::block_status
            ORDER BY block_number DESC
            LIMIT 1
            "#,
            self.chain_id
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row.map(|r| Block {
            id: r.id,
            chain_id: r.chain_id,
            block_number: r.block_number as u64,
            block_hash: B256::from_slice(&r.block_hash),
            parent_hash: B256::from_slice(&r.parent_hash),
            status: r.status,
            created_at: r.created_at,
        }))
    }
}
