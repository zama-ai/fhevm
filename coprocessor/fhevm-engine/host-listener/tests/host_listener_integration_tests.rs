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
use alloy::rpc::types::BlockNumberOrTag;
use alloy::rpc::types::{Filter, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use bigdecimal::BigDecimal;
use fhevm_engine_common::chain_id::ChainId;
use futures_util::future::try_join_all;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::collections::HashSet;
use std::process::Command;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicU32, AtomicU64};
use test_harness::health_check;
use test_harness::instance::ImportMode;
use tracing::{info, warn, Level};

use host_listener::cmd::block_history::BlockSummary;
use host_listener::cmd::main;
use host_listener::cmd::Args;
use host_listener::cmd::InfiniteLogIter;
use host_listener::database::ingest::{
    ingest_block_logs, update_finalized_blocks, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::{
    Database, ProducerBlock, ToType,
};

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

sol!(
    #[sol(rpc)]
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    KMSGenerationTest,
    "artifacts/KMSGenerationTest.sol/KMSGenerationTest.json"
);

use crate::ACLTest::ACLTestInstance;
use crate::FHEVMExecutorTest::FHEVMExecutorTestInstance;
use crate::KMSGenerationTest::KMSGenerationTestInstance;

const NB_EVENTS_PER_WALLET: i64 = 50;

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
    kms_generation_contract: KMSGenerationTestInstance<SetupProvider>,
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
    let kms_generation_contract =
        KMSGenerationTest::deploy(provider.clone()).await?;
    let args = Args {
        url,
        initial_block_time: 1,
        acl_contract_address: acl_contract.address().to_string(),
        tfhe_contract_address: tfhe_contract.address().to_string(),
        kms_generation_address: kms_generation_contract.address().to_string(),
        confidential_bridge_address: String::new(),
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
        catchup_finalization_in_blocks: 3,
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
        kms_generation_contract,
        db_pool,
        _test_instance: test_instance,
        health_check_url,
        chain_id,
    })
}

async fn setup(node_chain_id: Option<u64>) -> Result<Setup, anyhow::Error> {
    setup_with_block_time(node_chain_id, 1.0).await
}

#[tokio::test]
#[serial(db)]
async fn test_mark_block_as_valid_repairs_missing_parent_hash(
) -> Result<(), Box<dyn std::error::Error>> {
    let test_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let chain_id = ChainId::try_from(42_u64)?;
    let db_url = test_instance.db_url.clone();
    let db = Database::new(&db_url, chain_id, 128).await?;
    let pool = db.pool.read().await.clone();

    let block_hash = FixedBytes::<32>::from([0x22; 32]);
    let parent_hash = FixedBytes::<32>::from([0x11; 32]);

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (
            chain_id, block_hash, block_number, block_status
         )
         VALUES ($1, $2, $3, 'pending')",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.to_vec())
    .bind(12_i64)
    .execute(&pool)
    .await?;

    let mut tx = db.new_transaction().await?;
    db.mark_block_as_valid(
        &mut tx,
        &BlockSummary {
            number: 12,
            hash: block_hash,
            parent_hash,
            timestamp: 0,
        },
        false,
    )
    .await?;
    tx.commit().await?;

    let repaired_parent_hash: Option<Vec<u8>> = sqlx::query_scalar(
        "SELECT parent_hash
         FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.to_vec())
    .fetch_one(&pool)
    .await?;

    assert_eq!(repaired_parent_hash, Some(parent_hash.to_vec()));

    Ok(())
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
    let kms_generation_address = Some(*setup.kms_generation_contract.address());

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
            finalized: false,
        };
        ingest_block_logs(
            db.chain_id,
            db,
            &block_logs,
            &acl_address,
            &tfhe_address,
            &kms_generation_address,
            &None,
            options,
        )
        .await?;
    }
    Ok(())
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
        FROM computations_branch
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

