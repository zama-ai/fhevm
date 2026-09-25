use super::*;
use sqlx::PgPool;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

pub(super) async fn setup() -> (DBInstance, PgPool) {
    let db = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(db.db_url()).await.unwrap();
    (db, pool)
}

pub(super) fn handle(n: u8) -> Vec<u8> {
    let mut h = vec![n; 32];
    h[30] = 4; // FheUint64
    h
}

pub(super) async fn drift(pool: &PgPool, n: u8) {
    sqlx::query(
        "INSERT INTO drifted_handle (consensus_epoch, coprocessor_context_id,
        host_chain_id, block_number, block_hash, handle, detection_kind, reason,
        local_present, quorum_present)
        SELECT consensus_epoch, $1, 1, 1, $1, $1, 'inferred', 'ct64_mismatch', FALSE, FALSE
        FROM blue_green_consensus_epoch",
    )
    .bind(handle(n))
    .execute(pool)
    .await
    .unwrap();
}

/// A ct64 finding recorded by another stack's epoch.
pub(super) async fn drift_in_other_epoch(pool: &PgPool, n: u8) {
    drift(pool, n).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE handle = $1")
        .bind(handle(n))
        .execute(pool)
        .await
        .unwrap();
}

/// This stack's own stored copy of a handle.
pub(super) async fn stored_ciphertext(pool: &PgPool, n: u8) {
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 0)",
    )
    .bind(handle(n))
    .bind(vec![n; 4])
    .execute(pool)
    .await
    .unwrap();
}

pub(super) async fn computation(
    pool: &PgPool,
    out: u8,
    input: u8,
    tx: u8,
    boundary: bool,
    dcid: u8,
) {
    let mut mask = vec![0; 32];
    mask[31] = u8::from(boundary);
    sqlx::query(
        "INSERT INTO computations (output_handle, dependencies, fhe_operation,
        is_scalar, transaction_id, is_allowed, schedule_order, host_chain_id,
        operand_boundary_mask, dependence_chain_id)
        VALUES ($1, ARRAY[$2]::bytea[], $3, FALSE, $4, TRUE,
                NOW() + make_interval(secs => $5), 1, $6, $7)",
    )
    .bind(handle(out))
    .bind(handle(input))
    .bind(SupportedFheOperations::FheNot as i16)
    .bind(handle(tx))
    .bind(f64::from(out))
    .bind(mask)
    .bind(handle(dcid))
    .execute(pool)
    .await
    .unwrap();
}
