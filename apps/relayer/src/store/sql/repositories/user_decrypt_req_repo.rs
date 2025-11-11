use anyhow::Result;
use serde_json::Value;
use sqlx::types::Uuid;
use std::collections::HashMap;

use crate::store::sql::{
    client::PgClient,
    models::models::{UserDecryptReq, UserDecryptReqStatus},
};

pub struct UserDecryptReqRepository {
    pool: PgClient,
}

impl UserDecryptReqRepository {
    pub fn new(pool: PgClient) -> Self {
        Self { pool }
    }

    /// (Medium frequency): Insert user decrypt request with internal Id (hash) and request.
    pub async fn insert_request(
        &self,
        ext_req_id: Uuid,
        internal_decryption_id: &str,
        req: Value,
    ) -> Result<UserDecryptReq> {
        let req = sqlx::query_as!(
            UserDecryptReq,
            r#"
            INSERT INTO user_decrypt_req (ext_req_id, internal_decryption_id, req)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                ext_req_id,
                internal_decryption_id,
                gw_decryption_id,
                req,
                res,
                status AS "status: _",
                tx_hash,
                consensus_reached,
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
            UPDATE user_decrypt_req
            SET gw_decryption_id = $1, tx_hash = $2, status = 'tx_sent'
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
            UPDATE user_decrypt_req
            SET err_reason = $1, status = 'failure'
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

    /// (Medium/high frequency): Update by gateway Id with response jsonb and status to completed.
    pub async fn update_with_response(&self, gw_decryption_id: i32, res: Value) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET res = $1, status = 'completed'
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
    ) -> Result<Option<UserDecryptReqStatus>> {
        let result = sqlx::query_as(
            r#"
            SELECT res, internal_decryption_id, status
            FROM user_decrypt_req
            WHERE ext_req_id = $1
            "#,
        )
        .bind(ext_req_id)
        .fetch_optional(&self.pool.get_pool())
        .await?;
        Ok(result)
    }

    /// (Internal transaction poller): Update status to in_flight for the oldest queued request and retrieve it.
    pub async fn fetch_and_mark_oldest_queued(&self) -> Result<Option<UserDecryptReq>> {
        // This query atomically finds the oldest 'queued' entry, updates its status,
        // and returns it, preventing race conditions between multiple pollers.
        let req = sqlx::query_as!(
            UserDecryptReq,
            r#"
            UPDATE user_decrypt_req
            SET status = 'in_flight'
            WHERE id = (
                SELECT id
                FROM user_decrypt_req
                WHERE status = 'queued'
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
                status AS "status: _",
                tx_hash,
                consensus_reached,
                err_reason,
                created_at,
                updated_at
            "#
        )
        .fetch_optional(&self.pool.get_pool())
        .await?;
        Ok(req)
    }

    /// (At start): Query all tx_sent transactions to populate a consensus hashmap.
    pub async fn find_all_sent_tx_for_consensus_map(&self) -> Result<HashMap<i32, bool>> {
        let rows = sqlx::query!(
            r#"
            SELECT gw_decryption_id, consensus_reached
            FROM user_decrypt_req
            WHERE status = 'tx_sent' AND gw_decryption_id IS NOT NULL
            "#
        )
        .fetch_all(&self.pool.get_pool())
        .await?;

        let consensus_map = rows
            .into_iter()
            .map(|row| (row.gw_decryption_id.unwrap(), row.consensus_reached))
            .collect();

        Ok(consensus_map)
    }

    /// (Medium update): Update the consensus_reached field by gateway id.
    pub async fn update_consensus_reached(
        &self,
        gw_decryption_id: i32,
        consensus: bool,
    ) -> Result<u64> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE user_decrypt_req
            SET consensus_reached = $1
            WHERE gw_decryption_id = $2
            "#,
            consensus,
            gw_decryption_id,
        )
        .execute(&self.pool.get_pool())
        .await?
        .rows_affected();
        Ok(rows_affected)
    }
}
