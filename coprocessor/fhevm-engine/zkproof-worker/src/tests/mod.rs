use serial_test::serial;
use std::sync::Arc;
use std::time::SystemTime;
use test_harness::db_utils::ACL_CONTRACT_ADDR;
use tokio::sync::RwLock;

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
    assert!(utils::is_valid(
        &pool,
        utils::insert_proof(
            &pool,
            102,
            &utils::generate_zk_pok_with_inputs(&pool, &aux.1, &inputs).await,
            &aux.0
        )
        .await
        .expect("valid db insert"),
        5000
    )
    .await
    .expect("non-expired db query"));
}

#[tokio::test]
#[serial(db)]
async fn processes_only_configured_host_chain() {
    let (pool_mngr, instance) = utils::setup_pool().await.expect("valid setup");
    let pool = pool_mngr.pool();

    utils::insert_second_host_chain(&pool).await;

    let selected_chain_request_id =
        utils::insert_valid_proof_for_chain(&pool, 201, utils::DEFAULT_HOST_CHAIN_ID).await;
    let other_chain_request_id =
        utils::insert_valid_proof_for_chain(&pool, 202, utils::SECOND_HOST_CHAIN_ID).await;

    let worker = utils::spawn_worker(
        pool_mngr.clone(),
        utils::build_test_conf(
            instance.db_url.clone(),
            Some(utils::DEFAULT_HOST_CHAIN_ID),
            1,
        ),
    );

    assert!(utils::is_valid(&pool, selected_chain_request_id, 300)
        .await
        .expect("selected proof status"));

    instance.parent_token.cancel();
    let worker_result = worker.await.expect("worker task join");
    assert!(worker_result.is_ok(), "worker result: {worker_result:?}");
    utils::assert_verified_is_null(&pool, other_chain_request_id).await;
}

#[tokio::test]
#[serial(db)]
async fn processes_all_chains_when_filter_unset() {
    let (pool_mngr, instance) = utils::setup_pool().await.expect("valid setup");
    let pool = pool_mngr.pool();

    utils::insert_second_host_chain(&pool).await;

    let first_request_id =
        utils::insert_valid_proof_for_chain(&pool, 301, utils::DEFAULT_HOST_CHAIN_ID).await;
    let second_request_id =
        utils::insert_valid_proof_for_chain(&pool, 302, utils::SECOND_HOST_CHAIN_ID).await;

    let worker = utils::spawn_worker(
        pool_mngr.clone(),
        utils::build_test_conf(instance.db_url.clone(), None, 1),
    );

    assert!(utils::is_valid(&pool, first_request_id, 300)
        .await
        .expect("first proof status"));
    assert!(utils::is_valid(&pool, second_request_id, 300)
        .await
        .expect("second proof status"));

    instance.parent_token.cancel();
    let worker_result = worker.await.expect("worker task join");
    assert!(worker_result.is_ok(), "worker result: {worker_result:?}");
}

#[tokio::test]
#[serial(db)]
async fn fails_startup_for_unknown_configured_host_chain() {
    let (pool_mngr, instance) = utils::setup_pool().await.expect("valid setup");

    let result = crate::verifier::execute_verify_proofs_loop(
        pool_mngr,
        utils::build_test_conf(
            instance.db_url.clone(),
            Some(utils::UNKNOWN_HOST_CHAIN_ID),
            1,
        ),
        Arc::new(RwLock::new(SystemTime::now())),
    )
    .await;

    match result {
        Err(crate::ExecutionError::UnknownChainId(chain_id)) => {
            assert_eq!(chain_id, utils::UNKNOWN_HOST_CHAIN_ID);
        }
        other => panic!(
            "expected UnknownChainId({}), got {other:?}",
            utils::UNKNOWN_HOST_CHAIN_ID
        ),
    }
}
