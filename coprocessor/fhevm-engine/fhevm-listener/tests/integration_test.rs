use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use futures_util::future::try_join_all;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use tracing::Level;

use alloy_primitives::U256;
use alloy_provider::{Provider, ProviderBuilder, WalletProvider, WsConnect};

use alloy_rpc_types::TransactionRequest;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;

use fhevm_listener::cmd::main;
use fhevm_listener::cmd::Args;
use fhevm_listener::database::tfhe_event_propagate::{Database, ToType};

// contracts are compiled in build.rs/build_contract() using solc
// json are generated in build.rs/build_contract() using solc
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

use crate::ACLTest::ACLTestInstance;
use crate::FHEVMExecutorTest::FHEVMExecutorTestInstance;

const NB_EVENTS_PER_WALLET: i64 = 400;
const DATABASE_URL: &str =
    "postgresql://postgres:postgres@localhost:5432/coprocessor";

async fn emit_events<P, N>(
    wallets: &[EthereumWallet],
    url: &str,
    tfhe_contract: FHEVMExecutorTestInstance<P, N>,
    acl_contract: ACLTestInstance<P, N>,
) where
    P: Clone + alloy_provider::Provider<N> + 'static,
    N: Clone
        + alloy_provider::Network<TransactionRequest = TransactionRequest>
        + 'static,
{
    let mut providers = vec![];
    for wallet in wallets {
        let provider = ProviderBuilder::new()
            .wallet(wallet.clone())
            .connect_ws(WsConnect::new(url))
            .await
            .unwrap();
        providers.push(provider);
    }
    static UNIQUE_INT: AtomicU32 = AtomicU32::new(1); // to counter avoid idempotency
    let mut threads = vec![];
    for provider in providers.iter() {
        let tfhe_contract = tfhe_contract.clone();
        let acl_contract = acl_contract.clone();
        let provider = provider.clone();
        let thread = tokio::spawn(async move {
            for _ in 1..=NB_EVENTS_PER_WALLET {
                let to_type: ToType = 4_u8;
                let pt = U256::from(UNIQUE_INT.fetch_add(1, Ordering::SeqCst));
                let txn_req = tfhe_contract
                    .trivialEncrypt(pt, to_type)
                    .into_transaction_request();
                let pending_txn =
                    provider.send_transaction(txn_req).await.unwrap();
                let receipt = pending_txn.get_receipt().await.unwrap();
                assert!(receipt.status());
                let add: Vec<_> = provider.signer_addresses().collect();
                let txn_req = acl_contract
                    .allow(pt.into(), add[0])
                    .into_transaction_request();
                let pending_txn =
                    provider.send_transaction(txn_req).await.unwrap();
                let receipt = pending_txn.get_receipt().await.unwrap();
                assert!(receipt.status());
            }
        });
        threads.push(thread);
    }
    if let Err(err) = try_join_all(threads).await {
        eprintln!("{err}");
        panic!("Failed to join futures");
    }
}

