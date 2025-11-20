use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::models::user_decrypt_req_model::ConsensusReqState;
use crate::store::sql::{
    client::PgClient,
    models::{
        user_decrypt_req_model::UserDecryptResponseModel,
        user_decrypt_share_model::UserDecryptShare,
    },
};
use anyhow::Result;
use sqlx::types::Json;
use sqlx::types::Uuid;

pub struct UserDecryptRepository {
    pool: PgClient,
}

impl UserDecryptRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with err_reason = 'response timed out' (ACL propagation error).
    // OR IN THE TIMEOUT REPO.

    /* NOTE: max size for indexes
        B-Tree (Default)	~2,704 bytes	Used for your UNIQUE indexes (int_indexer_id).
        Hash (USING HASH)	Unlimited (1 GB)	Used for your non-unique lookups (ext_reference_id).
    */

    // GENERAL REQUESTS.

    /// Check if there is already existing internal_indexer_id and return ext_reference_id if there is one
    pub async fn find_ext_reference_id_by_int_indexer_id(
        &self,
        int_indexer_id_bytes: &[u8],
    ) -> Result<Option<Uuid>> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT ext_reference_id
            FROM user_decrypt_req
            WHERE int_indexer_id = $1
            LIMIT 1
            "#,
            int_indexer_id_bytes
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    /// Insert req, ext_reference_id, int_indexer_id.
    /// If conflict on int_indexer_id, it returns the EXISTING ext_reference_id.
    /// If no conflict, it inserts and returns the NEW ext_reference_id.
    pub async fn insert_data_on_conflict_and_get_ext_reference_id(
        &self,
        ext_reference_id: Uuid,
        int_indexer_id_bytes: &[u8],
        req: serde_json::Value,
    ) -> Result<Uuid> {
        let result = sqlx::query_scalar!(
            r#"
            INSERT INTO user_decrypt_req (
                ext_reference_id,
                int_indexer_id,
                req
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (int_indexer_id) 
            DO UPDATE SET updated_at = NOW() -- Dummy update to ensure RETURNING works
            RETURNING ext_reference_id
            "#,
            ext_reference_id,
            int_indexer_id_bytes,
            req,
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // GW READINESS LOGIC CHECK.
    /// update user_decrypt_req by int_indexer_id for to req_status processing
    /// Update req_status to 'processing' by int_indexer_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_processing(&self, int_indexer_id_bytes: &[u8]) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET req_status = 'processing'::req_status
            WHERE int_indexer_id = $1
            "#,
            int_indexer_id_bytes
        )
        .execute(&self.pool.get_pool()) // Using execute() since we don't need to return data
        .await?;

        Ok(result.rows_affected())
    }

    // if not ready after 30min..
    /// Update req_status to 'timed_out' and set err_reason by int_indexer_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_timed_out(
        &self,
        int_indexer_id_bytes: &[u8],
        err_reason: &str,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET 
                req_status = 'timed_out'::req_status,
                err_reason = $1
            WHERE int_indexer_id = $2
            "#,
            err_reason,
            int_indexer_id_bytes
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    // TRANSACTION REQUESTS.

    /// Updating the req_status to receipt_received, gw_req_tx_hash, gw_reference_id.
    /// Returns the number of rows affected (should be 1 or retry).
    pub async fn update_status_to_receipt_received_on_tx_success(
        &self,
        int_indexer_id_bytes: &[u8],
        gw_req_tx_hash: &str,
        gw_reference_id: i32,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET 
                req_status = 'receipt_received'::req_status,
                gw_req_tx_hash = $1,
                gw_reference_id = $2
            WHERE int_indexer_id = $3
            "#,
            gw_req_tx_hash,
            gw_reference_id,
            int_indexer_id_bytes
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    /// update req_status to failure and apply err_reason by internal_indexer_id
    pub async fn update_status_to_failure_on_tx_failed(
        &self,
        int_indexer_id_bytes: &[u8],
        err_reason: &str,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET 
                req_status = 'failure'::req_status,
                err_reason = $1
            WHERE int_indexer_id = $2
            "#,
            err_reason,
            int_indexer_id_bytes
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    // LISTENER REQUESTS.
    // If we recieve consensus reached tx:
    // update user_decrypt_req for gw_consensus_tx_hash by gw_reference_id
    // and if consensus_tx_hash = null and if status = 'receipt_recieved'
    // but in any case return (status, updated_at, err_reason, int_indexer_id) for `gw_reference_id`
    /// Attempts to update consensus hash ONLY IF status is 'receipt_received' AND hash is null.
    /// ALWAYS returns the current state of the row (req_status, updated_at, etc.) regardless of update success.
    /// Step 6: Handle Consensus Tx.
    pub async fn update_consensus_hash_and_return_state(
        &self,
        gw_reference_id: i32,
        gw_consensus_tx_hash: &str,
    ) -> Result<Option<ConsensusReqState>> {
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
                  AND req_status = 'receipt_received'::req_status
                RETURNING req_status, updated_at, err_reason, int_indexer_id
            )
            -- 1. Select from updated_row
            SELECT 
                req_status as "req_status!: ReqStatus", 
                updated_at as "updated_at!",
                err_reason,
                int_indexer_id as "int_indexer_id!"
            FROM updated_row
            
            UNION ALL
            
            -- 2. Select from original table
            SELECT 
                req_status as "req_status!: ReqStatus", 
                updated_at as "updated_at!",
                err_reason, 
                int_indexer_id as "int_indexer_id!"
            FROM user_decrypt_req
            WHERE id = (SELECT id FROM target)
              AND NOT EXISTS (SELECT 1 FROM updated_row)
            "#,
            gw_consensus_tx_hash,
            gw_reference_id
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // We recieve a share event from the gw.
    /// Insert in user_decrypt_share table all the fields: gw_reference_id, share_index, share, kms_signature, extra_data and return number of shares for gw_reference_id in the table.
    /// Insert a share and return the total count of shares for this gw_reference_id.
    /// NOTE: This lead to possibility of non relevant shares, we can recieve unrelated shares non related to relayer events, or timed_out shares, we register them anyway.
    pub async fn insert_share_and_return_count(
        &self,
        gw_reference_id: i32,
        share_index: i32,
        share: &str,
        kms_signature: &str,
        extra_data: Option<&str>,
    ) -> Result<i64> {
        // We use a CTE to Insert and Count in one atomic operation
        let count = sqlx::query_scalar!(
            r#"
            WITH inserted AS (
                INSERT INTO user_decrypt_share (
                    gw_reference_id,
                    share_index,
                    share,
                    kms_signature,
                    extra_data
                )
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (gw_reference_id, share_index) DO NOTHING
            )
            SELECT COUNT(*) as "count!" -- The "!" tells SQLx this is never null
            FROM user_decrypt_share
            WHERE gw_reference_id = $1
            "#,
            gw_reference_id,
            share_index,
            share,
            kms_signature,
            extra_data
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(count)
    }

    /// update user_decrypt_reqf req_status to completed by gw_reference_id and return all shares + int_indexer_id + status + updated_at + err_reason from user_decrypt_share table by gw_reference_id.
    /// Step 6 (Share Flow): Update to 'completed' and return Metadata + All Shares.
    /// Returns a tuple: (ConsensusReqState, Vec<UserDecryptShare>).
    /// Fails if the request is 'timed_out' or does not exist.
    pub async fn complete_req_and_get_shares_metadata(
        &self,
        gw_reference_id: i32,
    ) -> Result<(ConsensusReqState, Vec<UserDecryptShare>)> {
        let records = sqlx::query!(
            r#"
            WITH updated_req AS (
                UPDATE user_decrypt_req
                SET req_status = 'completed'::req_status
                WHERE gw_reference_id = $1
                  AND req_status != 'timed_out'::req_status -- Prevent updating if already timed out
                RETURNING int_indexer_id, req_status, updated_at, err_reason
            )
            SELECT 
                -- Metadata from the Update (Force non-null types for SQLx)
                u.int_indexer_id as "int_indexer_id!",
                u.req_status as "req_status!: ReqStatus",
                u.updated_at as "updated_at!",
                u.err_reason,
                
                -- Share Data
                s.id as share_id,
                s.gw_reference_id,
                s.share_index,
                s.share,
                s.kms_signature,
                s.extra_data,
                s.created_at as share_created_at,
                s.updated_at as share_updated_at
            FROM user_decrypt_share s, updated_req u
            WHERE s.gw_reference_id = $1
            ORDER BY s.share_index ASC
            "#,
            gw_reference_id
        )
        .fetch_all(&self.pool.get_pool())
        .await?;

        // If empty, it means either:
        // 1. The gw_reference_id doesn't exist.
        // 2. The request was 'timed_out' (so the UPDATE returned 0 rows).
        // 3. There are no shares (unlikely if we reached threshold logic).
        if records.is_empty() {
            return Err(anyhow::anyhow!(
                "Request not found, timed out, or no shares available for gw_ref_id: {}",
                gw_reference_id
            ));
        }

        // 1. Extract Metadata from the first row (it's identical for all rows)
        let first = &records[0];
        let metadata = ConsensusReqState {
            int_indexer_id: first.int_indexer_id.clone(),
            req_status: first.req_status,
            updated_at: first.updated_at,
            err_reason: first.err_reason.clone(),
        };

        // 2. Map all rows to UserDecryptShare struct
        let shares: Vec<UserDecryptShare> = records
            .into_iter()
            .map(|r| UserDecryptShare {
                id: r.share_id,
                gw_reference_id: r.gw_reference_id,
                share_index: r.share_index,
                share: r.share,
                kms_signature: r.kms_signature,
                extra_data: r.extra_data,
                created_at: r.share_created_at,
                updated_at: r.share_updated_at,
            })
            .collect();

        Ok((metadata, shares))
    }

    // GET REQUESTS RESULTS.
    /// Select in user_decrypt_req by ext_reference_id and get all the shares on gw_reference_id to construct the the final response,
    /// fields in return: ext_reference_id, req_status, shares, updated_at, err_reason, gw_req_tx_hash, gw_consensus_tx_hash.
    pub async fn find_req_and_shares_by_ext_reference_id(
        &self,
        ext_reference_id: Uuid,
    ) -> Result<Option<UserDecryptResponseModel>> {
        let result = sqlx::query_as!(
            UserDecryptResponseModel,
            r#"
            SELECT 
                r.ext_reference_id,
                r.req_status as "req_status!: ReqStatus", -- Force non-null Enum type
                r.updated_at,
                r.err_reason,
                r.gw_req_tx_hash,
                r.gw_consensus_tx_hash,
                -- Aggregate shares into a JSON List. 
                -- If no shares exist, return an empty JSON array '[]'
                COALESCE(
                    jsonb_agg(to_jsonb(s.*) ORDER BY s.share_index) 
                    FILTER (WHERE s.id IS NOT NULL), 
                    '[]'::jsonb
                ) as "shares!: Json<Vec<UserDecryptShare>>"
            FROM user_decrypt_req r
            LEFT JOIN user_decrypt_share s ON r.gw_reference_id = s.gw_reference_id
            WHERE r.ext_reference_id = $1
            GROUP BY r.id
            "#,
            ext_reference_id
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }
}
