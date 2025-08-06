use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::node_bindings::AnvilInstance;
use alloy::primitives::U256;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
    NonceFiller, WalletFiller,
};
use alloy::providers::{
    Provider, ProviderBuilder, RootProvider, WalletProvider, WsConnect,
};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use futures_util::future::try_join_all;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use test_harness::instance::ImportMode;
use tracing::Level;

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

const NB_EVENTS_PER_WALLET: i64 = 400;

async fn emit_events<P, N>(
    wallets: &[EthereumWallet],
    url: &str,
    tfhe_contract: FHEVMExecutorTestInstance<P, N>,
    acl_contract: ACLTestInstance<P, N>,
) where
    P: Clone + alloy::providers::Provider<N> + 'static,
    N: Clone
        + alloy::providers::Network<TransactionRequest = TransactionRequest>
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
}

async fn setup(node_chain_id: Option<u64>) -> Result<Setup, anyhow::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
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
        no_block_immediate_recheck: false,
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
    };

    Ok(Setup {
        args,
        anvil,
        wallets,
        acl_contract,
        tfhe_contract,
        db_pool,
        _test_instance: test_instance,
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
async fn test_listener_restart() -> Result<(), anyhow::Error> {
    let setup = setup(None).await?;
    let args = setup.args.clone();

    // Start listener in background task
    let listener_handle = tokio::spawn(main(args.clone()));

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

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
    let nb_wallets = setup.wallets.len() as i64;
    assert_eq!(tfhe_events_count, nb_wallets * NB_EVENTS_PER_WALLET);
    assert_eq!(acl_events_count, nb_wallets * NB_EVENTS_PER_WALLET);
    eprintln!("Total kills: {nb_kill}");
    assert!(3 < nb_kill);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_health() -> Result<(), anyhow::Error> {
    let mut setup = setup(None).await.expect("setup failed");

    const LIVENESS_URL: &str = "http://0.0.0.0:8081/liveness";
    const HEALTHZ_URL: &str = "http://0.0.0.0:8081/healthz";

    // Start listener in background task
    let listener_handle = tokio::spawn(main(setup.args.clone()));
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
    setup.anvil.child_mut().kill().unwrap();
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
