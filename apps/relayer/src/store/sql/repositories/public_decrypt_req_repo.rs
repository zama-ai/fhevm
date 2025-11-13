use anyhow::Result;
use serde_json::Value;
use sqlx::types::Uuid;

use crate::store::sql::{
    client::PgClient,
    models::public_decrypt_req_model::{PublicDecryptReq, PublicDecryptReqStatus},
};

pub struct PublicDecryptReqRepository {
    pool: PgClient,
}

impl PublicDecryptReqRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// (Medium frequency): Insert public decrypt request with internal Id (hash) and request.
    pub async fn insert_request(
        &self,
        ext_req_id: Uuid,
        internal_decryption_id: &str,
        req: Value,
    ) -> Result<PublicDecryptReq> {
        let req = sqlx::query_as!(
            PublicDecryptReq,
            r#"
            INSERT INTO public_decrypt_req (ext_req_id, internal_decryption_id, req)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                ext_req_id,
                internal_decryption_id,
                gw_decryption_id,
                req,
                res,
                req_status AS "req_status: _",
                tx_hash,
                err_reason,
                created_at,
                updated_at
            "#,
            ext_req_id,
            internal_decryption_id,
            req,
        )
        .fetch_one(&self.pool.get_pool())
        .await?;
        Ok(req)
    }

    /// (Medium frequency): Update request on successful transaction.
    pub async fn update_on_tx_success(
        &self,
        internal_decryption_id: &str,
        gw_decryption_id: i32,
        tx_hash: &str,
    ) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET gw_decryption_id = $1, tx_hash = $2, req_status = 'tx_sent'
            WHERE internal_decryption_id = $3
            "#,
            gw_decryption_id,
            tx_hash,
            internal_decryption_id
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }

    /// (Medium frequency): Update request on failed transaction.
    pub async fn update_on_tx_failure(
        &self,
        internal_decryption_id: &str,
        err_reason: &str,
    ) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET err_reason = $1, req_status = 'failure'
            WHERE internal_decryption_id = $2
            "#,
            err_reason,
            internal_decryption_id
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }

    /// (Medium/high frequency): Update by gateway Id with response jsonb and req_status to completed.
    pub async fn update_with_response(&self, gw_decryption_id: i32, res: Value) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE public_decrypt_req
            SET res = $1, req_status = 'completed'
            WHERE gw_decryption_id = $2
            "#,
            res,
            gw_decryption_id
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
    ) -> Result<Option<PublicDecryptReqStatus>> {
        let result = sqlx::query_as(
            r#"
            SELECT res, internal_decryption_id, req_status
            FROM public_decrypt_req
            WHERE ext_req_id = $1
            "#,
        )
        .bind(ext_req_id)
        .fetch_optional(&self.pool.get_pool())
        .await?;
        Ok(result)
    }

    /// (Internal transaction poller): Update req_status to in-flight for the oldest queued request and retrieve it.
    pub async fn fetch_and_mark_oldest_queued(&self) -> Result<Option<PublicDecryptReq>> {
        let req = sqlx::query_as!(
            PublicDecryptReq,
            r#"
            UPDATE public_decrypt_req
            SET req_status = 'in_flight'
            WHERE id = (
                SELECT id
                FROM public_decrypt_req
                WHERE req_status = 'queued'
                ORDER BY created_at ASC
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            RETURNING
                id,
                ext_req_id,
                internal_decryption_id,
                gw_decryption_id,
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
