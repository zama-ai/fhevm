use sqlx::{Pool, Postgres, postgres::types::PgInterval};
use std::time::Duration;
use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

const DELETE_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL: &str = "
            DELETE FROM solana_native_decryption_requests_v0 AS req
            WHERE status IN ('completed', 'failed')
            AND NOW() - updated_at > $1
            AND NOT EXISTS (
                SELECT 1
                FROM solana_native_decryption_responses_v0 AS resp
                WHERE resp.request_hash = req.request_hash
            )
        ";

const DELETE_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL: &str = "
            DELETE FROM solana_native_decryption_responses_v0
            WHERE status IN ('completed', 'failed')
            AND NOW() - updated_at > $1
        ";

const UNLOCK_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL: &str = "
            UPDATE solana_native_decryption_requests_v0 SET status = 'pending'
            WHERE status = 'under_process' AND NOW() - updated_at > $1
        ";

const UNLOCK_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL: &str = "
            UPDATE solana_native_decryption_responses_v0 SET status = 'pending'
            WHERE status = 'under_process' AND NOW() - updated_at > $1
        ";

pub fn spawn_garbage_collection_routine(
    period: Duration,
    decryption_expiry: PgInterval,
    under_process_limit: PgInterval,
    db_pool: Pool<Postgres>,
    cancel_token: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        select! {
            _ = run_garbage_collection_routine(period, decryption_expiry, under_process_limit, db_pool) => {}
            _ = cancel_token.cancelled() => {}
        }
    })
}

async fn run_garbage_collection_routine(
    period: Duration,
    decryption_expiry: PgInterval,
    under_process_limit: PgInterval,
    db_pool: Pool<Postgres>,
) {
    loop {
        delete_completed_and_failed_public_decryption_requests(&db_pool, decryption_expiry).await;
        delete_completed_and_failed_public_decryption_responses(&db_pool, decryption_expiry).await;
        delete_completed_and_failed_user_decryption_requests(&db_pool, decryption_expiry).await;
        delete_completed_and_failed_user_decryption_responses(&db_pool, decryption_expiry).await;
        delete_completed_and_failed_solana_native_decryption_responses(&db_pool, decryption_expiry)
            .await;
        delete_completed_and_failed_solana_native_decryption_requests(&db_pool, decryption_expiry)
            .await;

        unlock_public_decryption_requests(&db_pool, under_process_limit).await;
        unlock_public_decryption_responses(&db_pool, under_process_limit).await;
        unlock_user_decryption_requests(&db_pool, under_process_limit).await;
        unlock_user_decryption_responses(&db_pool, under_process_limit).await;
        unlock_solana_native_decryption_requests(&db_pool, under_process_limit).await;
        unlock_solana_native_decryption_responses(&db_pool, under_process_limit).await;

        tokio::time::sleep(period).await;
    }
}

pub async fn delete_completed_and_failed_public_decryption_requests(
    db_pool: &Pool<Postgres>,
    expiry: PgInterval,
) {
    match sqlx::query!(
        "
            DELETE FROM public_decryption_requests
            WHERE status IN ('completed', 'failed')
            AND NOW() - updated_at > $1
        ",
        expiry
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully deleted {} public decryption requests",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to delete completed/failed public decryption requests: {e}"),
    }
}

pub async fn delete_completed_and_failed_user_decryption_requests(
    db_pool: &Pool<Postgres>,
    expiry: PgInterval,
) {
    match sqlx::query!(
        "
            DELETE FROM user_decryption_requests
            WHERE status IN ('completed', 'failed')
            AND NOW() - updated_at > $1
        ",
        expiry
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully deleted {} user decryption requests",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to delete completed/failed user decryption requests: {e}"),
    }
}

pub async fn delete_completed_and_failed_public_decryption_responses(
    db_pool: &Pool<Postgres>,
    expiry: PgInterval,
) {
    match sqlx::query!(
        "
            DELETE FROM public_decryption_responses
            WHERE status IN ('completed', 'failed')
            AND NOW() - updated_at > $1
        ",
        expiry
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully deleted {} public decryption responses",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to delete completed/failed public decryption responses: {e}"),
    }
}

pub async fn delete_completed_and_failed_user_decryption_responses(
    db_pool: &Pool<Postgres>,
    expiry: PgInterval,
) {
    match sqlx::query!(
        "
            DELETE FROM user_decryption_responses
            WHERE status IN ('completed', 'failed')
            AND NOW() - updated_at > $1
        ",
        expiry
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully deleted {} user decryption responses",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to delete completed/failed user decryption responses: {e}"),
    }
}

