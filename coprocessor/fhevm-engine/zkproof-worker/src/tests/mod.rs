use std::{
    sync::{atomic::AtomicI64, Arc},
    time::{Duration, SystemTime},
};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::pg_pool::PostgresPoolManager;
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use serial_test::serial;
use test_harness::db_utils::ACL_CONTRACT_ADDR;
use test_harness::instance::ImportMode;
use tokio::sync::RwLock;

use crate::MAX_INPUT_INDEX;

mod utils;

#[tokio::test]
#[serial(db)]
async fn test_verify_proof() {
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    // Generate Valid ZkPok
    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&material, &aux.1).await;
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

    let request_id_null = 103;
    sqlx::query(
        "INSERT INTO verify_proofs
             (zk_proof_id, input, chain_id, contract_address, user_address, verified)
         VALUES ($1, NULL, $2, $3, $4, NULL)",
    )
    .bind(request_id_null)
    .bind(aux.chain_id.as_i64())
    .bind(aux.contract_address)
    .bind(aux.user_address)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("SELECT pg_notify('fhevm', '')")
        .execute(&pool)
        .await
        .unwrap();

    for retry in 0..max_retries {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let state = sqlx::query_as::<_, (Option<bool>, Option<Vec<u8>>)>(
            "SELECT verified, handles FROM verify_proofs WHERE zk_proof_id = $1",
        )
        .bind(request_id_null)
        .fetch_one(&pool)
        .await
        .unwrap();
        if state.0 == Some(false) {
            assert_eq!(state.1, Some(Vec::new()));
            break;
        }
        assert!(retry < max_retries - 1, "NULL input was not rejected");
    }
}

#[tokio::test]
#[serial(db)]
async fn test_quorum_accepted_proof_is_replayed_only_locally() {
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux = utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&material, &aux.1).await;
    let request_id = utils::insert_proof(&pool, 150, &zk_pok, &aux.0)
        .await
        .unwrap();
    let handles = utils::wait_for_handles(&pool, request_id, 1000)
        .await
        .unwrap();
    assert!(
        !handles.is_empty(),
        "initial verification should produce handles"
    );
    let handles = handles.concat();

    sqlx::query(
        "UPDATE verify_proofs
         SET verified = NULL,
             verified_at = NOW(),
             handles = $2,
             retry_count = 0,
             last_retry_at = NULL
         WHERE zk_proof_id = $1",
    )
    .bind(request_id)
    .bind(handles)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("SELECT pg_notify('fhevm', '')")
        .execute(&pool)
        .await
        .unwrap();

    for retry in 0..1000 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let remaining: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM verify_proofs WHERE zk_proof_id = $1")
                .bind(request_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        if remaining == 0 {
            return;
        }
        assert!(
            retry < 999,
            "successful local replay should delete the proof row"
        );
    }
}

