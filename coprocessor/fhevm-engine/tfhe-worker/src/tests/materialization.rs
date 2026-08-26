use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    setup_event_harness, wait_until_computed, zero_address, EventHarness,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;

/// Under the minted-in-transaction handle discriminant, a consumer's
/// representation is pinned by its handle: operands minted in the consuming
/// transaction are read raw, everything else is read in its canonical
/// persisted form — and a representation-mixing alias mints a DIFFERENT
/// handle, so no byte-equality obligation ties the two variants below
/// together (on chain they could never collide). What this pins instead:
/// both sourcing shapes compute correctly through the scheduler — a
/// cross-transaction boundary consumer (canonical decompression path) and a
/// local recompute whose add reads its in-transaction trivial encrypts
/// raw — and
/// deterministic trivial encrypts persist byte-identical ciphertexts from
/// either transaction.
#[tokio::test]
#[serial(db)]
async fn boundary_and_local_sourcing_both_compute_and_persist(
) -> Result<(), Box<dyn std::error::Error>> {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
    let caller = zero_address();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");

    // Variant ONE: `combined` consumes persisted, allowed boundary inputs
    // produced by a DIFFERENT transaction.
    let produce_tx = next_handle();
    let boundary_b = next_handle();
    let boundary_c = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, produce_tx, 7, 5, boundary_b, true).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, produce_tx, 5, 5, boundary_c, true).await?;
    allow_handle(&listener_db, &mut tx, &boundary_b).await?;
    allow_handle(&listener_db, &mut tx, &boundary_c).await?;

    let storage_tx = next_handle();
    let combined_from_boundaries = next_handle();
    insert_event(
        &listener_db,
        &mut tx,
        storage_tx,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: boundary_b,
            rhs: boundary_c,
            scalarByte: scalar_flag(false),
            result: combined_from_boundaries,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &combined_from_boundaries).await?;

    // Variant TWO: one transaction recomputes the same inputs locally and
    // combines them. The inputs are allowed (persisted), but the add was
    // minted in the same transaction as its operands, so it consumes the
    // raw working values; on chain its handle folds zero boundary bits and
    // cannot collide with variant ONE's.
    let local_tx = next_handle();
    let local_b = next_handle();
    let local_c = next_handle();
    let combined_locally = next_handle();
    insert_trivial_encrypt(&listener_db, &mut tx, local_tx, 7, 5, local_b, true).await?;
    insert_trivial_encrypt(&listener_db, &mut tx, local_tx, 5, 5, local_c, true).await?;
    allow_handle(&listener_db, &mut tx, &local_b).await?;
    allow_handle(&listener_db, &mut tx, &local_c).await?;
    insert_event(
        &listener_db,
        &mut tx,
        local_tx,
        TfheContractEvents::FheAdd(TfheContract::FheAdd {
            caller,
            lhs: local_b,
            rhs: local_c,
            scalarByte: scalar_flag(false),
            result: combined_locally,
        }),
        true,
    )
    .await?;
    allow_handle(&listener_db, &mut tx, &combined_locally).await?;

    tx.commit().await?;
    wait_until_computed(&app).await?;

    let fetch = |handle: Vec<u8>| {
        let pool = pool.clone();
        async move {
            sqlx::query_scalar::<_, Vec<u8>>("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
                .bind(handle)
                .fetch_one(&pool)
                .await
        }
    };

    // The deterministic trivial encrypts must already persist identical
    // bytes; this isolates a failure below to the add's input sourcing.
    let boundary_b_bytes = fetch(boundary_b.to_vec()).await?;
    let local_b_bytes = fetch(local_b.to_vec()).await?;
    assert_eq!(
        boundary_b_bytes, local_b_bytes,
        "deterministic trivial encrypts must persist identical bytes"
    );

    // Both outputs persist (allowed) and decrypt correctly; their bytes are
    // intentionally NOT compared — the variants mint different handles on
    // chain, so each carries its own byte identity.
    let from_boundaries = fetch(combined_from_boundaries.to_vec()).await?;
    let from_local = fetch(combined_locally.to_vec()).await?;
    assert!(!from_boundaries.is_empty());
    assert!(!from_local.is_empty());

    let plaintexts = decrypt_handles(&pool, &[combined_from_boundaries, combined_locally]).await?;
    assert_eq!(plaintexts[0].value, "12");
    assert_eq!(plaintexts[1].value, "12");
    Ok(())
}

