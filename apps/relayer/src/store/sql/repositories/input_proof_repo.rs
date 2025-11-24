use crate::core::event::{InputProofRequest, InputProofResponse};
use crate::store::sql::models::input_proof_req_model::InputProofResponseModel;
use crate::store::sql::models::req_status_enum_model::ReqStatus;
use crate::store::sql::{
    client::PgClient,
    error::{SqlError, SqlResult},
};
use alloy::primitives::U256;
use uuid::Uuid;

pub struct InputProofRepository {
    pool: PgClient,
}

impl InputProofRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // NOTE: We have a query which is performed at the database level in a pg_cron job instead of being called by the internals. and is trigged on this condition:
    // If status == 'receipt_recieved' and now - `updated_at` > 30 min roughly (TBD.)
    // Update status to timed_out with err_reason = 'response timed out' (ACL propagation error).
    // OR IN THE TIMEOUT REPO.

    // insert ext_reference_id, int_request_id (uuidv7), req into input_proof_req table return ext_reference_id
    // TODO: Ensure, rows affected was 1, else return an errror ? UUID is not needed in this case. But an error in case of no rows inserted.
    /// Insert ext_reference_id, int_request_id, req into input_proof_req table.
    /// Returns the ext_reference_id.
    pub async fn insert_new_input_proof(
        &self,
        ext_reference_id: Uuid,
        int_request_id: Uuid,
        request: InputProofRequest,
    ) -> SqlResult<Uuid> {
        let req = serde_json::to_value(&request).map_err(|e| {
            SqlError::conversion_error(
                "request",
                "InputProofRequest",
                format!("Failed to serialize: {}", e),
            )
        })?;
        let result = sqlx::query_scalar!(
            r#"
            INSERT INTO input_proof_req (
                ext_reference_id,
                int_request_id,
                req,
                req_status
            )
            VALUES ($1, $2, $3, 'processing'::req_status)
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
        gw_reference_id: U256,
    ) -> SqlResult<u64> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
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
            gw_ref_id,
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
    ) -> SqlResult<u64> {
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
        gw_reference_id: U256,
        response: InputProofResponse,
        gw_response_tx_hash: &str,
    ) -> SqlResult<Uuid> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let res = serde_json::to_value(&response).map_err(|e| {
            SqlError::conversion_error(
                "response",
                "InputProofResponse",
                format!("Failed to serialize: {}", e),
            )
        })?;
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
            gw_ref_id
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
        gw_reference_id: U256,
        rejection_reason: String,
        gw_response_tx_hash: &str,
    ) -> SqlResult<Uuid> {
        let id_as_bytes_array: [u8; 32] = gw_reference_id.to_be_bytes();
        let gw_ref_id = id_as_bytes_array.to_vec();
        let result = sqlx::query_scalar!(
            r#"
            UPDATE input_proof_req
            SET
                accepted = false,
                req_status = 'completed'::req_status,
                gw_response_tx_hash = $1,
                err_reason = $2
            WHERE gw_reference_id = $3
            RETURNING int_request_id
            "#,
            gw_response_tx_hash,
            rejection_reason,
            gw_ref_id
        )
        .fetch_one(&self.pool.get_pool())
        .await?;

        Ok(result)
    }

    // GET REQUEST.
    // select by ext_reference_id and return res, err_reason, accepted, updated_at
    /// Select status, res, err_reason, accepted, and updated_at by ext_reference_id.
    pub async fn find_status_by_ext_id(
        &self,
        ext_reference_id: Uuid,
    ) -> SqlResult<Option<InputProofResponseModel>> {
        let result = sqlx::query_as!(
            InputProofResponseModel,
            r#"
            SELECT
                req_status as "req_status!: ReqStatus",
                res,
                err_reason,
                accepted,
                updated_at
            FROM input_proof_req
            WHERE ext_reference_id = $1
            "#,
            ext_reference_id
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;

        Ok(result)
    }
}
