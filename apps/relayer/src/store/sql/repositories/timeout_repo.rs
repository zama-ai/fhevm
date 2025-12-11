use crate::store::sql::client::PgClient;
use anyhow::Result;

pub struct TimeoutRepository {
    pool: PgClient,
}

impl TimeoutRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: This is an alternative/fallback solution to the time out cron job migration if we are not able to manage installation of the pg_cron extension into our databse.
    // Comment cron job migration file and use this if we are not able to install it.

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

// SUBSEQUENT TOKIO TASK IF WE CANNOT INSTALL OUR CRON...
/*
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

// Pass your db pool to this function
pub fn spawn_timeout_worker(pool: PgClient) {
    tokio::spawn(async move {
        let repo = TimeoutRepository::new(pool);
        let mut ticker = interval(Duration::from_secs(60)); // Run every 60 seconds

        loop {
            ticker.tick().await; // Wait for next tick

            match repo.time_out_stale_requests().await {
                Ok(count) => {
                    if count > 0 {
                        info!("Timeout Worker: Timed out {} stale requests", count);
                    }
                }
                Err(e) => {
                    error!("Timeout Worker Failed: {:?}", e);
                }
            }
        }
    });
}

*/

// ALTERNATIVE FOR DISTRIBUTED CRON JOBS EVEN IN THE INTERNALS.
/*
use anyhow::Result;
use crate::store::sql::client::PgClient;

// A random constant integer ID for this specific job.
// Postgres uses i64 keys. This ensures no other app logic conflicts with this lock.
const TIMEOUT_JOB_LOCK_ID: i64 = 847202384;

pub struct TimeoutRepository {
    pool: PgClient,
}

impl TimeoutRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// Updates stale requests safely in a distributed environment.
    /// Only ONE instance will execute this at a time. Others will skip.
    pub async fn time_out_stale_requests(&self) -> Result<u64> {
        // 1. Start a Transaction
        let mut tx = self.pool.get_pool().begin().await?;

        // 2. Try to acquire the Advisory Lock for this Transaction
        // 'pg_try_advisory_xact_lock' returns true if obtained, false if someone else has it.
        // The lock is auto-released when 'tx' commits or rolls back.
        let got_lock: bool = sqlx::query_scalar!(
            "SELECT pg_try_advisory_xact_lock($1)",
            TIMEOUT_JOB_LOCK_ID
        )
        .fetch_one(&mut *tx)
        .await?
        .unwrap_or(false);

        // 3. If we didn't get the lock, another instance is doing it. Abort.
        if !got_lock {
            // tracing::debug!("Timeout job already running on another instance. Skipping.");
            return Ok(0);
        }

        // --- WE ARE THE LEADER NOW ---

        // 4. Execute Updates (using the transaction 'tx')

        // User Decrypt
        let r1 = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET req_status = 'timed_out'::req_status,
                err_reason = 'Gateway chain did not respond within the expected timeframe'
            WHERE req_status = 'receipt_received'::req_status
              AND updated_at < NOW() - INTERVAL '30 minutes'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // Public Decrypt
        let r2 = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET req_status = 'timed_out'::req_status,
                err_reason = 'Gateway chain did not respond within the expected timeframe'
            WHERE req_status = 'receipt_received'::req_status
              AND updated_at < NOW() - INTERVAL '30 minutes'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // Input Proof
        let r3 = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET req_status = 'timed_out'::req_status,
                err_reason = 'Gateway chain did not respond within the expected timeframe'
            WHERE req_status = 'receipt_received'::req_status
              AND updated_at < NOW() - INTERVAL '30 minutes'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // 5. Commit the transaction
        // This applies the updates AND releases the lock atomically.
        tx.commit().await?;

        Ok(r1 + r2 + r3)
    }
}
*/
