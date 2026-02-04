use std::time::Duration;

use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder, WalletProvider, WsConnect};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use tokio::time::sleep;

use fhevm_engine_common::utils::DatabaseURL;
use host_listener::database::tfhe_event_propagate::Database;
use host_listener::poller::{run_poller, PollerConfig};
use test_harness::instance::ImportMode;

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    FHEVMExecutorTest,
    "artifacts/FHEVMExecutorTest.sol/FHEVMExecutorTest.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    ACLTest,
    "artifacts/ACLTest.sol/ACLTest.json"
);

#[tokio::test]
#[serial(db)]
async fn poller_state_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_instance.db_url())
        .await?;

    let coprocessor_api_key =
        sqlx::query!("SELECT tenant_api_key FROM tenants LIMIT 1")
            .fetch_one(&pool)
            .await?
            .tenant_api_key;

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let mut db = Database::new(&db_url, &coprocessor_api_key, 128).await?;
    let chain_id = i64::try_from(db.chain_id).unwrap();

    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id)
        .execute(&pool)
        .await?;

    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, None);

    db.poller_set_last_caught_up_block(chain_id, 5).await?;
    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, Some(5));

    db.reconnect().await;
    db.poller_set_last_caught_up_block(chain_id, 7).await?;
    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, Some(7));

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EventKind {
    Tfhe,
    Acl,
}

#[tokio::test]
#[serial(db)]
async fn poller_catches_up_to_safe_tip(
) -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(db_instance.db_url())
        .await?;

    let coprocessor_api_key: Uuid =
        sqlx::query!("SELECT tenant_api_key FROM tenants LIMIT 1")
            .fetch_one(&pool)
            .await?
            .tenant_api_key;

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let db = Database::new(&db_url, &coprocessor_api_key, 128).await?;
    let chain_id = db.chain_id as i64;
    let tenant_id = db.tenant_id;
    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id)
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE chain_id = $1")
        .bind(chain_id)
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM computations WHERE tenant_id = $1")
        .bind(tenant_id)
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM allowed_handles WHERE tenant_id = $1")
        .bind(tenant_id)
        .execute(&pool)
        .await?;

    // Spin up a local chain and emit events so the poller starts behind the head.
    let anvil = Anvil::new().chain_id(chain_id as u64).spawn();
    let ws_url = anvil.ws_endpoint();
    let http_url = anvil.endpoint();

    let signer: PrivateKeySigner = anvil.first_key().clone().into();
    let wallet = EthereumWallet::new(signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .connect_ws(WsConnect::new(ws_url.clone()))
        .await?;

    let tfhe_contract = FHEVMExecutorTest::deploy(provider.clone()).await?;
    let acl_contract = ACLTest::deploy(provider.clone()).await?;
    let signer_address = provider
        .signer_addresses()
        .next()
        .expect("anvil provides at least one signer");

    let mut receipts: Vec<(u64, EventKind)> = Vec::new();
    for i in 0..3u64 {
        let tfhe_txn_req = tfhe_contract
            .trivialEncrypt(U256::from(i + 1), 4_u8)
            .into_transaction_request();
        let tfhe_receipt = provider
            .send_transaction(tfhe_txn_req)
            .await?
            .get_receipt()
            .await?;
        assert!(tfhe_receipt.status());
        receipts.push((
            tfhe_receipt
                .block_number
                .expect("trivialEncrypt block number"),
            EventKind::Tfhe,
        ));

        let acl_txn_req = acl_contract
            .allow(U256::from(i + 1).into(), signer_address)
            .into_transaction_request();
        let acl_receipt = provider
            .send_transaction(acl_txn_req)
            .await?
            .get_receipt()
            .await?;
        assert!(acl_receipt.status());
        receipts.push((
            acl_receipt.block_number.expect("allow block number"),
            EventKind::Acl,
        ));
    }

    let latest_block = provider.get_block_number().await?;
    let finality_lag = 2u64;
    let safe_tip = latest_block.saturating_sub(finality_lag);

    let expected_tfhe = receipts
        .iter()
        .filter(|(block, kind)| *block <= safe_tip && *kind == EventKind::Tfhe)
        .count() as i64;
    let expected_acl = receipts
        .iter()
        .filter(|(block, kind)| *block <= safe_tip && *kind == EventKind::Acl)
        .count() as i64;
    assert!(expected_tfhe > 0, "no finalized TFHE events to ingest");
    assert!(expected_acl > 0, "no finalized ACL events to ingest");

    let config = PollerConfig {
        url: http_url,
        acl_address: *acl_contract.address(),
        tfhe_address: *tfhe_contract.address(),
        database_url: db_url.clone(),
        coprocessor_api_key,
        finality_lag,
        batch_size: 2,
        poll_interval: Duration::from_millis(200),
        retry_interval: Duration::from_millis(200),
        service_name: String::new(),
        max_http_retries: 0,
        rpc_compute_units_per_second: 1000,
        health_port: 18081,
        dependence_cache_size: 10_000,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_rate_per_min: 0,
        dependent_ops_burst: 0,
    };

    let poller_handle = tokio::spawn(run_poller(config));

    // Wait for the poller to advance to the safe tip.
    let mut attempts = 0;
    loop {
        let anchor = sqlx::query_scalar::<_, i64>(
            "SELECT last_caught_up_block FROM host_listener_poller_state \
             WHERE chain_id = $1",
        )
        .bind(chain_id)
        .fetch_optional(&pool)
        .await?;

        if anchor.map(|a| a as u64) == Some(safe_tip) {
            break;
        }

        attempts += 1;
        if attempts > 100 {
            poller_handle.abort();
            panic!(
                "host listener poller did not reach safe tip {safe_tip} (latest block \
                 {latest_block})"
            );
        }

        sleep(Duration::from_millis(100)).await;
    }

    // Allow the last ingest transaction to complete before stopping the task.
    sleep(Duration::from_millis(200)).await;
    poller_handle.abort();
    let _ = poller_handle.await;

    let computations_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM computations WHERE tenant_id = $1",
    )
    .bind(tenant_id)
    .fetch_one(&pool)
    .await?;
    let allowed_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM allowed_handles WHERE tenant_id = $1",
    )
    .bind(tenant_id)
    .fetch_one(&pool)
    .await?;
    let last_valid_block = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT MAX(block_number) FROM host_chain_blocks_valid \
         WHERE chain_id = $1",
    )
    .bind(chain_id)
    .fetch_one(&pool)
    .await?
    .unwrap_or_default();

    assert_eq!(computations_count, expected_tfhe);
    assert_eq!(allowed_count, expected_acl);
    assert_eq!(last_valid_block as u64, safe_tip);

    Ok(())
}
