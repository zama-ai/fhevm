use std::time::Duration;

use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{FixedBytes, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
    sol,
};

use gw_listener::{gw_listener::GatewayListener, ConfigSettings};
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use test_harness::instance::ImportMode;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

struct TestEnvironment {
    wallet: EthereumWallet,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    _test_instance: Option<test_harness::instance::DBInstance>, // maintain db alive
    db_pool: Pool<Postgres>,
    anvil: AnvilInstance,
}

impl TestEnvironment {
    async fn new() -> anyhow::Result<Self> {
        let mut conf = ConfigSettings::default();

        let mut _test_instance = None;
        if std::env::var("FORCE_DATABASE_URL").is_err() {
            let instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
                .await
                .expect("valid db instance");
            eprintln!("New test database on {}", instance.db_url());
            conf.database_url = instance.db_url.clone();
            _test_instance = Some(instance);
        };
        conf.error_sleep_initial_secs = 1;
        conf.error_sleep_max_secs = 1;
        let db_pool = PgPoolOptions::new()
            .max_connections(16)
            .acquire_timeout(Duration::from_secs(5))
            .connect(conf.database_url.as_str())
            .await?;

        // Delete all proofs from the database.
        sqlx::query!("TRUNCATE verify_proofs",)
            .execute(&db_pool)
            .await?;

        // Delete last block.
        sqlx::query!("TRUNCATE gw_listener_last_block",)
            .execute(&db_pool)
            .await?;

        let anvil = Anvil::new().block_time(1).chain_id(12345).try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = signer.into();
        Ok(Self {
            wallet,
            conf,
            cancel_token: CancellationToken::new(),
            db_pool,
            _test_instance,
            anvil,
        })
    }
}

const RETRY_EVENT_TO_DB: u64 = 20;
const RETRY_DELAY: Duration = Duration::from_millis(500);

