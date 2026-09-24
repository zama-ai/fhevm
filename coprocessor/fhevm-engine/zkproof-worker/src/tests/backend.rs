use super::utils::{self, ZkInput};
use crate::verifier::{
    proven_list_conformance_params, rerand_and_expand_verified_list, verify_proof,
    verify_proof_only,
};
use crate::ExecutionError;
use fhevm_engine_common::types::{FhevmError, SupportedFheCiphertexts};
use fhevm_engine_common::utils::{safe_deserialize_conformant, safe_serialize};
use serial_test::serial;
use test_harness::db_utils::ACL_CONTRACT_ADDR;
use tfhe::integer::ciphertext::DataKind;

async fn wait_for_verdict(pool: &sqlx::PgPool, request_id: i64) -> (bool, Vec<u8>) {
    tokio::time::timeout(std::time::Duration::from_secs(120), async {
        loop {
            let (verified, handles): (Option<bool>, Option<Vec<u8>>) = sqlx::query_as(
                "SELECT verified, handles FROM verify_proofs WHERE zk_proof_id = $1",
            )
            .bind(request_id)
            .fetch_one(pool)
            .await
            .unwrap();
            if let Some(verified) = verified {
                return (verified, handles.expect("completed request has handles"));
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("worker must complete the request, not leave it pending after a panic")
}

// Run this test with and without --features gpu. All requests go through the
// same single-worker service, so rejection must leave that worker usable.
#[tokio::test]
#[serial(db)]
async fn test_input_types_backend_and_worker_progress() {
    let (pool_mngr, _instance, material) = utils::setup().await.unwrap();
    let pool = pool_mngr.pool();
    let (aux, metadata) = utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let valid = utils::generate_zk_pok_with_inputs(
        &material,
        &metadata,
        &[ZkInput::U8(42), ZkInput::U8(7)],
    )
    .await;

    let key = material.key.clone();
    let crs = material.crs.clone();
    let valid_for_check = valid.clone();
    let aux_for_check = aux.clone();
    let rejected_lists = tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(key.sks.clone());
        let params = proven_list_conformance_params(&key.pks, &crs);
        let list: tfhe::ProvenCompactCiphertextList =
            safe_deserialize_conformant(&valid_for_check, &params).unwrap();
        // Mirror the serde field order to mutate only the unauthenticated type
        // metadata, leaving the real ciphertexts and their ZK proof intact.
        type ListParts = (
            (
                tfhe::shortint::ciphertext::ProvenCompactCiphertextList,
                Vec<DataKind>,
            ),
            tfhe::Tag,
        );
        let ((ciphertexts, info), tag): ListParts =
            bincode::deserialize(&bincode::serialize(&list).unwrap()).unwrap();
        let DataKind::Unsigned(block_count) = info[0] else {
            panic!("expected unsigned fixture");
        };
        let string = DataKind::String {
            n_chars: 1,
            padded: false,
        };
        let unsigned = DataKind::Unsigned(block_count);
        let cases = [
            (vec![string, unsigned], tfhe::FheTypes::AsciiString),
            (vec![unsigned, string], tfhe::FheTypes::AsciiString),
            (
                vec![
                    unsigned,
                    unsigned,
                    DataKind::String {
                        n_chars: 0,
                        padded: false,
                    },
                ],
                tfhe::FheTypes::AsciiString,
            ),
            (
                vec![DataKind::Signed(block_count), unsigned],
                tfhe::FheTypes::Int8,
            ),
            (
                vec![
                    DataKind::Unsigned((block_count.get() - 1).try_into().unwrap()),
                    DataKind::Unsigned((block_count.get() + 1).try_into().unwrap()),
                ],
                tfhe::FheTypes::Uint6,
            ),
        ];

        let mut rejected = cases
            .into_iter()
            .map(|(info, expected)| {
                let crafted: tfhe::ProvenCompactCiphertextList = bincode::deserialize(
                    &bincode::serialize(&((&ciphertexts, info), &tag)).unwrap(),
                )
                .unwrap();
                let bytes = safe_serialize(&crafted);
                // The metadata keeps the same block count. Prove that neither
                // conformance nor ZK verification already rejects the fixture.
                let conformant: tfhe::ProvenCompactCiphertextList =
                    safe_deserialize_conformant(&bytes, &params).unwrap();
                assert!(!conformant
                    .verify(&crs.crs, &key.pks, &metadata)
                    .is_invalid());
                // This stage performs no expansion: the allowlist must reject
                // the type here, rather than relying on extract_ct_list later.
                assert!(matches!(
                    verify_proof_only(1, &bytes, &key, &crs, &aux_for_check),
                    Err(ExecutionError::FailedFhevm(
                        FhevmError::CiphertextExpansionUnsupportedCiphertextKind(kind)
                    )) if kind == expected
                ));
                bytes
            })
            .collect::<Vec<_>>();

        // safe deserialization accepts the unversioned wire format too. Its
        // serde implementation consults the current server key, so exercise it
        // explicitly after a GPU expansion below.
        let string_list: tfhe::ProvenCompactCiphertextList =
            safe_deserialize_conformant(&rejected[0], &params).unwrap();
        let mut unversioned_string = Vec::new();
        tfhe::safe_serialization::SerializationConfig::new(
            fhevm_engine_common::utils::SAFE_SER_DESER_LIMIT,
        )
        .disable_versioning()
        .serialize_into(&string_list, &mut unversioned_string)
        .unwrap();
        rejected.push(unversioned_string);

        // Assert where expansion actually happened, rather than testing only
        // the build feature or the startup log. A CPU key in a GPU build must
        // make this assertion fail.
        let expanded = rerand_and_expand_verified_list(2, &list, &[0; 32], &key).unwrap();
        for ct in &expanded {
            let SupportedFheCiphertexts::FheUint8(ct) = ct else {
                panic!("expected Uint8 input");
            };
            #[cfg(feature = "gpu")]
            assert_eq!(ct.current_device(), tfhe::Device::CudaGpu);
            #[cfg(not(feature = "gpu"))]
            assert_eq!(ct.current_device(), tfhe::Device::Cpu);
        }
        assert_eq!(expanded.len(), 2);
        // Reuse this same thread after GPU expansion. Deserialization of the
        // next unsupported request must still happen on CPU, safely.
        assert!(matches!(
            verify_proof(3, &key, &crs, &aux_for_check, rejected.last().unwrap()),
            Err(ExecutionError::FailedFhevm(
                FhevmError::CiphertextExpansionUnsupportedCiphertextKind(
                    tfhe::FheTypes::AsciiString
                )
            ))
        ));
        rejected
    })
    .await
    .unwrap();

    for (index, input) in rejected_lists.iter().enumerate() {
        let id = 200 + index as i64;
        utils::insert_proof(&pool, id, input, &aux).await.unwrap();
        assert_eq!(wait_for_verdict(&pool, id).await, (false, vec![]));
    }

    let empty = utils::generate_empty_input_list(&material, &metadata).await;
    utils::insert_proof(&pool, 210, &empty, &aux).await.unwrap();
    assert_eq!(wait_for_verdict(&pool, 210).await, (true, vec![]));

    utils::insert_proof(&pool, 211, &valid, &aux).await.unwrap();
    let (verified, handles) = wait_for_verdict(&pool, 211).await;
    assert!(verified);
    assert_eq!(handles.len(), 64);
    let handles: Vec<Vec<u8>> = handles.chunks_exact(32).map(<[u8]>::to_vec).collect();
    let decrypted = utils::decrypt_ciphertexts(&pool, &material, &handles)
        .await
        .unwrap();
    assert_eq!(decrypted.len(), 2);
    assert_eq!(decrypted[0].value, "42");
    assert_eq!(decrypted[1].value, "7");
}
