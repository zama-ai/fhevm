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

    // update status to failure and err_reason, by 'int_request_id'
    /// Update req_status to 'failure' and set err_reason by int_request_id.
    /// Returns number of rows affected.
    pub async fn update_status_to_failure(
        &self,
        int_request_id: Uuid,
        err_reason: &str,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET 
                req_status = 'failure'::req_status,
                err_reason = $1
            WHERE int_request_id = $2
            "#,
            err_reason,
            int_request_id
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }

    // LISTENER

    // update by gateway_reference_id ->accepted = true res, req_status to 'completed' and gw_response_tx_hash, returns int_request_id
    /// Update res, req_status to 'completed', gw_response_tx_hash, and accepted status.
    /// Returns the int_request_id.
    pub async fn accept_and_complete_input_proof_req(
        &self,
        gw_reference_id: i32,
        res: serde_json::Value,
        gw_response_tx_hash: &str,
    ) -> Result<Uuid> {
        let result = sqlx::query_scalar!(
            r#"
            UPDATE input_proof_req
            SET 
                res = $1,
                req_status = 'completed'::req_status,
                gw_response_tx_hash = $2,
                accepted = true
            WHERE gw_reference_id = $3
            RETURNING int_request_id
            "#,
            res,
            gw_response_tx_hash,
            gw_reference_id
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // update accepted to false , req_status=completed, gw_response_tx_hash, and res, return int_request_id
    /// Update accepted to false, req_status to 'completed', set res and tx hash.
    /// Returns the int_request_id.
    pub async fn reject_and_complete_input_proof_req(
        &self,
        gw_reference_id: i32,
        res: serde_json::Value,
        gw_response_tx_hash: &str,
    ) -> Result<Uuid> {
        let result = sqlx::query_scalar!(
            r#"
            UPDATE input_proof_req
            SET 
                accepted = false,
                req_status = 'completed'::req_status,
                gw_response_tx_hash = $1,
                res = $2
            WHERE gw_reference_id = $3
            RETURNING int_request_id
            "#,
            gw_response_tx_hash,
            res,
            gw_reference_id
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }
}
