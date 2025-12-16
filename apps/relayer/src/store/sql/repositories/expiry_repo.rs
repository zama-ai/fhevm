use std::time::Instant;

use crate::{metrics, store::sql::client::PgClient};
use anyhow::Result;

pub struct ExpiryRepository {
    pool: PgClient,
}

impl ExpiryRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // TODO: make the values configurables, and pass 7 days user-decryption and input proof to 1 day.
    pub async fn purge_stale_data(&self) -> Result<u64> {
        let mut total_deleted = 0;

        let mut conn = self.pool.get_cron_connection().await?;
        let query_start = Instant::now();

        let result = sqlx::query!(
            r#"
                DELETE FROM public_decrypt_req 
                WHERE updated_at < NOW() - INTERVAL '365 days'
                "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
        }
        total_deleted += result?.rows_affected();

        let mut conn = self.pool.get_cron_connection().await?;
        let query_start = Instant::now();

        let result = sqlx::query!(
            r#"
                DELETE FROM user_decrypt_share 
                WHERE updated_at < NOW() - INTERVAL '7 days'
                "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(
                metrics::Table::UserDecryptReq, // mapped to Parent Table category
                query_start.elapsed(),
            ),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }
        total_deleted += result?.rows_affected();

        let mut conn = self.pool.get_cron_connection().await?;
        let query_start = Instant::now();

        let result = sqlx::query!(
            r#"
                DELETE FROM user_decrypt_req 
                WHERE updated_at < NOW() - INTERVAL '7 days'
                "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }
        total_deleted += result?.rows_affected();

        // ---------------------------------------------------------------------
        // 4. Input Proof Requests (7 days)
        // ---------------------------------------------------------------------
        let mut conn = self.pool.get_cron_connection().await?;
        let query_start = Instant::now();

        let result = sqlx::query!(
            r#"
                DELETE FROM input_proof_req 
                WHERE updated_at < NOW() - INTERVAL '7 days'
                "#
        )
        .execute(&mut *conn)
        .await;

        match &result {
            Ok(_) => metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
        }
        total_deleted += result?.rows_affected();

        Ok(total_deleted)
    }
}
