use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::node_bindings::AnvilInstance;
use alloy::primitives::{keccak256, Address, FixedBytes, U256};
use alloy::providers::ext::AnvilApi;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
    NonceFiller, WalletFiller,
};
use alloy::providers::{
    Provider, ProviderBuilder, RootProvider, WalletProvider, WsConnect,
};
use alloy::rpc::types::anvil::{ReorgOptions, TransactionData};
use alloy::rpc::types::{Filter, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use fhevm_engine_common::chain_id::ChainId;
use futures_util::future::try_join_all;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashSet;
use std::process::Command;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicU32, AtomicU64};
use test_harness::health_check;
use test_harness::instance::ImportMode;
use tracing::{info, warn, Level};

use host_listener::cmd::main;
use host_listener::cmd::Args;
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
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

const NB_EVENTS_PER_WALLET: i64 = 200;

async fn emit_events<P, N>(
    wallets: &[EthereumWallet],
    url: &str,
    tfhe_contract: FHEVMExecutorTestInstance<P, N>,
    acl_contract: ACLTestInstance<P, N>,
    reorg: bool,
    nb_events_per_wallet: i64,
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
            for i_message in 1..=nb_events_per_wallet {
                eprintln!("Emitting event {i_message} for wallet {i_wallet}");
                let reorg_point =
                    reorg && i_message == (2 * nb_events_per_wallet) / 3;
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
    chain_id: ChainId,
}

async fn setup_with_block_time(
    node_chain_id: Option<u64>,
    block_time_secs: f64,
) -> Result<Setup, anyhow::Error> {
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

    let anvil = Anvil::new()
        .block_time_f64(block_time_secs)
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
        database_url: test_instance.db_url.clone(),
        start_at_block: None,
        end_at_block: None,
        only_catchup_loop: false,
        catchup_loop_sleep_secs: 60,
        catchup_margin: 5,
        catchup_paging: 3,
        log_level: Level::INFO,
        health_port: 8081,
        dependence_cache_size: 128,
        reorg_maximum_duration_in_blocks: 100, // to go beyond chain start
        service_name: "host-listener-test".to_string(),
        catchup_finalization_in_blocks: 2,
        dependence_by_connexity: false,
        dependence_cross_block: true,
        dependent_ops_max_per_chain: 0,
        timeout_request_websocket: 30,
    };
    let health_check_url = format!("http://127.0.0.1:{}", args.health_port);

    let chain_id = ChainId::try_from(if let Some(chain_id) = node_chain_id {
        chain_id
    } else {
        provider.get_chain_id().await?
    })?;

    Ok(Setup {
        args,
        anvil,
        wallets,
        acl_contract,
        tfhe_contract,
        db_pool,
        _test_instance: test_instance,
        health_check_url,
        chain_id,
    })
}

async fn setup(node_chain_id: Option<u64>) -> Result<Setup, anyhow::Error> {
    setup_with_block_time(node_chain_id, 1.0).await
}

fn trivial_encrypt_handle(val: U256, to_type: u8) -> FixedBytes<32> {
    let mut payload = Vec::with_capacity(
        "trivialEncrypt".len() + std::mem::size_of::<[u8; 32]>() + 1,
    );
    payload.extend_from_slice("trivialEncrypt".as_bytes());
    payload.extend_from_slice(&val.to_be_bytes::<32>());
    payload.push(to_type);
    keccak256(payload)
}

fn fhe_add_handle(
    lhs: FixedBytes<32>,
    rhs: FixedBytes<32>,
    scalar_byte: u8,
) -> FixedBytes<32> {
    let mut payload = Vec::with_capacity(
        "fheAdd".len()
            + std::mem::size_of::<[u8; 32]>()
            + std::mem::size_of::<[u8; 32]>()
            + 1,
    );
    payload.extend_from_slice("fheAdd".as_bytes());
    payload.extend_from_slice(lhs.as_slice());
    payload.extend_from_slice(rhs.as_slice());
    payload.push(scalar_byte);
    keccak256(payload)
}