#[tokio::test]
#[serial(db)]
async fn test_failed_replay_is_delayed_and_retained() {
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux = utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&material, &aux.1).await;
    let request_id = utils::insert_proof(&pool, 150, &zk_pok, &aux.0)
        .await
        .unwrap();
    utils::wait_for_handles(&pool, request_id, 1000)
        .await
        .unwrap();

    sqlx::query(
        "UPDATE verify_proofs
         SET verified = NULL,
             verified_at = NOW(),
             handles = $2,
             retry_count = 0,
             last_retry_at = NULL
         WHERE zk_proof_id = $1",
    )
    .bind(request_id)
    .bind(vec![7u8; 32])
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("SELECT pg_notify('fhevm', '')")
        .execute(&pool)
        .await
        .unwrap();

    for retry in 0..1000 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let row = sqlx::query_as::<_, (i32, Option<String>, bool)>(
            "SELECT retry_count, last_error, last_retry_at IS NOT NULL
             FROM verify_proofs WHERE zk_proof_id = $1",
        )
        .bind(request_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        if row.0 == 1 {
            assert!(row.1.is_some());
            assert!(row.2);
            break;
        }
        assert!(retry < 999, "failed replay should record one retry");
    }

    sqlx::query(
        "UPDATE verify_proofs
         SET verified_at = NOW() - INTERVAL '8 days', last_retry_at = NULL
         WHERE zk_proof_id = $1",
    )
    .bind(request_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("SELECT pg_notify('fhevm', '')")
        .execute(&pool)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;

    let retry_count: i32 =
        sqlx::query_scalar("SELECT retry_count FROM verify_proofs WHERE zk_proof_id = $1")
            .bind(request_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(retry_count, 1, "expired replay payload should be retained");
}

#[tokio::test]
#[serial(db)]
async fn test_rolled_back_claim_is_reprocessed_exactly_once() {
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux = utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&material, &aux.1).await;
    let request_id: i64 = 201;

    // Insert WITHOUT notifying so the idle worker doesn't race us to the row.
    sqlx::query(
        "INSERT INTO verify_proofs (zk_proof_id, input, chain_id, contract_address, user_address, verified)
         VALUES ($1, $2, $3, $4, $5, NULL)",
    )
    .bind(request_id)
    .bind(&zk_pok)
    .bind(aux.0.chain_id.as_i64())
    .bind(aux.0.contract_address.clone())
    .bind(aux.0.user_address.clone())
    .execute(&pool)
    .await
    .unwrap();

    // A worker that claims the row then crashes mid-flight: claim it, never commit.
    {
        let mut txn = pool.begin().await.unwrap();
        let claimed = sqlx::query(
            "SELECT zk_proof_id FROM verify_proofs
             WHERE verified IS NULL AND zk_proof_id = $1
             FOR UPDATE SKIP LOCKED",
        )
        .bind(request_id)
        .fetch_optional(&mut *txn)
        .await
        .unwrap();
        assert!(
            claimed.is_some(),
            "freshly inserted row should be claimable"
        );
        txn.rollback().await.unwrap(); // == crash before commit
    }

    // Rollback left no partial state: the row is still pending.
    let verified: Option<bool> =
        sqlx::query_scalar("SELECT verified FROM verify_proofs WHERE zk_proof_id = $1")
            .bind(request_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        verified.is_none(),
        "row must stay unverified after a mid-flight crash"
    );

    // Wake the worker; it must re-pick and complete the request.
    sqlx::query("SELECT pg_notify('fhevm', '')")
        .execute(&pool)
        .await
        .unwrap();
    assert!(
        utils::is_valid(&pool, request_id, 1000).await.unwrap(),
        "request must be re-processed after a mid-flight crash"
    );

    let verified_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM verify_proofs WHERE zk_proof_id = $1 AND verified = true",
    )
    .bind(request_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(verified_count, 1, "request must be verified exactly once");
}

// A transient backend termination (DB still up) must not take the worker down:
// sqlx reconnects, so it should keep processing.
#[tokio::test]
#[serial(db)]
async fn test_worker_recovers_after_backend_termination() {
    let instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
        .await
        .expect("valid db instance");

    let conf = crate::Config {
        database_url: instance.db_url.clone(),
        listen_database_channel: "fhevm".to_string(),
        notify_database_channel: "notify".to_string(),
        pg_pool_connections: 10,
        pg_polling_interval: 60,
        worker_thread_count: 1,
        pg_timeout: Duration::from_secs(60),
        pg_auto_explain_with_min_duration: None,
        gcs_mode: false,
    };

    let pool_mngr = PostgresPoolManager::connect_pool(
        instance.parent_token.child_token(),
        conf.database_url.as_str(),
        conf.pg_timeout,
        conf.pg_pool_connections,
        Duration::from_secs(2),
        conf.pg_auto_explain_with_min_duration,
    )
    .await
    .expect("pool should connect");

    let pool = pool_mngr.pool();

    // Build the proof before starting the worker. Generating it loads the large
    // keyset and CRS; doing that while the worker is also loading them contends
    // for the shared pool and can exhaust the acquire timeout on a slow runner.
    let aux = utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let material = utils::load_proof_material(&pool)
        .await
        .expect("proof material should load");
    let zk_pok = utils::generate_sample_zk_pok(&material, &aux.1).await;

    let _service_task = tokio::spawn(crate::verifier::execute_verify_proofs_loop(
        pool_mngr,
        conf,
        Arc::new(RwLock::new(SystemTime::now())),
        Arc::new(AtomicI64::new(-1)),
    ));

    // Process one proof so the worker is fully up and running.
    let first = utils::insert_proof(&pool, 301, &zk_pok, &aux.0)
        .await
        .unwrap();
    assert!(
        utils::is_valid(&pool, first, 2000).await.unwrap(),
        "worker should verify a proof before the disconnect"
    );

    // Sever every backend connection; the DB itself stays up.
    let admin_pool = sqlx::PgPool::connect(instance.db_url.as_str())
        .await
        .expect("admin pool should connect");
    let terminated: Vec<bool> = sqlx::query_scalar(
        "SELECT pg_terminate_backend(pid)
         FROM pg_stat_activity
         WHERE datname = current_database()
           AND pid <> pg_backend_pid()",
    )
    .fetch_all(&admin_pool)
    .await
    .expect("backend termination should succeed");
    assert!(
        !terminated.is_empty() && terminated.into_iter().all(|res| res),
        "expected to terminate at least one worker backend"
    );

    // The worker must reconnect (listener + pool) and verify a new proof.
    let second = utils::insert_proof(&pool, 302, &zk_pok, &aux.0)
        .await
        .unwrap();
    assert!(
        utils::is_valid(&pool, second, 2000).await.unwrap(),
        "worker should reconnect and verify a proof after backend termination"
    );
}

