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

            let status_result = sqlx::query!(
                r#"
                SELECT req_status as "req_status!: ReqStatus"
                FROM public_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
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

            let status_result = sqlx::query!(
                r#"
                SELECT req_status as "req_status!: ReqStatus"
                FROM user_decrypt_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
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

            let status_result = sqlx::query!(
                r#"
                SELECT req_status as "req_status!: ReqStatus"
                FROM input_proof_req
                WHERE updated_at < NOW() - make_interval(secs => $1)
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
