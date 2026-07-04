/// Block-scoped execution tests.
///
/// These tests verify the block-scoped materialization path:
/// - boundary inputs are decompressed + re-randomized once at the worker level
/// - same-block intermediates propagate as working ciphertexts (no re-compress/decompress)
/// - per-operation re-randomization is skipped (only block-scoped re-rand runs)
///
/// All tests require a test database and TFHE keys.  Run with:
///   COPROCESSOR_TEST_LOCAL_DB=1 cargo test block_scoped
use alloy::primitives::FixedBytes;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::{
    Database as ListenerDatabase, Handle, LogTfhe, ProducerBlock, Transaction,
};
use serial_test::serial;
use sqlx::types::time::PrimitiveDateTime;

use crate::tests::event_helpers::{
    allow_handle, as_scalar_uint, insert_event, insert_trivial_encrypt, log_with_tx, next_handle,
    setup_event_harness, setup_event_harness_with_worker_config, tfhe_event, to_ty,
    upsert_test_dcid, zero_address, EventHarness, TEST_CHAIN_ID,
};
use crate::tests::utils::{
    decrypt_ciphertexts, reset_local_test_db_if_needed, wait_until_all_allowed_handles_computed,
};
use bigdecimal::num_bigint::BigInt;

// ---------------------------------------------------------------------------
// Helpers for block-aware event insertion
// ---------------------------------------------------------------------------

pub(crate) fn make_block_hash(seed: u64) -> Handle {
    let mut out = [0_u8; 32];
    out[..8].copy_from_slice(&seed.to_be_bytes());
    Handle::from(out)
}

/// Insert a TFHE event associated with a specific block context.
pub(crate) async fn insert_event_in_block(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    event: TfheContractEvents,
    is_allowed: bool,
    block_number: u64,
    block_hash: Handle,
) -> Result<(), sqlx::Error> {
    let log = log_with_tx(tx_id, tfhe_event(event));
    let event = LogTfhe {
        event: log.inner,
        transaction_hash: Some(tx_id),
        is_allowed,
        block_number,
        block_hash,
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain: tx_id,
        tx_depth_size: 0,
        log_index: log.log_index,
    };
    upsert_test_dcid(tx.as_mut(), tx_id.as_slice(), block_number, block_hash.as_slice()).await?;
    listener_db.insert_tfhe_event(tx, &event).await?;
    Ok(())
}

/// Register a block in `host_chain_blocks_valid` so the worker can build
/// ancestry chains for block-scoped execution.
pub(crate) async fn register_block(
    pool: &sqlx::PgPool,
    chain_id: i64,
    block_number: i64,
    block_hash: &[u8],
    parent_hash: &[u8],
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO host_chain_blocks_valid
              (chain_id, block_hash, parent_hash, block_number, block_status)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (chain_id, block_hash) DO NOTHING"#,
    )
    .bind(chain_id)
    .bind(block_hash)
    .bind(parent_hash)
    .bind(block_number)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

async fn mark_test_dcid_ready(
    pool: &sqlx::PgPool,
    dependence_chain_id: &Handle,
    block_number: u64,
    block_hash: &Handle,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO dependence_chain (
              dependence_chain_id,
              status,
              last_updated_at,
              dependency_count,
              block_hash,
              block_height,
              schedule_priority
           ) VALUES ($1, 'updated', NOW(), 0, $2, $3, 0)
           ON CONFLICT (dependence_chain_id) DO UPDATE
           SET status = 'updated',
               worker_id = NULL,
               lock_acquired_at = NULL,
               lock_expires_at = NULL,
               dependency_count = 0,
               block_hash = EXCLUDED.block_hash,
               block_height = EXCLUDED.block_height,
               schedule_priority = EXCLUDED.schedule_priority"#,
    )
    .bind(dependence_chain_id.as_slice())
    .bind(block_hash.as_slice())
    .bind(block_number as i64)
    .execute(pool)
    .await?;
    sqlx::query("NOTIFY work_available").execute(pool).await?;
    Ok(())
}

