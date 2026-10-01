use super::*;
use crate::manifest_consensus::containment::{
    enforce_guaranteed_containment, DRIFT_CONTAINMENT_BARRIER,
};

/// The local operator is outside a two-peer quorum, so its ct64 is a local drift.
async fn schedule_drift(pool: &PgPool) -> FakePeerSource {
    let signers = test_signers();
    seed_registry(pool, &signers, 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(pool, &local, 0).await;
    let source = FakePeerSource::default();
    for peer in &signers[1..] {
        source.set_manifest(
            peer.address(),
            &sign_payload(peer, payload(peer.address(), 9)).await,
        );
    }
    source
}

async fn add_root_producer(pool: &PgPool) {
    sqlx::query("INSERT INTO handle_producer_block (host_chain_id, handle, producer_block_number, producer_block_hash) VALUES ($1, $2, $3, $4)")
        .bind(TEST_CHAIN_ID).bind(B256::repeat_byte(1).as_slice()).bind(TEST_BLOCK_NUMBER)
        .bind(test_block_hash().as_slice()).execute(pool).await.unwrap();
}

#[tokio::test]
#[serial(db)]
async fn verification_returns_while_detached_containment_waits_and_marks_descendants() {
    let (_instance, pool) = setup_download_db().await;
    let source = schedule_drift(&pool).await;
    add_root_producer(&pool).await;
    let child = B256::repeat_byte(2);
    sqlx::query("INSERT INTO handle_producer_block (host_chain_id, handle, producer_block_number, producer_block_hash) VALUES ($1, $2, 43, $3)")
        .bind(TEST_CHAIN_ID).bind(child.as_slice()).bind(B256::repeat_byte(0xab).as_slice()).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number, is_completed, is_allowed) VALUES ($1, ARRAY[$2, $3]::bytea[], 0, TRUE, $1, $4, 43, TRUE, TRUE)")
        .bind(child.as_slice()).bind(B256::repeat_byte(1).as_slice()).bind(B256::ZERO.as_slice()).bind(TEST_CHAIN_ID).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) VALUES ($1, $2, 0, 0)")
        .bind(child.as_slice()).bind(vec![1u8]).execute(&pool).await.unwrap();
    let mut batch = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(batch.as_mut())
        .await
        .unwrap();
    let task_pool = pool.clone();
    let verification = tokio::spawn(async move {
        run_peer_manifest_download_once(
            &task_pool,
            &source,
            "reactive",
            Duration::from_secs(60),
            CONSENSUS_EPOCH,
        )
        .await
    });
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM drifted_handle")
                .fetch_one(&pool)
                .await
                .unwrap();
            if count == 2 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("optimistic findings must commit before waiting for the barrier");
    let attempt: i32 =
        sqlx::query_scalar("SELECT attempt_count FROM block_manifest_verification_task")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(attempt, 1);
    let result = tokio::time::timeout(Duration::from_secs(1), verification)
        .await
        .unwrap()
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(result.outcome, VerificationOutcome::Drift);
    let contained: bool = sqlx::query_scalar("SELECT BOOL_OR(is_contained) FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!contained);
    batch.commit().await.unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let pending: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM drifted_handle WHERE NOT is_contained")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            if pending == 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let findings: Vec<(String, bool)> =
        sqlx::query_as("SELECT detection_kind, is_contained FROM drifted_handle ORDER BY handle")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        findings,
        vec![("verified".into(), true), ("inferred".into(), true)]
    );
}

#[tokio::test]
#[serial(db)]
async fn containment_failure_preserves_verification_and_remains_recoverable() {
    let (_instance, pool) = setup_download_db().await;
    let source = schedule_drift(&pool).await;
    // Missing producer metadata makes propagation fail after verification commits.
    let result = run_peer_manifest_download_once(
        &pool,
        &source,
        "reactive-failure",
        Duration::from_secs(60),
        CONSENSUS_EPOCH,
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(result.outcome, VerificationOutcome::Drift);
    let state: (i32, String) =
        sqlx::query_as("SELECT attempt_count, state FROM block_manifest_verification_task")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(state, (1, "verified".into()));
    let contained: bool = sqlx::query_scalar("SELECT is_contained FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!contained);
    let child = B256::repeat_byte(2);
    sqlx::query("INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number, is_completed, is_allowed) VALUES ($1, ARRAY[$2, $3]::bytea[], 0, TRUE, $1, $4, 43, TRUE, TRUE)")
        .bind(child.as_slice()).bind(B256::repeat_byte(1).as_slice()).bind(B256::ZERO.as_slice()).bind(TEST_CHAIN_ID).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) VALUES ($1, $2, 0, 0)")
        .bind(child.as_slice()).bind(vec![1u8]).execute(&pool).await.unwrap();
    assert!(enforce_guaranteed_containment(&pool).await.is_err());
    add_root_producer(&pool).await;
    sqlx::query("INSERT INTO handle_producer_block (host_chain_id, handle, producer_block_number, producer_block_hash) VALUES ($1, $2, 43, $3)")
        .bind(TEST_CHAIN_ID).bind(child.as_slice()).bind(B256::repeat_byte(0xab).as_slice()).execute(&pool).await.unwrap();
    enforce_guaranteed_containment(&pool).await.unwrap();
    let contained: bool = sqlx::query_scalar("SELECT is_contained FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(contained);
}
