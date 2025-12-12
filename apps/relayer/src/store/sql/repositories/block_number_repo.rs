use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
    models::gateway_block_number_model::GatewayBlockNumber,
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

    /// Get the last block info - returns None if no row exists (matches current behavior)
    pub async fn get_last_block_info(&self) -> SqlResult<Option<BlockInfo>> {
        let result = sqlx::query!(
            r#"
            SELECT last_block_number, last_block_hash, updated_at
            FROM gateway_block_number_store
            WHERE id = 1
            "#
        )
        .fetch_optional(&self.pool.get_pool())
        .await
        .map_err(SqlError::from)?;

        match result {
            Some(row) => Ok(Some(BlockInfo {
                block_number: row.last_block_number as u64,
                block_hash: row.last_block_hash,
                updated_at: row.updated_at,
            })),
            None => Ok(None),
        }
    }

    /// Update block info - fast UPDATE for normal operation (assumes row exists)
    pub async fn update_block_info(&self, block_number: u64, block_hash: String) -> SqlResult<()> {
        sqlx::query!(
            r#"
            UPDATE gateway_block_number_store
            SET last_block_number = $1,
                last_block_hash = $2,
                updated_at = NOW()
            WHERE id = 1
            "#,
            block_number as i64,
            block_hash
        )
        .execute(&self.pool.get_pool())
        .await
        .map_err(SqlError::from)?;

        Ok(())
    }

    /// Insert initial block info - for first-time setup (when going from None to first value)
    pub async fn insert_initial_block_info(
        &self,
        block_number: u64,
        block_hash: String,
    ) -> SqlResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO gateway_block_number_store (id, last_block_number, last_block_hash)
            VALUES (1, $1, $2)
            "#,
            block_number as i64,
            block_hash
        )
        .execute(&self.pool.get_pool())
        .await
        .map_err(SqlError::from)?;

        Ok(())
    }

    /// Returns full model with all fields
    pub async fn get_gateway_block_number(&self) -> SqlResult<Option<GatewayBlockNumber>> {
        let result = sqlx::query!(
            r#"
            SELECT id, last_block_number, last_block_hash, created_at, updated_at
            FROM gateway_block_number_store
            WHERE id = 1
            "#
        )
        .fetch_optional(&self.pool.get_pool())
        .await
        .map_err(SqlError::from)?;

        match result {
            Some(row) => Ok(Some(GatewayBlockNumber {
                id: row.id,
                last_block_number: row.last_block_number,
                last_block_hash: row.last_block_hash,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })),
            None => Ok(None),
        }
    }
}
