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
/// # Epoch fencing (build-order step 8)
///
/// [`Self::advance`] is monotonic (never moves the cursor backwards) but that alone does not
/// stop a stalled ex-holder from pushing it *forward* past events its successor has not
/// finished handling yet - the failover interleaving `public_decrypt_repo`'s module doc
/// describes applies here too, just against the resume point instead of a request row. So
/// `advance` also stamps and checks `owner_epoch`, exactly like a request-row write: `NULL`
/// (unclaimed, including every pre-migration cursor) or an epoch at or below this pod's current
/// one (`<=`, a fencing token, not an equality check - see the rationale in
/// `public_decrypt_repo`'s module doc) is required to move the row.
///
/// This does not gate event processing itself - every pod's listener still dispatches every
/// event it observes into its own orchestrator, unconditionally, since listeners are not
/// epoch-scoped (see `gateway/handled_events.rs`). It only decides who may record the
/// *resume point*, and a write this pod loses the fence on is indistinguishable from the
/// pre-existing `Ok(false)` case ("another listener already recorded a further block") that
/// callers already treat as benign - see [`Self::advance`]'s doc comment.
///
/// One real limitation this creates, unlike a request row: once some real epoch has stamped
/// the cursor, a `None`-epoch pod (one that has never acquired the lock, including every
/// non-holder at any replica count above one) can never advance it - `NULL <= x` is never true.
/// There is also no claim mechanism for the cursor the way `claim_incomplete_requests` exists
/// for request rows; `advance` is its only writer. So if the *current holder's own listener*
/// stalls - a chain RPC problem specific to that component, with the lock and its dedicated
/// connection otherwise healthy - the resume point simply freezes at whatever block it last
/// reached, for as long as that holder keeps the lock: no other pod's epoch is high enough to
/// take over, and the holder's own epoch has nothing newer to yield to. It only moves again
/// once that listener recovers, or once the lock actually changes hands to a pod minting a
/// genuinely higher epoch, whose listener can then advance the cursor immediately (no
/// staleness wait - see `Self::advance`'s doc comment). A resume point that stops moving costs
/// replay range on the next restart, not correctness - events already handled stay
/// deduplicated regardless of the cursor (see `gateway/handled_events.rs`).
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

    /// Record `block_number` as handled, and report whether the row moved. The statement
    /// refuses a lower block, so a listener that completes a range late - in this process or
    /// another - cannot pull the position backwards. It also refuses to move the row at all
    /// unless this pod's current epoch owns it (`NULL`, i.e. unclaimed, or a match) - see the
    /// epoch-fencing doc block on the struct. A refusal for either reason reads the same to the
    /// caller: `Ok(false)`, already handled as "someone else moved this forward".
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