#[tokio::test]
#[serial(db)]
async fn verify_proof_request_inserted_into_db() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let input_verification = InputVerification::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
    );

    let db_pool = env.db_pool.clone();
    let run_handle = tokio::spawn(async move { gw_listener.run(db_pool).await });

    let contract_address = PrivateKeySigner::random().address();
    let user_address = PrivateKeySigner::random().address();
    let txn_req = input_verification
        .verifyProofRequest(
            U256::from(42),
            contract_address,
            user_address,
            (&[1u8; 2048]).into(),
            Vec::<u8>::new().into(),
        )
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query!(
            "SELECT zk_proof_id, chain_id, contract_address, user_address, input, extra_data
             FROM verify_proofs",
        )
        .fetch_all(&env.db_pool)
        .await?;
        if !rows.is_empty() {
            let row = &rows[0];
            assert_eq!(row.chain_id, 42);
            assert_eq!(row.contract_address, contract_address.to_string());
            assert_eq!(row.user_address, user_address.to_string());
            assert_eq!(row.input, Some([1u8; 2048].to_vec()));
            assert!(row.extra_data.is_empty());
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for event to be processed"
        );
    }

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn quorum_acceptance_schedules_locally_rejected_proof_for_replay() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let input_verification = InputVerification::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
    );
    let db_pool = env.db_pool.clone();
    let run_handle = tokio::spawn(async move { gw_listener.run(db_pool).await });

    let request = input_verification.verifyProofRequest(
        U256::from(42),
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        (&[1u8; 32]).into(),
        Vec::<u8>::new().into(),
    );
    assert!(request.send().await?.get_receipt().await?.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let inserted = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM verify_proofs")
            .fetch_one(&env.db_pool)
            .await?;
        if inserted == 1 {
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for proof request"
        );
    }

    sqlx::query(
        "UPDATE verify_proofs
         SET verified = FALSE, verified_at = NULL, handles = NULL
         WHERE zk_proof_id = 0",
    )
    .execute(&env.db_pool)
    .await?;

    let consensus_handles = vec![FixedBytes::from([7u8; 32])];
    let response = input_verification.emitVerifyProofResponse(U256::ZERO, consensus_handles);
    assert!(response.send().await?.get_receipt().await?.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let row = sqlx::query_as::<_, (Option<bool>, Option<Vec<u8>>, i32, bool, bool)>(
            "SELECT verified, handles, retry_count,
                    verified_at IS NOT NULL, last_retry_at IS NULL
             FROM verify_proofs WHERE zk_proof_id = 0",
        )
        .fetch_one(&env.db_pool)
        .await?;
        if row.1.is_some() {
            assert!(row.0.is_none());
            assert_eq!(row.1, Some(vec![7u8; 32]));
            assert_eq!(row.2, 0);
            assert!(row.3);
            assert!(row.4);
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for quorum acceptance"
        );
    }

    sqlx::query(
        "UPDATE verify_proofs
         SET retry_count = 3, last_retry_at = NOW()
         WHERE zk_proof_id = 0",
    )
    .execute(&env.db_pool)
    .await?;
    let duplicate_receipt = input_verification
        .emitVerifyProofResponse(U256::ZERO, vec![FixedBytes::from([7u8; 32])])
        .send()
        .await?
        .get_receipt()
        .await?;
    assert!(duplicate_receipt.status());
    let duplicate_block = duplicate_receipt
        .block_number
        .expect("receipt has block number") as i64;

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let processed = sqlx::query_scalar::<_, Option<i64>>(
            "SELECT last_block_num FROM gw_listener_last_block WHERE dummy_id = TRUE",
        )
        .fetch_optional(&env.db_pool)
        .await?
        .flatten()
        .is_some_and(|block| block >= duplicate_block);
        if processed {
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for duplicate acceptance"
        );
    }
    let replay_state = sqlx::query_as::<_, (i32, bool)>(
        "SELECT retry_count, last_retry_at IS NOT NULL
         FROM verify_proofs WHERE zk_proof_id = 0",
    )
    .fetch_one(&env.db_pool)
    .await?;
    assert_eq!(replay_state.0, 3);
    assert!(replay_state.1);

    let handle = vec![7u8; 32];
    let ciphertext = vec![1u8];
    test_harness::db_utils::insert_ciphertext64(&env.db_pool, &handle, &ciphertext).await?;
    let available_receipt = input_verification
        .emitVerifyProofResponse(U256::ZERO, vec![FixedBytes::from([7u8; 32])])
        .send()
        .await?
        .get_receipt()
        .await?;
    assert!(available_receipt.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let retained = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM verify_proofs WHERE zk_proof_id = 0",
        )
        .fetch_one(&env.db_pool)
        .await?;
        if retained == 0 {
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for already-available proof cleanup"
        );
    }

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn quorum_rejection_discards_retained_proof_without_replay() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let input_verification = InputVerification::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
    );
    let db_pool = env.db_pool.clone();
    let run_handle = tokio::spawn(async move { gw_listener.run(db_pool).await });

    let request = input_verification.verifyProofRequest(
        U256::from(42),
        PrivateKeySigner::random().address(),
        PrivateKeySigner::random().address(),
        (&[1u8; 32]).into(),
        Vec::<u8>::new().into(),
    );
    assert!(request.send().await?.get_receipt().await?.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let inserted = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM verify_proofs")
            .fetch_one(&env.db_pool)
            .await?;
        if inserted == 1 {
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for proof request"
        );
    }

    sqlx::query(
        "UPDATE verify_proofs
         SET verified = FALSE, verified_at = NULL, handles = NULL
         WHERE zk_proof_id = 0",
    )
    .execute(&env.db_pool)
    .await?;
    let response = input_verification.emitRejectProofResponse(U256::ZERO);
    assert!(response.send().await?.get_receipt().await?.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let remaining = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM verify_proofs")
            .fetch_one(&env.db_pool)
            .await?;
        if remaining == 0 {
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for quorum rejection"
        );
    }

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}
