use crate::store::sql::models::req_status_enum_model::ReqStatus;
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

pub struct UserDecryptReqRepository {
    pool: PgClient,
}

impl UserDecryptReqRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

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

    // VERSION 1 for 2 next queries:
    // TODO: two next queries should be in the same trasnaction.

    // We recieve a share event from the gw.
    /// Insert in user_decrypt_share table all the fields: gw_reference_id, share_index, share, kms_signature, extra_data and return number of shares for gw_reference_id in the table.
    /// Insert a share and return the total count of shares for this gw_reference_id.
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

    /// update user_decrypt_reqf req_status to completed by gw_reference_id and return all shares + int_indexer_id from user_decrypt_share table by gw_reference_id.
    /// Update req_status to 'completed' and return all associated shares.
    /// Returns a Vector of UserDecryptShare.
    /// Update req_status to 'completed' and return (int_indexer_id, shares).
    /// Returns a Tuple: (internal_indexer_id_bytes, List of shares).
    pub async fn complete_req_and_get_shares(
        &self,
        gw_reference_id: i32,
    ) -> Result<(Vec<u8>, Vec<UserDecryptShare>)> {
        let records = sqlx::query!(
            r#"
            WITH updated_req AS (
                UPDATE user_decrypt_req
                SET req_status = 'completed'::req_status
                WHERE gw_reference_id = $1
                RETURNING int_indexer_id
            )
            SELECT 
                u.int_indexer_id, -- We grab this from the update
                s.id,
                s.gw_reference_id,
                s.share_index,
                s.share,
                s.kms_signature,
                s.extra_data,
                s.created_at,
                s.updated_at
            FROM user_decrypt_share s, updated_req u
            WHERE s.gw_reference_id = $1
            "#,
            gw_reference_id
        )
        .fetch_all(&self.pool.get_pool())
        .await?;

        // 2. Handle the case where no rows are returned (Should not happen if shares exist)
        if records.is_empty() {
            return Err(anyhow::anyhow!(
                "No shares found for gw_reference_id {}",
                gw_reference_id
            ));
        }

        // 3. Extract int_indexer_id from the first row
        let int_indexer_id = records[0].int_indexer_id.clone();

        // 4. Map the anonymous records to your UserDecryptShare struct
        let shares: Vec<UserDecryptShare> = records
            .into_iter()
            .map(|r| UserDecryptShare {
                id: r.id,
                gw_reference_id: r.gw_reference_id,
                share_index: r.share_index,
                share: r.share,
                kms_signature: r.kms_signature,
                extra_data: r.extra_data,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect();

        Ok((int_indexer_id, shares))
    }

    // TODO: Combine two last queries in one single db transaction.

    // Update user_decrypt_req table with gw_consensus_tx_hash by gw_reference_id only if gw_consensus_tx_hash is null.
    /// Update gw_consensus_tx_hash by gw_reference_id, but ONLY if it is currently NULL.
    /// Returns 1 if updated, 0 if it was already set or id not found.
    pub async fn update_consensus_hash_if_missing(
        &self,
        gw_reference_id: i32,
        gw_consensus_tx_hash: &str,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET gw_consensus_tx_hash = $1
            WHERE gw_reference_id = $2 
              AND gw_consensus_tx_hash IS NULL
            "#,
            gw_consensus_tx_hash,
            gw_reference_id
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
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
