use anyhow::Result;
use uuid::Uuid;

use crate::store::sql::client::PgClient;

pub struct InputProofRepository {
    pool: PgClient,
}

impl InputProofRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // insert ext_reference_id, int_request_id (uuidv7), req into input_proof_req table return ext_reference_id
    /// Insert ext_reference_id, int_request_id, req into input_proof_req table.
    /// Returns the ext_reference_id.
    pub async fn insert_new_input_proof(
        &self,
        ext_reference_id: Uuid,
        int_request_id: Uuid,
        req: serde_json::Value,
    ) -> Result<Uuid> {
        let result = sqlx::query_scalar!(
            r#"
            INSERT INTO input_proof_req (
                ext_reference_id, 
                int_request_id, 
                req, 
                req_status
            )
            VALUES ($1, $2, $3, 'queued'::req_status)
            RETURNING ext_reference_id
            "#,
            ext_reference_id,
            int_request_id,
            req
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // update the status to 'receipt_recieved' + gw_req_tx_hash + gw_reference_id by int_request_id
    /// Update req_status to 'receipt_received', set tx hash and gw_ref_id by int_request_id.
    /// Returns number of rows affected.
    pub async fn update_input_proof_status_to_receipt_received(
        &self,
        int_request_id: Uuid,
        gw_req_tx_hash: &str,
        gw_reference_id: i32,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET 
                req_status = 'receipt_received'::req_status,
                gw_req_tx_hash = $1,
                gw_reference_id = $2
            WHERE int_request_id = $3
            "#,
            gw_req_tx_hash,
            gw_reference_id,
            int_request_id
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }
}