// Polls Anvil until the block number advances past `after_block`.
// If `after_block` is `None`, queries the current block first.
async fn wait_for_next_block(
    url: &str,
    after_block: Option<u64>,
    timeout: tokio::time::Duration,
) -> Result<u64, anyhow::Error> {
    let provider = ProviderBuilder::new()
        .connect_ws(WsConnect::new(url))
        .await?;
    let current = match after_block {
        Some(b) => b,
        None => provider.get_block_number().await?,
    };
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let block = provider.get_block_number().await?;
        if block > current {
            return Ok(block);
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timeout waiting for block > {current}, still at {block}"
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

// Polls the database until both branch-context event counts satisfy `predicate`,
// returning the final `(tfhe_count, acl_count)`.
//
// Wave-1 also keeps branchless mirror rows for legacy-only ACL writes. Listener
// event assertions must ignore those setup/backcompat rows and count only rows
// carrying block context from host-chain events.
// Panics with `context` if `timeout` elapses before the condition is met.
async fn wait_for_event_counts(
    db_pool: &sqlx::PgPool,
    timeout: tokio::time::Duration,
    context: &str,
    predicate: impl Fn(i64, i64) -> bool,
) -> Result<(i64, i64), anyhow::Error> {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let tfhe = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM computations_branch WHERE producer_block_hash <> ''::BYTEA",
        )
        .fetch_one(db_pool)
        .await?;
        let acl = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM allowed_handles_branch
            WHERE producer_block_hash <> ''::BYTEA
               OR block_hash <> ''::BYTEA
            "#,
        )
        .fetch_one(db_pool)
        .await?;
        if predicate(tfhe, acl) {
            return Ok((tfhe, acl));
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timeout {context}: tfhe={tfhe}, acl={acl}"
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_threshold_matrix_locally() -> Result<(), anyhow::Error>
{
    let setup = setup_with_block_time(None, 3.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let cases = [
        ("below_cap", 62_usize, 64_u32, 0_i16, 11_u64),
        ("at_cap", 63_usize, 64_u32, 0_i16, 12_u64),
        ("above_cap", 64_usize, 64_u32, 1_i16, 13_u64),
    ];

    let mut seen_chains = HashSet::new();
    for (name, depth, cap, expected_priority, seed) in cases {
        let last_handle = ingest_dependent_burst_seeded(
            &mut db, &setup, None, depth, seed, cap,
        )
        .await?;
        let dep_chain_id =
            dep_chain_id_for_output_handle(&setup, last_handle).await?;
        assert!(
            seen_chains.insert(dep_chain_id.clone()),
            "matrix case {name} reused an existing dependence chain"
        );

        let schedule_priority = sqlx::query_scalar::<_, i16>(
            "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(&dep_chain_id)
        .fetch_one(&setup.db_pool)
        .await?;
        assert_eq!(
            schedule_priority, expected_priority,
            "case={name} depth={depth} cap={cap}"
        );
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_schedule_priority_migration_contract() -> Result<(), anyhow::Error>
{
    let test_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await
            .expect("valid db instance");

    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(test_instance.db_url())
        .await?;

    let column_row = sqlx::query_as::<_, (String, String, Option<String>)>(
        r#"
        SELECT data_type, is_nullable, column_default
        FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'dependence_chain'
          AND column_name = 'schedule_priority'
        "#,
    )
    .fetch_one(&db_pool)
    .await?;

    assert_eq!(column_row.0, "smallint");
    assert_eq!(column_row.1, "NO");
    let default_expr = column_row
        .2
        .expect("schedule_priority column default must exist");
    assert!(
        default_expr.contains('0'),
        "unexpected schedule_priority default: {default_expr}"
    );

    let index_def = sqlx::query_scalar::<_, String>(
        r#"
        SELECT pg_get_indexdef(i.indexrelid)
        FROM pg_index i
        JOIN pg_class c ON c.oid = i.indexrelid
        WHERE c.relname = 'idx_pending_dependence_chain'
        "#,
    )
    .fetch_one(&db_pool)
    .await?;

    let lowered = index_def.to_lowercase();
    let pos_schedule = lowered
        .find("schedule_priority")
        .expect("index must include schedule_priority");
    let pos_updated = lowered
        .find("last_updated_at")
        .expect("index must include last_updated_at");
    let pos_dep_chain = lowered
        .find("dependence_chain_id")
        .expect("index must include dependence_chain_id");
    assert!(
        pos_schedule < pos_updated && pos_updated < pos_dep_chain,
        "index key order must be schedule_priority, last_updated_at, dependence_chain_id: {index_def}"
    );
    for token in [
        "where",
        "status",
        "updated",
        "worker_id",
        "is null",
        "dependency_count",
        "= 0",
    ] {
        assert!(
            lowered.contains(token),
            "index predicate missing `{token}` in: {index_def}"
        );
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_cross_block_sustained_below_cap_stays_fast_locally(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 1.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let cap = 64_u32;
    let burst_depth = 8_usize;
    let rounds = 4_u64;

    let mut current_handle: Option<FixedBytes<32>> = None;
    let mut seen_block_numbers = HashSet::new();

    for round in 0..rounds {
        let seed = 101_u64 + round;
        let (receipts, last_output_handle) = emit_dependent_burst_seeded(
            &setup,
            current_handle,
            burst_depth,
            seed,
        )
        .await?;

        for receipt in &receipts {
            let block_number =
                receipt.block_number.expect("receipt has block number");
            seen_block_numbers.insert(block_number);
        }

        ingest_blocks_for_receipts(
            &mut db,
            &setup,
            &receipts,
            IngestOptions {
                dependence_by_connexity: false,
                dependence_cross_block: true,
                dependent_ops_max_per_chain: cap,
            },
        )
        .await?;

        current_handle = Some(last_output_handle);
        let last_block = receipts
            .last()
            .and_then(|r| r.block_number)
            .expect("receipt has block number");
        wait_for_next_block(
            &setup.args.url,
            Some(last_block),
            tokio::time::Duration::from_secs(10),
        )
        .await?;
    }

    assert!(
        seen_block_numbers.len() > 1,
        "test must span multiple blocks"
    );

    let dep_chain_id = dep_chain_id_for_output_handle(
        &setup,
        current_handle.expect("final output handle exists"),
    )
    .await?;
    let schedule_priority = sqlx::query_scalar::<_, i16>(
        "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&dep_chain_id)
    .fetch_one(&setup.db_pool)
    .await?;

    assert_eq!(
        schedule_priority, 0,
        "current behavior: below-cap batches do not accumulate into slow lane across blocks"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_cross_block_parent_lookup_finds_known_slow_parent_locally(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 3.0).await?;
    let db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let slow_parent = FixedBytes::<32>::from([0x11; 32]);
    let fast_parent = FixedBytes::<32>::from([0x22; 32]);

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_priority)
        VALUES ($1, 'updated', NOW(), NOW(), 1, 1)
        "#,
    )
    .bind(slow_parent.as_slice())
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO dependence_chain
            (dependence_chain_id, status, last_updated_at, block_timestamp, block_height, schedule_priority)
        VALUES ($1, 'updated', NOW(), NOW(), 1, 0)
        "#,
    )
    .bind(fast_parent.as_slice())
    .execute(&setup.db_pool)
    .await?;

    let mut tx = db.new_transaction().await?;
    let found = db
        .find_slow_dep_chain_ids(
            &mut tx,
            &[slow_parent.to_vec(), fast_parent.to_vec(), vec![0x33; 32]],
        )
        .await?;

    assert!(found.contains(&slow_parent));
    assert!(!found.contains(&fast_parent));
    assert_eq!(found.len(), 1);
    tx.rollback().await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_priority_is_monotonic_across_blocks_locally(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 1.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let first_output =
        ingest_dependent_burst_seeded(&mut db, &setup, None, 4, 50_u64, 1)
            .await?;
    let slow_dep_chain_id =
        dep_chain_id_for_output_handle(&setup, first_output).await?;
    let initial_priority = sqlx::query_scalar::<_, i16>(
        "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&slow_dep_chain_id)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(initial_priority, 1, "first pass should mark chain slow");

    wait_for_next_block(
        &setup.args.url,
        None,
        tokio::time::Duration::from_secs(10),
    )
    .await?;

    let second_output = ingest_dependent_burst_seeded(
        &mut db,
        &setup,
        Some(first_output),
        1,
        51_u64,
        64,
    )
    .await?;
    let second_dep_chain_id =
        dep_chain_id_for_output_handle(&setup, second_output).await?;
    assert_eq!(
        second_dep_chain_id, slow_dep_chain_id,
        "continuation should stay on the same dependence chain"
    );

    let final_priority = sqlx::query_scalar::<_, i16>(
        "SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&slow_dep_chain_id)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(
        final_priority, 1,
        "priority must not downgrade from slow to fast"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_slow_lane_off_mode_promotes_all_chains_on_startup_locally(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 3.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
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

    let _ = last_handle;
    let promoted = db.promote_all_dep_chains_to_fast_priority().await?;
    assert!(promoted > 0, "startup promotion should reset slow chains");

    let remaining_slow = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM dependence_chain WHERE schedule_priority = 1",
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(
        remaining_slow, 0,
        "off mode startup should promote all slow chains back to fast"
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
        setup.chain_id,
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
#[serial(db)]
async fn test_update_finalized_blocks_drives_orphan_cleanup_via_rpc_caller(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 1.0).await?;
    let mut db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let provider = ProviderBuilder::new()
        .wallet(setup.wallets[0].clone())
        .connect_ws(WsConnect::new(setup.args.url.clone()))
        .await?;
    let latest_block_number = provider.get_block_number().await?;
    let target_block_number = latest_block_number.saturating_sub(1);
    let canonical_block = provider
        .get_block_by_number(BlockNumberOrTag::Number(target_block_number))
        .await?
        .expect("target canonical block exists");

    let canonical_hash = canonical_block.header.hash;
    let orphan_hash = FixedBytes::<32>::from([0x22; 32]);
    let orphan_descendant_hash = FixedBytes::<32>::from([0x23; 32]);
    let canonical_handle = FixedBytes::<32>::from([0x33; 32]);
    let orphan_handle = FixedBytes::<32>::from([0x44; 32]);
    let orphan_descendant_handle = FixedBytes::<32>::from([0x45; 32]);
    let canonical_txn = FixedBytes::<32>::from([0x55; 32]);
    let orphan_txn = FixedBytes::<32>::from([0x66; 32]);
    let orphan_descendant_txn = FixedBytes::<32>::from([0x67; 32]);
    let key_id_gw = [0x77_u8; 32];

    sqlx::query!(
        r#"
        INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, parent_hash, block_number, block_status)
        VALUES
            ($1, $2, $3, $4, 'pending'),
            ($1, $5, $6, $4, 'pending'),
            ($1, $7, $5, $8, 'pending')
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
        canonical_block.header.parent_hash.to_vec(),
        target_block_number as i64,
        orphan_hash.to_vec(),
        canonical_block.header.parent_hash.to_vec(),
        orphan_descendant_hash.to_vec(),
        target_block_number as i64 + 1,
    )
    .execute(&setup.db_pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO computations_branch (
            output_handle,
            dependencies,
            fhe_operation,
            is_scalar,
            transaction_id,
            host_chain_id,
            block_number,
            producer_block_hash
        )
        VALUES
            ($1, $2, $3, FALSE, $4, $5, $6, $7),
            ($8, $9, $3, FALSE, $10, $5, $6, $11),
            ($12, $13, $3, FALSE, $14, $5, $15, $16)
        "#,
        canonical_handle.to_vec(),
        &vec![FixedBytes::<32>::from([0x88; 32]).to_vec()],
        1_i16,
        canonical_txn.to_vec(),
        setup.chain_id.as_i64(),
        target_block_number as i64,
        canonical_hash.to_vec(),
        orphan_handle.to_vec(),
        &vec![FixedBytes::<32>::from([0x99; 32]).to_vec()],
        orphan_txn.to_vec(),
        orphan_hash.to_vec(),
        orphan_descendant_handle.to_vec(),
        &vec![FixedBytes::<32>::from([0x9A; 32]).to_vec()],
        orphan_descendant_txn.to_vec(),
        target_block_number as i64 + 1,
        orphan_descendant_hash.to_vec(),
    )
    .execute(&setup.db_pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO allowed_handles_branch (
            handle,
            account_address,
            event_type,
            transaction_id,
            host_chain_id,
            block_number,
            producer_block_hash
        )
        VALUES
            ($1, $2, $3, $4, $5, $6, $7),
            ($8, $2, $3, $9, $5, $6, $10),
            ($11, $2, $3, $12, $5, $13, $14)
        "#,
        canonical_handle.to_vec(),
        "0xAccount",
        0_i16,
        canonical_txn.to_vec(),
        setup.chain_id.as_i64(),
        target_block_number as i64,
        canonical_hash.to_vec(),
        orphan_handle.to_vec(),
        orphan_txn.to_vec(),
        orphan_hash.to_vec(),
        orphan_descendant_handle.to_vec(),
        orphan_descendant_txn.to_vec(),
        target_block_number as i64 + 1,
        orphan_descendant_hash.to_vec(),
    )
    .execute(&setup.db_pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO ciphertext_digest_branch (
            host_chain_id,
            key_id_gw,
            handle,
            producer_block_hash,
            block_number,
            transaction_id
        )
        VALUES
            ($1, $2, $3, $4, $5, $6),
            ($1, $2, $7, $8, $5, $9),
            ($1, $2, $10, $11, $12, $13)
        "#,
        setup.chain_id.as_i64(),
        &key_id_gw,
        canonical_handle.to_vec(),
        canonical_hash.to_vec(),
        target_block_number as i64,
        canonical_txn.to_vec(),
        orphan_handle.to_vec(),
        orphan_hash.to_vec(),
        orphan_txn.to_vec(),
        orphan_descendant_handle.to_vec(),
        orphan_descendant_hash.to_vec(),
        target_block_number as i64 + 1,
        orphan_descendant_txn.to_vec(),
    )
    .execute(&setup.db_pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO ciphertexts_branch (
            handle,
            producer_block_hash,
            ciphertext,
            ciphertext_version,
            ciphertext_type
        )
        VALUES
            ($1, $2, $3, $4, $5),
            ($6, $7, $8, $4, $5),
            ($9, $10, $11, $4, $5)
        "#,
        canonical_handle.to_vec(),
        canonical_hash.to_vec(),
        vec![0xAA_u8],
        0_i16,
        0_i16,
        orphan_handle.to_vec(),
        orphan_hash.to_vec(),
        vec![0xBB_u8],
        orphan_descendant_handle.to_vec(),
        orphan_descendant_hash.to_vec(),
        vec![0xBC_u8],
    )
    .execute(&setup.db_pool)
    .await?;

    let mut log_iter = InfiniteLogIter::new(&setup.args);
    log_iter.init_provider_for_rpc().await?;
    update_finalized_blocks(
        &mut db,
        &mut log_iter,
        latest_block_number,
        latest_block_number - target_block_number,
    )
    .await;

    let canonical_status = sqlx::query_scalar!(
        r#"
        SELECT block_status
        FROM host_chain_blocks_valid
        WHERE chain_id = $1 AND block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_status, "finalized");

    let orphan_status = sqlx::query_scalar!(
        r#"
        SELECT block_status
        FROM host_chain_blocks_valid
        WHERE chain_id = $1 AND block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        orphan_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(orphan_status, "orphaned");

    let orphan_rows = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM computations_branch
        WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])
        "#,
        setup.chain_id.as_i64(),
        &vec![orphan_hash.to_vec(), orphan_descendant_hash.to_vec()],
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(orphan_rows, 0);

    let canonical_rows = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM computations_branch
        WHERE host_chain_id = $1 AND producer_block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_rows, 1);

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_update_block_as_finalized_returns_direct_and_descendant_orphans(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 1.0).await?;
    let db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let block_number = 10_i64;
    let canonical_hash = FixedBytes::<32>::from([0x11; 32]);
    let orphan_hash = FixedBytes::<32>::from([0x22; 32]);
    let orphan_descendant_hash = FixedBytes::<32>::from([0x23; 32]);

    sqlx::query!(
        r#"
        INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, parent_hash, block_number, block_status)
        VALUES
            ($1, $2, $3, $4, 'pending'),
            ($1, $5, $6, $4, 'pending'),
            ($1, $7, $5, $8, 'pending')
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
        Option::<Vec<u8>>::None,
        block_number,
        orphan_hash.to_vec(),
        Option::<Vec<u8>>::None,
        orphan_descendant_hash.to_vec(),
        block_number + 1,
    )
    .execute(&setup.db_pool)
    .await?;

    let mut tx = db.new_transaction().await?;
    let orphaned_hashes = db
        .update_block_as_finalized(&mut tx, block_number, &canonical_hash)
        .await?;

    let orphaned_hashes = orphaned_hashes.into_iter().collect::<HashSet<_>>();
    assert!(orphaned_hashes.contains(&orphan_hash.to_vec()));
    assert!(orphaned_hashes.contains(&orphan_descendant_hash.to_vec()));
    assert_eq!(orphaned_hashes.len(), 2);

    tx.rollback().await?;

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_update_block_as_finalized_does_not_resurrect_orphaned_block(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 1.0).await?;
    let db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let block_number = 11_i64;
    let stale_hash = FixedBytes::<32>::from([0x31; 32]);
    let sibling_hash = FixedBytes::<32>::from([0x32; 32]);

    sqlx::query(
        r#"
        INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, parent_hash, block_number, block_status)
        VALUES
            ($1, $2, NULL, $4, 'orphaned'),
            ($1, $3, NULL, $4, 'pending')
        "#,
    )
    .bind(setup.chain_id.as_i64())
    .bind(stale_hash.to_vec())
    .bind(sibling_hash.to_vec())
    .bind(block_number)
    .execute(&setup.db_pool)
    .await?;

    let mut tx = db.new_transaction().await?;
    let orphaned_hashes = db
        .update_block_as_finalized(&mut tx, block_number, &stale_hash)
        .await?;
    tx.commit().await?;

    assert!(
        orphaned_hashes.is_empty(),
        "stale finalization should not orphan sibling branches"
    );

    let statuses = sqlx::query(
        r#"
        SELECT block_hash, block_status
        FROM host_chain_blocks_valid
        WHERE chain_id = $1
          AND block_number = $2
        "#,
    )
    .bind(setup.chain_id.as_i64())
    .bind(block_number)
    .fetch_all(&setup.db_pool)
    .await?
    .into_iter()
    .map(|row| {
        (
            row.get::<Vec<u8>, _>("block_hash"),
            row.get::<String, _>("block_status"),
        )
    })
    .collect::<HashSet<_>>();

    assert!(statuses.contains(&(stale_hash.to_vec(), "orphaned".to_owned())));
    assert!(statuses.contains(&(sibling_hash.to_vec(), "pending".to_owned())));

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_legacy_allowed_handle_writes_are_mirrored_branchless(
) -> Result<(), anyhow::Error> {
    let db_instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .expect("valid db instance");
    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_instance.db_url())
        .await?;
    let handle = FixedBytes::<32>::from([0xA1; 32]);
    let transaction_id = FixedBytes::<32>::from([0xB2; 32]);
    let host_chain_id = 4242_i64;

    sqlx::query(
        r#"
        INSERT INTO allowed_handles (
            tenant_id,
            handle,
            account_address,
            event_type,
            transaction_id,
            host_chain_id,
            block_number
        )
        VALUES (0, $1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(handle.to_vec())
    .bind("0xlegacy")
    .bind(0_i16)
    .bind(transaction_id.to_vec())
    .bind(host_chain_id)
    .bind(10_i64)
    .execute(&db_pool)
    .await?;

    let mirrored = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND host_chain_id = $3
          AND producer_block_hash = ''::BYTEA
          AND block_hash = ''::BYTEA
        "#,
    )
    .bind(handle.to_vec())
    .bind("0xlegacy")
    .bind(host_chain_id)
    .fetch_one(&db_pool)
    .await?;
    assert_eq!(mirrored, 1);

    sqlx::query(
        "UPDATE allowed_handles SET txn_is_sent = TRUE WHERE handle = $1 AND account_address = $2",
    )
    .bind(handle.to_vec())
    .bind("0xlegacy")
    .execute(&db_pool)
    .await?;

    let txn_is_sent = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT txn_is_sent
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = ''::BYTEA
          AND block_hash = ''::BYTEA
        "#,
    )
    .bind(handle.to_vec())
    .bind("0xlegacy")
    .fetch_one(&db_pool)
    .await?;
    assert!(txn_is_sent);

    sqlx::query("DELETE FROM allowed_handles WHERE handle = $1 AND account_address = $2")
        .bind(handle.to_vec())
        .bind("0xlegacy")
        .execute(&db_pool)
        .await?;

    let mirrored = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = ''::BYTEA
          AND block_hash = ''::BYTEA
        "#,
    )
    .bind(handle.to_vec())
    .bind("0xlegacy")
    .fetch_one(&db_pool)
    .await?;
    assert_eq!(mirrored, 0);

    let wave1_handle = FixedBytes::<32>::from([0xA2; 32]);
    let wave1_transaction_id = FixedBytes::<32>::from([0xB3; 32]);
    let producer_block_hash = FixedBytes::<32>::from([0xC4; 32]);
    let acl_block_hash = FixedBytes::<32>::from([0xD5; 32]);

    sqlx::query(
        r#"
        INSERT INTO allowed_handles_branch (
            handle,
            account_address,
            event_type,
            transaction_id,
            host_chain_id,
            block_number,
            block_hash,
            producer_block_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(0_i16)
    .bind(wave1_transaction_id.to_vec())
    .bind(host_chain_id)
    .bind(11_i64)
    .bind(acl_block_hash.to_vec())
    .bind(producer_block_hash.to_vec())
    .execute(&db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO allowed_handles (
            tenant_id,
            handle,
            account_address,
            event_type,
            transaction_id,
            host_chain_id,
            block_number
        )
        VALUES (0, $1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(0_i16)
    .bind(wave1_transaction_id.to_vec())
    .bind(host_chain_id)
    .bind(11_i64)
    .execute(&db_pool)
    .await?;

    let branchless_duplicates = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = ''::BYTEA
          AND block_hash = ''::BYTEA
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .fetch_one(&db_pool)
    .await?;
    assert_eq!(branchless_duplicates, 0);

    let branchful_rows = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = $3
          AND block_hash = $4
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(producer_block_hash.to_vec())
    .bind(acl_block_hash.to_vec())
    .fetch_one(&db_pool)
    .await?;
    assert_eq!(branchful_rows, 1);

    let wave1_txn_hash = FixedBytes::<32>::from([0xE6; 32]);
    sqlx::query(
        r#"
        UPDATE allowed_handles
        SET txn_is_sent = TRUE,
            txn_limited_retries_count = 4,
            txn_unlimited_retries_count = 7,
            txn_hash = $3,
            txn_block_number = 99,
            txn_last_error = 'sent from legacy sender',
            txn_last_error_at = NOW()
        WHERE handle = $1
          AND account_address = $2
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(wave1_txn_hash.to_vec())
    .execute(&db_pool)
    .await?;

    let branchless_duplicates = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = ''::BYTEA
          AND block_hash = ''::BYTEA
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .fetch_one(&db_pool)
    .await?;
    assert_eq!(branchless_duplicates, 0);

    let (
        txn_is_sent,
        txn_limited_retries_count,
        txn_unlimited_retries_count,
        txn_hash,
        txn_block_number,
        txn_last_error,
    ) = sqlx::query_as::<
        _,
        (bool, i32, i32, Option<Vec<u8>>, Option<i64>, Option<String>),
    >(
        r#"
        SELECT
            txn_is_sent,
            txn_limited_retries_count,
            txn_unlimited_retries_count,
            txn_hash,
            txn_block_number,
            txn_last_error
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = $3
          AND block_hash = $4
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(producer_block_hash.to_vec())
    .bind(acl_block_hash.to_vec())
    .fetch_one(&db_pool)
    .await?;
    assert!(txn_is_sent);
    assert_eq!(txn_limited_retries_count, 4);
    assert_eq!(txn_unlimited_retries_count, 7);
    assert_eq!(txn_hash, Some(wave1_txn_hash.to_vec()));
    assert_eq!(txn_block_number, Some(99));
    assert_eq!(txn_last_error.as_deref(), Some("sent from legacy sender"));

    sqlx::query(
        r#"
        INSERT INTO allowed_handles_branch (
            handle,
            account_address,
            event_type,
            transaction_id,
            host_chain_id,
            block_number,
            producer_block_hash,
            block_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6, ''::BYTEA, ''::BYTEA)
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(0_i16)
    .bind(wave1_transaction_id.to_vec())
    .bind(host_chain_id)
    .bind(11_i64)
    .execute(&db_pool)
    .await?;

    sqlx::raw_sql(include_str!(
        "../../db-migration/migrations/20260610145000_branch_digest_late_backfill.sql"
    ))
    .execute(&db_pool)
    .await?;

    let branchless_duplicates = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = ''::BYTEA
          AND block_hash = ''::BYTEA
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .fetch_one(&db_pool)
    .await?;
    assert_eq!(branchless_duplicates, 0);

    let txn_is_sent = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT txn_is_sent
        FROM allowed_handles_branch
        WHERE handle = $1
          AND account_address = $2
          AND producer_block_hash = $3
          AND block_hash = $4
        "#,
    )
    .bind(wave1_handle.to_vec())
    .bind("0xwave1")
    .bind(producer_block_hash.to_vec())
    .bind(acl_block_hash.to_vec())
    .fetch_one(&db_pool)
    .await?;
    assert!(txn_is_sent);

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_finalization_cleanup_removes_orphaned_branch_rows_locally(
) -> Result<(), anyhow::Error> {
    let setup = setup_with_block_time(None, 1.0).await?;
    let db = Database::new(
        &setup.args.database_url,
        setup.chain_id,
        setup.args.dependence_cache_size,
    )
    .await?;

    let block_number = 10_i64;
    let canonical_hash = FixedBytes::<32>::from([0x11; 32]);
    let orphan_hash = FixedBytes::<32>::from([0x22; 32]);
    let orphan_descendant_hash = FixedBytes::<32>::from([0x23; 32]);
    let canonical_handle = FixedBytes::<32>::from([0x33; 32]);
    let orphan_handle = FixedBytes::<32>::from([0x44; 32]);
    let orphan_descendant_handle = FixedBytes::<32>::from([0x45; 32]);
    let canonical_txn = FixedBytes::<32>::from([0x55; 32]);
    let orphan_txn = FixedBytes::<32>::from([0x66; 32]);
    let orphan_descendant_txn = FixedBytes::<32>::from([0x67; 32]);
    let key_id_gw = [0x77_u8; 32];
    let canonical_dependencies =
        vec![FixedBytes::<32>::from([0x88; 32]).to_vec()];
    let orphan_dependencies = vec![FixedBytes::<32>::from([0x99; 32]).to_vec()];
    let orphan_descendant_dependencies =
        vec![FixedBytes::<32>::from([0x9A; 32]).to_vec()];
    let orphaned_hashes =
        vec![orphan_hash.to_vec(), orphan_descendant_hash.to_vec()];

    sqlx::query!(
        r#"
        INSERT INTO host_chain_blocks_valid
            (chain_id, block_hash, parent_hash, block_number, block_status)
        VALUES
            ($1, $2, $3, $4, 'pending'),
            ($1, $5, $6, $4, 'pending'),
            ($1, $7, $5, $8, 'pending')
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
        Option::<Vec<u8>>::None,
        block_number,
        orphan_hash.to_vec(),
        Option::<Vec<u8>>::None,
        orphan_descendant_hash.to_vec(),
        block_number + 1,
    )
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO computations_branch (
            output_handle,
            dependencies,
            fhe_operation,
            is_scalar,
            transaction_id,
            host_chain_id,
            block_number,
            producer_block_hash
        )
        VALUES
            ($1, $2, $3, FALSE, $4, $5, $6, $7),
            ($8, $9, $3, FALSE, $10, $5, $6, $11),
            ($12, $13, $3, FALSE, $14, $5, $15, $16)
        "#,
    )
    .bind(canonical_handle.to_vec())
    .bind(&canonical_dependencies)
    .bind(1_i16)
    .bind(canonical_txn.to_vec())
    .bind(setup.chain_id.as_i64())
    .bind(block_number)
    .bind(canonical_hash.to_vec())
    .bind(orphan_handle.to_vec())
    .bind(&orphan_dependencies)
    .bind(orphan_txn.to_vec())
    .bind(orphan_hash.to_vec())
    .bind(orphan_descendant_handle.to_vec())
    .bind(&orphan_descendant_dependencies)
    .bind(orphan_descendant_txn.to_vec())
    .bind(block_number + 1)
    .bind(orphan_descendant_hash.to_vec())
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO computations (
            output_handle,
            dependencies,
            fhe_operation,
            is_scalar,
            transaction_id,
            host_chain_id,
            block_number
        )
        VALUES
            ($1, $2, $3, FALSE, $4, $5, $6),
            ($7, $8, $3, FALSE, $9, $5, $6),
            ($10, $11, $3, FALSE, $12, $5, $13)
        "#,
    )
    .bind(canonical_handle.to_vec())
    .bind(&canonical_dependencies)
    .bind(1_i16)
    .bind(canonical_txn.to_vec())
    .bind(setup.chain_id.as_i64())
    .bind(block_number)
    .bind(orphan_handle.to_vec())
    .bind(&orphan_dependencies)
    .bind(orphan_txn.to_vec())
    .bind(orphan_descendant_handle.to_vec())
    .bind(&orphan_descendant_dependencies)
    .bind(orphan_descendant_txn.to_vec())
    .bind(block_number + 1)
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO pbs_computations_branch (
            handle,
            transaction_id,
            host_chain_id,
            block_number,
            producer_block_hash
        )
        VALUES
            ($1, $2, $3, $4, $5),
            ($6, $7, $3, $4, $8),
            ($9, $10, $3, $11, $12)
        "#,
    )
    .bind(canonical_handle.to_vec())
    .bind(canonical_txn.to_vec())
    .bind(setup.chain_id.as_i64())
    .bind(block_number)
    .bind(canonical_hash.to_vec())
    .bind(orphan_handle.to_vec())
    .bind(orphan_txn.to_vec())
    .bind(orphan_hash.to_vec())
    .bind(orphan_descendant_handle.to_vec())
    .bind(orphan_descendant_txn.to_vec())
    .bind(block_number + 1)
    .bind(orphan_descendant_hash.to_vec())
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO allowed_handles_branch (
            handle,
            account_address,
            event_type,
            transaction_id,
            host_chain_id,
            block_number,
            producer_block_hash
        )
        VALUES
            ($1, $2, $3, $4, $5, $6, $7),
            ($8, $2, $3, $9, $5, $6, $10),
            ($11, $2, $3, $12, $5, $13, $14)
        "#,
    )
    .bind(canonical_handle.to_vec())
    .bind("0xAccount")
    .bind(0_i16)
    .bind(canonical_txn.to_vec())
    .bind(setup.chain_id.as_i64())
    .bind(block_number)
    .bind(canonical_hash.to_vec())
    .bind(orphan_handle.to_vec())
    .bind(orphan_txn.to_vec())
    .bind(orphan_hash.to_vec())
    .bind(orphan_descendant_handle.to_vec())
    .bind(orphan_descendant_txn.to_vec())
    .bind(block_number + 1)
    .bind(orphan_descendant_hash.to_vec())
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO ciphertext_digest_branch (
            host_chain_id,
            key_id_gw,
            handle,
            producer_block_hash,
            block_number,
            transaction_id
        )
        VALUES
            ($1, $2, $3, $4, $5, $6),
            ($1, $2, $7, $8, $5, $9),
            ($1, $2, $10, $11, $12, $13)
        "#,
    )
    .bind(setup.chain_id.as_i64())
    .bind(key_id_gw.to_vec())
    .bind(canonical_handle.to_vec())
    .bind(canonical_hash.to_vec())
    .bind(block_number)
    .bind(canonical_txn.to_vec())
    .bind(orphan_handle.to_vec())
    .bind(orphan_hash.to_vec())
    .bind(orphan_txn.to_vec())
    .bind(orphan_descendant_handle.to_vec())
    .bind(orphan_descendant_hash.to_vec())
    .bind(block_number + 1)
    .bind(orphan_descendant_txn.to_vec())
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO ciphertexts_branch (
            handle,
            producer_block_hash,
            ciphertext,
            ciphertext_version,
            ciphertext_type
        )
        VALUES
            ($1, $2, $3, $4, $5),
            ($6, $7, $8, $4, $5),
            ($9, $10, $11, $4, $5)
        "#,
    )
    .bind(canonical_handle.to_vec())
    .bind(canonical_hash.to_vec())
    .bind(vec![0xAA_u8])
    .bind(0_i16)
    .bind(0_i16)
    .bind(orphan_handle.to_vec())
    .bind(orphan_hash.to_vec())
    .bind(vec![0xBB_u8])
    .bind(orphan_descendant_handle.to_vec())
    .bind(orphan_descendant_hash.to_vec())
    .bind(vec![0xBC_u8])
    .execute(&setup.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO ciphertexts128_branch (
            handle,
            producer_block_hash,
            ciphertext
        )
        VALUES
            ($1, $2, $3),
            ($4, $5, $6),
            ($7, $8, $9)
        "#,
    )
    .bind(canonical_handle.to_vec())
    .bind(canonical_hash.to_vec())
    .bind(vec![0xCC_u8])
    .bind(orphan_handle.to_vec())
    .bind(orphan_hash.to_vec())
    .bind(vec![0xDD_u8])
    .bind(orphan_descendant_handle.to_vec())
    .bind(orphan_descendant_hash.to_vec())
    .bind(vec![0xDE_u8])
    .execute(&setup.db_pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO delegate_user_decrypt (
            delegator,
            delegate,
            contract_address,
            delegation_counter,
            old_expiration_date,
            new_expiration_date,
            host_chain_id,
            block_number,
            block_hash,
            transaction_id,
            on_gateway,
            reorg_out
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, FALSE, FALSE
        )
        "#,
        Address::from([0xA1; 20]).into_array().to_vec(),
        Address::from([0xA2; 20]).into_array().to_vec(),
        Address::from([0xA3; 20]).into_array().to_vec(),
        1_i64,
        BigDecimal::from(0_i64),
        BigDecimal::from(1_i64),
        setup.chain_id.as_i64(),
        block_number + 1,
        orphan_descendant_hash.to_vec(),
        orphan_descendant_txn.to_vec(),
    )
    .execute(&setup.db_pool)
    .await?;

    let mut tx = db.new_transaction().await?;
    db.mark_block_as_valid(
        &mut tx,
        &BlockSummary {
            number: block_number as u64,
            hash: canonical_hash,
            parent_hash: FixedBytes::<32>::ZERO,
            timestamp: 0,
        },
        true,
    )
    .await?;
    tx.commit().await?;

    let canonical_status = sqlx::query_scalar!(
        r#"
        SELECT block_status
        FROM host_chain_blocks_valid
        WHERE chain_id = $1 AND block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_status, "finalized");

    let orphan_status = sqlx::query_scalar!(
        r#"
        SELECT block_status
        FROM host_chain_blocks_valid
        WHERE chain_id = $1 AND block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        orphan_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(orphan_status, "orphaned");
    let orphan_descendant_status = sqlx::query_scalar!(
        r#"
        SELECT block_status
        FROM host_chain_blocks_valid
        WHERE chain_id = $1 AND block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        orphan_descendant_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(orphan_descendant_status, "orphaned");

    let canonical_computations = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM computations_branch
        WHERE host_chain_id = $1 AND producer_block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    let orphan_computations = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM computations_branch
        WHERE host_chain_id = $1
          AND producer_block_hash = ANY($2::bytea[])
        "#,
        setup.chain_id.as_i64(),
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(canonical_computations, 1);
    assert_eq!(orphan_computations, 0);

    let canonical_pbs = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM pbs_computations_branch
        WHERE host_chain_id = $1 AND producer_block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    let orphan_pbs = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM pbs_computations_branch
        WHERE host_chain_id = $1
          AND producer_block_hash = ANY($2::bytea[])
        "#,
        setup.chain_id.as_i64(),
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(canonical_pbs, 1);
    assert_eq!(orphan_pbs, 0);

    let canonical_digest = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM ciphertext_digest_branch
        WHERE host_chain_id = $1 AND producer_block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    let orphan_digest = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM ciphertext_digest_branch
        WHERE host_chain_id = $1
          AND producer_block_hash = ANY($2::bytea[])
        "#,
        setup.chain_id.as_i64(),
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(canonical_digest, 1);
    assert_eq!(orphan_digest, 0);

    let canonical_allowed = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE host_chain_id = $1 AND producer_block_hash = $2
        "#,
        setup.chain_id.as_i64(),
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    let orphan_allowed = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM allowed_handles_branch
        WHERE host_chain_id = $1
          AND producer_block_hash = ANY($2::bytea[])
        "#,
        setup.chain_id.as_i64(),
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(canonical_allowed, 1);
    assert_eq!(orphan_allowed, 0);

    let canonical_ct64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM ciphertexts_branch
        WHERE producer_block_hash = $1
        "#,
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    let orphan_ct64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM ciphertexts_branch
        WHERE producer_block_hash = ANY($1::bytea[])
        "#,
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(canonical_ct64, 1);
    assert_eq!(orphan_ct64, 0);

    let canonical_ct128 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM ciphertexts128_branch
        WHERE producer_block_hash = $1
        "#,
        canonical_hash.to_vec(),
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    let orphan_ct128 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM ciphertexts128_branch
        WHERE producer_block_hash = ANY($1::bytea[])
        "#,
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(canonical_ct128, 1);
    assert_eq!(orphan_ct128, 0);

    let canonical_computations_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM computations_branch WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(setup.chain_id.as_i64())
    .bind(canonical_hash.to_vec())
    .fetch_one(&setup.db_pool)
    .await?;
    let orphan_computations_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM computations_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])",
    )
    .bind(setup.chain_id.as_i64())
    .bind(&orphaned_hashes)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_computations_branch, 1);
    assert_eq!(orphan_computations_branch, 0);

    let legacy_orphan_computations = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM computations
         WHERE host_chain_id = $1
           AND output_handle = ANY($2::bytea[])",
    )
    .bind(setup.chain_id.as_i64())
    .bind(vec![
        orphan_handle.to_vec(),
        orphan_descendant_handle.to_vec(),
    ])
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(legacy_orphan_computations, 0);

    let canonical_pbs_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pbs_computations_branch WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(setup.chain_id.as_i64())
    .bind(canonical_hash.to_vec())
    .fetch_one(&setup.db_pool)
    .await?;
    let orphan_pbs_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pbs_computations_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])",
    )
    .bind(setup.chain_id.as_i64())
    .bind(&orphaned_hashes)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_pbs_branch, 1);
    assert_eq!(orphan_pbs_branch, 0);

    let canonical_digest_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ciphertext_digest_branch WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(setup.chain_id.as_i64())
    .bind(canonical_hash.to_vec())
    .fetch_one(&setup.db_pool)
    .await?;
    let orphan_digest_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ciphertext_digest_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])",
    )
    .bind(setup.chain_id.as_i64())
    .bind(&orphaned_hashes)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_digest_branch, 1);
    assert_eq!(orphan_digest_branch, 0);

    let canonical_allowed_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM allowed_handles_branch WHERE host_chain_id = $1 AND producer_block_hash = $2",
    )
    .bind(setup.chain_id.as_i64())
    .bind(canonical_hash.to_vec())
    .fetch_one(&setup.db_pool)
    .await?;
    let orphan_allowed_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM allowed_handles_branch WHERE host_chain_id = $1 AND producer_block_hash = ANY($2::bytea[])",
    )
    .bind(setup.chain_id.as_i64())
    .bind(&orphaned_hashes)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_allowed_branch, 1);
    assert_eq!(orphan_allowed_branch, 0);

    let canonical_ct64_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ciphertexts_branch WHERE producer_block_hash = $1",
    )
    .bind(canonical_hash.to_vec())
    .fetch_one(&setup.db_pool)
    .await?;
    let orphan_ct64_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ciphertexts_branch WHERE producer_block_hash = ANY($1::bytea[])",
    )
    .bind(&orphaned_hashes)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_ct64_branch, 1);
    assert_eq!(orphan_ct64_branch, 0);

    let canonical_ct128_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ciphertexts128_branch WHERE producer_block_hash = $1",
    )
    .bind(canonical_hash.to_vec())
    .fetch_one(&setup.db_pool)
    .await?;
    let orphan_ct128_branch = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ciphertexts128_branch WHERE producer_block_hash = ANY($1::bytea[])",
    )
    .bind(&orphaned_hashes)
    .fetch_one(&setup.db_pool)
    .await?;
    assert_eq!(canonical_ct128_branch, 1);
    assert_eq!(orphan_ct128_branch, 0);

    let orphan_delegations = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM delegate_user_decrypt
        WHERE host_chain_id = $1
          AND block_hash = ANY($2::bytea[])
        "#,
        setup.chain_id.as_i64(),
        &orphaned_hashes as _,
    )
    .fetch_one(&setup.db_pool)
    .await?
    .unwrap_or(0);
    assert_eq!(orphan_delegations, 0);

    Ok(())
}