async fn ingest_blocks_for_receipts(
    db: &mut Database,
    setup: &Setup,
    receipts: &[alloy::rpc::types::TransactionReceipt],
    options: IngestOptions,
) -> Result<(), anyhow::Error> {
    let mut blocks: Vec<(u64, FixedBytes<32>)> = receipts
        .iter()
        .map(|receipt| {
            (
                receipt.block_number.expect("receipt has block number"),
                receipt.block_hash.expect("receipt has block hash"),
            )
        })
        .collect();
    blocks.sort_by_key(|(number, _)| *number);
    blocks.dedup_by_key(|(number, _)| *number);

    let acl_address = Some(*setup.acl_contract.address());
    let tfhe_address = Some(*setup.tfhe_contract.address());

    let provider = ProviderBuilder::new()
        .wallet(setup.wallets[0].clone())
        .connect_ws(WsConnect::new(setup.args.url.clone()))
        .await?;

    for (_, block_hash) in blocks {
        let filter = Filter::new().at_block_hash(block_hash).address(vec![
            *setup.acl_contract.address(),
            *setup.tfhe_contract.address(),
        ]);
        let logs = provider.get_logs(&filter).await?;
        let block = provider
            .get_block_by_hash(block_hash)
            .await?
            .expect("block exists");
        let block_logs = BlockLogs {
            logs,
            summary: block.header.into(),
            catchup: false,
        };
        ingest_block_logs(
            db.chain_id,
            db,
            &block_logs,
            &acl_address,
            &tfhe_address,
            options,
        )
        .await?;
    }
    Ok(())
}

async fn emit_dependent_burst(
    setup: &Setup,
    input_handle: Option<FixedBytes<32>>,
    depth: usize,
) -> Result<
    (Vec<alloy::rpc::types::TransactionReceipt>, FixedBytes<32>),
    anyhow::Error,
> {
    emit_dependent_burst_seeded(setup, input_handle, depth, 1_u64).await
}

async fn ingest_dependent_burst_seeded(
    db: &mut Database,
    setup: &Setup,
    input_handle: Option<FixedBytes<32>>,
    depth: usize,
    seed: u64,
    dependent_ops_max_per_chain: u32,
) -> Result<FixedBytes<32>, anyhow::Error> {
    let (receipts, last_output_handle) =
        emit_dependent_burst_seeded(setup, input_handle, depth, seed).await?;
    ingest_blocks_for_receipts(
        db,
        setup,
        &receipts,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain,
        },
    )
    .await?;
    Ok(last_output_handle)
}

async fn emit_dependent_burst_seeded(
    setup: &Setup,
    input_handle: Option<FixedBytes<32>>,
    depth: usize,
    seed: u64,
) -> Result<
    (Vec<alloy::rpc::types::TransactionReceipt>, FixedBytes<32>),
    anyhow::Error,
> {
    let provider = ProviderBuilder::new()
        .wallet(setup.wallets[0].clone())
        .connect_ws(WsConnect::new(setup.args.url.clone()))
        .await?;
    let signer_address: Address = provider
        .signer_addresses()
        .next()
        .expect("anvil signer available");

    let mut pending = Vec::new();
    let mut current = input_handle
        .unwrap_or_else(|| trivial_encrypt_handle(U256::from(seed), 4_u8));

    if input_handle.is_none() {
        let trivial_tx = setup
            .tfhe_contract
            .trivialEncrypt(U256::from(seed), 4_u8)
            .into_transaction_request();
        pending.push(provider.send_transaction(trivial_tx).await?);
        let allow_trivial_tx = setup
            .acl_contract
            .allow(current, signer_address)
            .into_transaction_request();
        pending.push(provider.send_transaction(allow_trivial_tx).await?);
    }

    for _ in 0..depth {
        let next = fhe_add_handle(current, current, 0_u8);
        let add_tx = setup
            .tfhe_contract
            .fheAdd(current, current, FixedBytes::<1>::from([0_u8]))
            .into_transaction_request();
        pending.push(provider.send_transaction(add_tx).await?);
        let allow_tx = setup
            .acl_contract
            .allow(next, signer_address)
            .into_transaction_request();
        pending.push(provider.send_transaction(allow_tx).await?);
        current = next;
    }

    let receipts = try_join_all(
        pending
            .into_iter()
            .map(|pending_tx| async move { pending_tx.get_receipt().await }),
    )
    .await?;
    assert!(
        receipts.iter().all(|receipt| receipt.status()),
        "every burst tx must succeed"
    );
    Ok((receipts, current))
}