/// Experiment, not a gate: does decompress(compress(x)) reproduce x
/// bit-exactly for noisy production-parameter ciphertexts? FFT rounding
/// makes inexactness possible rather than guaranteed; the answer decides
/// whether an equivocation alias can diverge bytes at all (and therefore
/// whether the drift choreography deserves a gate). Run explicitly:
/// cargo test --release -p tfhe-worker -- --ignored round_trip_bit_exactness
#[test]
#[ignore]
fn compression_round_trip_bit_exactness_survey() {
    use fhevm_engine_common::types::SupportedFheCiphertexts;
    use fhevm_engine_common::utils::{safe_deserialize_key, safe_serialize};
    use tfhe::prelude::*;
    use tfhe::xof_key_set::CompressedXofKeySet;

    let keyset_bytes = std::fs::read("../fhevm-keys/xof-keyset").expect("keyset fixture");
    let keyset: CompressedXofKeySet =
        safe_deserialize_key(&keyset_bytes).expect("deserialize keyset");
    let (compact_public_key, server_key) = keyset
        .decompress()
        .expect("decompress keyset")
        .into_raw_parts();
    tfhe::set_server_key(server_key);

    // Noisy seeds: compact-list encryption carries real encryption noise.
    let mut builder = tfhe::CompactCiphertextList::builder(&compact_public_key);
    builder.push(123_456_789_u64);
    builder.push(987_654_321_u64);
    let expanded = builder.build().expand().expect("expand");
    let a: tfhe::FheUint64 = expanded.get(0).expect("get a").expect("a");
    let mut x: tfhe::FheUint64 = expanded.get(1).expect("get b").expect("b");

    let iterations = 200;
    let mut mismatches = 0usize;
    let mut first_mismatch = None;
    for index in 0..iterations {
        // Grow noise with a real op before each round trip.
        x = &x + &a;
        let ct = SupportedFheCiphertexts::FheUint64(x.clone());
        let before = safe_serialize(&x);
        let ct_type = ct.type_num();
        let compressed = ct.compress().expect("compress");
        let restored = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed)
            .expect("decompress");
        let SupportedFheCiphertexts::FheUint64(x2) = restored else {
            panic!("type changed in round trip");
        };
        let after = safe_serialize(&x2);
        if before != after {
            mismatches += 1;
            if first_mismatch.is_none() {
                first_mismatch = Some(index);
            }
        }
        // Continue the chain from the round-tripped value half the time so
        // both lineages are surveyed.
        if index % 2 == 0 {
            x = x2;
        }
    }
    println!(
        "round-trip survey: {mismatches}/{iterations} bit-mismatches (first at {first_mismatch:?})"
    );
}

/// Deterministic handles disjoint from `next_handle`'s namespace so two app
/// instances (fresh databases) can stage byte-identical fixtures.
#[cfg(feature = "gpu")]
fn byte_gate_handle(index: u8) -> host_listener::database::tfhe_event_propagate::Handle {
    let mut out = [0_u8; 32];
    out[0] = 0x81;
    out[30] = 5; // euint64
    out[31] = index;
    out.into()
}

/// Stages four independent transactions of trivial encrypts + adds and
/// returns every persisted (handle, ciphertext) pair once computed.
#[cfg(feature = "gpu")]
async fn run_byte_gate_fixture() -> Result<Vec<(Vec<u8>, Vec<u8>)>, Box<dyn std::error::Error>> {
    let EventHarness {
        app,
        pool,
        listener_db,
    } = setup_event_harness().await?;
    let caller = zero_address();
    let mut tx = listener_db
        .new_transaction()
        .await?
        .expect("new_transaction() returns Some on a live stack");
    let mut index = 0u8;
    let mut next = || {
        index += 1;
        byte_gate_handle(index)
    };
    let mut all_handles = Vec::new();
    for _ in 0..4 {
        let txid = next();
        let a = next();
        let b = next();
        insert_trivial_encrypt(&listener_db, &mut tx, txid, 11, 5, a, true).await?;
        insert_trivial_encrypt(&listener_db, &mut tx, txid, 31, 5, b, true).await?;
        allow_handle(&listener_db, &mut tx, &a).await?;
        allow_handle(&listener_db, &mut tx, &b).await?;
        let mut lhs = a;
        for _ in 0..3 {
            let out = next();
            insert_event(
                &listener_db,
                &mut tx,
                txid,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs,
                    rhs: b,
                    scalarByte: scalar_flag(false),
                    result: out,
                }),
                true,
            )
            .await?;
            allow_handle(&listener_db, &mut tx, &out).await?;
            all_handles.push(out);
            lhs = out;
        }
        all_handles.push(a);
        all_handles.push(b);
    }
    tx.commit().await?;
    wait_until_computed(&app).await?;
    let mut out = Vec::with_capacity(all_handles.len());
    for handle in all_handles {
        let bytes: Vec<u8> =
            sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
                .bind(handle.to_vec())
                .fetch_one(&pool)
                .await?;
        out.push((handle.to_vec(), bytes));
    }
    Ok(out)
}

