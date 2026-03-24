use std::time::Instant;

use crate::{
    metrics,
    store::sql::{
        client::PgClient,
        error::{SqlError, SqlResult},
        models::gateway_block_number_model::GatewayBlockNumber,
    },
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub block_number: u64,
    pub block_hash: String,
    pub updated_at: DateTime<Utc>,
}

pub struct BlockNumberRepository {
    pool: PgClient,
}

impl BlockNumberRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// Get the last block info for a specific listener instance
    pub async fn get_last_block_info(&self, instance_id: usize) -> SqlResult<Option<BlockInfo>> {
        let mut conn = self.pool.get_app_connection().await?;
        let instance = instance_id as i32;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            SELECT last_block_number, last_block_hash, updated_at
            FROM gateway_block_number_store
            WHERE instance_id = $1
            "#,
            instance
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(
                metrics::Table::GatewayBlockNumberStore,
                query_start.elapsed(),
            ),
            Err(_) => metrics::increment_error(metrics::Table::GatewayBlockNumberStore),
        }

        let record = result.map_err(SqlError::from)?;

        match record {
            Some(row) => Ok(Some(BlockInfo {
                block_number: row.last_block_number as u64,
                block_hash: row.last_block_hash,
                updated_at: row.updated_at,
            })),
            None => Ok(None),
        }
    }

    /// Update block info for a specific listener instance
    pub async fn update_block_info(
        &self,
        block_number: u64,
        block_hash: String,
        instance_id: usize,
    ) -> SqlResult<()> {
        let mut conn = self.pool.get_app_connection().await?;
        let instance = instance_id as i32;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            UPDATE gateway_block_number_store
            SET last_block_number = $1,
                last_block_hash = $2,
                updated_at = NOW()
            WHERE instance_id = $3
            "#,
            block_number as i64,
            block_hash,
            instance
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(
                metrics::Table::GatewayBlockNumberStore,
                query_start.elapsed(),
            ),
            Err(_) => metrics::increment_error(metrics::Table::GatewayBlockNumberStore),
        }
        result.map_err(SqlError::from)?;

        Ok(())
    }

    /// Insert initial block info for a specific listener instance
    pub async fn insert_initial_block_info(
        &self,
        block_number: u64,
        block_hash: String,
        instance_id: usize,
    ) -> SqlResult<()> {
        let mut conn = self.pool.get_app_connection().await?;
        let instance = instance_id as i32;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            INSERT INTO gateway_block_number_store (instance_id, last_block_number, last_block_hash)
            VALUES ($1, $2, $3)
            ON CONFLICT (instance_id)
            DO UPDATE SET
                last_block_number = EXCLUDED.last_block_number,
                last_block_hash = EXCLUDED.last_block_hash,
                updated_at = NOW()
            "#,
            instance,
            block_number as i64,
            block_hash
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(
                metrics::Table::GatewayBlockNumberStore,
                query_start.elapsed(),
            ),
            Err(_) => metrics::increment_error(metrics::Table::GatewayBlockNumberStore),
        }
        result.map_err(SqlError::from)?;

        Ok(())
    }

    /// Returns full model with all fields for a specific listener instance
    pub async fn get_gateway_block_number(
        &self,
        instance_id: usize,
    ) -> SqlResult<Option<GatewayBlockNumber>> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            SELECT instance_id, last_block_number, last_block_hash, created_at, updated_at
            FROM gateway_block_number_store
            WHERE instance_id = $1
            "#,
            instance_id as i32
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(
                metrics::Table::GatewayBlockNumberStore,
                query_start.elapsed(),
            ),
            Err(_) => metrics::increment_error(metrics::Table::GatewayBlockNumberStore),
        }

        let record = result.map_err(SqlError::from)?;

        match record {
            Some(row) => Ok(Some(GatewayBlockNumber {
                instance_id: row.instance_id,
                last_block_number: row.last_block_number,
                last_block_hash: row.last_block_hash,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })),
            None => Ok(None),
        }
    }
}
