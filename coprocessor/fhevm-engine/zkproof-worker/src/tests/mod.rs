use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use serial_test::serial;
use test_harness::db_utils::ACL_CONTRACT_ADDR;

use crate::MAX_INPUT_INDEX;

mod utils;

#[tokio::test]
#[serial(db)]
async fn test_verify_proof() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    // Generate Valid ZkPok
    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&pool, &aux.1).await;
    // Insert ZkPok into database
    let request_id_valid = utils::insert_proof(&pool, 101, &zk_pok, &aux.0)
        .await
        .unwrap();

    // Generate ZkPok with invalid aux data
    let mut aux = aux.0.clone();
    aux.user_address = "0x".to_owned() + &"1".repeat(40);
    let request_id_invalid = utils::insert_proof(&pool, 102, &zk_pok, &aux)
        .await
        .unwrap();

    let max_retries = 1000;

    // Check if it's valid
    assert!(utils::is_valid(&pool, request_id_valid, max_retries)
        .await
        .unwrap(),);

    // Check if it's invalid
    assert!(!utils::is_valid(&pool, request_id_invalid, max_retries)
        .await
        .unwrap());
}

#[tokio::test]
#[serial(db)]
async fn test_verify_empty_input_list() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let input = utils::generate_empty_input_list(&pool, &aux.1).await;
    let request_id = utils::insert_proof(&pool, 101, &input, &aux.0)
        .await
        .unwrap();

    let max_retries = 50;

    assert!(utils::is_valid(&pool, request_id, max_retries)
        .await
        .unwrap());

    let handles = utils::wait_for_handles(&pool, request_id, max_retries)
        .await
        .unwrap();
    assert!(handles.is_empty());
    assert!(utils::fetch_stored_ciphertexts(&pool, &handles)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
#[serial(db)]
async fn test_max_input_index() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());

    // Ensure this fails because we exceed the MAX_INPUT_INDEX constraint
    let inputs = vec![utils::ZkInput::U8(1); MAX_INPUT_INDEX as usize + 2];

    assert!(!utils::is_valid(
        &pool,
        utils::insert_proof(
            &pool,
            101,
            &utils::generate_zk_pok_with_inputs(&pool, &aux.1, &inputs).await,
            &aux.0
        )
        .await
        .expect("valid db insert"),
        5000
    )
    .await
    .expect("non-expired db query"));

    // Test with highest number of inputs - 255
    let inputs = vec![utils::ZkInput::U64(2); MAX_INPUT_INDEX as usize + 1];
    let request_id = utils::insert_proof(
        &pool,
        102,
        &utils::generate_zk_pok_with_inputs(&pool, &aux.1, &inputs).await,
        &aux.0,
    )
    .await
    .expect("valid db insert");
    assert!(utils::is_valid(&pool, request_id, 5000)
        .await
        .expect("non-expired db query"));

    let handles = utils::wait_for_handles(&pool, request_id, 5000)
        .await
        .expect("wait for handles");
    assert_eq!(handles.len(), MAX_INPUT_INDEX as usize + 1);
    assert_eq!(handles.first().expect("first handle")[21], 0);
    assert_eq!(handles.last().expect("last handle")[21], MAX_INPUT_INDEX);
    assert_eq!(
        &handles.last().expect("last handle")[22..30],
        &aux.0.chain_id.as_u64().to_be_bytes()
    );
    assert_eq!(
        handles.last().expect("last handle")[31],
        current_ciphertext_version() as u8
    );
}