/// Release gate (one H100): the persisted ciphertext bytes must not depend
/// on the stream/scheduling shape. Run explicitly:
/// CUDA_VISIBLE_DEVICES=0 cargo test --release -p tfhe-worker --features gpu \
///   -- --ignored gpu_ciphertext_bytes_repeatable_across_stream_counts --nocapture
#[cfg(feature = "gpu")]
#[tokio::test]
#[serial(db)]
#[ignore]
async fn gpu_ciphertext_bytes_repeatable_across_stream_counts(
) -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("FHEVM_TEST_DCID_BATCH_EXECUTION", "true");
    std::env::set_var("FHEVM_TEST_GPU_STREAMS", "1");
    let single = run_byte_gate_fixture().await;
    std::env::set_var("FHEVM_TEST_GPU_STREAMS", "16");
    let wide = run_byte_gate_fixture().await;
    std::env::remove_var("FHEVM_TEST_GPU_STREAMS");
    std::env::remove_var("FHEVM_TEST_DCID_BATCH_EXECUTION");
    let single = single?;
    let wide = wide?;
    assert_eq!(single.len(), wide.len());
    for ((handle, a), (handle2, b)) in single.iter().zip(wide.iter()) {
        assert_eq!(
            handle, handle2,
            "fixture handle order must be deterministic"
        );
        assert_eq!(
            a,
            b,
            "ciphertext bytes for {} differ between 1 and 16 streams/device",
            hex::encode(handle)
        );
    }
    println!("stream-count byte gate: {} handles identical", single.len());
    Ok(())
}

/// Release gate (two homogeneous H100s): the persisted bytes must not depend
/// on the physical device. Digest-file protocol because CUDA visibility is
/// process-level: the first run records digests, the second compares.
/// FHEVM_BYTE_GATE_DIGESTS=/tmp/gate.json CUDA_VISIBLE_DEVICES=0 cargo test ... --ignored gpu_ciphertext_bytes_repeatable_across_physical_devices
/// FHEVM_BYTE_GATE_DIGESTS=/tmp/gate.json CUDA_VISIBLE_DEVICES=1 cargo test ... (same filter)
#[cfg(feature = "gpu")]
#[tokio::test]
#[serial(db)]
#[ignore]
async fn gpu_ciphertext_bytes_repeatable_across_physical_devices(
) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::var("FHEVM_BYTE_GATE_DIGESTS")
        .expect("set FHEVM_BYTE_GATE_DIGESTS to a shared digest file path");
    let rows = run_byte_gate_fixture().await?;
    let digests: Vec<(String, String)> = rows
        .iter()
        .map(|(handle, bytes)| (hex::encode(handle), hex::encode(bytes)))
        .collect();
    let serialized = serde_json::to_string(&digests)?;
    if std::path::Path::new(&path).exists() {
        let previous: Vec<(String, String)> =
            serde_json::from_str(&std::fs::read_to_string(&path)?)?;
        assert_eq!(
            previous, digests,
            "ciphertext digests differ between physical devices"
        );
        println!("device byte gate: {} handles identical", digests.len());
    } else {
        std::fs::write(&path, serialized)?;
        println!(
            "device byte gate: recorded {} digests; rerun on the other device",
            digests.len()
        );
    }
    Ok(())
}