async fn set_test_component_dcid(
    pool: &sqlx::PgPool,
    transaction_id: &Handle,
    producer_block_hash: &Handle,
    component_dcid: &Handle,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE computations_branch
         SET dependence_chain_id = $1
         WHERE transaction_id = $2
           AND producer_block_hash = $3",
    )
    .bind(component_dcid.as_slice())
    .bind(transaction_id.as_slice())
    .bind(producer_block_hash.as_slice())
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert a TrivialEncrypt event in a specific block.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn insert_trivial_encrypt_in_block(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    value: u64,
    to_type: i32,
    result: Handle,
    is_allowed: bool,
    block_number: u64,
    block_hash: Handle,
) -> Result<(), sqlx::Error> {
    insert_event_in_block(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::TrivialEncrypt(TfheContract::TrivialEncrypt {
            caller: zero_address(),
            pt: as_scalar_uint(&BigInt::from(value)),
            toType: to_ty(to_type),
            result,
        }),
        is_allowed,
        block_number,
        block_hash,
    )
    .await
}

/// Insert an FheAdd event in a specific block.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn insert_fhe_add_in_block(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    tx_id: Handle,
    lhs: Handle,
    rhs: Handle,
    result: Handle,
    is_scalar: bool,
    is_allowed: bool,
    block_number: u64,
    block_hash: Handle,
) -> Result<(), sqlx::Error> {
    use host_listener::database::tfhe_event_propagate::ScalarByte;
    insert_event_in_block(
        listener_db,
        tx,
        tx_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs,
            rhs,
            scalarByte: ScalarByte::from(is_scalar as u8),
            result,
        }),
        is_allowed,
        block_number,
        block_hash,
    )
    .await
}

/// Mark a handle as allowed for decryption in a specific block context.
pub(crate) async fn allow_handle_in_block(
    listener_db: &ListenerDatabase,
    tx: &mut Transaction<'_>,
    handle: &Handle,
    block_number: u64,
    block_hash: &Handle,
) -> Result<(), sqlx::Error> {
    use fhevm_engine_common::types::AllowEvents;
    listener_db
        .insert_allowed_handle(
            tx,
            handle.to_vec(),
            String::new(),
            AllowEvents::AllowedForDecryption,
            None,
            ProducerBlock::new(block_hash.as_slice(), block_number),
        )
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Test that block-scoped execution produces correct results for a simple
/// same-block computation: TrivialEncrypt(7) + TrivialEncrypt(3) = 10.
///
/// Validates: D5 (worker materialization), D2 (intra-tx Value), D6 (no per-op re-rand).
#[tokio::test]
#[serial(db)]
async fn test_block_scoped_simple_add() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let lhs_handle = next_handle();
    let rhs_handle = next_handle();
    let output_handle = next_handle();
    let transaction_id = next_handle();

    let mut tx = listener_db.new_transaction().await?;
    // FheUint32 = type 4
    insert_trivial_encrypt(
        &listener_db,
        &mut tx,
        transaction_id,
        7,
        4,
        lhs_handle,
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &lhs_handle).await?;
    insert_trivial_encrypt(
        &listener_db,
        &mut tx,
        transaction_id,
        3,
        4,
        rhs_handle,
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &rhs_handle).await?;
    insert_event(
        &listener_db,
        &mut tx,
        transaction_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs: lhs_handle,
            rhs: rhs_handle,
            scalarByte: FixedBytes::from([0_u8]),
            result: output_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &output_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, vec![output_handle.to_vec()]).await?;
    assert_eq!(resp[0].value, "10", "block-scoped add: 7+3 should be 10");

    Ok(())
}

