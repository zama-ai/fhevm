use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::node_bindings::AnvilInstance;
use alloy::primitives::U256;
use alloy::providers::ext::AnvilApi;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
    NonceFiller, WalletFiller,
};
use alloy::providers::{
    Provider, ProviderBuilder, RootProvider, WalletProvider, WsConnect,
};
use alloy::rpc::types::anvil::{ReorgOptions, TransactionData};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use futures_util::future::try_join_all;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use std::process::Command;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use test_harness::health_check;
use test_harness::instance::ImportMode;
use tracing::{warn, Level};

use host_listener::cmd::main;
use host_listener::cmd::Args;
use host_listener::database::tfhe_event_propagate::{Database, ToType};

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

const NB_EVENTS_PER_WALLET: i64 = 100;

async fn emit_events<P, N>(
    wallets: &[EthereumWallet],
    url: &str,
    tfhe_contract: FHEVMExecutorTestInstance<P, N>,
    acl_contract: ACLTestInstance<P, N>,
    reorg: bool,
) where
    P: Clone + alloy::providers::Provider<N> + 'static,
    N: Clone
        + alloy::providers::Network<TransactionRequest = TransactionRequest>
        + 'static,
{
    static UNIQUE_INT: AtomicU32 = AtomicU32::new(1); // to counter avoid idempotency
    let mut threads = vec![];
    for (i_wallet, wallet) in wallets.iter().enumerate() {
        let wallet = wallet.clone();
        let tfhe_contract = tfhe_contract.clone();
        let acl_contract = acl_contract.clone();
        let url = url.to_string();
        let thread = tokio::spawn(async move {
            for i_message in 1..=NB_EVENTS_PER_WALLET {
                let reorg_point =
                    reorg && i_message == (2 * NB_EVENTS_PER_WALLET) / 3;
                let provider = ProviderBuilder::new()
                    .wallet(wallet.clone())
                    .connect_ws(WsConnect::new(url.to_string()))
                    .await
                    .unwrap();
                let to_type: ToType = 4_u8;
                let pt = U256::from(UNIQUE_INT.fetch_add(1, Ordering::SeqCst));
                let tfhe_txn_req = tfhe_contract
                    .trivialEncrypt(pt, to_type)
                    .into_transaction_request();
                let pending_txn = provider
                    .send_transaction(tfhe_txn_req.clone())
                    .await
                    .unwrap();
                let receipt = pending_txn.get_receipt().await.unwrap();
                assert!(receipt.status());
                let add: Vec<_> = provider.signer_addresses().collect();
                let acl_txn_req = acl_contract
                    .allow(pt.into(), add[0])
                    .into_transaction_request();
                if reorg_point && i_wallet == 0 {
                    // ensure no event is lost also on losing chain to facilitate the test assert
                    tokio::time::sleep(tokio::time::Duration::from_secs(5))
                        .await;
                    // ACL event is only in the past of winning chain in reorg
                    let cur_block = receipt.block_number.unwrap();
                    warn!("Start reorg");
                    provider
                        .anvil_reorg(ReorgOptions {
                            // Use a large reorg depth (25) to ensure Anvil triggers subscription events correctly;
                            // smaller depths may not reliably cause event notifications.
                            depth: 25,
                            tx_block_pairs: vec![
                                (TransactionData::JSON(tfhe_txn_req), 24),
                                // this event is only on winning chain
                                (TransactionData::JSON(acl_txn_req), 0),
                            ],
                        })
                        .await
                        .unwrap();
                    warn!("Reorg happened at block {cur_block}");
                } else {
                    let pending_txn = provider
                        .send_transaction(acl_txn_req.clone())
                        .await
                        .unwrap();
                    let receipt = pending_txn.get_receipt().await.unwrap();
                    assert!(receipt.status());
                    if reorg_point {
                        // ensure no event is lost also on losing chain to facilitate the test assert
                        tokio::time::sleep(tokio::time::Duration::from_secs(5))
                            .await;
                    }
                }
            }
        });
        threads.push(thread);
    }
    if let Err(err) = try_join_all(threads).await {
        eprintln!("{err}");
        panic!("One event emission failed: {err}");
    }
}

fn wallets(anvil: &AnvilInstance) -> Vec<EthereumWallet> {
    let mut wallets = vec![];
    for key in anvil.keys().iter() {
        let signer: PrivateKeySigner = key.clone().into();
        let wallet = EthereumWallet::new(signer);
        wallets.push(wallet);
    }
    wallets
}

type SetupProvider = FillProvider<
    JoinFill<
        JoinFill<
            alloy::providers::Identity,
            JoinFill<
                GasFiller,
                JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
            >,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

struct Setup {
    args: Args,
    anvil: AnvilInstance,
    wallets: Vec<EthereumWallet>,
    acl_contract: ACLTestInstance<SetupProvider>,
    tfhe_contract: FHEVMExecutorTestInstance<SetupProvider>,
    db_pool: sqlx::Pool<sqlx::Postgres>,
    _test_instance: test_harness::instance::DBInstance, // maintain db alive
    health_check_url: String,
}

