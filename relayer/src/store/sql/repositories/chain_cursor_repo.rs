use std::time::Instant;

use crate::{
    metrics,
    store::sql::{
        client::PgClient,
        error::{SqlError, SqlResult},
    },
};

/// The relayer's position on the gateway chain: every event up to this block has been
/// handled. One row, so no listener owns it and no listener inherits another's.
pub struct ChainCursorRepository {
    pool: PgClient,
}

impl ChainCursorRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// The block a listener resumes from, or `None` before the first one is recorded.
    pub async fn get(&self) -> SqlResult<Option<u64>> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            SELECT last_block_number
            FROM gateway_chain_cursor
            WHERE id
            "#
        )
        .fetch_optional(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::GatewayChainCursor, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::GatewayChainCursor),
        }

        Ok(result
            .map_err(SqlError::from)?
            .map(|row| row.last_block_number as u64))
    }

    /// Record `block_number` as handled, and report whether the row moved. The statement
    /// refuses a lower block, so a listener that completes a range late - in this process or
    /// another - cannot pull the position backwards.
    pub async fn advance(&self, block_number: u64) -> SqlResult<bool> {
        let mut conn = self.pool.get_app_connection().await?;

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            INSERT INTO gateway_chain_cursor (id, last_block_number)
            VALUES (TRUE, $1)
            ON CONFLICT (id) DO UPDATE
                SET last_block_number = EXCLUDED.last_block_number
                WHERE gateway_chain_cursor.last_block_number < EXCLUDED.last_block_number
            "#,
            block_number as i64
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::GatewayChainCursor, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::GatewayChainCursor),
        }
        Ok(result.map_err(SqlError::from)?.rows_affected() > 0)
    }
}