/// Test that a prior-block boundary handle consumed by multiple operations
/// in a later block produces correct results under block-scoped mode.
///
/// Block 1 (hash=0xCC..): TrivialEncrypt(5) -> h
/// Block 2 (hash=0xDD.., parent=0xCC..): h + h = out1 (10), h + 2(scalar) = out2 (7)
///
/// Handle h is a boundary input to block 2 — it crosses the block boundary
/// and must be materialized (decompress + re-rand with block 2's hash) once.
/// Both operations in block 2 share the same materialized working ciphertext.
///
/// Validates: D5 (boundary materialization dedup for prior-block handle).
#[tokio::test]
#[serial(db)]
async fn test_block_scoped_boundary_handle_multi_use() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let block_1_hash = make_block_hash(0xCC);
    let block_2_hash = make_block_hash(0xDD);
    let genesis_hash = Handle::ZERO;

    register_block(
        &pool,
        chain_id,
        1,
        block_1_hash.as_slice(),
        genesis_hash.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        block_2_hash.as_slice(),
        block_1_hash.as_slice(),
        "pending",
    )
    .await?;

    let input_handle = next_handle();
    let add_output = next_handle();
    let add_scalar_output = next_handle();
    let tx1_id = next_handle();
    let tx2_id = next_handle();

    // Block 1: TrivialEncrypt(5) -> input_handle
    let mut tx = listener_db.new_transaction().await?;
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx1_id,
        5,
        4,
        input_handle,
        true,
        1,
        block_1_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &input_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 2: h + h = out1, h + 2(scalar) = out2
    let mut tx = listener_db.new_transaction().await?;
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx2_id,
        input_handle,
        input_handle,
        add_output,
        false,
        true,
        2,
        block_2_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &add_output).await?;

    let scalar_2 = {
        let mut out = [0_u8; 32];
        out[31] = 2;
        Handle::from(out)
    };
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx2_id,
        input_handle,
        scalar_2,
        add_scalar_output,
        true,
        true,
        2,
        block_2_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &add_scalar_output).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp =
        decrypt_ciphertexts(&pool, vec![add_output.to_vec(), add_scalar_output.to_vec()]).await?;
    assert_eq!(resp[0].value, "10", "h+h: 5+5 should be 10");
    assert_eq!(resp[1].value, "7", "h+2: 5+2 should be 7");

    Ok(())
}

/// Test that two transactions in the same block can share a handle's output
/// under block-scoped mode.
///
/// Tx1: TrivialEncrypt(4) -> a, a + a = b (allowed).
/// Tx2: b + b = c (allowed).
/// Both in the same dependence chain (same block), so b propagates as a
/// working ciphertext to tx2 via the inter-tx Value path.
///
/// Validates: D3 (inter-tx Value propagation).
#[tokio::test]
#[serial(db)]
async fn test_block_scoped_inter_tx_reuse() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let a_handle = next_handle();
    let b_handle = next_handle();
    let c_handle = next_handle();
    let tx1_id = next_handle();
    let tx2_id = next_handle();

    // Tx1: TrivialEncrypt(4) -> a, a + a = b
    let mut tx = listener_db.new_transaction().await?;
    insert_trivial_encrypt(&listener_db, &mut tx, tx1_id, 4, 4, a_handle, true).await?;
    allow_handle(&listener_db, &mut tx, &a_handle).await?;
    insert_event(
        &listener_db,
        &mut tx,
        tx1_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs: a_handle,
            rhs: a_handle,
            scalarByte: FixedBytes::from([0_u8]),
            result: b_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &b_handle).await?;
    tx.commit().await?;

    // Tx2: b + b = c (depends on tx1's output)
    let mut tx = listener_db.new_transaction().await?;
    insert_event(
        &listener_db,
        &mut tx,
        tx2_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs: b_handle,
            rhs: b_handle,
            scalarByte: FixedBytes::from([0_u8]),
            result: c_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &c_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, vec![b_handle.to_vec(), c_handle.to_vec()]).await?;
    assert_eq!(resp[0].value, "8", "b = a+a = 4+4 = 8");
    assert_eq!(resp[1].value, "16", "c = b+b = 8+8 = 16");

    Ok(())
}