#[tokio::test]
#[serial(db)]
async fn test_verify_empty_input_list() {
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let input = utils::generate_empty_input_list(&material, &aux.1).await;
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
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
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
            &utils::generate_zk_pok_with_inputs(&material, &aux.1, &inputs).await,
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
        &utils::generate_zk_pok_with_inputs(&material, &aux.1, &inputs).await,
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
    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
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
    let zk_pok = utils::generate_zk_pok_with_inputs(&material, &aux.1, &inputs).await;
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

    let baseline = utils::compress_inputs_without_rerandomization(&material, &zk_pok)
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

    let decrypted = utils::decrypt_ciphertexts(&pool, &material, &handles)
        .await
        .unwrap();
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
/// in `host_chains` must not stop processing for chains that are registered.
///
/// The worker's SELECT pre-filters by HostChainsCache, so unknown-chain rows
/// are never fetched. They stay `verified IS NULL` and will be picked up on
/// the next poll if/when the chain gets registered (cache reloads on pod
/// restart, forced by the chart's checksum/host-chains annotation).
///
/// This enforces the invariant "one unknown chain on the queue must not stop
/// processing for known chains" (Amina's invariant) — violated by the
/// Polygon-on-zws-dev incident where a missing Polygon row in `host_chains`
/// took Sepolia down with it.
#[tokio::test]
#[serial(db)]
async fn test_unknown_chain_id_does_not_stop_known_chain_processing() {
    use sqlx::Row;

    let (pool_mngr, _instance, material) = utils::setup().await.expect("valid setup");
    let pool = pool_mngr.pool();

    let aux: (crate::auxiliary::ZkData, [u8; 92]) =
        utils::aux_fixture(ACL_CONTRACT_ADDR.to_owned());
    let zk_pok = utils::generate_sample_zk_pok(&material, &aux.1).await;

    // Request 1 (LOWER zk_proof_id): chain_id 99_999, not in host_chains.
    // Before the filter, this would be the first row the worker fetched and
    // would crash all workers on the poison message. With the filter, it is
    // simply not selected — the worker moves on to request 2.
    let mut aux_unknown = aux.0.clone();
    aux_unknown.chain_id = ChainId::try_from(99_999_u64).expect("valid u64 -> ChainId");
    let request_id_unknown = utils::insert_proof(&pool, 301, &zk_pok, &aux_unknown)
        .await
        .unwrap();

    // Request 2 (HIGHER zk_proof_id): chain_id 12_345, the registered chain
    // from setup_test_db. The worker should pick this up (filter excludes
    // request 1) and verify it successfully.
    let request_id_known = utils::insert_proof(&pool, 302, &zk_pok, &aux.0)
        .await
        .unwrap();

    // Known chain: must be processed normally.
    assert!(
        utils::is_valid(&pool, request_id_known, 1000)
            .await
            .unwrap(),
        "registered-chain request should be verified despite an unknown-chain row sitting at lower zk_proof_id",
    );

    // Unknown chain: must remain `verified IS NULL`. It is waiting for its
    // chain to be registered; the row is recoverable, not failed.
    let row = sqlx::query("SELECT verified FROM verify_proofs WHERE zk_proof_id = $1")
        .bind(request_id_unknown)
        .fetch_one(&pool)
        .await
        .unwrap();
    let verified: Option<bool> = row.try_get("verified").unwrap();
    assert!(
        verified.is_none(),
        "unregistered-chain request should stay verified=NULL (waiting), not be marked failed",
    );
}
