use sqlx::Error;

use super::tfhe_event_propagate::Database;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatchupCursor {
    pub next_block_number: u64,
    pub generation: i64,
}

impl Database {
    /// Persist the first uncommitted catchup height before fetching a page.
    /// Explicit replays and reconnect overlap may rewind; neither can skip an
    /// existing gap, including one left by an earlier process.
    pub async fn initialize_catchup_cursor(
        &self,
        listener_id: &str,
        requested_start: u64,
    ) -> Result<CatchupCursor, Error> {
        let requested_start = i64::try_from(requested_start)
            .map_err(|err| Error::Protocol(err.to_string()))?;
        let (next, generation): (i64, i64) = sqlx::query_as(
            r#"
            INSERT INTO host_listener_catchup_state
                (chain_id, listener_id, next_block_number)
            VALUES ($1, $2, $3)
            ON CONFLICT (chain_id, listener_id) DO UPDATE
            SET generation = CASE
                    WHEN EXCLUDED.next_block_number < host_listener_catchup_state.next_block_number
                    THEN host_listener_catchup_state.generation + 1
                    ELSE host_listener_catchup_state.generation
                END,
                next_block_number = LEAST(
                host_listener_catchup_state.next_block_number,
                EXCLUDED.next_block_number
            )
            RETURNING next_block_number, generation
            "#,
        )
        .bind(self.chain_id.as_i64())
        .bind(listener_id)
        .bind(requested_start)
        .fetch_one(&self.pool().await)
        .await?;
        Ok(CatchupCursor {
            next_block_number: next as u64,
            generation,
        })
    }

    /// Advance only across a fully committed, contiguous page. Another
    /// instance may have rewound the cursor while this page was processing;
    /// in that case preserve its earlier gap and return it to the caller.
    pub async fn finish_catchup_page(
        &self,
        listener_id: &str,
        page_start: u64,
        next_page: u64,
        generation: i64,
    ) -> Result<CatchupCursor, Error> {
        let page_start = i64::try_from(page_start)
            .map_err(|err| Error::Protocol(err.to_string()))?;
        let next_page = i64::try_from(next_page)
            .map_err(|err| Error::Protocol(err.to_string()))?;
        let (next, generation): (i64, i64) = sqlx::query_as(
            r#"
            UPDATE host_listener_catchup_state
            SET next_block_number = CASE
                WHEN generation = $5 AND next_block_number >= $3 AND next_block_number < $4
                THEN $4 ELSE next_block_number
            END
            WHERE chain_id = $1 AND listener_id = $2
            RETURNING next_block_number, generation
            "#,
        )
        .bind(self.chain_id.as_i64())
        .bind(listener_id)
        .bind(page_start)
        .bind(next_page)
        .bind(generation)
        .fetch_one(&self.pool().await)
        .await?;
        Ok(CatchupCursor {
            next_block_number: next as u64,
            generation,
        })
    }
}
