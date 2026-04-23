use std::time::Duration;

use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::U256,
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
        let wallet = signer.clone().into();
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

    let run_handle = tokio::spawn(async move { gw_listener.run().await });

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
