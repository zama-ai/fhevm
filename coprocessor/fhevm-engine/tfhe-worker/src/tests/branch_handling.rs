/// M3 residual branch handling tests (T1-T3).
///
/// These tests verify that the branch-aware infrastructure from M1 correctly
/// handles RFC 011 fork scenarios:
/// - Two competing same-height blocks can produce different ciphertexts for the same handle
/// - The worker resolves the correct branch-specific ciphertext via ancestry
/// - Finality cleanup removes orphaned branch state while preserving canonical rows
/// - Branch ancestry rebuilds from DB after a worker restart
///
/// All tests require a test database and TFHE keys.  Run with:
///   COPROCESSOR_TEST_LOCAL_DB=1 cargo test branch_handling
///
/// In local persistent-DB mode, the suite resets the test tables at the start
/// of each test so stale rows from aborted prior runs do not strand the worker.
use host_listener::database::tfhe_event_propagate::Handle;
use serial_test::serial;

use crate::tests::block_scoped::{
    allow_handle_in_block, insert_fhe_add_in_block, insert_trivial_encrypt_in_block,
    make_block_hash, mark_test_dcid_ready, register_block,
};
use crate::tests::event_helpers::{next_handle, setup_event_harness, EventHarness, TEST_CHAIN_ID};
use crate::tests::utils::{
    decrypt_ciphertexts, reset_local_test_db_if_needed, wait_until_all_allowed_handles_computed,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Count how many rows exist in `ciphertexts` for a given handle (all branches).
async fn count_ciphertext_rows(pool: &sqlx::PgPool, handle: &[u8]) -> i64 {
    let row =
        sqlx::query_as::<_, (i64,)>("SELECT count(*) FROM ciphertexts_branch WHERE handle = $1")
            .bind(handle)
            .fetch_one(pool)
            .await
            .expect("count query failed");
    row.0
}

/// Count how many rows exist in `ciphertexts` for a given handle and
/// producer_block_hash.
async fn count_ciphertext_rows_for_block(
    pool: &sqlx::PgPool,
    handle: &[u8],
    producer_block_hash: &[u8],
) -> i64 {
    let row = sqlx::query_as::<_, (i64,)>(
        "SELECT count(*) FROM ciphertexts_branch WHERE handle = $1 AND producer_block_hash = $2",
    )
    .bind(handle)
    .bind(producer_block_hash)
    .fetch_one(pool)
    .await
    .expect("count query failed");
    row.0
}

/// Count how many rows exist in `computations` for a given output_handle
/// and producer_block_hash.
async fn count_computation_rows(
    pool: &sqlx::PgPool,
    handle: &[u8],
    producer_block_hash: &[u8],
) -> i64 {
    let row = sqlx::query_as::<_, (i64,)>(
        "SELECT count(*) FROM computations_branch WHERE output_handle = $1 AND producer_block_hash = $2",
    )
    .bind(handle)
    .bind(producer_block_hash)
    .fetch_one(pool)
    .await
    .expect("count query failed");
    row.0
}

/// Count how many rows exist in `allowed_handles` for a given handle
/// and producer_block_hash.
async fn count_allowed_handle_rows(
    pool: &sqlx::PgPool,
    handle: &[u8],
    producer_block_hash: &[u8],
) -> i64 {
    let row = sqlx::query_as::<_, (i64,)>(
        "SELECT count(*) FROM allowed_handles_branch WHERE handle = $1 AND producer_block_hash = $2",
    )
    .bind(handle)
    .bind(producer_block_hash)
    .fetch_one(pool)
    .await
    .expect("count query failed");
    row.0
}

// ---------------------------------------------------------------------------
// T1: Fork-and-resolve
// ---------------------------------------------------------------------------

/// Two competing blocks at the same height produce different values for the
/// same handle H.  Descendant blocks on each branch resolve to the correct
/// branch-specific ciphertext.
///
/// ```text
/// genesis (0x00)
///   ├── block 1A (0x1A): TrivialEncrypt(10) → H
///   └── block 1B (0x1B): TrivialEncrypt(20) → H
///
/// block 2A (0x2A, parent=0x1A): H + H = out_A → expects 20
/// block 2B (0x2B, parent=0x1B): H + H = out_B → expects 40
/// ```
///
/// Validates: M3 exit criteria 1 + 2.
#[tokio::test]
#[serial(db)]
async fn test_fork_and_resolve() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;

    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let genesis = Handle::ZERO;
    let hash_1a = make_block_hash(0x1A);
    let hash_1b = make_block_hash(0x1B);
    let hash_2a = make_block_hash(0x2A);
    let hash_2b = make_block_hash(0x2B);

    // Register the fork topology.
    register_block(
        &pool,
        chain_id,
        1,
        hash_1a.as_slice(),
        genesis.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        1,
        hash_1b.as_slice(),
        genesis.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        hash_2a.as_slice(),
        hash_1a.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        hash_2b.as_slice(),
        hash_1b.as_slice(),
        "pending",
    )
    .await?;

    // Same handle H on both branches — this is the RFC 011 residual case.
    let h_handle = next_handle();
    let out_a = next_handle();
    let out_b = next_handle();
    let tx_1a = next_handle();
    let tx_1b = next_handle();
    let tx_2a = next_handle();
    let tx_2b = next_handle();

    // Block 1A: TrivialEncrypt(10) → H
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx_1a,
        10,
        4,
        h_handle,
        true,
        1,
        hash_1a,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &h_handle, 1, &hash_1a).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_1a, 1, &hash_1a).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 1B: TrivialEncrypt(20) → H (same handle, different value, different branch)
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx_1b,
        20,
        4,
        h_handle,
        true,
        1,
        hash_1b,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &h_handle, 1, &hash_1b).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_1b, 1, &hash_1b).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Step 4: Assert that both branch-specific ciphertexts coexist in DB
    // with distinct producer_block_hash values.
    assert_eq!(
        count_ciphertext_rows(&pool, h_handle.as_slice()).await,
        2,
        "handle H must have two ciphertext rows (one per branch)"
    );
    assert_eq!(
        count_ciphertext_rows_for_block(&pool, h_handle.as_slice(), hash_1a.as_slice()).await,
        1,
        "handle H must have exactly one ciphertext row for branch A (0x1A)"
    );
    assert_eq!(
        count_ciphertext_rows_for_block(&pool, h_handle.as_slice(), hash_1b.as_slice()).await,
        1,
        "handle H must have exactly one ciphertext row for branch B (0x1B)"
    );

    // Block 2A: H + H = out_A  (on branch A)
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx_2a,
        h_handle,
        h_handle,
        out_a,
        false,
        true,
        2,
        hash_2a,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &out_a, 2, &hash_2a).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_2a, 2, &hash_2a).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 2B: H + H = out_B  (on branch B)
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx_2b,
        h_handle,
        h_handle,
        out_b,
        false,
        true,
        2,
        hash_2b,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &out_b, 2, &hash_2b).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_2b, 2, &hash_2b).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Decrypt and verify branch-correct resolution.
    let resp = decrypt_ciphertexts(&pool, vec![out_a.to_vec(), out_b.to_vec()]).await?;
    assert_eq!(resp[0].value, "20", "branch A: H(10)+H(10) should be 20");
    assert_eq!(resp[1].value, "40", "branch B: H(20)+H(20) should be 40");

    Ok(())
}

