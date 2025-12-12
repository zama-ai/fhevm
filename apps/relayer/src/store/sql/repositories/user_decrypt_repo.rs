use crate::core::event::UserDecryptRequest;
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::models::user_decrypt_req_model::ConsensusReqState;
use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
    models::{
        user_decrypt_req_model::{UserDecryptResponseModel, UserDecryptResponseShare},
        user_decrypt_share_model::{ShareInsertParams, UserDecryptShare},
    },
    repositories::utils::compute_advisory_lock_id,
};
use alloy::primitives::U256;
use sqlx::types::{Json, Uuid};

// Import conversion functions privately within this repository
use crate::store::sql::conversion::u256_to_i32;

pub struct UserDecryptRepository {
    pool: PgClient,
}

impl UserDecryptRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with configured timeout message.
    // OR IN THE TIMEOUT REPO.

    /* NOTE: max size for indexes
        B-Tree (Default)	~2,704 bytes	Used for your UNIQUE indexes (int_job_id).
        Hash (USING HASH)	Unlimited (1 GB)	Used for your non-unique lookups (ext_job_id).
    */

    // GENERAL REQUESTS.

    /// Check if there is already existing internal_indexer_id and return ext_job_id if there is one
    pub async fn find_ext_job_id_by_int_job_id(
        &self,
        int_job_id_bytes: &[u8],
    ) -> SqlResult<Option<Uuid>> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT ext_job_id
            FROM user_decrypt_req
            WHERE int_job_id = $1
            LIMIT 1
            "#,
            int_job_id_bytes
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    /// Insert req, ext_job_id, int_job_id.
    /// If conflict on int_job_id, it returns the EXISTING ext_job_id.
    /// If no conflict, it inserts and returns the NEW ext_job_id.
    pub async fn insert_data_on_conflict_and_get_ext_job_id(
        &self,
        ext_job_id: Uuid,
        int_job_id_bytes: &[u8],
        request: UserDecryptRequest,
    ) -> SqlResult<Uuid> {
        let req = serde_json::to_value(&request).map_err(|e| {
            SqlError::conversion_error(
                "request",
                "UserDecryptRequest",
                format!("Failed to serialize: {}", e),
            )
        })?;
        let result = sqlx::query_scalar!(
            r#"
            INSERT INTO user_decrypt_req (
                ext_job_id,
                int_job_id,
                req
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (int_job_id)
            WHERE req_status NOT IN ('failure'::req_status, 'timed_out'::req_status)
            DO UPDATE SET updated_at = NOW() -- Dummy update to ensure RETURNING works
            RETURNING ext_job_id
            "#,
            ext_job_id,
            int_job_id_bytes,
            req,
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // GW READINESS LOGIC CHECK.
    /// update user_decrypt_req by int_job_id for to req_status processing
    /// Update req_status to 'processing' by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_processing(&self, int_job_id_bytes: &[u8]) -> SqlResult<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET req_status = 'processing'::req_status
            WHERE int_job_id = $1
            "#,
            int_job_id_bytes
        )
        .execute(&self.pool.get_pool()) // Using execute() since we don't need to return data
        .await?;

        Ok(result.rows_affected())
    }

    // if not ready after 30min..
    /// Update req_status to 'timed_out' and set err_reason by int_job_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_timed_out(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET
                req_status = 'timed_out'::req_status,
                err_reason = $1
            WHERE int_job_id = $2
            "#,
            err_reason,
            int_job_id_bytes
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    // TRANSACTION REQUESTS.

    /// Updating the req_status to receipt_received, gw_req_tx_hash, gw_reference_id by int_job_id
    /// Returns the number of rows affected (should be 1 or retry).
    pub async fn update_status_to_receipt_received_on_tx_success(
        &self,
        int_job_id_bytes: &[u8],
        gw_req_tx_hash: &str,
        gw_reference_id: U256,
    ) -> SqlResult<u64> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET
                req_status = 'receipt_received'::req_status,
                gw_req_tx_hash = $1,
                gw_reference_id = $2
            WHERE int_job_id = $3
            "#,
            gw_req_tx_hash,
            gw_ref_id,
            int_job_id_bytes
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    /// update req_status to failure and apply err_reason by internal_indexer_id
    pub async fn update_status_to_failure_on_tx_failed(
        &self,
        int_job_id_bytes: &[u8],
        err_reason: &str,
    ) -> SqlResult<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET
                req_status = 'failure'::req_status,
                err_reason = $1
            WHERE int_job_id = $2
            "#,
            err_reason,
            int_job_id_bytes
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    // LISTENER REQUESTS.
    // If we recieve consensus reached tx:
    // update user_decrypt_req for gw_consensus_tx_hash by gw_reference_id
    // and if consensus_tx_hash = null and if status = 'receipt_recieved'
    // but in any case return (status, updated_at, err_reason, int_job_id) for `gw_reference_id`
    /// Attempts to update consensus hash ONLY IF status is 'receipt_received' AND hash is null.
    /// ALWAYS returns the current state of the row (req_status, updated_at, etc.) regardless of update success.
    /// Step 6: Handle Consensus Tx.
    pub async fn update_consensus_hash_and_return_state(
        &self,
        gw_reference_id: U256,
        gw_consensus_tx_hash: &str,
    ) -> SqlResult<Option<ConsensusReqState>> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let result = sqlx::query_as!(
            ConsensusReqState,
            r#"
            WITH target AS (
                SELECT id FROM user_decrypt_req WHERE gw_reference_id = $2
            ),
            updated_row AS (
                UPDATE user_decrypt_req
                SET gw_consensus_tx_hash = $1
                WHERE id = (SELECT id FROM target)
                  AND gw_consensus_tx_hash IS NULL
                  AND req_status IN ('receipt_received'::req_status, 'completed'::req_status)
                RETURNING req_status, updated_at, err_reason, int_job_id
            )
            -- 1. Select from updated_row
            SELECT
                req_status as "req_status!: ReqStatus",
                updated_at as "updated_at!",
                err_reason,
                int_job_id as "int_job_id!"
            FROM updated_row

            UNION ALL

            -- 2. Select from original table
            SELECT
                req_status as "req_status!: ReqStatus",
                updated_at as "updated_at!",
                err_reason,
                int_job_id as "int_job_id!"
            FROM user_decrypt_req
            WHERE id = (SELECT id FROM target)
              AND NOT EXISTS (SELECT 1 FROM updated_row)
            "#,
            gw_consensus_tx_hash,
            gw_ref_id
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // We recieve a share event from the gw.
    /// Insert share and atomically complete request if threshold is reached.
    /// Returns (share_count, completion_result) where completion_result contains
    /// metadata and shares if threshold was reached and completion successful.
    ///
    /// This prevents race conditions between share insertion and request completion
    /// by performing all operations within a single atomic transaction.
    pub async fn insert_share_and_complete_if_threshold_reached(
        &self,
        params: ShareInsertParams<'_>,
        threshold: i64,
    ) -> SqlResult<(i64, Option<(ConsensusReqState, Vec<UserDecryptShare>)>)> {
        let id_as_bytes_array: [u8; 32] = params.gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let share_index = u256_to_i32(params.share_index)
            .map_err(|e| SqlError::conversion_error("share_index", params.share_index, e))?;

        // Use advisory locks to serialize share operations per gw_reference_id
        //
        // Advisory locks provide two benefits:
        // 1. Prevents wasted sequence IDs:
        //    Without serialization, concurrent INSERT attempts hitting UNIQUE constraints:
        //    - Multiple transactions call nextval() concurrently
        //    - All but one fail due to UNIQUE constraint on (gw_reference_id, share_index)
        //    - Failed transactions waste their sequence IDs, creating gaps
        //    See: https://www.postgresql.org/docs/current/functions-sequence.html
        //
        // 2. Ensures correct counts by serializing inserts:
        //    When transactions execute sequentially, COUNT(*) sees all previous INSERTs
        //    for the same gw_reference_id, providing accurate share counts.
        //
        // 3. Prevents race conditions between threshold check and completion:
        //    By including completion logic in the same transaction, we prevent pg_cron
        //    timeout jobs from interfering between share insertion and request completion.
        //
        // Advisory lock automatically released when transaction commits/rollbacks.
        // See: https://www.postgresql.org/docs/current/explicit-locking.html#ADVISORY-LOCKS
        let mut tx = self.pool.get_pool().begin().await?;

        // Acquire advisory lock - this WAITS for other transactions, doesn't fail
        let lock_id = compute_advisory_lock_id(&gw_ref_id);
        sqlx::query!("SELECT pg_advisory_xact_lock($1)", lock_id)
            .execute(&mut *tx)
            .await?;

        // First, do the INSERT
        sqlx::query!(
            r#"
            INSERT INTO user_decrypt_share (
                gw_reference_id,
                tx_hash,
                share_index,
                share,
                kms_signature,
                extra_data
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (gw_reference_id, share_index) DO NOTHING
            "#,
            gw_ref_id,
            params.tx_hash,
            share_index,
            params.share,
            params.kms_signature,
            params.extra_data
        )
        .execute(&mut *tx)
        .await?;

        // Count shares within the same transaction
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM user_decrypt_share
            WHERE gw_reference_id = $1
            "#,
            gw_ref_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // If threshold reached, complete the request atomically
        let completion_result = if count == threshold {
            // Attempt to update request to completed status
            let update_result = sqlx::query!(
                r#"
                UPDATE user_decrypt_req
                SET req_status = 'completed'::req_status
                WHERE gw_reference_id = $1
                  AND req_status != 'timed_out'::req_status
                RETURNING int_job_id, req_status as "req_status: ReqStatus", updated_at, err_reason
                "#,
                gw_ref_id
            )
            .fetch_optional(&mut *tx)
            .await?;

            // If update succeeded, fetch shares
            if let Some(req_data) = update_result {
                let metadata = ConsensusReqState {
                    int_job_id: req_data.int_job_id,
                    req_status: ReqStatus::Completed,
                    updated_at: req_data.updated_at,
                    err_reason: req_data.err_reason,
                };

                // Fetch shares ordered by creation time, limited to threshold
                let share_records = sqlx::query!(
                    r#"
                    SELECT id, gw_reference_id, tx_hash, share_index, share, kms_signature, extra_data, created_at, updated_at
                    FROM user_decrypt_share
                    WHERE gw_reference_id = $1
                    ORDER BY created_at ASC, share_index ASC
                    LIMIT $2
                    "#,
                    gw_ref_id,
                    threshold
                )
                .fetch_all(&mut *tx)
                .await?;

                let shares: Vec<UserDecryptShare> = share_records
                    .into_iter()
                    .map(|r| UserDecryptShare {
                        id: r.id,
                        gw_reference_id: r.gw_reference_id,
                        tx_hash: r.tx_hash,
                        share_index: r.share_index,
                        share: r.share,
                        kms_signature: r.kms_signature,
                        extra_data: r.extra_data,
                        created_at: r.created_at,
                        updated_at: r.updated_at,
                    })
                    .collect();

                Some((metadata, shares))
            } else {
                // Request was already timed_out or doesn't exist
                None
            }
        } else {
            None
        };

        tx.commit().await?;
        Ok((count, completion_result))
    }

    // GET REQUESTS RESULTS.
    /// Select in user_decrypt_req by ext_job_id and get all the shares on gw_reference_id to construct the the final response,
    /// fields in return: ext_job_id, req_status, shares, updated_at, err_reason, gw_req_tx_hash, gw_consensus_tx_hash.
    pub async fn find_req_and_shares_by_ext_job_id(
        &self,
        ext_job_id: Uuid,
        threshold: i64,
    ) -> SqlResult<Option<UserDecryptResponseModel>> {
        let result = sqlx::query_as!(
            UserDecryptResponseModel,
            r#"
            SELECT
                r.ext_job_id,
                r.req_status as "req_status!: ReqStatus", -- Force non-null Enum type
                r.updated_at,
                r.err_reason,
                r.gw_req_tx_hash,
                r.gw_consensus_tx_hash,
                -- Aggregate shares into a JSON List.
                -- If no shares exist, return an empty JSON array '[]'
                -- Only select needed fields to avoid BYTEA deserialization issues
                COALESCE(
                    jsonb_agg(
                        jsonb_build_object(
                            'share', s.share,
                            'kms_signature', s.kms_signature,
                            'extra_data', s.extra_data
                        )
                        ORDER BY s.share_index
                    )
                    FILTER (WHERE s.id IS NOT NULL),
                    '[]'::jsonb
                ) as "shares!: Json<Vec<UserDecryptResponseShare>>"
            FROM user_decrypt_req r
            LEFT JOIN (
                SELECT * FROM user_decrypt_share
                WHERE gw_reference_id IN (
                    SELECT gw_reference_id FROM user_decrypt_req WHERE ext_job_id = $1
                )
                ORDER BY created_at ASC, share_index ASC
                LIMIT $2  -- Limit to exact threshold number of shares
            ) s ON r.gw_reference_id = s.gw_reference_id
            WHERE r.ext_job_id = $1
            GROUP BY r.id
            "#,
            ext_job_id,
            threshold
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }
}
