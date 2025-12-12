use sqlx::{Pool, Postgres, postgres::types::PgInterval};
use std::time::Duration;
use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

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

        unlock_public_decryption_requests(&db_pool, under_process_limit).await;
        unlock_public_decryption_responses(&db_pool, under_process_limit).await;
        unlock_user_decryption_requests(&db_pool, under_process_limit).await;
        unlock_user_decryption_responses(&db_pool, under_process_limit).await;

        tokio::time::sleep(period).await;
    }
}

async fn delete_completed_and_failed_public_decryption_requests(
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

async fn delete_completed_and_failed_user_decryption_requests(
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

async fn delete_completed_and_failed_public_decryption_responses(
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

async fn delete_completed_and_failed_user_decryption_responses(
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

async fn unlock_public_decryption_requests(
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

async fn unlock_user_decryption_requests(
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

async fn unlock_public_decryption_responses(
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

async fn unlock_user_decryption_responses(
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