async fn setup(node_chain_id: Option<u64>) -> Result<Setup, anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .try_init()
        .ok();

    let test_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await
            .expect("valid db instance");

    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(test_instance.db_url())
        .await?;

    let coprocessor_api_key =
        sqlx::query!("SELECT tenant_api_key FROM tenants LIMIT 1")
            .fetch_one(&db_pool)
            .await?
            .tenant_api_key;

    let anvil = Anvil::new()
        .block_time_f64(1.0)
        .args(["--accounts", "15"])
        .chain_id(node_chain_id.unwrap_or(12345))
        .spawn();

    let wallets = wallets(&anvil);
    let url = anvil.ws_endpoint().clone();

    let provider = ProviderBuilder::new()
        .wallet(wallets[0].clone())
        .connect_ws(WsConnect::new(url.clone()))
        .await?;

    let tfhe_contract = FHEVMExecutorTest::deploy(provider.clone()).await?;
    let acl_contract = ACLTest::deploy(provider.clone()).await?;
    let args = Args {
        url,
        initial_block_time: 1,
        acl_contract_address: acl_contract.address().to_string(),
        tfhe_contract_address: tfhe_contract.address().to_string(),
        database_url: test_instance.db_url().to_string(),
        coprocessor_api_key: Some(coprocessor_api_key),
        start_at_block: None,
        end_at_block: None,
        catchup_margin: 5,
        catchup_paging: 3,
        log_level: Level::INFO,
        health_port: 8081,
        dependence_cache_size: 128,
        reorg_maximum_duration_in_blocks: 100, // to go beyond chain start
        service_name: "host-listener-test".to_string(),
    };
    let health_check_url = format!("http://127.0.0.1:{}", args.health_port);

    Ok(Setup {
        args,
        anvil,
        wallets,
        acl_contract,
        tfhe_contract,
        db_pool,
        _test_instance: test_instance,
        health_check_url,
    })
}

#[tokio::test]
#[serial(db)]
async fn test_bad_chain_id() {
    let setup = setup(Some(54321)).await.expect("setup failed");
    let listener_handle = tokio::spawn(main(setup.args.clone()));
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    assert!(listener_handle.is_finished(), "Listener should have failed");
    let result = listener_handle.await;
    if let Ok(Err(e)) = result {
        assert!(e.to_string().contains("Chain ID mismatch"));
    } else {
        panic!("Listener should have failed due to chain ID mismatch");
    }
}

#[tokio::test]
#[serial(db)]
async fn test_listener_restart_and_chain_reorg() -> Result<(), anyhow::Error> {
    test_listener_no_event_loss(true, true).await
}

async fn test_listener_no_event_loss(
    kill: bool,
    reorg: bool,
) -> Result<(), anyhow::Error> {
    let setup = setup(None).await?;
    let args = setup.args.clone();

    // Start listener in background task
    let listener_handle = tokio::spawn(main(args.clone()));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);

    // Emit first batch of events
    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let tfhe_contract_clone = setup.tfhe_contract.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    let event_source = tokio::spawn(async move {
        emit_events(
            &wallets_clone,
            &url_clone,
            tfhe_contract_clone,
            acl_contract_clone,
            reorg,
        )
        .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Kill the listener
    eprintln!("First kill, check database valid block has been updated");
    listener_handle.abort();
    let mut database = Database::new(
        &args.database_url,
        &args.coprocessor_api_key.unwrap(),
        args.dependence_cache_size,
    )
    .await
    .unwrap();
    let last_block = database.read_last_valid_block().await;
    assert!(last_block.is_some());
    assert!(last_block.unwrap() > 1);

    let mut tfhe_events_count = 0;
    let mut acl_events_count = 0;
    let mut nb_kill = 1;
    // Restart/kill many time until no more events are consumned.
    for _ in 1..120 {
        // 10 mins max to avoid stalled CI
        let listener_handle = tokio::spawn(main(args.clone()));
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let tfhe_new_count = sqlx::query!("SELECT COUNT(*) FROM computations")
            .fetch_one(&setup.db_pool)
            .await?
            .count
            .unwrap_or(0);
        let acl_new_count =
            sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
                .fetch_one(&setup.db_pool)
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
        if kill {
            listener_handle.abort();
            while !listener_handle.is_finished() {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
            nb_kill += 1;
        }
        let nb_wallets = setup.wallets.len() as i64;
        eprintln!(
            "Kill {nb_kill} ongoing, event source ongoing: {}, {} {} (vs {})",
            event_source.is_finished(),
            tfhe_events_count,
            acl_events_count,
            nb_wallets * NB_EVENTS_PER_WALLET,
        );
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.5)).await;
    }
    let nb_wallets = setup.wallets.len() as i64;
    if !reorg {
        assert_eq!(tfhe_events_count, nb_wallets * NB_EVENTS_PER_WALLET);
    } else {
        // 1 event appears in both chain with a different transaction id
        assert_eq!(tfhe_events_count, nb_wallets * NB_EVENTS_PER_WALLET + 1);
    }
    assert_eq!(acl_events_count, nb_wallets * NB_EVENTS_PER_WALLET);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_health() -> Result<(), anyhow::Error> {
    let setup = setup(None).await.expect("setup failed");
    let args = setup.args.clone();

    // Start listener in background task
    let listener_handle = tokio::spawn(main(args.clone()));
    assert!(health_check::wait_alive(&setup.health_check_url, 60, 1).await);
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);

    let mut suspend_anvil = Command::new("kill")
        .args(["-s", "STOP", &setup.anvil.child().id().to_string()])
        .spawn()?;
    suspend_anvil
        .wait()
        .expect("Failed to suspend Anvil process");
    warn!("Anvil is suspended");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // time to detect issue
    warn!("Checking health");
    assert!(!health_check::wait_healthy(&setup.health_check_url, 10, 1).await);

    let mut continue_anvil = Command::new("kill")
        .args(["-s", "CONT", &setup.anvil.child().id().to_string()])
        .spawn()?;
    continue_anvil
        .wait()
        .expect("Failed to continue Anvil process");
    warn!("Anvil is back");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // time to recover
    assert!(health_check::wait_healthy(&setup.health_check_url, 10, 1).await);
    warn!("Test is killing the listener");
    listener_handle.abort();
    Ok(())
}