/// Test cross-block boundary materialization with distinct block hashes.
///
/// Block 1 (hash=0xAA..): TrivialEncrypt(6) -> a_handle
/// Block 2 (hash=0xBB.., parent=0xAA..): a_handle + a_handle = output (12)
///
/// The handle `a_handle` is a boundary input to block 2 — it must be
/// decompressed and re-randomized with block 2's hash (0xBB..).
///
/// Validates: D5 (worker materialization with a non-trivial block hash),
///            cross-block boundary reuse.
#[tokio::test]
#[serial(db)]
async fn test_cross_block_reuse_materializes() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let block_a_hash = make_block_hash(0xAA);
    let block_b_hash = make_block_hash(0xBB);
    let genesis_hash = Handle::ZERO;

    // Register blocks so the worker can build ancestry.
    register_block(
        &pool,
        chain_id,
        1,
        block_a_hash.as_slice(),
        genesis_hash.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        block_b_hash.as_slice(),
        block_a_hash.as_slice(),
        "pending",
    )
    .await?;

    let a_handle = next_handle();
    let output_handle = next_handle();
    let tx1_id = next_handle();
    let tx2_id = next_handle();

    // Block 1: TrivialEncrypt(6) -> a_handle
    // Must complete before block 2 is inserted, because the worker resolves
    // cross-block dependencies from the ciphertexts table at query time.
    let mut tx = listener_db.new_transaction().await?;
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx1_id,
        6,
        4,
        a_handle,
        true,
        1,
        block_a_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &a_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 2: a_handle + a_handle = output_handle
    // Now a_handle's ciphertext is in the DB, so block 2 can resolve it.
    let mut tx = listener_db.new_transaction().await?;
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx2_id,
        a_handle,
        a_handle,
        output_handle,
        false,
        true,
        2,
        block_b_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &output_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, vec![output_handle.to_vec()]).await?;
    assert_eq!(resp[0].value, "12", "cross-block add: 6+6 should be 12");

    Ok(())
}

/// Test that two independent transactions consuming the same prior-block
/// boundary handle both produce correct results under block-scoped mode.
///
/// Block 1 (hash=0xEE..): TrivialEncrypt(5) -> input_handle
/// Block 2 (hash=0xFF.., parent=0xEE..):
///   Tx1: input_handle + input_handle = out1 (10)
///   Tx2: input_handle + 3(scalar)    = out2 (8)
///
/// Neither tx1 nor tx2 produces the other's input — they are independent
/// consumers of the same boundary handle. With MaxParallelism partitioning,
/// they have no graph edge and may land in different partitions. The worker
/// must materialize input_handle once before scheduling so both partitions
/// receive the same working ciphertext.
///
/// Validates: D5 (worker-level boundary materialization dedup across
///            independent partitions).
#[tokio::test]
#[serial(db)]
async fn test_cross_partition_boundary_reuse() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let block_1_hash = make_block_hash(0xEE);
    let block_2_hash = make_block_hash(0xFF);
    let genesis_hash = Handle::ZERO;

    register_block(
        &pool,
        chain_id,
        1,
        block_1_hash.as_slice(),
        genesis_hash.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        block_2_hash.as_slice(),
        block_1_hash.as_slice(),
        "pending",
    )
    .await?;

    let input_handle = next_handle();
    let out1_handle = next_handle();
    let out2_handle = next_handle();
    let tx_setup_id = next_handle();
    let tx1_id = next_handle();
    let tx2_id = next_handle();

    // Block 1: TrivialEncrypt(5) -> input_handle
    let mut tx = listener_db.new_transaction().await?;
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx_setup_id,
        5,
        4,
        input_handle,
        true,
        1,
        block_1_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &input_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    // Block 2, Tx1: input_handle + input_handle = out1 (independent)
    let mut tx = listener_db.new_transaction().await?;
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx1_id,
        input_handle,
        input_handle,
        out1_handle,
        false,
        true,
        2,
        block_2_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &out1_handle).await?;
    tx.commit().await?;

    // Block 2, Tx2: input_handle + 3(scalar) = out2 (independent)
    let mut tx = listener_db.new_transaction().await?;
    let scalar_3 = {
        let mut out = [0_u8; 32];
        out[31] = 3;
        Handle::from(out)
    };
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx2_id,
        input_handle,
        scalar_3,
        out2_handle,
        true,
        true,
        2,
        block_2_hash,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &out2_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, vec![out1_handle.to_vec(), out2_handle.to_vec()]).await?;
    assert_eq!(resp[0].value, "10", "input+input: 5+5 should be 10");
    assert_eq!(resp[1].value, "8", "input+3: 5+3 should be 8");

    Ok(())
}

