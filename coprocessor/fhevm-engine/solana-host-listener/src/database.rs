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

    pub async fn mark_block_finalized(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        block_number: u64,
        block_hash: &[u8],
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
            VALUES ($1, $2, $3, 'finalized')
            ON CONFLICT (chain_id, block_hash) DO UPDATE
            SET block_status = 'finalized'
            "#,
        )
        .bind(self.host_chain_id as i64)
        .bind(block_hash)
        .bind(block_number as i64)
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }
}
