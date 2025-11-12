use anyhow::Result;
use serde_json::Value;
use sqlx::types::Uuid;

use crate::store::sql::{
    client::PgClient,
    models::models::{InputProofReq, InputProofReqStatus},
};

pub struct InputProofReqRepository {
    pool: PgClient,
}

impl InputProofReqRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// (Medium frequency): Insert input proof request with internal Id and request.
    pub async fn insert_request(
        &self,
        ext_req_id: Uuid,
        internal_input_proof_id: Uuid,
        req: Value,
    ) -> Result<InputProofReq> {
        let req = sqlx::query_as!(
            InputProofReq,
            r#"
            INSERT INTO input_proof_req (ext_req_id, internal_input_proof_id, req)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                ext_req_id,
                internal_input_proof_id,
                gw_input_proof_id,
                req,
                res,
                req_status AS "req_status: _",
                tx_hash,
                err_reason,
                created_at,
                updated_at
            "#,
            ext_req_id,
            internal_input_proof_id,
            req,
        )
        .fetch_one(&self.pool.get_pool())
        .await?;
        Ok(req)
    }

    /// (Medium frequency): Update request on successful transaction.
    pub async fn update_on_tx_success(
        &self,
        internal_input_proof_id: Uuid,
        gw_input_proof_id: i32,
        tx_hash: &str,
    ) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET gw_input_proof_id = $1, tx_hash = $2, req_status = 'tx_sent'
            WHERE internal_input_proof_id = $3
            "#,
            gw_input_proof_id,
            tx_hash,
            internal_input_proof_id
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }

    /// (Medium frequency): Update request on failed transaction.
    pub async fn update_on_tx_failure(
        &self,
        internal_input_proof_id: Uuid,
        err_reason: &str,
    ) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET err_reason = $1, req_status = 'failure'
            WHERE internal_input_proof_id = $2
            "#,
            err_reason,
            internal_input_proof_id
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }

    /// (Medium/high frequency): Update by gateway Id with response jsonb and status to completed.
    pub async fn update_with_response(&self, gw_input_proof_id: i32, res: Value) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE input_proof_req
            SET res = $1, req_status = 'completed'
            WHERE gw_input_proof_id = $2
            "#,
            res,
            gw_input_proof_id
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }

    /// (High frequency): Select by external Id to get the response, internal id, and status.
    pub async fn find_status_by_ext_req_id(
        &self,
        ext_req_id: Uuid,
    ) -> Result<Option<InputProofReqStatus>> {
        let result = sqlx::query_as(
            r#"
            SELECT res, internal_input_proof_id, req_status
            FROM input_proof_req
            WHERE ext_req_id = $1
            "#,
        )
        .bind(ext_req_id)
        .fetch_optional(&self.pool.get_pool())
        .await?;
        Ok(result)
    }

    /// (Internal transaction poller): Update req_status to in-flight for the oldest queued request and retrieve it.
    pub async fn fetch_and_mark_oldest_queued(&self) -> Result<Option<InputProofReq>> {
        let req = sqlx::query_as!(
            InputProofReq,
            r#"
            UPDATE input_proof_req
            SET req_status = 'in_flight'
            WHERE id = (
                SELECT id
                FROM input_proof_req
                WHERE req_status = 'queued'
                ORDER BY created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING
                id,
                ext_req_id,
                internal_input_proof_id,
                gw_input_proof_id,
                req,
                res,
                req_status AS "req_status: _",
                tx_hash,
                err_reason,
                created_at,
                updated_at
            "#
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;
        Ok(req)
    }
}