/// Fetch raw ciphertext bytes from the DB for a given output handle.
async fn query_raw_ciphertext(pool: &sqlx::PgPool, handle: &[u8]) -> Option<Vec<u8>> {
    let row = sqlx::query_as::<_, (Vec<u8>,)>(
        "SELECT ciphertext FROM ciphertexts_branch WHERE handle = $1 LIMIT 1",
    )
    .bind(handle)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();
    row.map(|(ct,)| ct)
}

/// Two distinct transactions in the same block can deterministically produce
/// the same `output_handle` (handle derivation in `FHEVMExecutor` is a pure
/// function of op, operands, ACL, chain-id, and block context). The scheduler
/// executes both producers, and the ciphertexts_branch PK collapses the writes
/// to a single row via `ON CONFLICT DO NOTHING`. Consensus safety therefore
/// hinges on the invariant: "both in-flight executions produce byte-identical
/// ciphertexts."
///
/// The test records three ciphertext byte-strings and requires all three to
/// be equal:
/// - `ct_single`: baseline — one producer tx, one FheAdd event.
/// - `ct_multi_1`: two producer txs in the same block, same operands,
///                 same result handle.
/// - `ct_multi_2`: fresh harness, same scenario as `ct_multi_1`, to catch
///                 any run-to-run non-determinism that would only surface
///                 across coprocessors in a consensus quorum.
#[tokio::test]
#[serial(db)]
async fn test_multi_producer_same_handle_bit_identity() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    async fn run(multi_producer: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let EventHarness {
            app,
            pool,
            listener_db,
        } = setup_event_harness().await?;

        let lhs = next_handle();
        let rhs = next_handle();
        let out = next_handle();
        let tid_a = next_handle();
        let tid_b = next_handle();

        let mut tx = listener_db.new_transaction().await?;
        insert_trivial_encrypt(&listener_db, &mut tx, tid_a, 9, 4, lhs, true).await?;
        allow_handle(&listener_db, &mut tx, &lhs).await?;
        insert_trivial_encrypt(&listener_db, &mut tx, tid_a, 1, 4, rhs, true).await?;
        allow_handle(&listener_db, &mut tx, &rhs).await?;
        insert_event(
            &listener_db,
            &mut tx,
            tid_a,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller: zero_address(),
                lhs,
                rhs,
                scalarByte: FixedBytes::from([0_u8]),
                result: out,
            }),
            true,
        )
        .await?;
        if multi_producer {
            // Second tx in the same block, same operands, same result handle.
            // This is the exact scenario flagged by the scheduler's
            // "Handle collision for computation output" branch.
            insert_event(
                &listener_db,
                &mut tx,
                tid_b,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller: zero_address(),
                    lhs,
                    rhs,
                    scalarByte: FixedBytes::from([0_u8]),
                    result: out,
                }),
                true,
            )
            .await?;
        }
        allow_handle(&listener_db, &mut tx, &out).await?;
        tx.commit().await?;

        wait_until_all_allowed_handles_computed(&app).await?;

        // Plaintext must be correct.
        let resp = decrypt_ciphertexts(&pool, vec![out.to_vec()]).await?;
        assert_eq!(resp[0].value, "10", "fheAdd(9, 1) must decrypt to 10");

        // With two producer rows, both must end up completed. Otherwise
        // the listener or scheduler silently dropped one.
        if multi_producer {
            let rows = sqlx::query_as::<_, (Vec<u8>, bool, bool)>(
                "SELECT transaction_id, is_completed, is_error \
                 FROM computations_branch WHERE output_handle = $1",
            )
            .bind(out.as_slice())
            .fetch_all(&pool)
            .await?;
            assert_eq!(
                rows.len(),
                2,
                "both producer rows must exist in computations_branch"
            );
            for (tid, is_completed, is_error) in &rows {
                assert!(
                    *is_completed,
                    "producer row tid={} must be completed",
                    hex::encode(tid)
                );
                assert!(
                    !*is_error,
                    "producer row tid={} must not be marked in error",
                    hex::encode(tid)
                );
            }
        }

        // Both producers collapse to a single stored row in ciphertexts_branch
        // (PK includes producer_block_hash, so distinct producers on different
        // branches would land separately — here they share a branch).
        let rows = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*)::bigint FROM ciphertexts_branch WHERE handle = $1",
        )
        .bind(out.as_slice())
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            rows.0, 1,
            "ciphertexts_branch must hold exactly one row per output handle \
             regardless of producer count when both share the same branch"
        );

        let raw = query_raw_ciphertext(&pool, out.as_slice())
            .await
            .expect("ciphertext row must exist after computation");
        Ok(raw)
    }

    let ct_single = run(false).await?;
    let ct_multi_1 = run(true).await?;
    let ct_multi_2 = run(true).await?;

    assert_eq!(
        ct_single, ct_multi_1,
        "multi-producer ciphertext must match single-producer ciphertext \
         byte-for-byte — any divergence is a silent consensus hazard"
    );
    assert_eq!(
        ct_multi_1, ct_multi_2,
        "two independent multi-producer runs must produce identical \
         ciphertext bytes — any divergence is non-determinism that would \
         surface as drift across a consensus quorum"
    );

    Ok(())
}