#[tokio::test]
#[serial(db)]
async fn test_listener_restart() -> Result<(), anyhow::Error> {
    let anvil = Anvil::new()
        .block_time_f64(1.0)
        .args(["--accounts", "15"])
        .spawn();
    let chain_id = anvil.chain_id();
    let nb_wallet = anvil.keys().len() as i64;
    eprintln!("Nb wallet {}", nb_wallet);
    let mut wallets = vec![];
    for key in anvil.keys().iter() {
        let signer: PrivateKeySigner = key.clone().into();
        let wallet = EthereumWallet::new(signer);
        wallets.push(wallet);
    }
    let url = anvil.ws_endpoint();

    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DATABASE_URL)
        .await?;

    sqlx::query!("TRUNCATE computations")
        .execute(&db_pool)
        .await?;
    sqlx::query!("TRUNCATE blocks_valid")
        .execute(&db_pool)
        .await?;
    let count = sqlx::query!("SELECT COUNT(*) FROM computations")
        .fetch_one(&db_pool)
        .await?
        .count
        .unwrap_or(0);
    assert_eq!(count, 0);
    sqlx::query!("TRUNCATE allowed_handles")
        .execute(&db_pool)
        .await?;

    let coprocessor_api_key =
        sqlx::query!("SELECT tenant_api_key FROM tenants LIMIT 1")
            .fetch_one(&db_pool)
            .await?
            .tenant_api_key;

    let provider = ProviderBuilder::new()
        .wallet(wallets[0].clone())
        .connect_ws(WsConnect::new(url.clone()))
        .await?;
    let tfhe_contract = FHEVMExecutorTest::deploy(provider.clone()).await?;
    let acl_contract = ACLTest::deploy(provider.clone()).await?;
    let args = Args {
        url: url.clone(),
        initial_block_time: 1,
        no_block_immediate_recheck: false,
        ignore_tfhe_events: false,
        ignore_acl_events: false,
        acl_contract_address: None,
        tfhe_contract_address: None,
        database_url: DATABASE_URL.into(),
        coprocessor_api_key: Some(coprocessor_api_key),
        start_at_block: None,
        end_at_block: None,
        catchup_margin: 5,
        log_level: Level::INFO,
        health_port: 8082,
    };

    // Start listener in background task
    let listener_handle = tokio::spawn(main(args.clone()));

    for delay in 0..60 {
        const HEALTHZ_URL: &str = "http://127.0.0.1:8082/healthz";
        let response = reqwest::get(HEALTHZ_URL).await;
        if response.is_ok() && response.unwrap().status().is_success() {
            eprintln!("Listener healthy after {} seconds", delay);
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // Emit first batch of events
    let wallets_clone = wallets.clone();
    let url_clone = url.clone();
    let tfhe_contract_clone = tfhe_contract.clone();
    let acl_contract_clone = acl_contract.clone();
    let event_source = tokio::spawn(async move {
        emit_events(
            &wallets_clone,
            &url_clone,
            tfhe_contract_clone,
            acl_contract_clone,
        )
        .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    // Kill the listener
    eprintln!("First kill, check database valid block has been updated");
    listener_handle.abort();
    let mut database =
        Database::new(DATABASE_URL, &coprocessor_api_key, chain_id).await;
    let last_block = database.read_last_valid_block().await;
    assert!(last_block.is_some());
    assert!(last_block.unwrap() > 1);

    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DATABASE_URL)
        .await?;
    let mut tfhe_events_count = 0;
    let mut acl_events_count = 0;
    let mut nb_kill = 1;
    // Restart/kill many time until no more events are consumned.
    for _ in 1..120 {
        // 10 mins max to avoid stalled CI
        let listener_handle = tokio::spawn(main(args.clone()));
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let tfhe_new_count = sqlx::query!("SELECT COUNT(*) FROM computations")
            .fetch_one(&db_pool)
            .await?
            .count
            .unwrap_or(0);
        let acl_new_count =
            sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
                .fetch_one(&db_pool)
                .await?
                .count
                .unwrap_or(0);
        let no_count_change = tfhe_events_count == tfhe_new_count
            && acl_events_count == acl_new_count;
        if event_source.is_finished() && no_count_change {
            listener_handle.abort();
            break;
        };
        tfhe_events_count = tfhe_new_count;
        acl_events_count = acl_new_count;
        listener_handle.abort();
        nb_kill += 1;
        eprintln!(
            "Kill {nb_kill} ongoing, event source ongoing: {}, {} {}",
            event_source.is_finished(),
            tfhe_events_count,
            acl_events_count
        );
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.5)).await;
    }

    assert_eq!(tfhe_events_count, nb_wallet * NB_EVENTS_PER_WALLET);
    assert_eq!(acl_events_count, nb_wallet * NB_EVENTS_PER_WALLET);
    eprintln!("Total kills: {nb_kill}");
    assert!(3 < nb_kill);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_health() -> Result<(), anyhow::Error> {
    let mut anvil = Anvil::new()
        .block_time_f64(1.0)
        .args(["--accounts", "1"])
        .spawn();
    let url = anvil.ws_endpoint();

    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DATABASE_URL)
        .await?;

    let coprocessor_api_key = Some(
        sqlx::query!("SELECT tenant_api_key FROM tenants LIMIT 1")
            .fetch_one(&db_pool)
            .await?
            .tenant_api_key,
    );
    let args = Args {
        url: url.clone(),
        initial_block_time: 1,
        no_block_immediate_recheck: false,
        ignore_tfhe_events: false,
        ignore_acl_events: false,
        acl_contract_address: None,
        tfhe_contract_address: None,
        database_url: DATABASE_URL.into(),
        coprocessor_api_key,
        start_at_block: None,
        end_at_block: None,
        catchup_margin: 5,
        log_level: Level::INFO,
        health_port: 8083,
    };

    const LIVENESS_URL: &str = "http://127.0.0.1:8083/liveness";
    const HEALTHZ_URL: &str = "http://127.0.0.1:8083/healthz";

    // Start listener in background task
    let listener_handle: tokio::task::JoinHandle<()> =
        tokio::spawn(main(args.clone()));
    for _ in 1..10 {
        let response = reqwest::get(LIVENESS_URL).await;
        if response.is_ok() && response.unwrap().status().is_success() {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    let response = reqwest::get(LIVENESS_URL).await;
    assert!(response.is_ok());
    assert!(response.unwrap().status().is_success());
    let response = reqwest::get(HEALTHZ_URL).await;
    let Ok(response) = response else {
        return Err(anyhow::anyhow!("Failed to get healthz"));
    };
    if !response.status().is_success() {
        eprintln!("response: {:?}", response.text().await);
        return Err(anyhow::anyhow!("Failed to get healthz"));
    }
    anvil.child_mut().kill().unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let response = reqwest::get(HEALTHZ_URL).await;
    let Ok(response) = response else {
        return Err(anyhow::anyhow!("Failed to get healthz"));
    };
    if response.status().is_success() {
        return Err(anyhow::anyhow!("Healthz should be unhealthy"));
    }
    listener_handle.abort();
    Ok(())
}