#[tokio::test]
async fn test_only_catchup_loop_requires_negative_start_at_block(
) -> Result<(), anyhow::Error> {
    let args = Args {
        url: "ws://127.0.0.1:8545".to_string(),
        acl_contract_address: "".to_string(),
        tfhe_contract_address: "".to_string(),
        kms_generation_address: String::new(),
        confidential_bridge_address: String::new(),
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
        catchup_finalization_in_blocks: 3,
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

async fn check_finalization_status(setup: &Setup) {
    let provider = ProviderBuilder::new()
        .wallet(setup.wallets[0].clone())
        .connect_ws(WsConnect::new(setup.args.url.to_string()))
        .await
        .unwrap();
    let deadline =
        tokio::time::Instant::now() + tokio::time::Duration::from_secs(15);
    loop {
        // Verify block finalization status: for each block number, one should be
        // finalized and others orphaned. During a deep reorg the listener can be
        // between inserting missing ancestors and applying the status transition,
        // so retry transient mismatches before failing the test.
        let blocks = sqlx::query!(
            "SELECT block_number, block_hash, block_status FROM host_chain_blocks_valid",
        )
        .fetch_all(&setup.db_pool)
        .await
        .expect("Failed to fetch blocks from database");

        let block_max = blocks
            .iter()
            .map(|b| b.block_number)
            .max()
            .expect("At least one block should be ingested");

        let mut blocks_by_number: std::collections::HashMap<
            i64,
            Vec<(Vec<u8>, String)>,
        > = std::collections::HashMap::new();
        for block in blocks {
            if block.block_number > block_max - 5 {
                continue; // pending blocks within finalization window can be ignored for this assert
            }
            blocks_by_number
                .entry(block.block_number)
                .or_default()
                .push((block.block_hash, block.block_status));
        }

        let mut mismatch = None;
        for (block_number, block_variants) in blocks_by_number.iter() {
            let finalized_count = block_variants
                .iter()
                .filter(|(_, status)| status == "finalized")
                .count();
            let orphan_count = block_variants
                .iter()
                .filter(|(_, status)| status == "orphaned")
                .count();
            if finalized_count != 1 {
                mismatch = Some(format!(
                    "Block {block_number} should have exactly one finalized variant, found {finalized_count}"
                ));
                break;
            }
            if orphan_count != block_variants.len() - 1 {
                mismatch = Some(format!(
                    "Block {block_number} should have remaining variants as orphan"
                ));
                break;
            }
            let finalized_hash = block_variants
                .iter()
                .find(|(_, status)| status == "finalized")
                .map(|(hash, _)| hash)
                .unwrap();
            let expected_hash = provider
                .get_block_by_number((*block_number as u64).into())
                .await
                .unwrap()
                .unwrap()
                .header
                .hash;
            if expected_hash.0 != finalized_hash.as_slice() {
                mismatch = Some(format!(
                    "Finalized block hash for block {block_number} does not match expected"
                ));
                break;
            }
        }
        if mismatch.is_none() {
            return;
        }
        let mismatch = mismatch.unwrap();
        assert!(tokio::time::Instant::now() < deadline, "{mismatch}");
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
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
    // Restart/kill many times until no more events are consumed.
    //
    // Under branch-context orphan cleanup (finalization deletes orphaned
    // rows from legacy `computations`/`allowed_handles`), the reorg path
    // cannot satisfy an equal-to-total-emissions assertion: ~25 blocks
    // worth of events get orphaned and pruned after finality. For the
    // reorg variant the test therefore asserts on canonical-chain
    // presence (through `check_finalization_status` + a plateau check)
    // rather than a fixed count. The no-reorg variant keeps the exact
    // count assertion.
    let expected_tfhe_events = nb_wallets * NB_EVENTS_PER_WALLET;
    let expected_acl_events = nb_wallets * NB_EVENTS_PER_WALLET;
    let mut plateau_ticks = 0;
    for _ in 1..40 {
        // 4 mins max to avoid stalled CI
        let listener_handle = tokio::spawn(main(args.clone()));
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        check_finalization_status(&setup).await;
        let tfhe_new_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM computations_branch WHERE producer_block_hash <> ''::BYTEA",
        )
        .fetch_one(&setup.db_pool)
        .await?;
        let acl_new_count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM allowed_handles_branch
            WHERE producer_block_hash <> ''::BYTEA
               OR block_hash <> ''::BYTEA
            "#,
        )
        .fetch_one(&setup.db_pool)
        .await?;
        let no_count_change = tfhe_events_count == tfhe_new_count
            && acl_events_count == acl_new_count;
        let reached_expected = tfhe_new_count >= expected_tfhe_events
            && acl_new_count >= expected_acl_events;
        let reorg_plateau = reorg
            && event_source.is_finished()
            && no_count_change
            && tfhe_new_count > 0
            && acl_new_count > 0;
        if reorg_plateau {
            plateau_ticks += 1;
        } else {
            plateau_ticks = 0;
        }
        let stable_under_reorg = reorg_plateau && plateau_ticks >= 3;
        if event_source.is_finished()
            && no_count_change
            && (reached_expected || stable_under_reorg)
        {
            listener_handle.abort();
            break;
        };
        tfhe_events_count = tfhe_new_count;
        acl_events_count = acl_new_count;
        if kill {
            listener_handle.abort();
            while !listener_handle.is_finished() {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
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
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    if reorg {
        // Orphan cleanup removes any events from the orphaned branch, so
        // the final counts are bounded above by the raw emission totals
        // and below by the canonical-chain subset. Require both non-zero,
        // plus the finalization invariant (already checked every loop),
        // plus the orphan-cleanup invariant: no row in `computations` may
        // reference a producer_block_hash whose block is now orphaned.
        assert!(
            tfhe_events_count > 0,
            "reorg variant: at least one tfhe event must survive cleanup"
        );
        assert!(
            tfhe_events_count <= expected_tfhe_events + 1,
            "reorg variant: tfhe events must not exceed raw emissions (+1 reorg replay) {tfhe_events_count} > {expected_tfhe_events}"
        );
        assert!(
            acl_events_count > 0,
            "reorg variant: at least one acl event must survive cleanup"
        );
        assert!(
            acl_events_count <= expected_acl_events,
            "reorg variant: acl events must not exceed raw emissions {acl_events_count} > {expected_acl_events}"
        );
        check_finalization_status(&setup).await;
        let orphaned_leftovers = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM computations_branch c
            JOIN host_chain_blocks_valid b
              ON c.host_chain_id = b.chain_id
             AND c.producer_block_hash = b.block_hash
            WHERE b.block_status = 'orphaned'
            "#,
        )
        .fetch_one(&setup.db_pool)
        .await?;
        assert_eq!(
            orphaned_leftovers, 0,
            "reorg variant: computations must not retain rows for orphaned blocks"
        );

        let legacy_orphaned_leftovers = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM computations c
            JOIN host_chain_blocks_valid b
              ON c.host_chain_id = b.chain_id
             AND c.block_number = b.block_number
            WHERE b.block_status = 'orphaned'
            "#,
        )
        .fetch_one(&setup.db_pool)
        .await?;
        assert_eq!(
            legacy_orphaned_leftovers, 0,
            "reorg variant: legacy computations must not retain rows for orphaned blocks"
        );
    } else {
        assert_eq!(tfhe_events_count, expected_tfhe_events);
        assert_eq!(acl_events_count, expected_acl_events);
    }

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
    let nb_wallets = setup.wallets.len() as i64;
    let expected = nb_wallets * nb_event_per_wallet;
    let (tfhe_events_count, acl_events_count) = wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(30),
        &format!("waiting for first catchup (expected {expected})"),
        |tfhe, acl| tfhe >= expected && acl >= expected,
    )
    .await?;
    assert_eq!(tfhe_events_count, expected);
    assert_eq!(acl_events_count, expected);
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

    let expected2 = 2 * nb_wallets * nb_event_per_wallet;
    let (tfhe_events_count, acl_events_count) = wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(30),
        &format!("waiting for second batch (expected {expected2})"),
        |tfhe, acl| tfhe >= expected2 && acl >= expected2,
    )
    .await?;
    assert_eq!(tfhe_events_count, expected2);
    assert_eq!(acl_events_count, expected2);
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
    let nb_wallets = setup.wallets.len() as i64;
    let expected = nb_wallets * nb_event_per_wallet;
    let (tfhe_events_count, acl_events_count) = wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(30),
        &format!("waiting for catchup (expected {expected})"),
        |tfhe, acl| tfhe >= expected && acl >= expected,
    )
    .await?;
    eprintln!("End block {:?}", args.end_at_block);
    assert_eq!(tfhe_events_count, expected);
    assert_eq!(acl_events_count, expected);
    // Allow the listener to finish after ingesting all events
    let finish_deadline =
        tokio::time::Instant::now() + tokio::time::Duration::from_secs(10);
    while !listener_handle.is_finished() {
        assert!(
            tokio::time::Instant::now() < finish_deadline,
            "Listener should stop"
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
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
    let nb_wallets = setup.wallets.len() as i64;
    let expected = nb_wallets * nb_event_per_wallet;
    let (tfhe_events_count, acl_events_count) = wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(sleep_secs.max(30)),
        &format!("waiting for catchup in scenario (expected {expected})"),
        |tfhe, acl| tfhe >= expected && acl >= expected,
    )
    .await?;

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

    // Poll until second catchup iteration ingests additional events
    wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(30),
        "waiting for second catchup iteration",
        |tfhe, acl| {
            tfhe > first_tfhe_events_count && acl > first_acl_events_count
        },
    )
    .await?;

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