/// Regression test for the same-block batch-closure invariant (commit
/// "keep block context within worker batches").
///
/// RFC 020 forbids a same-block intermediate from being fetched back from the
/// DB and re-randomized as a boundary input: same-block producer -> consumer
/// edges must never cross a worker batch boundary. The worker enforces this by
/// closing every batch over the same-block connected component, so a producer
/// and any same-block consumer are always executed together and the producer
/// output propagates in memory (no decompress, no boundary re-rand).
///
/// This test forces exactly the split the invariant must defend against:
/// the producer (`TrivialEncrypt(5) -> src`, allowed) is computed and persisted
/// to `ciphertexts_branch` in one worker cycle, and *only afterwards* is the
/// same-block consumer (`FheAdd(src, src) -> out`) inserted, landing it in a
/// later cycle. If batch closure ever regressed, the consumer's cycle would
/// treat the already-persisted `src` as DB materialized input and
/// decompress it -- producing different `out` bytes than the single-cycle
/// path.
///
/// We require the consumer output to be byte-identical whether producer and
/// consumer run in one cycle ("together") or split across cycles ("split").
/// Any divergence is precisely the consensus bug batch closure prevents.
/// The test runs with DCID locking enabled and puts producer/consumer in
/// different transactions. The synthetic direct inserts are then assigned the
/// same component DCID that host-listener ingestion would assign for this
/// same-block producer/consumer edge.
#[tokio::test]
#[serial(db)]
async fn test_same_block_split_across_cycles_bit_identity() -> Result<(), Box<dyn std::error::Error>>
{
    reset_local_test_db_if_needed().await?;

    async fn run(split: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let EventHarness {
            app,
            pool,
            listener_db,
        } = setup_event_harness_with_worker_config(1, 8).await?;

        let src = next_handle();
        let out = next_handle();
        let tid_producer = next_handle();
        let tid_consumer = next_handle();
        let block_number = 1_u64;
        let block_hash = make_block_hash(1);
        let parent_hash = Handle::ZERO;

        register_block(
            &pool,
            TEST_CHAIN_ID as i64,
            block_number as i64,
            block_hash.as_slice(),
            parent_hash.as_slice(),
            "pending",
        )
        .await?;
        sqlx::query(
            "INSERT INTO coprocessor_settlement (chain_id, settled_height) \
             VALUES ($1, $2) \
             ON CONFLICT (chain_id) DO UPDATE SET settled_height = EXCLUDED.settled_height",
        )
        .bind(TEST_CHAIN_ID as i64)
        .bind(0_i64)
        .execute(&pool)
        .await?;

        // Producer: TrivialEncrypt(5) -> src. Allowed so it is persisted to
        // ciphertexts_branch -- this is the byte-image the regressed boundary
        // path would wrongly re-fetch and re-randomize for the consumer.
        let mut tx = listener_db.new_transaction().await?;
        insert_trivial_encrypt_in_block(
            &listener_db,
            &mut tx,
            tid_producer,
            5,
            4,
            src,
            true,
            block_number,
            block_hash,
        )
        .await?;
        allow_handle_in_block(&listener_db, &mut tx, &src, block_number, &block_hash).await?;
        if !split {
            // Together: the consumer is in before any cycle runs, so producer
            // and consumer are taken in a single batch.
            insert_fhe_add_in_block(
                &listener_db,
                &mut tx,
                tid_consumer,
                src,
                src,
                out,
                false,
                true,
                block_number,
                block_hash,
            )
            .await?;
            allow_handle_in_block(&listener_db, &mut tx, &out, block_number, &block_hash).await?;
        }
        tx.commit().await?;
        if !split {
            set_test_component_dcid(&pool, &tid_consumer, &block_hash, &tid_producer).await?;
        }
        mark_test_dcid_ready(&pool, &tid_producer, block_number, &block_hash).await?;

        // First cycle: producer (and, in the "together" case, the consumer).
        wait_until_all_allowed_handles_computed(&app).await?;

        if split {
            // The producer is now completed and persisted. Insert the same-block
            // consumer afterwards, forcing it into a separate worker cycle. The
            // batch-closure query must re-include the persisted producer so
            // `src` still propagates in memory rather than being re-randomized.
            let persisted = query_raw_ciphertext(&pool, src.as_slice()).await;
            assert!(
                persisted.is_some(),
                "producer output must be persisted to ciphertexts_branch before \
                 the consumer is inserted, so the invalid same-block DB fetch path is reachable"
            );
            let mut tx = listener_db.new_transaction().await?;
            insert_fhe_add_in_block(
                &listener_db,
                &mut tx,
                tid_consumer,
                src,
                src,
                out,
                false,
                true,
                block_number,
                block_hash,
            )
            .await?;
            allow_handle_in_block(&listener_db, &mut tx, &out, block_number, &block_hash).await?;
            tx.commit().await?;
            set_test_component_dcid(&pool, &tid_consumer, &block_hash, &tid_producer).await?;
            mark_test_dcid_ready(&pool, &tid_producer, block_number, &block_hash).await?;
            wait_until_all_allowed_handles_computed(&app).await?;
        }

        // Plaintext must be correct regardless of batching.
        let resp = decrypt_ciphertexts(&pool, vec![out.to_vec()]).await?;
        assert_eq!(resp[0].value, "10", "fheAdd(5, 5) must decrypt to 10");

        let raw = query_raw_ciphertext(&pool, out.as_slice())
            .await
            .expect("consumer ciphertext row must exist after computation");
        Ok(raw)
    }

    let ct_together = run(false).await?;
    let ct_split = run(true).await?;

    assert_eq!(
        ct_together, ct_split,
        "same-block consumer output must be byte-identical whether the producer \
         and consumer run in one worker cycle or split across cycles -- a \
         divergence means a same-block intermediate was materialized from DB, \
         the consensus bug batch closure must prevent"
    );

    Ok(())
}

