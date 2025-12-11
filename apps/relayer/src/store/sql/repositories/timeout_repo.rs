use crate::store::sql::client::PgClient;
use anyhow::Result;

pub struct TimeoutRepository {
    pool: PgClient,
}

impl TimeoutRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // TODO: Make the timeout configurable from settings for each of the values here.

    /// Returns the total number of rows moved to 'timed_out'.
    /// Updates all requests that have been stuck in 'receipt_received' for > 30 minutes.
    pub async fn time_out_stale_requests(&self) -> Result<u64> {
        // 1. User Decrypt
        let r1 = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET req_status = 'timed_out'::req_status, 
                err_reason = 'Gateway chain did not respond within the expected timeframe'
            WHERE req_status = 'receipt_received'::req_status
              AND updated_at < NOW() - INTERVAL '30 minutes'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        // 2. Public Decrypt
        let r2 = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET req_status = 'timed_out'::req_status, 
                err_reason = 'Gateway chain did not respond within the expected timeframe'
            WHERE req_status = 'receipt_received'::req_status
              AND updated_at < NOW() - INTERVAL '30 minutes'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        // 3. Input Proof
        let r3 = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET req_status = 'timed_out'::req_status, 
                err_reason = 'Gateway chain did not respond within the expected timeframe'
            WHERE req_status = 'receipt_received'::req_status
              AND updated_at < NOW() - INTERVAL '30 minutes'
            "#
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();

        Ok(r1 + r2 + r3)
    }
}
