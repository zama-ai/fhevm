use serial_test::serial;
use test_harness::db_utils::ACL_CONTRACT_ADDR;

use crate::MAX_INPUT_INDEX;

mod utils;

#[tokio::test]
#[serial(db)]
async fn test_verify_proof() {
    let (pool, _instance) = utils::setup().await.expect("valid setup");

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
    let (pool, _instance) = utils::setup().await.expect("valid setup");

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
    let (db, _instance) = utils::setup().await.expect("valid setup");

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());

    // Ensure this fails because we exceed the MAX_INPUT_INDEX constraint
    let inputs = vec![utils::ZkInput::U8(1); MAX_INPUT_INDEX as usize + 2];

    assert!(!utils::is_valid(
        &db,
        utils::insert_proof(
            &db,
            101,
            &utils::generate_zk_pok_with_inputs(&db, &aux.1, &inputs).await,
            &aux.0
        )
        .await
        .expect("valid db insert"),
        50
    )
    .await
    .expect("non-expired db query"));

    // Test with highest number of inputs - 255
    let inputs = vec![utils::ZkInput::U64(2); MAX_INPUT_INDEX as usize + 1];
    assert!(utils::is_valid(
        &db,
        utils::insert_proof(
            &db,
            102,
            &utils::generate_zk_pok_with_inputs(&db, &aux.1, &inputs).await,
            &aux.0
        )
        .await
        .expect("valid db insert"),
        500
    )
    .await
    .expect("non-expired db query"));
}
