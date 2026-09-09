use std::time::Instant;

use crate::{
    config::settings::CronConfig,
    metrics,
    store::sql::{client::PgClient, models::req_status_enum_model::ReqStatus},
};
use anyhow::Result;
use tracing::info;

pub struct ExpiryRepository {
    pool: PgClient,
    cron_config: CronConfig,
}

impl ExpiryRepository {
    pub fn new(pool: PgClient, cron_config: CronConfig) -> Self {
        Self { pool, cron_config }
    }

    pub async fn purge_stale_data(&self) -> Result<u64> {
        let mut total_deleted = 0;
        let public_decrypt_expiry_secs = self.cron_config.public_decrypt_expiry.as_secs_f64();
        let user_decrypt_expiry_secs = self.cron_config.user_decrypt_expiry.as_secs_f64();
        let input_proof_expiry_secs = self.cron_config.input_proof_expiry.as_secs_f64();

        // Public Decrypt Requests
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            // Terminal statuses only: no query writes `req_status` starting from one of these,
            // so such a row is finished and safe to purge by age. An idle `updated_at` on any
            // other status means stuck, not finished, and deleting it would hide the stall.
            // Keep this predicate identical to the DELETE below, or the gauges drift from the
            // rows actually removed.
            let status_result = sqlx::query!(
                r#"
                SELECT req_status as "req_status!: ReqStatus"
                FROM public_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                  AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                "#,
                public_decrypt_expiry_secs
            )
            .fetch_all(&mut *conn)
            .await;

            match &status_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
            }

            let statuses = status_result?;

            let query_start = Instant::now();
            let delete_result = sqlx::query!(
                r#"
                DELETE FROM public_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                  AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                "#,
                public_decrypt_expiry_secs
            )
            .execute(&mut *conn)
            .await;

            match &delete_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::PublicDecryptReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::PublicDecryptReq),
            }

            let total_deleted_rows = delete_result?.rows_affected();
            total_deleted += total_deleted_rows;

            for row in statuses {
                metrics::decrement_req_status_count(
                    metrics::RequestType::PublicDecrypt,
                    row.req_status,
                );
            }

            info!(
                table = metrics::Table::PublicDecryptReq.as_str(),
                deleted_rows = total_deleted_rows,
                "Expiry repo successfully cleaned up rows"
            );
        }

        // User Decrypt Requests
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            // Terminal statuses only -- see the comment on the public-decrypt block above.
            let status_result = sqlx::query!(
                r#"
                SELECT req_status as "req_status!: ReqStatus"
                FROM user_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                  AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                "#,
                user_decrypt_expiry_secs
            )
            .fetch_all(&mut *conn)
            .await;

            match &status_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
            }

            let statuses = status_result?;

            // Shares go with the request they belong to, in one statement. Completion counts
            // them with `COUNT(*) == threshold`, so expiring shares on their own age would
            // strand a live request below a threshold it can then never reach. `ON DELETE
            // CASCADE` would say this in the schema, but `gw_reference_id` is nullable and not
            // unique, so it cannot be a foreign key target yet.
            let query_start = Instant::now();
            let delete_result = sqlx::query!(
                r#"
                WITH deleted_reqs AS (
                    DELETE FROM user_decrypt_req
                    WHERE updated_at < NOW() - make_interval(secs => $1)
                      AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                    RETURNING gw_reference_id
                ),
                deleted_shares AS (
                    DELETE FROM user_decrypt_share
                    WHERE gw_reference_id IN (
                        SELECT gw_reference_id FROM deleted_reqs WHERE gw_reference_id IS NOT NULL
                    )
                    RETURNING 1 AS deleted
                )
                SELECT
                    (SELECT COUNT(*) FROM deleted_reqs) as "reqs!",
                    (SELECT COUNT(*) FROM deleted_shares) as "shares!"
                "#,
                user_decrypt_expiry_secs
            )
            .fetch_one(&mut *conn)
            .await;

            match &delete_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed());
                    metrics::observe_query(
                        metrics::Table::UserDecryptShares,
                        query_start.elapsed(),
                    );
                }
                Err(_) => {
                    metrics::increment_error(metrics::Table::UserDecryptReq);
                    metrics::increment_error(metrics::Table::UserDecryptShares);
                }
            }

            let deleted = delete_result?;
            let deleted_reqs = deleted.reqs as u64;
            let deleted_shares = deleted.shares as u64;
            total_deleted += deleted_reqs + deleted_shares;

            for row in statuses {
                metrics::decrement_req_status_count(
                    metrics::RequestType::UserDecrypt,
                    row.req_status,
                );
            }

            info!(
                table = metrics::Table::UserDecryptReq.as_str(),
                deleted_rows = deleted_reqs,
                deleted_share_rows = deleted_shares,
                "Expiry repo successfully cleaned up rows"
            );
        }

        // Orphaned User Decrypt Shares
        //
        // Correlated deletion only reaches shares whose request still exists, so without this
        // the table stops self-clearing: every share left behind by the age-only expiry this
        // commit replaces, and every share inserted for a `gw_reference_id` with no request row
        // (the insert checks for none), would survive forever.
        //
        // The age guard covers the window where a share is recorded before its request carries
        // the `gw_reference_id` -- a bare anti-join would delete live evidence there.
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            let result = sqlx::query!(
                r#"
                DELETE FROM user_decrypt_share s
                WHERE s.updated_at < NOW() - make_interval(secs => $1)
                  AND NOT EXISTS (
                      SELECT 1 FROM user_decrypt_req r
                      WHERE r.gw_reference_id = s.gw_reference_id
                  )
                "#,
                user_decrypt_expiry_secs
            )
            .execute(&mut *conn)
            .await;

            match &result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::UserDecryptShares, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::UserDecryptShares),
            }

            let total_deleted_rows = result?.rows_affected();
            total_deleted += total_deleted_rows;

            info!(
                table = metrics::Table::UserDecryptShares.as_str(),
                deleted_rows = total_deleted_rows,
                "Expiry repo cleaned up orphaned shares"
            );
        }

        // Input Proof Requests
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            // Terminal statuses only -- see the comment on the public-decrypt block above.
            let status_result = sqlx::query!(
                r#"
                SELECT req_status as "req_status!: ReqStatus"
                FROM input_proof_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                  AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                "#,
                input_proof_expiry_secs
            )
            .fetch_all(&mut *conn)
            .await;

            match &status_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
            }

            let statuses = status_result?;

            let query_start = Instant::now();
            let delete_result = sqlx::query!(
                r#"
                DELETE FROM input_proof_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                  AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                "#,
                input_proof_expiry_secs
            )
            .execute(&mut *conn)
            .await;

            match &delete_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::InputProofReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::InputProofReq),
            }

            let total_deleted_rows = delete_result?.rows_affected();
            total_deleted += total_deleted_rows;

            for row in statuses {
                metrics::decrement_req_status_count(
                    metrics::RequestType::InputProof,
                    row.req_status,
                );
            }

            info!(
                table = metrics::Table::InputProofReq.as_str(),
                deleted_rows = total_deleted_rows,
                "Expiry repo successfully cleaned up rows"
            );
        }

        Ok(total_deleted)
    }
}
