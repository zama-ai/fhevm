use std::sync::Arc;
use uuid::Uuid;

use crate::store::client::PgClient;
use crate::store::error::SqlResult;
use crate::store::models::{Filter, FilterType};

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
        filter_type: FilterType,
    ) -> SqlResult<Option<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        self.add_filter_in(&mut conn, consumer_id, from, to, log_address, filter_type)
            .await
    }

    /// All additions become visible together; any error rolls the batch back.
    pub async fn add_filters_atomically(
        &self,
        filters: &[primitives::event::FilterCommand],
    ) -> SqlResult<()> {
        let mut conn = self.client.get_app_connection().await?;
        let mut tx = sqlx::Connection::begin(&mut *conn).await?;
        for filter in filters {
            let from = primitives::utils::checksum_optional_address(&filter.from);
            let to = primitives::utils::checksum_optional_address(&filter.to);
            let log_address = primitives::utils::checksum_optional_address(&filter.log_address);
            self.add_filter_in(
                &mut tx,
                &filter.consumer_id,
                from.as_deref(),
                to.as_deref(),
                log_address.as_deref(),
                filter.filter_type.unwrap_or_default().into(),
            )
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn add_filter_in(
        &self,
        conn: &mut sqlx::PgConnection,
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
        filter_type: FilterType,
    ) -> SqlResult<Option<Filter>> {
        let id = Uuid::new_v4();

        let row = sqlx::query_as!(
            Filter,
            r#"
            INSERT INTO filters (id, chain_id, consumer_id, "from", "to", "log_address", filter_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (chain_id, consumer_id, COALESCE("from", ''), COALESCE("to", ''), COALESCE("log_address", ''), filter_type)
            DO NOTHING
            RETURNING id, chain_id, consumer_id, "from", "to", "log_address", filter_type as "filter_type: FilterType", created_at
            "#,
            id,
            self.chain_id,
            consumer_id,
            from as Option<&str>,
            to as Option<&str>,
            log_address as Option<&str>,
            filter_type as FilterType,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row)
    }

    /// Fetch all active LIVE filters for this chain_id.
    /// Results are ordered by consumer_id for efficient grouping.
    ///
    /// FINAL filters are excluded: they belong to the finalized-delivery flow
    /// and must never receive head-of-chain events.
    /// Served by the partial index `idx_filters_chain_consumer_live`.
    pub async fn get_filters_by_chain_id(&self) -> SqlResult<Vec<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let rows = sqlx::query_as!(
            Filter,
            r#"
            SELECT id, chain_id, consumer_id, "from", "to", "log_address", filter_type as "filter_type: FilterType", created_at
            FROM filters
            WHERE chain_id = $1 AND filter_type = 'LIVE'::filter_type
            ORDER BY consumer_id
            "#,
            self.chain_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }

    /// Fetch all active LIVE filters for a single consumer on this chain.
    ///
    /// FINAL filters are excluded (see [`Self::get_filters_by_chain_id`]).
    /// Served by the partial index `idx_filters_chain_consumer_live`.
    pub async fn get_filters_by_consumer_id(&self, consumer_id: &str) -> SqlResult<Vec<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let rows = sqlx::query_as!(
            Filter,
            r#"
            SELECT id, chain_id, consumer_id, "from", "to", "log_address", filter_type as "filter_type: FilterType", created_at
            FROM filters
            WHERE chain_id = $1 AND consumer_id = $2 AND filter_type = 'LIVE'::filter_type
            ORDER BY id
            "#,
            self.chain_id,
            consumer_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }

    /// Fetch all active FINAL filters for this chain_id.
    /// Results are ordered by consumer_id for efficient grouping.
    ///
    /// LIVE filters are excluded: they belong to the head-of-chain flow and
    /// must never receive finalized-only events.
    /// Served by the partial index `idx_filters_chain_consumer_final`.
    pub async fn get_final_filters_by_chain_id(&self) -> SqlResult<Vec<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let rows = sqlx::query_as!(
            Filter,
            r#"
            SELECT id, chain_id, consumer_id, "from", "to", "log_address", filter_type as "filter_type: FilterType", created_at
            FROM filters
            WHERE chain_id = $1 AND filter_type = 'FINAL'::filter_type
            ORDER BY consumer_id
            "#,
            self.chain_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }

    /// Fetch all active FINAL filters for a single consumer on this chain.
    ///
    /// LIVE filters are excluded (see [`Self::get_final_filters_by_chain_id`]).
    /// Served by the partial index `idx_filters_chain_consumer_final`.
    /// Groundwork for the finality catchup feature.
    pub async fn get_final_filters_by_consumer_id(
        &self,
        consumer_id: &str,
    ) -> SqlResult<Vec<Filter>> {
        let mut conn = self.client.get_app_connection().await?;
        let rows = sqlx::query_as!(
            Filter,
            r#"
            SELECT id, chain_id, consumer_id, "from", "to", "log_address", filter_type as "filter_type: FilterType", created_at
            FROM filters
            WHERE chain_id = $1 AND consumer_id = $2 AND filter_type = 'FINAL'::filter_type
            ORDER BY id
            "#,
            self.chain_id,
            consumer_id,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(rows)
    }

    /// Remove a filter matching the given (chain_id, consumer_id, from, to, log_address, filter_type).
    ///
    /// Returns `Some(Filter)` if a filter was removed, `None` if no matching filter found.
    pub async fn remove_filter(
        &self,
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
        filter_type: FilterType,
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
              AND filter_type = $6
            RETURNING id, chain_id, consumer_id, "from", "to", "log_address", filter_type as "filter_type: FilterType", created_at
            "#,
            self.chain_id,
            consumer_id,
            from as Option<&str>,
            to as Option<&str>,
            log_address as Option<&str>,
            filter_type as FilterType,
        )
        .fetch_optional(&mut *conn)
        .await?;

        Ok(row)
    }
}

#[cfg(test)]
mod batch_tests {
    use super::*;
    use primitives::event::FilterCommand;

    #[tokio::test]
    #[ignore = "requires a disposable BATCH_WATCH_DATABASE_URL"]
    async fn batch_watch_commits_together_and_rolls_back_on_failure() {
        let pool = sqlx::PgPool::connect(&std::env::var("BATCH_WATCH_DATABASE_URL").unwrap())
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        let chain_id = 42;
        let config = serde_json::from_value(serde_json::json!({
            "db_url": std::env::var("BATCH_WATCH_DATABASE_URL").unwrap()
        }))
        .unwrap();
        let repo = FilterRepository::new(Arc::new(PgClient::new(&config).await.unwrap()), chain_id);
        let owner = Uuid::new_v4().to_string();
        let first = FilterCommand {
            consumer_id: owner.clone(),
            from: None,
            to: None,
            log_address: Some(
                "0x0000000000000000000000000000000000000001"
                    .parse()
                    .unwrap(),
            ),
            filter_type: None,
        };
        let mut second = first.clone();
        second.log_address = Some(
            "0x0000000000000000000000000000000000000002"
                .parse()
                .unwrap(),
        );
        // A database error on the second row must not expose the first row.
        sqlx::raw_sql(
            r#"
            CREATE FUNCTION reject_batch_test_filter() RETURNS trigger LANGUAGE plpgsql AS $$
            BEGIN
                IF NEW.log_address = '0x0000000000000000000000000000000000000002' THEN
                    RAISE EXCEPTION 'injected second insert failure';
                END IF;
                RETURN NEW;
            END $$;
            CREATE TRIGGER reject_batch_test_filter BEFORE INSERT ON filters
            FOR EACH ROW EXECUTE FUNCTION reject_batch_test_filter();
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();
        assert!(
            repo.add_filters_atomically(&[first.clone(), second.clone()])
                .await
                .is_err()
        );
        assert!(
            repo.get_filters_by_consumer_id(&owner)
                .await
                .unwrap()
                .is_empty()
        );
        sqlx::raw_sql("DROP TRIGGER reject_batch_test_filter ON filters; DROP FUNCTION reject_batch_test_filter();")
            .execute(&pool).await.unwrap();
        repo.add_filters_atomically(&[first.clone(), second.clone()])
            .await
            .unwrap();
        assert_eq!(
            repo.get_filters_by_consumer_id(&owner).await.unwrap().len(),
            2
        );
        repo.add_filters_atomically(&[first, second]).await.unwrap();
        assert_eq!(
            repo.get_filters_by_consumer_id(&owner).await.unwrap().len(),
            2
        );
        pool.close().await;
    }
}
