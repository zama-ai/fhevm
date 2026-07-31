//! DB-backed coverage for `branch::read_settled_height_updated_epoch`: the
//! reader that feeds the settlement-frontier-stall gauge. Pure-logic
//! coverage of `advance_settled_height`'s clamping helper lives directly in
//! `fhevm_engine_common::branch`'s unit tests (`next_settled_height`).

use fhevm_engine_common::branch::read_settled_height_updated_epoch;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::{setup_test_db, ImportMode};

const CHAIN_ID: i64 = 777;

/// `setup_test_db` only resets/migrates a fresh DB under
/// `COPROCESSOR_TEST_LOCALHOST_RESET`; plain `COPROCESSOR_TEST_LOCALHOST`
/// (the common local-dev invocation) reuses whatever's already at
/// `DATABASE_URL`. Delete our own fixture rows up front so each test is
/// idempotent across repeated local runs, mirroring the TRUNCATE-on-setup
/// pattern `tfhe-worker`'s `dependence_chain` tests use for the same reason.
async fn cleanup(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM coprocessor_settlement WHERE chain_id = ANY($1)")
        .bind(vec![CHAIN_ID, CHAIN_ID + 1])
        .execute(pool)
        .await
        .expect("cleanup coprocessor_settlement fixture rows");
}

#[tokio::test]
#[serial(db)]
async fn no_settlement_row_returns_none() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db.db_url())
        .await
        .expect("connect pool");
    cleanup(&pool).await;

    let mut tx = pool.begin().await.expect("begin tx");
    let epoch = read_settled_height_updated_epoch(&mut tx, CHAIN_ID)
        .await
        .expect("query should succeed");
    tx.rollback().await.expect("rollback");

    assert_eq!(epoch, None, "no settlement row yet -> None");
}

#[tokio::test]
#[serial(db)]
async fn settlement_row_returns_a_recent_epoch() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db.db_url())
        .await
        .expect("connect pool");
    cleanup(&pool).await;

    let before = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO coprocessor_settlement (chain_id, settled_height, updated_at)
         VALUES ($1, $2, CURRENT_TIMESTAMP)",
    )
    .bind(CHAIN_ID)
    .bind(42_i64)
    .execute(&pool)
    .await
    .expect("insert settlement row");

    let after = chrono::Utc::now().timestamp();

    let mut tx = pool.begin().await.expect("begin tx");
    let epoch = read_settled_height_updated_epoch(&mut tx, CHAIN_ID)
        .await
        .expect("query should succeed");
    tx.rollback().await.expect("rollback");

    let epoch = epoch.expect("a settlement row exists -> Some(epoch)");
    assert!(
        (before - 2..=after + 2).contains(&epoch),
        "epoch {epoch} should be within a couple seconds of [{before}, {after}]"
    );
}

#[tokio::test]
#[serial(db)]
async fn settlement_row_for_a_different_chain_is_not_returned() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db.db_url())
        .await
        .expect("connect pool");
    cleanup(&pool).await;

    sqlx::query(
        "INSERT INTO coprocessor_settlement (chain_id, settled_height, updated_at)
         VALUES ($1, $2, CURRENT_TIMESTAMP)",
    )
    .bind(CHAIN_ID + 1)
    .bind(1_i64)
    .execute(&pool)
    .await
    .expect("insert settlement row");

    let mut tx = pool.begin().await.expect("begin tx");
    let epoch = read_settled_height_updated_epoch(&mut tx, CHAIN_ID)
        .await
        .expect("query should succeed");
    tx.rollback().await.expect("rollback");

    assert_eq!(epoch, None);
}