pub async fn delete_completed_and_failed_solana_native_decryption_requests(
    db_pool: &Pool<Postgres>,
    expiry: PgInterval,
) {
    match sqlx::query(DELETE_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL)
        .bind(expiry)
        .execute(db_pool)
        .await
    {
        Ok(result) => info!(
            "Successfully deleted {} native Solana decryption requests",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to delete completed/failed native Solana requests: {e}"),
    }
}

pub async fn delete_completed_and_failed_solana_native_decryption_responses(
    db_pool: &Pool<Postgres>,
    expiry: PgInterval,
) {
    match sqlx::query(DELETE_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL)
        .bind(expiry)
        .execute(db_pool)
        .await
    {
        Ok(result) => info!(
            "Successfully deleted {} native Solana decryption responses",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to delete completed/failed native Solana responses: {e}"),
    }
}

pub async fn unlock_public_decryption_requests(
    db_pool: &Pool<Postgres>,
    under_process_limit: PgInterval,
) {
    match sqlx::query!(
        "
            UPDATE public_decryption_requests SET status = 'pending'
            WHERE status = 'under_process' AND NOW() - updated_at > $1
        ",
        under_process_limit
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully unlocked {} public decryption requests",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to unlock public decryption requests: {e}"),
    }
}

pub async fn unlock_solana_native_decryption_requests(
    db_pool: &Pool<Postgres>,
    under_process_limit: PgInterval,
) {
    match sqlx::query(UNLOCK_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL)
        .bind(under_process_limit)
        .execute(db_pool)
        .await
    {
        Ok(result) => info!(
            "Successfully unlocked {} native Solana decryption requests",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to unlock native Solana decryption requests: {e}"),
    }
}

pub async fn unlock_solana_native_decryption_responses(
    db_pool: &Pool<Postgres>,
    under_process_limit: PgInterval,
) {
    match sqlx::query(UNLOCK_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL)
        .bind(under_process_limit)
        .execute(db_pool)
        .await
    {
        Ok(result) => info!(
            "Successfully unlocked {} native Solana decryption responses",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to unlock native Solana decryption responses: {e}"),
    }
}

pub async fn unlock_user_decryption_requests(
    db_pool: &Pool<Postgres>,
    under_process_limit: PgInterval,
) {
    match sqlx::query!(
        "
            UPDATE user_decryption_requests SET status = 'pending'
            WHERE status = 'under_process' AND NOW() - updated_at > $1
        ",
        under_process_limit
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully unlocked {} user decryption requests",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to unlock user decryption requests: {e}"),
    }
}

pub async fn unlock_public_decryption_responses(
    db_pool: &Pool<Postgres>,
    under_process_limit: PgInterval,
) {
    match sqlx::query!(
        "
            UPDATE public_decryption_responses SET status = 'pending'
            WHERE status = 'under_process' AND NOW() - updated_at > $1
        ",
        under_process_limit
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully unlocked {} public decryption responses",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to unlock public decryption responses: {e}"),
    }
}

pub async fn unlock_user_decryption_responses(
    db_pool: &Pool<Postgres>,
    under_process_limit: PgInterval,
) {
    match sqlx::query!(
        "
            UPDATE user_decryption_responses SET status = 'pending'
            WHERE status = 'under_process' AND NOW() - updated_at > $1
        ",
        under_process_limit
    )
    .execute(db_pool)
    .await
    {
        Ok(result) => info!(
            "Successfully unlocked {} user decryption responses",
            result.rows_affected()
        ),
        Err(e) => error!("Failed to unlock user decryption responses: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_gc_request_delete_preserves_response_foreign_key() {
        assert!(
            DELETE_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL
                .contains("DELETE FROM solana_native_decryption_requests_v0 AS req")
        );
        assert!(
            DELETE_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL
                .contains("FROM solana_native_decryption_responses_v0 AS resp")
        );
        assert!(
            DELETE_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL
                .contains("WHERE resp.request_hash = req.request_hash")
        );
        assert!(DELETE_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL.contains("AND NOT EXISTS"));
    }

    #[test]
    fn native_gc_covers_response_delete_and_under_process_unlocks() {
        assert!(
            DELETE_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL
                .contains("DELETE FROM solana_native_decryption_responses_v0")
        );
        assert!(
            UNLOCK_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL
                .contains("UPDATE solana_native_decryption_requests_v0 SET status = 'pending'")
        );
        assert!(
            UNLOCK_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL
                .contains("UPDATE solana_native_decryption_responses_v0 SET status = 'pending'")
        );
        assert!(UNLOCK_SOLANA_NATIVE_DECRYPTION_REQUESTS_SQL.contains("status = 'under_process'"));
        assert!(UNLOCK_SOLANA_NATIVE_DECRYPTION_RESPONSES_SQL.contains("status = 'under_process'"));
    }
}
