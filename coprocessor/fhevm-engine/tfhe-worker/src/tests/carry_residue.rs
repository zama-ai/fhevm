/// Carry-residue regression tests.
///
/// Block-scoped execution forwards same-block intermediates raw, so
/// operations can receive trivial operands and tfhe's levelled fast paths
/// run without a terminal PBS. Through tfhe 1.6.2 the barrel shifter's
/// degree bookkeeping summed mutually-exclusive candidates, so a
/// trivial<<trivial result claimed non-empty carries and compression
/// rejected it; the coprocessor recovered with a `clean_carries` re-encode.
/// tfhe 1.6.3 fixed the bookkeeping upstream (degree clamped in
/// `block_barrel_shifter_impl`) and the recovery was removed — a result
/// that compression rejects is now a hard computation error.
///
/// These tests pin that premise: raw-forwarded trivial shift results must
/// compress directly, values must survive, and persisted bytes must match
/// an offline recomputation byte-for-byte (consensus stability). If the
/// premise assert ever fails, tfhe regressed its degree bookkeeping (or
/// reworded the compression panic) and the coprocessor — now without a
/// recovery path — will hard-fail such computations in production.
///
/// All tests require a test database and TFHE keys.  Run with:
///   COPROCESSOR_TEST_LOCAL_DB=1 cargo test carry_residue
#[cfg(not(feature = "gpu"))]
use fhevm_engine_common::db_keys::DbKeyCache;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;

use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_event, insert_trivial_encrypt, next_handle,
    next_handle_with_type, scalar_flag, setup_event_harness, zero_address, EventHarness,
};
use crate::tests::utils::{
    errors_on_allowed_handles, reset_local_test_db_if_needed,
    wait_until_all_allowed_handles_computed,
};

/// FheUint4 in the ciphertext type numbering.
const TYPE_FHE_UINT4: i32 = 1;

/// Recomputes what the scheduler must produce for `TrivialEncrypt(3) <<
/// TrivialEncrypt(3)` on FheUint4 and asserts the compression premise:
/// since tfhe 1.6.3 the raw result carries no residue and compresses
/// directly. Returns the compressed bytes. Caller must have set a server
/// key.
#[cfg(not(feature = "gpu"))]
fn compressed_trivial_shift_bytes() -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    use fhevm_engine_common::tfhe_ops::{perform_fhe_operation, trivial_encrypt_be_bytes};
    use fhevm_engine_common::types::{FhevmError, SupportedFheOperations};

    let shl = SupportedFheOperations::FheShl as i16;
    let operands = [
        trivial_encrypt_be_bytes(TYPE_FHE_UINT4 as i16, &[3])?,
        trivial_encrypt_be_bytes(TYPE_FHE_UINT4 as i16, &[3])?,
    ];
    let result = perform_fhe_operation(shl, &operands, 0, TYPE_FHE_UINT4 as i16)?;

    match result.compress() {
        Ok(bytes) => Ok(bytes),
        Err(FhevmError::CiphertextCompressionRequiresEmptyCarries) => panic!(
            "carry residue is back: trivial<<trivial on FheUint4 no longer \
             compresses — tfhe regressed the barrel-shifter degree bookkeeping \
             fixed in 1.6.3, and the coprocessor no longer has a cleaning \
             recovery, so block-scoped computations over raw trivial operands \
             will hard-fail; restore a recovery or get the bookkeeping fixed \
             upstream"
        ),
        Err(FhevmError::CiphertextCompressionPanic { message }) => panic!(
            "tfhe's compression panicked ({message:?}); if this is a reworded \
             empty-carries message, fix the string match in \
             SupportedFheCiphertexts::compress"
        ),
        Err(other) => panic!("unexpected compression error: {other:?}"),
    }
}

