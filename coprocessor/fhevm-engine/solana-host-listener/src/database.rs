use anyhow::Result;
use fhevm_engine_common::types::AllowEvents;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use time::PrimitiveDateTime;

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
    host_chain_id: u64,
}

pub struct ComputationRow {
    pub output_handle: Vec<u8>,
    pub dependencies: Vec<Vec<u8>>,
    pub fhe_operation: i16,
    pub is_scalar: bool,
    pub transaction_id: Vec<u8>,
    pub dependence_chain_id: Vec<u8>,
    pub is_allowed: bool,
    pub schedule_order: PrimitiveDateTime,
    pub block_number: u64,
}

pub struct DelegationRow {
    pub delegator: Vec<u8>,
    pub delegate: Vec<u8>,
    pub contract_address: Vec<u8>,
    pub delegation_counter: u64,
    pub old_expiration_date: u64,
    pub new_expiration_date: u64,
    pub block_number: u64,
    pub block_hash: Vec<u8>,
    pub transaction_id: Vec<u8>,
}

impl Database {
    pub async fn new(database_url: &str, host_chain_id: u64) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(database_url)
            .await?;
        Ok(Self {
            pool,
            host_chain_id,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn begin(&self) -> Result<Transaction<'_, Postgres>> {
        Ok(self.pool.begin().await?)
    }

    pub async fn get_last_caught_up_block(&self) -> Result<Option<i64>> {
        Ok(sqlx::query_scalar(
            "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1",
        )
        .bind(self.host_chain_id as i64)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn set_last_caught_up_block(&self, block_number: u64) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block)
            VALUES ($1, $2)
            ON CONFLICT (chain_id) DO UPDATE
            SET last_caught_up_block = EXCLUDED.last_caught_up_block,
                updated_at = NOW()
            "#,
        )
        .bind(self.host_chain_id as i64)
        .bind(block_number as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_pending_blocks_to_finalize(
        &self,
        last_block_max: u64,
    ) -> Result<Vec<i64>> {
        let rows = sqlx::query_scalar(
            r#"
            SELECT block_number
            FROM host_chain_blocks_valid
            WHERE chain_id = $1
              AND block_status = 'pending'
              AND block_number <= $2
            ORDER BY block_number DESC
            LIMIT 128
            "#,
        )
        .bind(self.host_chain_id as i64)
        .bind(last_block_max as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn insert_computation(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        row: &ComputationRow,
    ) -> Result<bool> {
        let inserted = sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar,
                dependence_chain_id,
                transaction_id,
                is_allowed,
                created_at,
                schedule_order,
                is_completed,
                host_chain_id,
                block_number
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), $8, $9, $10, $11)
            ON CONFLICT (output_handle, transaction_id) DO NOTHING
            "#,
        )
        .bind(&row.output_handle)
        .bind(&row.dependencies)
        .bind(row.fhe_operation)
        .bind(row.is_scalar)
        .bind(&row.dependence_chain_id)
        .bind(&row.transaction_id)
        .bind(row.is_allowed)
        .bind(row.schedule_order)
        .bind(false)
        .bind(self.host_chain_id as i64)
        .bind(row.block_number as i64)
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            > 0;
        Ok(inserted)
    }

    pub async fn lookup_dependency_chain_ids(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        dependency_handles: &[Vec<u8>],
        current_dependence_chain_id: &[u8],
    ) -> Result<Vec<Vec<u8>>> {
        if dependency_handles.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_scalar::<_, Vec<u8>>(
            r#"
            SELECT DISTINCT c.dependence_chain_id
            FROM computations c
            JOIN allowed_handles ah
              ON ah.handle = c.output_handle
             AND ah.host_chain_id = c.host_chain_id
            JOIN dependence_chain dc
              ON dc.dependence_chain_id = c.dependence_chain_id
            WHERE c.host_chain_id = $1
              AND c.dependence_chain_id IS NOT NULL
              AND c.dependence_chain_id <> $2
              AND dc.status <> 'processed'
              AND ah.handle = ANY($3::bytea[])
            "#,
        )
        .bind(self.host_chain_id as i64)
        .bind(current_dependence_chain_id)
        .bind(dependency_handles)
        .fetch_all(tx.as_mut())
        .await?;

        Ok(rows)
    }

    pub async fn upsert_dependence_chain(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        dependence_chain_id: &[u8],
        last_updated_at: PrimitiveDateTime,
        dependency_count: usize,
        block_hash: &[u8],
        block_number: u64,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO dependence_chain(
                dependence_chain_id,
                status,
                last_updated_at,
                dependency_count,
                dependents,
                block_hash,
                block_height,
                block_timestamp,
                schedule_priority
            )
            VALUES ($1, 'updated', $2, $3, '{}'::bytea[], $4, $5, $2, 0)
            ON CONFLICT (dependence_chain_id) DO UPDATE
            SET status = 'updated',
                last_updated_at = CASE
                    WHEN dependence_chain.status = 'processed' THEN EXCLUDED.last_updated_at
                    ELSE LEAST(dependence_chain.last_updated_at, EXCLUDED.last_updated_at)
                END,
                dependency_count = EXCLUDED.dependency_count,
                block_hash = EXCLUDED.block_hash,
                block_height = EXCLUDED.block_height,
                block_timestamp = EXCLUDED.block_timestamp
            "#,
        )
        .bind(dependence_chain_id)
        .bind(last_updated_at)
        .bind(dependency_count as i32)
        .bind(block_hash)
        .bind(block_number as i64)
        .execute(tx.as_mut())
        .await?;

        Ok(())
    }