async fn dep_chain_id_for_output_handle(
    setup: &Setup,
    output_handle: FixedBytes<32>,
) -> Result<Vec<u8>, anyhow::Error> {
    let dep_chain_id = sqlx::query_scalar::<_, Option<Vec<u8>>>(
        r#"
        SELECT dependence_chain_id
        FROM computations
        WHERE output_handle = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(output_handle.as_slice())
    .fetch_one(&setup.db_pool)
    .await?
    .ok_or_else(|| {
        anyhow::anyhow!("missing dependence_chain_id for output handle")
    })?;
    Ok(dep_chain_id)
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
async fn test_slow_lane_marks_heavy_chain_locally() -> Result<(), anyhow::Error>
{
    let setup = setup_with_block_time(None, 3.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        &setup.args.coprocessor_api_key.unwrap(),
        setup.args.dependence_cache_size,
    )
    .await?;

    let (receipts, _) = emit_dependent_burst(&setup, None, 4).await?;
    ingest_blocks_for_receipts(
        &mut db,
        &setup,
        &receipts,
        IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 1,
        },
    )
    .await?;

    let slow_chain_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM dependence_chain WHERE schedule_priority = 1",
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert!(
        slow_chain_count > 0,
        "heavy dependent chain should be assigned to slow lane"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_off_mode_promotes_seen_chain_locally(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 3.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        &setup.args.coprocessor_api_key.unwrap(),
        setup.args.dependence_cache_size,
    )
    .await?;

    let last_handle =
        ingest_dependent_burst_seeded(&mut db, &setup, None, 4, 1_u64, 1)
            .await?;
    let initially_slow = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM dependence_chain WHERE schedule_priority = 1",
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert!(
        initially_slow > 0,
        "setup phase should create at least one slow chain"
    );

    ingest_dependent_burst_seeded(
        &mut db,
        &setup,
        Some(last_handle),
        1,
        1_u64,
        0,
    )
    .await?;

    let remaining_slow = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM dependence_chain WHERE schedule_priority = 1",
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(
        remaining_slow, 0,
        "off mode should promote seen slow chains back to fast"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_contention_prefers_fast_chain(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 3.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        &setup.args.coprocessor_api_key.unwrap(),
        setup.args.dependence_cache_size,
    )
    .await?;

    let heavy_last_handle =
        ingest_dependent_burst_seeded(&mut db, &setup, None, 4, 1_u64, 2)
            .await?;

    let fast_last_handle =
        ingest_dependent_burst_seeded(&mut db, &setup, None, 1, 2_u64, 2)
            .await?;

    let heavy_dep_chain_id =
        dep_chain_id_for_output_handle(&setup, heavy_last_handle).await?;
    let fast_dep_chain_id =
        dep_chain_id_for_output_handle(&setup, fast_last_handle).await?;
    assert_ne!(
        heavy_dep_chain_id, fast_dep_chain_id,
        "contention test requires two independent chains"
    );

    let heavy_priority = sqlx::query_scalar::<_, i16>(
        "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&heavy_dep_chain_id)
    .fetch_one(&setup.db_pool)
    .await?;
    let fast_priority = sqlx::query_scalar::<_, i16>(
        "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&fast_dep_chain_id)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(heavy_priority, 1, "heavy chain must be marked slow");
    assert_eq!(fast_priority, 0, "light chain must stay fast");

    let ordered = sqlx::query_as::<_, (Vec<u8>, i16)>(
        r#"
        SELECT dependence_chain_id, schedule_priority
        FROM dependence_chain
        WHERE status = 'updated'
          AND worker_id IS NULL
          AND dependency_count = 0
        ORDER BY schedule_priority ASC, last_updated_at ASC
        LIMIT 2
        "#,
    )
    .fetch_all(&setup.db_pool)
    .await?;
    assert_eq!(ordered.len(), 2, "expected two schedulable chains");
    assert_eq!(
        ordered[0].0, fast_dep_chain_id,
        "fast chain should be acquired before slow chain under contention"
    );
    assert_eq!(ordered[0].1, 0);
    assert_eq!(ordered[1].0, heavy_dep_chain_id);
    assert_eq!(ordered[1].1, 1);

    sqlx::query(
        "UPDATE dependence_chain SET status = 'processed' WHERE dependence_chain_id = $1",
    )
    .bind(&fast_dep_chain_id)
    .execute(&setup.db_pool)
    .await?;

    let next = sqlx::query_as::<_, (Vec<u8>, i16)>(
        r#"
        SELECT dependence_chain_id, schedule_priority
        FROM dependence_chain
        WHERE status = 'updated'
          AND worker_id IS NULL
          AND dependency_count = 0
        ORDER BY schedule_priority ASC, last_updated_at ASC
        LIMIT 1
        "#,
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(
        next.0, heavy_dep_chain_id,
        "slow chain should still progress once fast lane is empty"
    );
    assert_eq!(next.1, 1);

    Ok(())
}

#[tokio::test]
async fn test_only_catchup_loop_requires_negative_start_at_block(
) -> Result<(), anyhow::Error> {
    let args = Args {
        url: "ws://127.0.0.1:8545".to_string(),
        acl_contract_address: "".to_string(),
        tfhe_contract_address: "".to_string(),
        database_url: fhevm_engine_common::utils::DatabaseURL::default(),
        start_at_block: Some(0),
        end_at_block: None,
        catchup_margin: 5,
        catchup_paging: 10,
        initial_block_time: 12,
        log_level: Level::INFO,
        health_port: 0,
        dependence_cache_size: 128,
        reorg_maximum_duration_in_blocks: 50,
        service_name: String::new(),
        catchup_finalization_in_blocks: 20,
        only_catchup_loop: true,
        catchup_loop_sleep_secs: 60,
        dependence_by_connexity: false,
        dependence_cross_block: true,
        dependent_ops_max_per_chain: 0,
        timeout_request_websocket: 30,
    };

    let result = main(args).await;
    assert!(
        result.is_err(),
        "Expected error for non-negative start_at_block"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("--only-catchup-loop requires negative --start-at-block"),
        "Unexpected error message: {err}"
    );
    Ok(())
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
    let mut args = setup.args.clone();
    // This test intentionally aborts/restarts the listener many times.
    // Keep telemetry disabled here to avoid coupling event-loss assertions
    // with exporter/shutdown timing.
    args.service_name.clear();

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
            NB_EVENTS_PER_WALLET,
        )
        .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Kill the listener
    eprintln!("First kill, check database valid block has been updated");
    listener_handle.abort();
    let database = Database::new(
        &args.database_url,
        setup.chain_id,
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
    let nb_wallets = setup.wallets.len() as i64;
    // Restart/kill many time until no more events are consumed.
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
        eprintln!(
            "Kill {nb_kill} ongoing, event source ongoing: {}, {} {} (vs {})",
            event_source.is_finished(),
            tfhe_events_count,
            acl_events_count,
            nb_wallets * NB_EVENTS_PER_WALLET,
        );
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(1.5)).await;
    }
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

#[tokio::test]
#[serial(db)]
async fn test_catchup_and_listen() -> Result<(), anyhow::Error> {
    let setup = setup(None).await?;
    let mut args = setup.args.clone();

    // Emit first batch of events
    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let tfhe_contract_clone = setup.tfhe_contract.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    let nb_event_per_wallet = 10;
    emit_events(
        &wallets_clone,
        &url_clone,
        tfhe_contract_clone,
        acl_contract_clone,
        false, // no reorg
        nb_event_per_wallet,
    )
    .await;

    // Start listener in background task
    args.start_at_block = Some(0);
    args.catchup_paging = 3;
    let listener_handle = tokio::spawn(main(args.clone()));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await; // time to catchup

    let tfhe_events_count = sqlx::query!("SELECT COUNT(*) FROM computations")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let acl_events_count = sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let nb_wallets = setup.wallets.len() as i64;
    assert_eq!(tfhe_events_count, nb_wallets * nb_event_per_wallet);
    assert_eq!(acl_events_count, nb_wallets * nb_event_per_wallet);
    assert!(!listener_handle.is_finished(), "Listener should continue");
    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let tfhe_contract_clone = setup.tfhe_contract.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    emit_events(
        &wallets_clone,
        &url_clone,
        tfhe_contract_clone,
        acl_contract_clone,
        false, // no reorg
        nb_event_per_wallet,
    )
    .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let tfhe_events_count = sqlx::query!("SELECT COUNT(*) FROM computations")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let acl_events_count = sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    assert_eq!(tfhe_events_count, 2 * nb_wallets * nb_event_per_wallet);
    assert_eq!(acl_events_count, 2 * nb_wallets * nb_event_per_wallet);
    listener_handle.abort();
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_catchup_only() -> Result<(), anyhow::Error> {
    let setup = setup(None).await?;
    let mut args = setup.args.clone();

    // Emit first batch of events
    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let tfhe_contract_clone = setup.tfhe_contract.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    let nb_event_per_wallet = 5;
    emit_events(
        &wallets_clone,
        &url_clone,
        tfhe_contract_clone,
        acl_contract_clone,
        false, // no reorg
        nb_event_per_wallet,
    )
    .await;

    // Start listener in background task
    args.start_at_block = Some(-30 + 2 * nb_event_per_wallet);
    args.end_at_block = Some(15 + 2 * nb_event_per_wallet);
    args.catchup_paging = 2;
    let listener_handle = tokio::spawn(main(args.clone()));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await; // time to catchup

    let tfhe_events_count = sqlx::query!("SELECT COUNT(*) FROM computations")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let acl_events_count = sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let nb_wallets = setup.wallets.len() as i64;
    eprintln!("End block {:?}", args.end_at_block);
    assert_eq!(tfhe_events_count, nb_wallets * nb_event_per_wallet);
    assert_eq!(acl_events_count, nb_wallets * nb_event_per_wallet);
    assert!(listener_handle.is_finished(), "Listener should stop");
    Ok(())
}

struct CatchupOutcome {
    // Keep setup alive so the Anvil node and DB instance outlive the test body
    _setup: Setup,
    listener_handle: tokio::task::JoinHandle<anyhow::Result<()>>,
    tfhe_events_count: i64,
    acl_events_count: i64,
    nb_wallets: i64,
}

async fn run_catchup_only_scenario<F>(
    nb_event_per_wallet: i64,
    sleep_secs: u64,
    configure_args: F,
) -> Result<CatchupOutcome, anyhow::Error>
where
    F: FnOnce(&mut Args),
{
    let setup = setup(None).await?;
    let mut args = setup.args.clone();

    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let tfhe_contract_clone = setup.tfhe_contract.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    emit_events(
        &wallets_clone,
        &url_clone,
        tfhe_contract_clone,
        acl_contract_clone,
        false,
        nb_event_per_wallet,
    )
    .await;

    configure_args(&mut args);
    args.only_catchup_loop = true;

    let listener_handle = tokio::spawn(main(args.clone()));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);
    tokio::time::sleep(tokio::time::Duration::from_secs(sleep_secs)).await;

    let tfhe_events_count = sqlx::query!("SELECT COUNT(*) FROM computations")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let acl_events_count = sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
        .fetch_one(&setup.db_pool)
        .await?
        .count
        .unwrap_or(0);
    let nb_wallets = setup.wallets.len() as i64;

    Ok(CatchupOutcome {
        _setup: setup,
        listener_handle,
        tfhe_events_count,
        acl_events_count,
        nb_wallets,
    })
}

#[tokio::test]
#[serial(db)]
async fn test_catchup_only_absolute_end() -> Result<(), anyhow::Error> {
    let nb_event_per_wallet = 5;
    let outcome = run_catchup_only_scenario(nb_event_per_wallet, 15, |args| {
        args.start_at_block = Some(-50);
        args.end_at_block = Some(50);
        args.catchup_loop_sleep_secs = 5;
        args.catchup_paging = 10;
    })
    .await?;

    assert_eq!(
        outcome.tfhe_events_count,
        outcome.nb_wallets * nb_event_per_wallet
    );
    assert_eq!(
        outcome.acl_events_count,
        outcome.nb_wallets * nb_event_per_wallet
    );

    // Listener should still be running (it's in a loop, sleeping between iterations)
    assert!(
        !outcome.listener_handle.is_finished(),
        "Listener should continue running in loop mode"
    );

    outcome.listener_handle.abort();
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_catchup_only_relative_end() -> Result<(), anyhow::Error> {
    let nb_event_per_wallet = 5;
    let outcome = run_catchup_only_scenario(nb_event_per_wallet, 15, |args| {
        args.start_at_block = Some(-50); // 50 blocks from current
        args.end_at_block = Some(-5); // 5 blocks from current (more recent)
        args.catchup_loop_sleep_secs = 5; // short sleep for testing
        args.catchup_paging = 10;
    })
    .await?;

    // Events should be captured (exact count may vary based on block timing)
    assert!(
        outcome.tfhe_events_count > 0,
        "Should have captured some TFHE events"
    );
    assert!(
        outcome.acl_events_count > 0,
        "Should have captured some ACL events"
    );
    assert!(
        outcome.tfhe_events_count <= outcome.nb_wallets * nb_event_per_wallet,
        "Should not exceed emitted events in first catchup"
    );
    assert!(
        outcome.acl_events_count <= outcome.nb_wallets * nb_event_per_wallet,
        "Should not exceed emitted events in first catchup"
    );

    let first_tfhe_events_count = outcome.tfhe_events_count;
    let first_acl_events_count = outcome.acl_events_count;

    // Emit a second batch of events to be picked up
    let setup = &outcome._setup;
    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let tfhe_contract_clone = setup.tfhe_contract.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    emit_events(
        &wallets_clone,
        &url_clone,
        tfhe_contract_clone,
        acl_contract_clone,
        false,
        nb_event_per_wallet,
    )
    .await;

    // Wait enough time for another catchup iteration to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

    let tfhe_events_count_after =
        sqlx::query!("SELECT COUNT(*) FROM computations")
            .fetch_one(&setup.db_pool)
            .await?
            .count
            .unwrap_or(0);
    let acl_events_count_after =
        sqlx::query!("SELECT COUNT(*) FROM allowed_handles")
            .fetch_one(&setup.db_pool)
            .await?
            .count
            .unwrap_or(0);

    assert!(
        tfhe_events_count_after > first_tfhe_events_count,
        "Second catchup iteration should ingest additional TFHE events"
    );
    assert!(
        acl_events_count_after > first_acl_events_count,
        "Second catchup iteration should ingest additional ACL events"
    );

    // Listener should still be running
    assert!(
        !outcome.listener_handle.is_finished(),
        "Listener should continue running in loop mode"
    );

    outcome.listener_handle.abort();
    Ok(())
}

const NB_DELEGATION_PER_WALLET: usize = 15;

async fn emit_delegations<P, N>(
    wallets: &[EthereumWallet],
    url: &str,
    acl_contract: ACLTestInstance<P, N>,
) where
    P: Clone + alloy::providers::Provider<N> + 'static,
    N: Clone
        + alloy::providers::Network<TransactionRequest = TransactionRequest>
        + 'static,
{
    static UNIQUE_INT: AtomicU64 = AtomicU64::new(1); // to counter avoid idempotency
    let mut threads = vec![];
    let delegate = *acl_contract.address();
    let contract_address = *acl_contract.address();
    for (i_wallet, wallet) in wallets.iter().enumerate() {
        let expiration_date = 3600_u64 + i_wallet as u64;
        let wallet = wallet.clone();
        let acl_contract = acl_contract.clone();
        let url = url.to_string();
        let thread = tokio::spawn(async move {
            let delegation_counter = UNIQUE_INT.fetch_add(1, Ordering::SeqCst);
            for _ in 1..=NB_DELEGATION_PER_WALLET {
                let provider = ProviderBuilder::new()
                    .wallet(wallet.clone())
                    .connect_ws(WsConnect::new(url.to_string()))
                    .await
                    .unwrap();
                let acl_txn_req = acl_contract
                    .delegateForUserDecryption(
                        delegate,
                        contract_address,
                        delegation_counter,
                        0,
                        expiration_date,
                    )
                    .into_transaction_request();
                let pending_txn = provider
                    .send_transaction(acl_txn_req.clone())
                    .await
                    .unwrap();
                let receipt = pending_txn.get_receipt().await.unwrap();
                assert!(receipt.status());
            }
        });
        threads.push(thread);
    }
    if let Err(err) = try_join_all(threads).await {
        eprintln!("{err}");
        panic!("One event emission failed: {err}");
    }
}

#[tokio::test]
#[serial(db)]
async fn test_listener_delegations() -> Result<(), anyhow::Error> {
    let setup = setup(None).await?;
    let args = setup.args.clone();

    // Start listener in background task
    let listener_handle = tokio::spawn(main(args.clone()));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);

    // Emit first batch of events
    let wallets_clone = setup.wallets.clone();
    let url_clone = setup.args.url.clone();
    let acl_contract_clone = setup.acl_contract.clone();
    let event_source = tokio::spawn(async move {
        emit_delegations(&wallets_clone, &url_clone, acl_contract_clone).await;
    });

    let mut delegation_set = HashSet::new();
    for _ in 1..30 {
        let _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let delegations = sqlx::query!(
            "SELECT block_number, new_expiration_date FROM delegate_user_decrypt"
        )
        .fetch_all(&setup.db_pool)
        .await?;
        for delegation in delegations {
            delegation_set.insert((
                delegation.block_number,
                delegation.new_expiration_date,
            ));
        }
        if delegation_set.len()
            >= setup.wallets.len() * NB_DELEGATION_PER_WALLET
        {
            info!("Delegations in database");
            break;
        }
    }
    event_source.await?;
    assert_eq!(
        delegation_set.len(),
        setup.wallets.len() * NB_DELEGATION_PER_WALLET
    );
    listener_handle.abort();
    Ok(())
}
