use std::time::Instant;

use crate::{
    metrics,
    orchestrator::DispatcherLock,
    store::sql::{
        client::PgClient,
        error::{SqlError, SqlResult},
    },
};

/// The relayer's position on the gateway chain: every event up to this block has been
/// handled. One row, so no listener owns it and no listener inherits another's.
///
/// [`Self::advance`] is monotonic, but that alone lets a stalled ex-holder push the cursor
/// *forward* past events its successor has not finished, so it also stamps and checks
/// `owner_epoch` exactly like a request-row write: `NULL` or `<=` this pod's current epoch
/// (a fencing token - see `public_decrypt_repo`'s module doc). The listeners read the
/// dispatch gate at loop boundaries only, so the fence guards the residual window: a range
/// already in flight when the gate closed, completing and reaching for the cursor afterwards.
/// A fence refusal reads as the benign `Ok(false)` callers already treat as "another listener
/// recorded a further block".
///
/// Limitation: once a real epoch has stamped the cursor, a `None`-epoch pod can never advance
/// it, and `advance` is the cursor's only writer - if the current holder's own listener
/// stalls, the resume point freezes until the listener recovers or the lock changes hands.
/// That costs replay range on the next restart, not correctness: handled events stay
/// deduplicated (see `gateway/handled_events.rs`).
pub struct ChainCursorRepository {
    pool: PgClient,
    dispatcher_lock: DispatcherLock,
}

impl ChainCursorRepository {
    pub fn new(pool: PgClient, dispatcher_lock: DispatcherLock) -> Self {
        Self {
            pool,
            dispatcher_lock,
        }
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

    /// Record `block_number` as handled, and report whether the row moved. The statement refuses
    /// a lower block, so a listener that completes a range late - in this process or another -
    /// cannot pull the position backwards.
    /// It also refuses to move the row at all unless this pod's current epoch owns it (`NULL`,
    /// i.e. unclaimed, or a match) - see the epoch-fencing doc block on the struct. A refusal for
    /// either reason reads the same to the caller: `Ok(false)`, already handled as "someone else
    /// moved this forward".
    pub async fn advance(&self, block_number: u64) -> SqlResult<bool> {
        let mut conn = self.pool.get_app_connection().await?;
        let epoch = self.dispatcher_lock.current_epoch();

        let query_start = Instant::now();
        let result = sqlx::query!(
            r#"
            INSERT INTO gateway_chain_cursor (id, last_block_number, owner_epoch)
            VALUES (TRUE, $1, $2)
            ON CONFLICT (id) DO UPDATE
                SET last_block_number = EXCLUDED.last_block_number,
                    owner_epoch = EXCLUDED.owner_epoch
                WHERE gateway_chain_cursor.last_block_number < EXCLUDED.last_block_number
                  AND (gateway_chain_cursor.owner_epoch IS NULL OR gateway_chain_cursor.owner_epoch <= EXCLUDED.owner_epoch)
            "#,
            block_number as i64,
            epoch,
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
