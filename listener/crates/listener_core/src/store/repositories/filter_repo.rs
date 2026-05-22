use std::sync::Arc;
use uuid::Uuid;

use crate::store::client::PgClient;
use crate::store::error::SqlResult;
use crate::store::models::Filter;

#[derive(Clone)]
pub struct FilterRepository {
    client: Arc<PgClient>,
    chain_id: i64,
}

impl FilterRepository {
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self { client, chain_id }
    }

    /// Insert a new filter.
    ///
    /// - If filter doesn't exist → inserts new row, returns `Some(Filter)`
    /// - If filter already exists (conflict) → no-op, returns `None`
    pub async fn add_filter(
        &self,
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
    ) -> SqlResult<Option<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let id = Uuid::new_v4();

        let row = sqlx::query_as!(
            Filter,
            r#"
            INSERT INTO filters (id, chain_id, consumer_id, "from", "to", "log_address")
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (chain_id, consumer_id, COALESCE("from", ''), COALESCE("to", ''), COALESCE("log_address", ''))
            DO NOTHING
            RETURNING id, chain_id, consumer_id, "from", "to", "log_address", created_at
            "#,
            id,
            self.chain_id,
            consumer_id,
            from as Option<&str>,
            to as Option<&str>,
            log_address as Option<&str>,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row)
    }

    /// Fetch all active filters for this chain_id.
    /// Results are ordered by consumer_id for efficient grouping.
    pub async fn get_filters_by_chain_id(&self) -> SqlResult<Vec<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let rows = sqlx::query_as!(
            Filter,
            r#"
            SELECT id, chain_id, consumer_id, "from", "to", "log_address", created_at
            FROM filters
            WHERE chain_id = $1
            ORDER BY consumer_id
            "#,
            self.chain_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }

    /// Fetch all active filters for a single consumer on this chain.
    ///
    /// Uses the leftmost-prefix of the existing unique index
    /// `(chain_id, consumer_id, ...)` — no dedicated index needed.
    pub async fn get_filters_by_consumer_id(&self, consumer_id: &str) -> SqlResult<Vec<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let rows = sqlx::query_as!(
            Filter,
            r#"
            SELECT id, chain_id, consumer_id, "from", "to", "log_address", created_at
            FROM filters
            WHERE chain_id = $1 AND consumer_id = $2
            ORDER BY id
            "#,
            self.chain_id,
            consumer_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }

    /// Remove a filter matching the given (chain_id, consumer_id, from, to).
    ///
    /// Returns `Some(Filter)` if a filter was removed, `None` if no matching filter found.
    pub async fn remove_filter(
        &self,
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
    ) -> SqlResult<Option<Filter>> {
        let mut conn = self.client.get_app_connection().await?;

        let row = sqlx::query_as!(
            Filter,
            r#"
            DELETE FROM filters
            WHERE chain_id = $1
              AND consumer_id = $2
              AND COALESCE("from", '') = COALESCE($3, '')
              AND COALESCE("to", '') = COALESCE($4, '')
              AND COALESCE("log_address", '') = COALESCE($5, '')
            RETURNING id, chain_id, consumer_id, "from", "to", "log_address", created_at
            "#,
            self.chain_id,
            consumer_id,
            from as Option<&str>,
            to as Option<&str>,
            log_address as Option<&str>,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row)
    }
}
