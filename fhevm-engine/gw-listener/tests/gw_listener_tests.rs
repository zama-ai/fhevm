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
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::Level;

sol!(
    #[sol(rpc)]
    ZKPoKManager,
    "artifacts/ZKPoKManager.sol/ZKPoKManager.json"
);

struct TestEnvironment {
    wallet: EthereumWallet,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    db_pool: Pool<Postgres>,
    anvil: AnvilInstance,
}

impl TestEnvironment {
    async fn new() -> anyhow::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();

        let conf = ConfigSettings::default();

        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&conf.database_url)
            .await?;

        // Delete all proofs from the database.
        sqlx::query!("TRUNCATE verify_proofs",)
            .execute(&db_pool)
            .await?;

        // Delete last block.
        sqlx::query!("TRUNCATE gw_listener_last_block",)
            .execute(&db_pool)
            .await?;

        let anvil = Anvil::new().block_time(1).try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = signer.clone().into();
        Ok(Self {
            wallet,
            conf,
            cancel_token: CancellationToken::new(),
            db_pool,
            anvil,
        })
    }
}

#[tokio::test]
#[serial(db)]
async fn verify_proof_request_inserted_into_db() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .on_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let zkpok_manager = ZKPoKManager::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *zkpok_manager.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
    );

    let run_handle = tokio::spawn(async move { gw_listener.run().await });

    let contract_address = PrivateKeySigner::random().address();
    let user_address = PrivateKeySigner::random().address();
    let txn_req = zkpok_manager
        .verifyProofRequest(
            U256::from(42),
            contract_address,
            user_address,
            (&[1u8; 2048]).into(),
        )
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());

    loop {
        let rows = sqlx::query!(
            "SELECT zk_proof_id, chain_id, contract_address, user_address, input
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
            break;
        }
        sleep(Duration::from_millis(500)).await;
    }

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}