/// Wave-2 upgrade compat: a dependency whose bytes exist only as a
/// branchless row (empty producer_block_hash — the shape written by the
/// zkproof-worker for user inputs and by the backfill migration for
/// pre-upgrade handles) must resolve for any consuming block.
#[tokio::test]
#[serial(db)]
async fn test_branchless_dependency_resolves() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let block_a_hash = make_block_hash(0xA7);
    let block_b_hash = make_block_hash(0xB7);
    register_block(
        &pool,
        chain_id,
        1,
        block_a_hash.as_slice(),
        Handle::ZERO.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        block_b_hash.as_slice(),
        block_a_hash.as_slice(),
        "pending",
    )
    .await?;

    let input_handle = next_handle();
    let output_handle = next_handle();
    let tx1_id = next_handle();
    let tx2_id = next_handle();

    // Block 1: produce real ciphertext bytes for input_handle.
    let mut tx = listener_db.new_transaction().await?;
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx1_id,
        5,
        4,
        input_handle,
        true,
        1,
        block_a_hash,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &input_handle, 1, &block_a_hash).await?;
    tx.commit().await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Rewrite input_handle's state into the branchless shape: bytes and allow
    // rows under an empty producer hash, no computed producer row (exactly
    // what a ZK-verified input or a backfilled pre-upgrade handle looks like —
    // their allow rows are keyed branchless too, and a leftover block-keyed
    // allow row would mask the branchless dependency fallback).
    sqlx::query("UPDATE ciphertexts_branch SET producer_block_hash = ''::BYTEA WHERE handle = $1")
        .bind(input_handle.to_vec())
        .execute(&pool)
        .await?;
    sqlx::query(
        "UPDATE allowed_handles_branch SET producer_block_hash = ''::BYTEA WHERE handle = $1",
    )
    .bind(input_handle.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query("DELETE FROM computations_branch WHERE output_handle = $1")
        .bind(input_handle.to_vec())
        .execute(&pool)
        .await?;

    // Block 2: consume the branchless input.
    let mut tx = listener_db.new_transaction().await?;
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx2_id,
        input_handle,
        input_handle,
        output_handle,
        false,
        true,
        2,
        block_b_hash,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &output_handle, 2, &block_b_hash).await?;
    tx.commit().await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, vec![output_handle.to_vec()]).await?;
    assert_eq!(resp[0].value, "10", "branchless add: 5+5 should be 10");

    Ok(())
}