#[tokio::test]
#[serial(db)]
async fn test_verify_proof_rerandomises_ciphertexts_before_storage() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let inputs = vec![
        utils::ZkInput::Bool(true),
        utils::ZkInput::U8(42),
        utils::ZkInput::U16(12345),
        utils::ZkInput::U32(67890),
        utils::ZkInput::U64(1234567890),
    ];
    let zk_pok = utils::generate_zk_pok_with_inputs(&pool, &aux.1, &inputs).await;
    let request_id = utils::insert_proof(&pool, 103, &zk_pok, &aux.0)
        .await
        .unwrap();

    assert!(utils::is_valid(&pool, request_id, 1000).await.unwrap());

    let handles = utils::wait_for_handles(&pool, request_id, 1000)
        .await
        .unwrap();
    assert_eq!(handles.len(), inputs.len());
    for (idx, handle) in handles.iter().enumerate() {
        assert_eq!(handle.len(), 32);
        assert_eq!(handle[21], idx as u8);
        assert_eq!(&handle[22..30], &aux.0.chain_id.as_u64().to_be_bytes());
        assert_eq!(handle[31], current_ciphertext_version() as u8);
    }

    let stored = utils::fetch_stored_ciphertexts(&pool, &handles)
        .await
        .unwrap();
    assert_eq!(stored.len(), inputs.len());
    assert_eq!(
        stored
            .iter()
            .map(|ct| ct.input_blob_index)
            .collect::<Vec<_>>(),
        (0..inputs.len() as i32).collect::<Vec<_>>()
    );
    assert_eq!(
        stored
            .iter()
            .map(|ct| ct.handle.as_slice())
            .collect::<Vec<_>>(),
        handles
            .iter()
            .map(|handle| handle.as_slice())
            .collect::<Vec<_>>()
    );

    let baseline = utils::compress_inputs_without_rerandomization(&pool, &zk_pok)
        .await
        .unwrap();
    assert_eq!(baseline.len(), stored.len());
    assert!(
        stored
            .iter()
            .zip(&baseline)
            .all(|(stored_ct, baseline_ct)| stored_ct.ciphertext != *baseline_ct),
        "stored ciphertexts should differ from the pre-rerandomization compression"
    );

    let decrypted = utils::decrypt_ciphertexts(&pool, &handles).await.unwrap();
    assert_eq!(
        decrypted
            .iter()
            .map(|result| result.value.clone())
            .collect::<Vec<_>>(),
        inputs
            .iter()
            .map(|input| input.cleartext())
            .collect::<Vec<_>>()
    );
}

/// Regression: a proof request referencing a chain_id that is not registered
/// in `host_chains` must not poison the worker. It is marked
/// `verified = false` and the worker keeps processing subsequent requests.
///
/// This enforces the invariant "one unknown chain on the queue must not stop
/// processing for known chains" (Amina's invariant) — violated by the
/// Polygon-on-zws-dev incident where a missing Polygon row in `host_chains`
/// took Sepolia down with it.
#[tokio::test]
#[serial(db)]
async fn test_unknown_chain_id_is_skipped_not_panicked() {
    let (pool_mngr, _instance) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&pool, &aux.1).await;

    // Request 1: chain_id 99_999 — not in host_chains. After the fix, the
    // worker must mark it verified=false and continue, NOT propagate an
    // UnknownChainId error that kills the worker on a poison message.
    let mut aux_unknown = aux.0.clone();
    aux_unknown.chain_id = ChainId::try_from(99_999_u64).expect("valid u64 -> ChainId");
    let request_id_unknown = utils::insert_proof(&pool, 301, &zk_pok, &aux_unknown)
        .await
        .unwrap();

    // Request 2: chain_id 12_345 — the registered chain from setup_test_db.
    // Inserted AFTER the unknown one, so if the worker had panicked on
    // request 1 this request would stay `verified IS NULL` (the failure
    // mode from the production incident).
    let request_id_known = utils::insert_proof(&pool, 302, &zk_pok, &aux.0)
        .await
        .unwrap();

    let max_retries = 1000;

    // Unknown chain: verified resolves to `false` (skipped, not panicked).
    assert!(
        !utils::is_valid(&pool, request_id_unknown, max_retries)
            .await
            .unwrap(),
        "unknown-chain request should be marked verified=false, not left at NULL",
    );

    // Known chain: the worker survived the unknown-chain skip and verifies
    // this one normally.
    assert!(
        utils::is_valid(&pool, request_id_known, max_retries)
            .await
            .unwrap(),
        "known-chain request should be processed after unknown-chain skip",
    );
}