/// Tests that the host-listener can re-process events after a revert.
///
/// 1. Start listener, emit events, wait until all are in the DB.
/// 2. Stop listener, run the revert SQL to delete half the blocks.
/// 3. Restart listener in catchup mode, wait until all events are back.
#[tokio::test]
#[serial(db)]
async fn test_host_listener_recovers_after_revert() -> Result<(), anyhow::Error>
{
    let setup = setup(None).await?;
    let chain_id = setup.chain_id.as_i64();
    let nb_events_per_wallet: i64 = 5;
    let expected = setup.wallets.len() as i64 * nb_events_per_wallet;

    // Start listener, emit events, wait for all to be processed.
    let listener = tokio::spawn(main(setup.args.clone()));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);
    emit_events(
        &setup.wallets,
        &setup.args.url,
        setup.tfhe_contract.clone(),
        setup.acl_contract.clone(),
        false,
        nb_events_per_wallet,
    )
    .await;
    wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(60),
        "waiting for initial processing",
        |tfhe, acl| tfhe >= expected && acl >= expected,
    )
    .await?;

    // Stop listener.
    listener.abort();
    let _ = listener.await;

    // Prepare: the revert script needs host_chains and poller_state rows.
    sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'test', '0x0') ON CONFLICT DO NOTHING")
        .bind(chain_id).execute(&setup.db_pool).await?;
    let max_block: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(block_number), 0) FROM transactions WHERE chain_id = $1")
        .bind(chain_id).fetch_one(&setup.db_pool).await?;
    sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES ($1, $2) ON CONFLICT (chain_id) DO UPDATE SET last_caught_up_block = $2")
        .bind(chain_id).bind(max_block).execute(&setup.db_pool).await?;

    // Revert to midway. Verify some data was deleted.
    let revert_to = max_block / 2;
    let sql = test_harness::db_utils::revert_coprocessor_db_state_sql(
        chain_id, revert_to,
    );
    sqlx::raw_sql(&sql)
        .execute(&setup.db_pool)
        .await
        .expect("revert failed");
    let after_revert: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM computations_branch WHERE producer_block_hash <> ''::BYTEA",
    )
    .fetch_one(&setup.db_pool)
    .await?;
    assert!(
        after_revert < expected,
        "revert should delete some computations: {after_revert} < {expected}"
    );

    // Restart listener in catchup mode, wait for all events to come back.
    let mut args = setup.args.clone();
    args.start_at_block = Some(0);
    let listener = tokio::spawn(main(args));
    assert!(health_check::wait_healthy(&setup.health_check_url, 60, 1).await);
    let (tfhe, acl) = wait_for_event_counts(
        &setup.db_pool,
        tokio::time::Duration::from_secs(60),
        "waiting for re-processing after revert",
        |tfhe, acl| tfhe >= expected && acl >= expected,
    )
    .await?;
    assert_eq!(tfhe, expected, "computations after revert");
    assert_eq!(acl, expected, "allowed_handles after revert");

    listener.abort();
    Ok(())
}