/// End-to-end through the block-scoped worker:
/// TrivialEncrypt(3: u4) << TrivialEncrypt(3: u4) = 8, then 8 +
/// TrivialEncrypt(1: u4) = 9 (the add consumes the forwarded working
/// ciphertext of the shift).
///
/// On CPU the persisted shift bytes are additionally asserted
/// byte-identical to an offline recomputation, pinning cross-coprocessor
/// byte identity of raw-forwarded trivial results. On GPU this is a
/// value-level smoke test only.
#[tokio::test]
#[serial(db)]
async fn test_block_scoped_trivial_shift_compresses_directly(
) -> Result<(), Box<dyn std::error::Error>> {
    reset_local_test_db_if_needed().await?;
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;

    let lhs_handle = next_handle_with_type(TYPE_FHE_UINT4);
    let amount_handle = next_handle_with_type(TYPE_FHE_UINT4);
    let one_handle = next_handle_with_type(TYPE_FHE_UINT4);
    let shift_handle = next_handle_with_type(TYPE_FHE_UINT4);
    let sum_handle = next_handle_with_type(TYPE_FHE_UINT4);
    let transaction_id = next_handle();

    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    insert_trivial_encrypt(
        &listener_db,
        &mut tx,
        transaction_id,
        3,
        TYPE_FHE_UINT4,
        lhs_handle,
        true,
    )
    .await?;
    insert_trivial_encrypt(
        &listener_db,
        &mut tx,
        transaction_id,
        3,
        TYPE_FHE_UINT4,
        amount_handle,
        true,
    )
    .await?;
    insert_trivial_encrypt(
        &listener_db,
        &mut tx,
        transaction_id,
        1,
        TYPE_FHE_UINT4,
        one_handle,
        true,
    )
    .await?;
    insert_event(
        &listener_db,
        &mut tx,
        transaction_id,
        TfheContractEvents::FheShl(TfheContract::FheShl {
            caller: zero_address(),
            lhs: lhs_handle,
            rhs: amount_handle,
            scalarByte: scalar_flag(false),
            result: shift_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &shift_handle).await?;
    insert_event(
        &listener_db,
        &mut tx,
        transaction_id,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller: zero_address(),
            lhs: shift_handle,
            rhs: one_handle,
            scalarByte: scalar_flag(false),
            result: sum_handle,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &sum_handle).await?;
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;
    let errors = errors_on_allowed_handles(&app).await?;
    assert!(
        errors.is_empty(),
        "no allowed computation may error; a compression rejection here \
         means carry residue is back without a recovery path: {errors:?}"
    );

    let resp = decrypt_handles(&pool, &[shift_handle, sum_handle]).await?;
    assert_eq!(resp.len(), 2, "both allowed handles must have persisted");
    assert_eq!(
        resp[0].value, "8",
        "3 << 3 on FheUint4 must compute correctly over raw trivial operands"
    );
    assert_eq!(
        resp[1].value, "9",
        "the consumer of the shift's working ciphertext must compute 8 + 1"
    );

    // The persisted bytes must equal a deterministic offline recomputation:
    // this pins that the shl really received raw trivial operands and that
    // compression of such results is consensus-stable across coprocessors.
    // CPU-only, like the compression premise itself.
    #[cfg(not(feature = "gpu"))]
    {
        use sqlx::Row;

        let key = DbKeyCache::new(1)
            .expect("db key cache")
            .fetch_latest_from_pool(&pool)
            .await
            .expect("fetch latest db key");
        let expected_bytes = tokio::task::spawn_blocking(move || {
            tfhe::set_server_key(key.sks);
            compressed_trivial_shift_bytes()
        })
        .await?
        // no std From<Box<dyn Error + Send + Sync>> for Box<dyn Error>; the
        // annotated closure return coerces by dropping the auto traits
        .map_err(|e| -> Box<dyn std::error::Error> { e })?;

        let row = sqlx::query(
            "SELECT ciphertext FROM ciphertexts_branch
             WHERE handle = $1 AND ciphertext_version = $2",
        )
        .bind(shift_handle.to_vec())
        .bind(fhevm_engine_common::tfhe_ops::current_ciphertext_version())
        .fetch_one(&pool)
        .await?;
        let persisted_bytes: Vec<u8> = row.try_get("ciphertext")?;
        assert_eq!(
            persisted_bytes, expected_bytes,
            "persisted shift bytes must equal the offline recomputation \
             (raw trivial forwarding, bytes consensus-stable)"
        );
    }

    Ok(())
}
