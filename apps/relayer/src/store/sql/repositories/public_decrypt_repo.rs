use anyhow::Result;
use uuid::Uuid;

use crate::store::sql::client::PgClient;

pub struct PublicDecryptRepository {
    pool: PgClient,
}

impl PublicDecryptRepository {
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

    // INITIAL POST REQUEST:

    // Check if there is already existing internal_indexer_id and return ext_reference_id if there is one
    /// Check if there is already an existing internal_indexer_id.
    /// Returns the ext_reference_id (UUID) if found.
    pub async fn find_ext_ref_by_int_indexer_id(
        &self,
        int_indexer_id_bytes: &[u8],
    ) -> Result<Option<Uuid>> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT ext_reference_id
            FROM public_decrypt_req
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
            INSERT INTO public_decrypt_req (
                ext_reference_id,
                int_indexer_id,
                req,
                req_status,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, 'queued'::req_status, NOW(), NOW())
            ON CONFLICT (int_indexer_id) 
            DO UPDATE SET updated_at = NOW() -- Forces Postgres to return the ID even on conflict
            RETURNING ext_reference_id
            "#,
            ext_reference_id,
            int_indexer_id_bytes,
            req
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // GATEWAY READINESS CHECK.
    /// update public_decrypt_req by int_indexer_id for to req_status processing
    /// Update req_status to 'processing' by int_indexer_id.
    /// Returns the number of rows affected (1 if found, 0 if not).
    pub async fn update_status_to_processing(&self, int_indexer_id_bytes: &[u8]) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET req_status = 'processing'::req_status
            WHERE int_indexer_id = $1
            "#,
            int_indexer_id_bytes
        )
        .execute(&self.pool.get_pool())
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
            UPDATE public_decrypt_req
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
    /// Updating the req_status to receipt_received, gw_req_tx_hash, gw_reference_id by int_indexer_id
    /// Returns the number of rows affected (should be 1 or retry).
    pub async fn update_status_to_receipt_received_on_tx_success(
        &self,
        int_indexer_id_bytes: &[u8],
        gw_req_tx_hash: &str,
        gw_reference_id: i32,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
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
}
