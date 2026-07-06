/// Carry-residue cleaning tests.
///
/// Block-scoped execution forwards same-block intermediates raw, so
/// operations can receive trivial operands; tfhe's levelled fast paths then
/// leave value bits in the carry space (see
/// `SupportedFheCiphertexts::clean_carries` for the mechanism), which
/// compression rejects and the scheduler's `compress_or_clean_carries`
/// recovers from. These tests pin that recovery: the dirty premise, value
/// preservation, byte determinism, and the worker persisting cleaned bytes.
///
/// All tests require a test database and TFHE keys.  Run with:
///   COPROCESSOR_TEST_LOCAL_DB=1 cargo test carry_cleaning
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
/// TrivialEncrypt(3)` on FheUint4: asserts the raw result is rejected by
/// compression with carry residue (the premise both tests rely on), then
/// returns the cleaned, compressed bytes. Caller must have set a server key.
#[cfg(not(feature = "gpu"))]
fn cleaned_trivial_shift_bytes(
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    use fhevm_engine_common::tfhe_ops::{perform_fhe_operation, trivial_encrypt_be_bytes};
    use fhevm_engine_common::types::{FhevmError, SupportedFheOperations};

    let shl = SupportedFheOperations::FheShl as i16;
    let operands = [
        trivial_encrypt_be_bytes(TYPE_FHE_UINT4 as i16, &[3])?,
        trivial_encrypt_be_bytes(TYPE_FHE_UINT4 as i16, &[3])?,
    ];
    let dirty = perform_fhe_operation(shl, &operands, 0, TYPE_FHE_UINT4 as i16)?;

    match dirty.compress() {
        Err(FhevmError::CiphertextCompressionRequiresEmptyCarries) => {}
        Ok(_) => panic!(
            "premise gone: trivial<<trivial on FheUint4 now compresses directly, \
             so tfhe's trivial fast-path bookkeeping changed and the cleaning \
             recovery is no longer exercised by this shape — re-derive a dirty \
             ciphertext or retire the recovery"
        ),
        Err(FhevmError::CiphertextCompressionPanic { message }) => panic!(
            "tfhe's compression panic message changed ({message:?}): production's \
             compress_or_clean_carries no longer recognizes carry residue and dirty \
             results become hard failures — fix the string match in \
             SupportedFheCiphertexts::compress before touching this test"
        ),
        Err(other) => panic!("unexpected compression error: {other:?}"),
    }

    Ok(dirty.clean_carries()?.compress()?)
}

/// Direct check of the cleaning mechanism against the fhevm op dispatch:
/// the dirty premise (via [`cleaned_trivial_shift_bytes`]), value
/// preservation, in-process byte determinism, and the legacy-flow control.
///
/// CPU-only: the dirty-ciphertext premise is verified against the CPU
/// trivial fast path; the CUDA backend tracks degrees differently.
#[cfg(not(feature = "gpu"))]
#[tokio::test]
#[serial(db)]
async fn test_clean_carries_mechanism() -> Result<(), Box<dyn std::error::Error>> {
    use fhevm_engine_common::tfhe_ops::{perform_fhe_operation, trivial_encrypt_be_bytes};
    use fhevm_engine_common::types::{SupportedFheCiphertexts, SupportedFheOperations};

    reset_local_test_db_if_needed().await?;
    let EventHarness { app: _app, pool, .. } = setup_event_harness().await?;
    let key = DbKeyCache::new(1)
        .expect("db key cache")
        .fetch_latest_from_pool(&pool)
        .await
        .expect("fetch latest db key");

    tokio::task::spawn_blocking(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tfhe::set_server_key(key.sks);
        let cks = key.cks.expect("test keys include a client key");
        let shl = SupportedFheOperations::FheShl as i16;

        let cleaned_bytes = cleaned_trivial_shift_bytes()?;
        let cleaned = SupportedFheCiphertexts::decompress_no_memcheck(
            TYPE_FHE_UINT4 as i16,
            &cleaned_bytes,
        )?;
        assert_eq!(
            cleaned.decrypt(&cks),
            "8",
            "clean_carries must preserve the value (3 << 3 mod 16)"
        );
        // In-process determinism; byte stability across tfhe versions is a
        // separate concern (golden-vector KAT territory).
        assert_eq!(
            cleaned_bytes,
            cleaned_trivial_shift_bytes()?,
            "clean_carries + compress must be deterministic so cleaned \
             ciphertexts stay byte-identical across coprocessors"
        );

        // Legacy-flow control: a compress/decompress round-trip of the
        // operands de-trivializes them, the shift takes the real PBS path,
        // and the result compresses directly — this is why the pre-RFC-020
        // flow never needed carry cleaning.
        let round_trip = |value: u8| -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
            let trivial = trivial_encrypt_be_bytes(TYPE_FHE_UINT4 as i16, &[value])?;
            Ok(SupportedFheCiphertexts::decompress_no_memcheck(
                TYPE_FHE_UINT4 as i16,
                &trivial.compress()?,
            )?)
        };
        let normalized = perform_fhe_operation(
            shl,
            &[round_trip(3)?, round_trip(3)?],
            0,
            TYPE_FHE_UINT4 as i16,
        )?;
        assert!(
            normalized.compress().is_ok(),
            "round-tripped operands must produce a directly compressible result"
        );
        assert_eq!(normalized.decrypt(&cks), "8");

        Ok(())
    })
    .await?
    // no std From<Box<dyn Error + Send + Sync>> for Box<dyn Error>; the
    // annotated closure return coerces by dropping the auto traits
    .map_err(|e| -> Box<dyn std::error::Error> { e })?;

    Ok(())
}

/// End-to-end through the block-scoped worker:
/// TrivialEncrypt(3: u4) << TrivialEncrypt(3: u4) = 8, then 8 +
/// TrivialEncrypt(1: u4) = 9 (the add consumes the forwarded working
/// ciphertext of the shift).
///
/// On CPU the shift result carries residue, so this exercises the
/// scheduler's cleaning recovery end-to-end, and the persisted bytes are
/// asserted byte-identical to an offline clean+compress recomputation —
/// which both proves the cleaning fired and pins cross-coprocessor byte
/// identity. On GPU the dirty premise is not established (see the mechanism
/// test), so there this is a value-level smoke test only.
#[tokio::test]
#[serial(db)]
async fn test_block_scoped_trivial_shift_cleans_carries(
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

    let mut tx = listener_db.new_transaction().await?;
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
        "no allowed computation may error; the carry-cleaning recovery \
         likely failed: {errors:?}"
    );

    let resp = decrypt_handles(&pool, &[shift_handle, sum_handle]).await?;
    assert_eq!(resp.len(), 2, "both allowed handles must have persisted");
    assert_eq!(
        resp[0].value, "8",
        "3 << 3 on FheUint4 must survive carry cleaning with its value intact"
    );
    assert_eq!(
        resp[1].value, "9",
        "the consumer of the shift's working ciphertext must compute 8 + 1"
    );

    // The worker must have persisted the CLEANED bytes: recompute the
    // cleaned compression offline (deterministic) and compare byte-for-byte
    // with the persisted row. This holds only if the shl really received
    // raw trivial operands AND the scheduler cleaned before persisting —
    // the two properties this test exists to guard. CPU-only, like the
    // dirty premise itself.
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
            cleaned_trivial_shift_bytes()
        })
        .await?
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
            "persisted shift bytes must equal the offline clean+compress \
             recomputation (cleaning fired, bytes consensus-stable)"
        );
    }

    Ok(())
}
