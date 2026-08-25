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

            // Terminal statuses only (`Completed`, `TimedOut`, `Failure`): no repository query
            // anywhere writes `req_status` starting from one of these, so a row in one of them
            // is done for good and safe to purge by age alone. A row still `Queued`,
            // `Processing`, `TxInFlight`, or `ReceiptReceived` can still receive a status write,
            // and an idle `updated_at` on one of those means it is stuck, not finished --
            // deleting it would silently drop a request instead of surfacing the stall (that is
            // what the always-on timeout worker is for on `ReceiptReceived`; the other three
            // have no staleness detector at all yet). Keep this predicate identical to the
            // DELETE's below, or the gauges the loop below decrements will drift from the rows
            // actually removed.
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

            let query_start = Instant::now();
            let delete_result = sqlx::query!(
                r#"
                DELETE FROM user_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
                  AND req_status IN ('completed'::req_status, 'timed_out'::req_status, 'failure'::req_status)
                "#,
                user_decrypt_expiry_secs
            )
            .execute(&mut *conn)
            .await;

            match &delete_result {
                Ok(_) => {
                    metrics::observe_query(metrics::Table::UserDecryptReq, query_start.elapsed())
                }
                Err(_) => metrics::increment_error(metrics::Table::UserDecryptReq),
            }

            let total_deleted_rows = delete_result?.rows_affected();
            total_deleted += total_deleted_rows;

            for row in statuses {
                metrics::decrement_req_status_count(
                    metrics::RequestType::UserDecrypt,
                    row.req_status,
                );
            }

            info!(
                table = metrics::Table::UserDecryptReq.as_str(),
                deleted_rows = total_deleted_rows,
                "Expiry repo successfully cleaned up rows"
            );
        }

        // User Decrypt Shares
        //
        // No status predicate here, unlike the three request tables above: `user_decrypt_share`
        // has no `req_status` column at all (see the `CREATE TABLE` in
        // `20251109145104_create_tables.sql`) and no lifecycle of its own -- it is evidence
        // accumulating against a `user_decrypt_req` row (one share per KMS node, deduplicated on
        // `(gw_reference_id, share_index)`), not a request that a later write could still land
        // on. Purging it by age alone is correct: a stale share is either already folded into a
        // completed request or belongs to one that will never reach threshold, and either way
        // nothing downstream reads it again.
        {
            let mut conn = self.pool.get_cron_connection().await?;
            let query_start = Instant::now();

            let result = sqlx::query!(
                r#"
                DELETE FROM user_decrypt_share
                WHERE updated_at < NOW() - make_interval(secs => $1)
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
                "Expiry repo successfully cleaned up rows"
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