// ---------------------------------------------------------------------------
// T2: Fork-finalization cleanup
// ---------------------------------------------------------------------------

/// After finalizing one branch of a same-handle fork, the orphaned branch's
/// rows (including descendants) are deleted while canonical rows are preserved.
///
/// ```text
/// genesis
///   ├── block 1A (finalized) → H ciphertext retained
///   └── block 1B (orphaned)  → H ciphertext deleted
///                        \
///                         → block 2B (orphaned descendant) → rows deleted
/// ```
///
/// Validates: M3 exit criterion 3 (including transitive orphaning).
#[tokio::test]
#[serial(db)]
async fn test_fork_finalization_cleanup() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;

    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let genesis = Handle::ZERO;
    let hash_1a = make_block_hash(0xF1);
    let hash_1b = make_block_hash(0xF2);
    let hash_2b = make_block_hash(0xF3);

    register_block(
        &pool,
        chain_id,
        1,
        hash_1a.as_slice(),
        genesis.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        1,
        hash_1b.as_slice(),
        genesis.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        hash_2b.as_slice(),
        hash_1b.as_slice(),
        "pending",
    )
    .await?;

    let h_handle = next_handle();
    let desc_handle = next_handle();
    let tx_1a = next_handle();
    let tx_1b = next_handle();
    let tx_2b = next_handle();

    // Block 1A: TrivialEncrypt(10) → H
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx_1a,
        10,
        4,
        h_handle,
        true,
        1,
        hash_1a,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &h_handle, 1, &hash_1a).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_1a, 1, &hash_1a).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 1B: TrivialEncrypt(20) → H
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx_1b,
        20,
        4,
        h_handle,
        true,
        1,
        hash_1b,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &h_handle, 1, &hash_1b).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_1b, 1, &hash_1b).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 2B (descendant of 1B): H + H = desc_handle
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx_2b,
        h_handle,
        h_handle,
        desc_handle,
        false,
        true,
        2,
        hash_2b,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &desc_handle, 2, &hash_2b).await?;
    tx.commit().await?;
    mark_test_dcid_ready(&pool, &tx_2b, 2, &hash_2b).await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Pre-finalization: verify both variants exist.
    assert_eq!(
        count_ciphertext_rows(&pool, h_handle.as_slice()).await,
        2,
        "pre-finalization: H must have 2 ciphertext rows"
    );
    assert!(
        count_computation_rows(&pool, h_handle.as_slice(), hash_1a.as_slice()).await > 0,
        "pre-finalization: computation for (H, 1A) must exist"
    );
    assert!(
        count_computation_rows(&pool, h_handle.as_slice(), hash_1b.as_slice()).await > 0,
        "pre-finalization: computation for (H, 1B) must exist"
    );
    assert!(
        count_computation_rows(&pool, desc_handle.as_slice(), hash_2b.as_slice()).await > 0,
        "pre-finalization: computation for (desc, 2B) must exist"
    );

    // Finalize block 1A → 1B and 2B become orphaned.
    let mut db_tx = pool.begin().await?;
    let orphaned = listener_db
        .update_block_as_finalized(&mut db_tx, 1, &hash_1a)
        .await?
        .expect("finalization passes the parent-linkage check for an ingested block");
    listener_db
        .cleanup_orphaned_branch_state(&mut db_tx, &orphaned)
        .await?;
    db_tx.commit().await?;

    // Post-finalization assertions.
    // Canonical branch (1A) rows preserved:
    assert!(
        count_computation_rows(&pool, h_handle.as_slice(), hash_1a.as_slice()).await > 0,
        "post-finalization: canonical computation (H, 1A) must be preserved"
    );
    assert!(
        count_ciphertext_rows_for_block(&pool, h_handle.as_slice(), hash_1a.as_slice()).await > 0,
        "post-finalization: canonical ciphertext (H, 1A) must be preserved"
    );
    assert!(
        count_allowed_handle_rows(&pool, h_handle.as_slice(), hash_1a.as_slice()).await > 0,
        "post-finalization: canonical allowed_handle (H, 1A) must be preserved"
    );

    // Orphaned branch (1B) rows deleted:
    assert_eq!(
        count_computation_rows(&pool, h_handle.as_slice(), hash_1b.as_slice()).await,
        0,
        "post-finalization: orphaned computation (H, 1B) must be deleted"
    );
    assert_eq!(
        count_ciphertext_rows_for_block(&pool, h_handle.as_slice(), hash_1b.as_slice()).await,
        0,
        "post-finalization: orphaned ciphertext (H, 1B) must be deleted"
    );
    assert_eq!(
        count_allowed_handle_rows(&pool, h_handle.as_slice(), hash_1b.as_slice()).await,
        0,
        "post-finalization: orphaned allowed_handle (H, 1B) must be deleted"
    );

    // Transitive orphan (2B) rows deleted:
    assert_eq!(
        count_computation_rows(&pool, desc_handle.as_slice(), hash_2b.as_slice()).await,
        0,
        "post-finalization: descendant computation (desc, 2B) must be deleted"
    );
    assert_eq!(
        count_ciphertext_rows_for_block(&pool, desc_handle.as_slice(), hash_2b.as_slice()).await,
        0,
        "post-finalization: descendant ciphertext (desc, 2B) must be deleted"
    );
    assert_eq!(
        count_allowed_handle_rows(&pool, desc_handle.as_slice(), hash_2b.as_slice()).await,
        0,
        "post-finalization: descendant allowed_handle (desc, 2B) must be deleted"
    );

    // Total ciphertext rows for H: exactly 1 (canonical only).
    assert_eq!(
        count_ciphertext_rows(&pool, h_handle.as_slice()).await,
        1,
        "post-finalization: exactly one ciphertext row for H must remain (canonical)"
    );

    Ok(())
}

