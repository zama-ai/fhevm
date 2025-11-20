use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;

use crate::store::sql::client::PgClient;

pub struct InputProofRepository {
    pool: PgClient,
}

impl InputProofRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    // insert ext_reference_id, int_request_id (uuidv7), req into input_proof_req table
    /// Insert a new Input Proof request.
    /// Returns the number of rows affected (should be 1).
    pub async fn insert_new_req(
        &self,
        ext_reference_id: Uuid,
        int_request_id: Uuid,
        req: Value,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO input_proof_req (
                ext_reference_id, 
                int_request_id, 
                req, 
                req_status
            )
            VALUES ($1, $2, $3, 'queued'::req_status)
            "#,
            ext_reference_id,
            int_request_id,
            req
        )
        .execute(&self.pool.get_pool())
        .await?;

        Ok(result.rows_affected())
    }
}
