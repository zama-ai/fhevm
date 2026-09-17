use fhevm_engine_common::drift_containment::{
    acquire_ct_computation_permit, block_ct_computations, DRIFT_CONTAINMENT_BARRIER,
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use test_harness::instance::{setup_test_db, ImportMode};

#[tokio::test]
async fn batches_share_the_barrier_until_commit_or_rollback(
) -> Result<(), Box<dyn std::error::Error>> {
    let db = setup_test_db(ImportMode::None).await?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(db.db_url())
        .await?;
    let mut first = pool.begin().await?;
    let mut second = pool.begin().await?;
    acquire_ct_computation_permit(&mut first).await?;
    tokio::time::timeout(
        Duration::from_secs(1),
        acquire_ct_computation_permit(&mut second),
    )
    .await??;
    sqlx::query("CREATE TABLE containment_commits (id INT)")
        .execute(first.as_mut())
        .await?;
    first.commit().await?;
    let mut exclusive = pool.begin().await?;
    let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .fetch_one(exclusive.as_mut())
        .await?;
    assert!(!acquired, "the second batch still holds its shared lock");
    second.rollback().await?;
    block_ct_computations(&mut exclusive).await?;
    let exists: bool = sqlx::query_scalar("SELECT to_regclass('containment_commits') IS NOT NULL")
        .fetch_one(exclusive.as_mut())
        .await?;
    assert!(exists, "the guaranteed pass sees committed batch writes");
    let mut next = pool.begin().await?;
    assert!(tokio::time::timeout(
        Duration::from_millis(100),
        acquire_ct_computation_permit(&mut next)
    )
    .await
    .is_err());
    exclusive.commit().await?;
    tokio::time::timeout(
        Duration::from_secs(1),
        acquire_ct_computation_permit(&mut next),
    )
    .await??;
    next.rollback().await?;
    Ok(())
}