// ---------------------------------------------------------------------------
// T3: Restart-and-resolve during reorg window
// ---------------------------------------------------------------------------

/// After a worker restart, branch ancestry is rebuilt from DB and both
/// branches of a fork still resolve correctly.
///
/// ```text
/// genesis
///   ├── block 1A: TrivialEncrypt(10) → H
///   └── block 1B: TrivialEncrypt(20) → H
///
///   -- worker restart --
///
/// block 2A (parent=1A): H + H = out_A → expects 20
/// block 2B (parent=1B): H + H = out_B → expects 40
/// ```
///
/// Validates: DB-backed ancestry rebuild on a real fork.
#[tokio::test]
#[serial(db)]
async fn test_restart_and_resolve() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let genesis = Handle::ZERO;
    let hash_1a = make_block_hash(0xA1);
    let hash_1b = make_block_hash(0xA2);
    let hash_2a = make_block_hash(0xA3);
    let hash_2b = make_block_hash(0xA4);

    let h_handle = next_handle();
    let out_a = next_handle();
    let out_b = next_handle();
    let tx_1a = next_handle();
    let tx_1b = next_handle();
    let tx_2a = next_handle();
    let tx_2b = next_handle();

    {
        let EventHarness {
            mut app,
            pool,
            listener_db,
        } = setup_event_harness().await?;

        register_block(
            &pool,
            chain_id,
            1,
            hash_1a.as_slice(),
            genesis.as_slice(),
            "pending",
        )
        .await?;
        register_block(
            &pool,
            chain_id,
            1,
            hash_1b.as_slice(),
            genesis.as_slice(),
            "pending",
        )
        .await?;
        register_block(
            &pool,
            chain_id,
            2,
            hash_2a.as_slice(),
            hash_1a.as_slice(),
            "pending",
        )
        .await?;
        register_block(
            &pool,
            chain_id,
            2,
            hash_2b.as_slice(),
            hash_1b.as_slice(),
            "pending",
        )
        .await?;

        let mut tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction() returns Some on a live stack");
        insert_trivial_encrypt_in_block(
            &listener_db,
            &mut tx,
            tx_1a,
            10,
            4,
            h_handle,
            true,
            1,
            hash_1a,
        )
        .await?;
        allow_handle_in_block(&listener_db, &mut tx, &h_handle, 1, &hash_1a).await?;
        tx.commit().await?;
        mark_test_dcid_ready(&pool, &tx_1a, 1, &hash_1a).await?;
        wait_until_all_allowed_handles_computed(&app).await?;

        let mut tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction() returns Some on a live stack");
        insert_trivial_encrypt_in_block(
            &listener_db,
            &mut tx,
            tx_1b,
            20,
            4,
            h_handle,
            true,
            1,
            hash_1b,
        )
        .await?;
        allow_handle_in_block(&listener_db, &mut tx, &h_handle, 1, &hash_1b).await?;
        tx.commit().await?;
        mark_test_dcid_ready(&pool, &tx_1b, 1, &hash_1b).await?;
        wait_until_all_allowed_handles_computed(&app).await?;

        // Restart only the worker; the database container and fork rows stay live.
        app.restart_worker().await;

        // Block 2A: H + H = out_A (branch A)
        let mut tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction() returns Some on a live stack");
        insert_fhe_add_in_block(
            &listener_db,
            &mut tx,
            tx_2a,
            h_handle,
            h_handle,
            out_a,
            false,
            true,
            2,
            hash_2a,
        )
        .await?;
        allow_handle_in_block(&listener_db, &mut tx, &out_a, 2, &hash_2a).await?;
        tx.commit().await?;
        mark_test_dcid_ready(&pool, &tx_2a, 2, &hash_2a).await?;
        wait_until_all_allowed_handles_computed(&app).await?;

        // Block 2B: H + H = out_B (branch B)
        let mut tx = listener_db
            .new_transaction()
            .await?
            .expect("new_transaction() returns Some on a live stack");
        insert_fhe_add_in_block(
            &listener_db,
            &mut tx,
            tx_2b,
            h_handle,
            h_handle,
            out_b,
            false,
            true,
            2,
            hash_2b,
        )
        .await?;
        allow_handle_in_block(&listener_db, &mut tx, &out_b, 2, &hash_2b).await?;
        tx.commit().await?;
        mark_test_dcid_ready(&pool, &tx_2b, 2, &hash_2b).await?;
        wait_until_all_allowed_handles_computed(&app).await?;

        let resp = decrypt_ciphertexts(&pool, vec![out_a.to_vec(), out_b.to_vec()]).await?;
        assert_eq!(
            resp[0].value, "20",
            "restart branch A: H(10)+H(10) should be 20"
        );
        assert_eq!(
            resp[1].value, "40",
            "restart branch B: H(20)+H(20) should be 40"
        );
    }

    Ok(())
}