/// Wave-1 contract: every event write must land in BOTH the legacy tables
/// (still feeding the running legacy pipeline) and the `*_branch` tables
/// (consumed by the wave-2 block-scoped readers).
#[tokio::test]
#[serial(db)]
async fn test_wave1_dual_writes_legacy_and_branch_tables(
) -> Result<(), Box<dyn std::error::Error>> {
    use alloy::primitives::Log as EventLog;
    use fhevm_engine_common::types::AllowEvents;
    use host_listener::contracts::TfheContract;
    use host_listener::contracts::TfheContract::TfheContractEvents;
    use host_listener::database::tfhe_event_propagate::{ClearConst, LogTfhe};
    use sqlx::types::time::PrimitiveDateTime;

    let test_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let chain_id = ChainId::try_from(42_u64)?;
    let db_url = test_instance.db_url.clone();
    let db = Database::new(&db_url, chain_id, 128).await?;
    let pool = db.pool.read().await.clone();

    let handle = FixedBytes::<32>::from([0x42; 32]);
    let tx_hash = FixedBytes::<32>::from([0x77; 32]);
    let block_hash = FixedBytes::<32>::from([0x33; 32]);
    let caller: Address =
        "0x1111111111111111111111111111111111111111".parse()?;

    // 1. Computation event (TrivialEncrypt producing `handle`).
    let event = LogTfhe {
        event: EventLog {
            address: Address::ZERO,
            data: TfheContractEvents::TrivialEncrypt(
                TfheContract::TrivialEncrypt {
                    caller,
                    pt: ClearConst::from_be_slice(&[7u8]),
                    toType: 4u8,
                    result: handle,
                },
            ),
        },
        transaction_hash: Some(tx_hash),
        is_allowed: true,
        block_number: 7,
        block_hash,
        block_timestamp: PrimitiveDateTime::MAX,
        dependence_chain: tx_hash,
        tx_depth_size: 0,
        log_index: None,
    };
    let mut tx = db.new_transaction().await?;
    db.insert_tfhe_event(&mut tx, &event).await?;

    // 2. Allowed handle + PBS computations.
    db.insert_allowed_handle(
        &mut tx,
        handle.to_vec(),
        format!("{caller:#x}"),
        AllowEvents::AllowedAccount,
        Some(tx_hash.to_vec()),
        ProducerBlock::new(block_hash.as_slice(), 7),
    )
    .await?;
    db.insert_pbs_computations(
        &mut tx,
        &[handle.to_vec()],
        Some(tx_hash.to_vec()),
        7,
        block_hash.as_slice(),
    )
    .await?;
    tx.commit().await?;

    // Every write must be visible in both the legacy and the branch table.
    for (legacy, branch, key_col) in [
        ("computations", "computations_branch", "output_handle"),
        ("allowed_handles", "allowed_handles_branch", "handle"),
        ("pbs_computations", "pbs_computations_branch", "handle"),
    ] {
        let legacy_count: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM {legacy} WHERE {key_col} = $1"
        ))
        .bind(handle.to_vec())
        .fetch_one(&pool)
        .await?;
        let branch_count: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*) FROM {branch} WHERE {key_col} = $1 AND producer_block_hash = $2"
        ))
        .bind(handle.to_vec())
        .bind(block_hash.to_vec())
        .fetch_one(&pool)
        .await?;
        assert_eq!(legacy_count, 1, "missing legacy row in {legacy}");
        assert_eq!(branch_count, 1, "missing branch row in {branch}");
    }

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_acl_branch_rows_keep_acl_block_context(
) -> Result<(), Box<dyn std::error::Error>> {
    use alloy::primitives::Log as EventLog;
    use host_listener::contracts::AclContract;
    use host_listener::contracts::AclContract::AclContractEvents;
    use host_listener::contracts::TfheContract;
    use host_listener::contracts::TfheContract::TfheContractEvents;
    use host_listener::database::tfhe_event_propagate::{ClearConst, LogTfhe};
    use sqlx::types::time::PrimitiveDateTime;

    let test_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let chain_id = ChainId::try_from(42_u64)?;
    let db_url = test_instance.db_url.clone();
    let db = Database::new(&db_url, chain_id, 128).await?;
    let pool = db.pool.read().await.clone();

    let handle = FixedBytes::<32>::from([0x46; 32]);
    let producer_tx = FixedBytes::<32>::from([0x47; 32]);
    let producer_hash = FixedBytes::<32>::from([0x48; 32]);
    let orphan_acl_hash = FixedBytes::<32>::from([0x49; 32]);
    let canonical_acl_hash = FixedBytes::<32>::from([0x4A; 32]);
    let orphan_only_handle = FixedBytes::<32>::from([0x4B; 32]);
    let orphan_only_producer_tx = FixedBytes::<32>::from([0x4C; 32]);
    let unrepaired_handle = FixedBytes::<32>::from([0x52; 32]);
    let branchless_handle = FixedBytes::<32>::from([0x53; 32]);
    let producer_block_number = 19_u64;
    let acl_block_number = 20_u64;
    let caller: Address =
        "0x2222222222222222222222222222222222222222".parse()?;

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (
            chain_id, block_hash, parent_hash, block_number, block_status
         )
         VALUES ($1, $2, NULL, $3, 'pending')",
    )
    .bind(chain_id.as_i64())
    .bind(producer_hash.to_vec())
    .bind(producer_block_number as i64)
    .execute(&pool)
    .await?;

    let mut tx = db.new_transaction().await?;
    db.insert_tfhe_event(
        &mut tx,
        &LogTfhe {
            event: EventLog {
                address: Address::ZERO,
                data: TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: ClearConst::from_be_slice(&[9u8]),
                        toType: 4u8,
                        result: handle,
                    },
                ),
            },
            transaction_hash: Some(producer_tx),
            is_allowed: true,
            block_number: producer_block_number,
            block_hash: producer_hash,
            block_timestamp: PrimitiveDateTime::MAX,
            dependence_chain: producer_tx,
            tx_depth_size: 0,
            log_index: None,
        },
    )
    .await?;

    db.insert_tfhe_event(
        &mut tx,
        &LogTfhe {
            event: EventLog {
                address: Address::ZERO,
                data: TfheContractEvents::TrivialEncrypt(
                    TfheContract::TrivialEncrypt {
                        caller,
                        pt: ClearConst::from_be_slice(&[10u8]),
                        toType: 4u8,
                        result: orphan_only_handle,
                    },
                ),
            },
            transaction_hash: Some(orphan_only_producer_tx),
            is_allowed: true,
            block_number: producer_block_number,
            block_hash: producer_hash,
            block_timestamp: PrimitiveDateTime::MAX,
            dependence_chain: orphan_only_producer_tx,
            tx_depth_size: 0,
            log_index: None,
        },
    )
    .await?;

    let orphan_acl_event = EventLog {
        address: Address::ZERO,
        data: AclContractEvents::AllowedForDecryption(
            AclContract::AllowedForDecryption {
                caller,
                handlesList: vec![handle, orphan_only_handle],
            },
        ),
    };
    let canonical_acl_event = EventLog {
        address: Address::ZERO,
        data: AclContractEvents::AllowedForDecryption(
            AclContract::AllowedForDecryption {
                caller,
                handlesList: vec![handle],
            },
        ),
    };
    db.handle_acl_event(
        &mut tx,
        &orphan_acl_event,
        &None,
        &BlockSummary {
            number: acl_block_number,
            hash: orphan_acl_hash,
            parent_hash: producer_hash,
            timestamp: 0,
        },
    )
    .await?;
    sqlx::query(
        "INSERT INTO ciphertext_digest (
            host_chain_id,
            key_id_gw,
            handle,
            ciphertext,
            ciphertext128
         )
         VALUES
            ($1, $2, $3, $4, $5),
            ($1, $2, $6, $7, $8)",
    )
    .bind(chain_id.as_i64())
    .bind(vec![0x4D_u8; 32])
    .bind(handle.to_vec())
    .bind(vec![0x4E_u8; 32])
    .bind(vec![0x4F_u8; 32])
    .bind(orphan_only_handle.to_vec())
    .bind(vec![0x50_u8; 32])
    .bind(vec![0x51_u8; 32])
    .execute(&mut *tx)
    .await?;
    db.handle_acl_event(
        &mut tx,
        &canonical_acl_event,
        &None,
        &BlockSummary {
            number: acl_block_number,
            hash: canonical_acl_hash,
            parent_hash: producer_hash,
            timestamp: 0,
        },
    )
    .await?;
    tx.commit().await?;

    for table in ["allowed_handles_branch", "pbs_computations_branch"] {
        let count: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*)
             FROM {table}
             WHERE host_chain_id = $1
               AND handle = $2
               AND producer_block_hash = $3
               AND block_number = $4
               AND block_hash = ANY($5::bytea[])"
        ))
        .bind(chain_id.as_i64())
        .bind(handle.to_vec())
        .bind(producer_hash.to_vec())
        .bind(acl_block_number as i64)
        .bind(vec![orphan_acl_hash.to_vec(), canonical_acl_hash.to_vec()])
        .fetch_one(&pool)
        .await?;
        assert_eq!(count, 2, "{table}");
    }

    let digest_context_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = $3
           AND block_number = $4
           AND block_hash = ANY($5::bytea[])",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .bind(producer_hash.to_vec())
    .bind(acl_block_number as i64)
    .bind(vec![orphan_acl_hash.to_vec(), canonical_acl_hash.to_vec()])
    .fetch_one(&pool)
    .await?;
    let branchless_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = ''::bytea
           AND block_hash = ''::bytea",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(digest_context_count, 2);
    assert_eq!(branchless_digest_count, 0);

    let orphan_only_digest_context_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = $3
           AND block_number = $4
           AND block_hash = $5",
    )
    .bind(chain_id.as_i64())
    .bind(orphan_only_handle.to_vec())
    .bind(producer_hash.to_vec())
    .bind(acl_block_number as i64)
    .bind(orphan_acl_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(orphan_only_digest_context_count, 1);

    sqlx::query(
        "INSERT INTO pbs_computations_branch (
            handle,
            transaction_id,
            host_chain_id,
            block_number,
            block_hash,
            producer_block_hash
         )
         VALUES
            ($1, NULL, $2, $3, ''::bytea, $4),
            ($5, NULL, $2, $3, ''::bytea, ''::bytea),
            ($5, NULL, $2, $3, $6, $4)",
    )
    .bind(unrepaired_handle.to_vec())
    .bind(chain_id.as_i64())
    .bind(acl_block_number as i64)
    .bind(producer_hash.to_vec())
    .bind(branchless_handle.to_vec())
    .bind(canonical_acl_hash.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query(
        "INSERT INTO pbs_computations(handle, host_chain_id, block_number)
         VALUES
            ($1, $2, $3),
            ($4, $2, $3)",
    )
    .bind(unrepaired_handle.to_vec())
    .bind(chain_id.as_i64())
    .bind(acl_block_number as i64)
    .bind(branchless_handle.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query(
        "INSERT INTO ciphertext_digest (
            host_chain_id,
            key_id_gw,
            handle,
            ciphertext,
            ciphertext128
         )
         VALUES
            ($1, $2, $3, $4, $5),
            ($1, $2, $6, $7, $8)",
    )
    .bind(chain_id.as_i64())
    .bind(vec![0x54_u8; 32])
    .bind(unrepaired_handle.to_vec())
    .bind(vec![0x55_u8; 32])
    .bind(vec![0x56_u8; 32])
    .bind(branchless_handle.to_vec())
    .bind(vec![0x57_u8; 32])
    .bind(vec![0x58_u8; 32])
    .execute(&pool)
    .await?;

    let branchless_digest_before_cleanup: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = ''::bytea
           AND block_hash = ''::bytea",
    )
    .bind(chain_id.as_i64())
    .bind(branchless_handle.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(branchless_digest_before_cleanup, 1);

    let mut tx = db.new_transaction().await?;
    db.cleanup_orphaned_branch_state(&mut tx, &[orphan_acl_hash.to_vec()])
        .await?;
    tx.commit().await?;

    for table in ["allowed_handles_branch", "pbs_computations_branch"] {
        let orphan_count: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*)
             FROM {table}
             WHERE host_chain_id = $1
               AND handle = $2
               AND block_hash = $3"
        ))
        .bind(chain_id.as_i64())
        .bind(handle.to_vec())
        .bind(orphan_acl_hash.to_vec())
        .fetch_one(&pool)
        .await?;
        let canonical_count: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*)
             FROM {table}
             WHERE host_chain_id = $1
               AND handle = $2
               AND block_hash = $3
               AND producer_block_hash = $4"
        ))
        .bind(chain_id.as_i64())
        .bind(handle.to_vec())
        .bind(canonical_acl_hash.to_vec())
        .bind(producer_hash.to_vec())
        .fetch_one(&pool)
        .await?;
        assert_eq!(orphan_count, 0, "{table}");
        assert_eq!(canonical_count, 1, "{table}");

        let orphan_only_count: i64 = sqlx::query_scalar(&format!(
            "SELECT COUNT(*)
             FROM {table}
             WHERE host_chain_id = $1
               AND handle = $2"
        ))
        .bind(chain_id.as_i64())
        .bind(orphan_only_handle.to_vec())
        .fetch_one(&pool)
        .await?;
        assert_eq!(orphan_only_count, 0, "{table}");
    }

    let orphan_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND block_hash = $3",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .bind(orphan_acl_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    let canonical_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND block_hash = $3
           AND producer_block_hash = $4",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .bind(canonical_acl_hash.to_vec())
    .bind(producer_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(orphan_digest_count, 0);
    assert_eq!(canonical_digest_count, 1);

    let orphan_only_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2",
    )
    .bind(chain_id.as_i64())
    .bind(orphan_only_handle.to_vec())
    .fetch_one(&pool)
    .await?;
    let legacy_orphan_only_pbs_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM pbs_computations
         WHERE host_chain_id = $1
           AND handle = $2",
    )
    .bind(chain_id.as_i64())
    .bind(orphan_only_handle.to_vec())
    .fetch_one(&pool)
    .await?;
    let legacy_orphan_only_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest
         WHERE host_chain_id = $1
           AND handle = $2",
    )
    .bind(chain_id.as_i64())
    .bind(orphan_only_handle.to_vec())
    .fetch_one(&pool)
    .await?;
    let legacy_orphan_only_allowed_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM allowed_handles
         WHERE host_chain_id = $1
           AND handle = $2",
    )
    .bind(chain_id.as_i64())
    .bind(orphan_only_handle.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(orphan_only_digest_count, 0);
    assert_eq!(legacy_orphan_only_pbs_count, 0);
    assert_eq!(legacy_orphan_only_digest_count, 0);
    assert_eq!(legacy_orphan_only_allowed_count, 0);

    let legacy_retained_pbs_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM pbs_computations
         WHERE host_chain_id = $1
           AND handle = $2",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .fetch_one(&pool)
    .await?;
    let legacy_retained_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest
         WHERE host_chain_id = $1
           AND handle = $2",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(legacy_retained_pbs_count, 1);
    assert_eq!(legacy_retained_digest_count, 1);

    let unrepaired_branch_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM pbs_computations_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = $3
           AND block_hash = ''::bytea",
    )
    .bind(chain_id.as_i64())
    .bind(unrepaired_handle.to_vec())
    .bind(producer_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    let unrepaired_digest_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = $3
           AND block_hash = ''::bytea",
    )
    .bind(chain_id.as_i64())
    .bind(unrepaired_handle.to_vec())
    .bind(producer_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    let branchless_digest_after_cleanup: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM ciphertext_digest_branch
         WHERE host_chain_id = $1
           AND handle = $2
           AND producer_block_hash = ''::bytea
           AND block_hash = ''::bytea",
    )
    .bind(chain_id.as_i64())
    .bind(branchless_handle.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(unrepaired_branch_count, 1);
    assert_eq!(unrepaired_digest_count, 1);
    assert_eq!(branchless_digest_after_cleanup, 1);

    let producer_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM computations_branch
         WHERE host_chain_id = $1
           AND output_handle = $2
           AND producer_block_hash = $3",
    )
    .bind(chain_id.as_i64())
    .bind(handle.to_vec())
    .bind(producer_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(producer_count, 1);

    let orphan_only_producer_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM computations_branch
         WHERE host_chain_id = $1
           AND output_handle = $2
           AND producer_block_hash = $3",
    )
    .bind(chain_id.as_i64())
    .bind(orphan_only_handle.to_vec())
    .bind(producer_hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(orphan_only_producer_count, 1);

    Ok(())
}