    pub async fn append_dependents(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        dependency_chain_ids: &[Vec<u8>],
        dependent_chain_id: &[u8],
    ) -> Result<()> {
        if dependency_chain_ids.is_empty() {
            return Ok(());
        }

        sqlx::query(
            r#"
            UPDATE dependence_chain
            SET dependents = (
                SELECT ARRAY(
                    SELECT DISTINCT dep
                    FROM unnest(dependence_chain.dependents || ARRAY[$2::bytea]) AS dep
                )
            )
            WHERE dependence_chain_id = ANY($1::bytea[])
            "#,
        )
        .bind(dependency_chain_ids)
        .bind(dependent_chain_id)
        .execute(tx.as_mut())
        .await?;

        Ok(())
    }

    pub async fn insert_allowed_handle(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        handle: &[u8],
        account_address: &str,
        event_type: AllowEvents,
        transaction_id: &[u8],
        block_number: u64,
    ) -> Result<bool> {
        let inserted = sqlx::query(
            r#"
            INSERT INTO allowed_handles(
                handle,
                account_address,
                event_type,
                transaction_id,
                host_chain_id,
                block_number
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(handle)
        .bind(account_address)
        .bind(event_type as i16)
        .bind(transaction_id)
        .bind(self.host_chain_id as i64)
        .bind(block_number as i64)
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            > 0;

        sqlx::query(
            r#"
            UPDATE computations
            SET is_allowed = TRUE
            WHERE output_handle = $1
              AND host_chain_id = $2
              AND is_completed = FALSE
              AND is_error = FALSE
            "#,
        )
        .bind(handle)
        .bind(self.host_chain_id as i64)
        .execute(tx.as_mut())
        .await?;

        Ok(inserted)
    }

    pub async fn insert_pbs_computation(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        handle: &[u8],
        transaction_id: &[u8],
        block_number: u64,
    ) -> Result<bool> {
        let inserted = sqlx::query(
            r#"
            INSERT INTO pbs_computations(handle, transaction_id, host_chain_id, block_number)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(handle)
        .bind(transaction_id)
        .bind(self.host_chain_id as i64)
        .bind(block_number as i64)
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            > 0;
        Ok(inserted)
    }

    pub async fn insert_delegation(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        row: &DelegationRow,
    ) -> Result<bool> {
        let inserted = sqlx::query(
            r#"
            INSERT INTO delegate_user_decrypt(
                delegator,
                delegate,
                contract_address,
                delegation_counter,
                old_expiration_date,
                new_expiration_date,
                host_chain_id,
                block_number,
                block_hash,
                transaction_id,
                on_gateway,
                reorg_out
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, FALSE, FALSE)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(&row.delegator)
        .bind(&row.delegate)
        .bind(&row.contract_address)
        .bind(row.delegation_counter as i64)
        .bind(row.old_expiration_date.to_string())
        .bind(row.new_expiration_date.to_string())
        .bind(self.host_chain_id as i64)
        .bind(row.block_number as i64)
        .bind(&row.block_hash)
        .bind(&row.transaction_id)
        .execute(tx.as_mut())
        .await?
        .rows_affected()
            > 0;
        Ok(inserted)
    }

    pub async fn update_block_as_finalized(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block_number: u64,
        block_hash: &[u8],
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE host_chain_blocks_valid
            SET block_status = CASE
                WHEN block_hash = $2
                    THEN 'finalized'
                    ELSE 'orphaned'
                END
            WHERE block_number = $3 AND chain_id = $1
            "#,
        )
        .bind(self.host_chain_id as i64)
        .bind(block_hash)
        .bind(block_number as i64)
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }

    pub async fn mark_block_as_valid(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block_number: u64,
        block_hash: &[u8],
        finalized: bool,
    ) -> Result<()> {
        let status = if finalized { "finalized" } else { "pending" };
        sqlx::query(
            r#"
            INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (chain_id, block_hash) DO NOTHING
            "#,
        )
        .bind(self.host_chain_id as i64)
        .bind(block_hash)
        .bind(block_number as i64)
        .bind(status)
        .execute(tx.as_mut())
        .await?;

        if finalized {
            self.update_block_as_finalized(tx, block_number, block_hash)
                .await?;
        }
        Ok(())
    }
}