/// Wave-2 upgrade compat: a dependency whose bytes exist only in the legacy
/// `ciphertexts` table (produced by the pre-cutover legacy pipeline) and
/// whose allow row is branchless (the backfill shape) must resolve through
/// the legacy read fallback.
#[tokio::test]
#[serial(db)]
async fn test_legacy_ciphertext_fallback_resolves() -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let chain_id = TEST_CHAIN_ID as i64;
    let block_a_hash = make_block_hash(0xA8);
    let block_b_hash = make_block_hash(0xB8);
    register_block(
        &pool,
        chain_id,
        1,
        block_a_hash.as_slice(),
        Handle::ZERO.as_slice(),
        "pending",
    )
    .await?;
    register_block(
        &pool,
        chain_id,
        2,
        block_b_hash.as_slice(),
        block_a_hash.as_slice(),
        "pending",
    )
    .await?;

    let input_handle = next_handle();
    let output_handle = next_handle();
    let tx1_id = next_handle();
    let tx2_id = next_handle();

    // Block 1: produce real ciphertext bytes for input_handle.
    let mut tx = listener_db.new_transaction().await?;
    insert_trivial_encrypt_in_block(
        &listener_db,
        &mut tx,
        tx1_id,
        6,
        4,
        input_handle,
        true,
        1,
        block_a_hash,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &input_handle, 1, &block_a_hash).await?;
    tx.commit().await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    // Rewrite input_handle's state into the pre-upgrade shape: bytes only in
    // the legacy ciphertexts table, allow row branchless (backfill shape),
    // no branch producer or byte rows at all.
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         SELECT handle, ciphertext, ciphertext_version, ciphertext_type
           FROM ciphertexts_branch WHERE handle = $1
         ON CONFLICT (handle, ciphertext_version) DO NOTHING",
    )
    .bind(input_handle.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query("DELETE FROM ciphertexts_branch WHERE handle = $1")
        .bind(input_handle.to_vec())
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM computations_branch WHERE output_handle = $1")
        .bind(input_handle.to_vec())
        .execute(&pool)
        .await?;
    sqlx::query(
        "UPDATE allowed_handles_branch SET producer_block_hash = ''::BYTEA WHERE handle = $1",
    )
    .bind(input_handle.to_vec())
    .execute(&pool)
    .await?;

    // Block 2: consume the legacy input.
    let mut tx = listener_db.new_transaction().await?;
    insert_fhe_add_in_block(
        &listener_db,
        &mut tx,
        tx2_id,
        input_handle,
        input_handle,
        output_handle,
        false,
        true,
        2,
        block_b_hash,
    )
    .await?;
    allow_handle_in_block(&listener_db, &mut tx, &output_handle, 2, &block_b_hash).await?;
    tx.commit().await?;
    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, vec![output_handle.to_vec()]).await?;
    assert_eq!(resp[0].value, "12", "legacy fallback add: 6+6 should be 12");

    Ok(())
}
