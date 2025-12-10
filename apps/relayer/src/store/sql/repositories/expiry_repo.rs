use crate::store::sql::client::PgClient;
use anyhow::Result;

// A unique constant integer ID for the Expiry/Cleanup job.
// Arbitrary number
const EXPIRY_JOB_LOCK_ID: i64 = 999111222;

pub struct ExpiryRepository {
    pool: PgClient,
}

impl ExpiryRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// Deletes old requests.
    ///
    /// Mechanism:
    /// 1. Starts a DB transaction.
    /// 2. Attempts to acquire a Postgres Advisory Lock (ID: 999111222).
    /// 3. If acquired: Executes all DELETE queries.
    /// 4. If not acquired: Returns immediately (another pod is doing it).
    ///
    /// Returns the total number of rows deleted.
    pub async fn purge_stale_data(&self) -> Result<u64> {
        // 1. Start a Transaction
        let mut tx = self.pool.get_pool().begin().await?;

        // 2. Try to acquire the Advisory Lock
        // Returns true if obtained, false if busy.
        let got_lock: bool =
            sqlx::query_scalar!("SELECT pg_try_advisory_xact_lock($1)", EXPIRY_JOB_LOCK_ID)
                .fetch_one(&mut *tx)
                .await?
                .unwrap_or(false);

        // 3. If busy, yield.
        if !got_lock {
            return Ok(0);
        }

        // --- LEADER SECTION ---

        // 4. Clean Public Decrypt Requests (Older than 365 days)
        // Uses index: idx_public_decrypt_req_created_at
        let r1 = sqlx::query!(
            r#"
            DELETE FROM public_decrypt_req 
            WHERE created_at < NOW() - INTERVAL '365 days'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // 5. Clean User Decrypt (Older than 24 hours)
        // Uses indexes: idx_user_decrypt_share_created_at, idx_user_decrypt_req_created_at
        // We delete shares first for logical consistency.
        let r2_shares = sqlx::query!(
            r#"
            DELETE FROM user_decrypt_share 
            WHERE created_at < NOW() - INTERVAL '24 hours'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        let r2_reqs = sqlx::query!(
            r#"
            DELETE FROM user_decrypt_req 
            WHERE created_at < NOW() - INTERVAL '24 hours'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // 6. Clean Input Proofs (Older than 24 hours)
        // Uses index: idx_input_proof_req_created_at
        let r3 = sqlx::query!(
            r#"
            DELETE FROM input_proof_req 
            WHERE created_at < NOW() - INTERVAL '24 hours'
            "#
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        // 7. Commit Transaction (Releases Lock)
        tx.commit().await?;

        Ok(r1 + r2_shares + r2_reqs + r3)
    }
}
