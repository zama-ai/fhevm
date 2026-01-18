use std::time::Instant;

use crate::{config::settings::CronConfig, metrics, store::sql::client::PgClient};
use anyhow::Result;

pub struct ExpiryRepository {
    pool: PgClient,
    cron_config: CronConfig,
}

impl ExpiryRepository {
    pub fn new(pool: PgClient, cron_config: CronConfig) -> Self {
        Self { pool, cron_config }
    }

    pub async fn purge_stale_data(&self) -> Result<u64> {
        let mut total_deleted = 0;
        let public_decrypt_expiry_secs = self.cron_config.public_decrypt_expiry.as_secs_f64();
        let user_decrypt_expiry_secs = self.cron_config.user_decrypt_expiry.as_secs_f64();
        let input_proof_expiry_secs = self.cron_config.input_proof_expiry.as_secs_f64();

        let mut conn = self.pool.get_cron_connection().await?;
        let query_start = Instant::now();

        let result = sqlx::query!(
            r#"
                DELETE FROM public_decrypt_req 
                WHERE updated_at < NOW() - make_interval(secs => $1)
                "#,
            public_decrypt_expiry_secs
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

        // User decrypt: Delete shares and requests atomically using JOIN.
        // This ensures shares are only deleted when their parent request is also expiring,
        // preventing orphaned shares or race conditions.
        let mut tx = self.pool.get_cron_pool().begin().await?;
        let query_start = Instant::now();

        // First, delete shares whose PARENT request is expiring (based on request.updated_at)
        let shares_result = sqlx::query!(
            r#"
                DELETE FROM user_decrypt_share s
                USING user_decrypt_req r
                WHERE s.gw_reference_id = r.gw_reference_id
                  AND r.updated_at < NOW() - make_interval(secs => $1)
                "#,
            user_decrypt_expiry_secs
        )
        .execute(&mut *tx)
        .await;

        match &shares_result {
            Ok(_) => {
                metrics::observe_query(metrics::Table::UserDecryptShares, query_start.elapsed())
            }
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptShares),
        }
        total_deleted += shares_result?.rows_affected();

        let query_start = Instant::now();

        // Then, delete expired requests
        let req_result = sqlx::query!(
            r#"
                DELETE FROM user_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                "#,
            user_decrypt_expiry_secs
        )
        .execute(&mut *tx)
        .await;

        match &req_result {
            Ok(_) => metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed()),
            Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
        }
        total_deleted += req_result?.rows_affected();

        tx.commit().await?;

        // ---------------------------------------------------------------------
        // 4. Input Proof Requests (7 days)
        // ---------------------------------------------------------------------
        let mut conn = self.pool.get_cron_connection().await?;
        let query_start = Instant::now();

        let result = sqlx::query!(
            r#"
                DELETE FROM input_proof_req 
                WHERE updated_at < NOW() - make_interval(secs => $1)
                "#,
            input_proof_expiry_secs
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
