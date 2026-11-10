//! Artifact stall regression: Anvil logs -> production ingestion -> PostgreSQL
//! -> the real CPU worker -> decrypted ciphertexts.
//!
//! Drop only the producer's ACL log on first delivery. Its computation is then
//! considered completed without a persisted ciphertext; downstream transactions
//! cannot run. Replay must repair that existing row and wake the dependency graph.
//! ACL delivery cases do not write computation or scheduler flags. The legacy
//! unreachable-chain cases explicitly inject only the old scheduler failure.

#![cfg(not(feature = "gpu"))]

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::{Address, B256, U256},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::{Filter, Log, TransactionReceipt},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolEvent,
};
use fhevm_engine_common::{
    chain_id::ChainId, db_keys::DbKeyCache, tfhe_ops::current_ciphertext_version,
    types::SupportedFheCiphertexts,
};
use host_listener::{
    cmd::block_history::BlockSummary,
    database::{
        ingest::{ingest_block_logs, BlockLogs, IngestOptions},
        tfhe_event_propagate::Database,
    },
};
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{error::Error, process::Stdio, time::Duration};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::{
    process::{Child, Command},
    time::timeout,
};

sol!(
    #[sol(rpc)]
    ACLTest,
    "tests/fixtures/ArtifactStallAcl.json"
);
sol!(
    #[sol(rpc)]
    ArtifactStallTest,
    "tests/fixtures/ArtifactStallTest.json"
);

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;
const CHAIN: u64 = 12345;
// CPU key loading and FHE execution are substantially slower in debug builds.
const WAIT: Duration = Duration::from_secs(300);
const OPTIONS: IngestOptions = IngestOptions {
    dependence_by_connexity: false,
    dependence_cross_block: true,
    dependent_ops_max_per_chain: 0,
    is_protocol_config_listener: false,
    disable_synthetic_ops: false,
};

#[derive(Clone, Copy, Debug)]
enum Replay {
    WarmFullBlock,
    ColdFullBlock,
    ColdAclOnly,
    FailedThenRetried,
}

fn start_worker(url: &str) -> Result<Child> {
    Ok(Command::new(env!("CARGO_BIN_EXE_tfhe_worker"))
        .args([
            "--run-bg-worker",
            "--database-url",
            url,
            "--worker-polling-interval-ms",
            "50",
            "--coprocessor-fhe-threads",
            "16",
            "--pg-pool-max-connections",
            "8",
            "--metrics-addr",
            "127.0.0.1:0",
            "--health-check-port",
            "0",
            "--dcid-ignore-dependency-count-threshold",
            "2",
            "--log-level",
            "warn",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?)
}

async fn stop_workers(workers: &mut [Child]) -> Result {
    for worker in workers {
        assert!(
            worker.try_wait()?.is_none(),
            "worker exited during recovery"
        );
        worker.kill().await?;
        worker.wait().await?;
    }
    Ok(())
}

async fn block_logs<P: Provider>(
    provider: &P,
    receipt: &TransactionReceipt,
    executor: Address,
    acl: Address,
) -> Result<BlockLogs<Log>> {
    assert!(receipt.status(), "fixture transaction reverted");
    let hash = receipt.block_hash.ok_or("receipt has no block hash")?;
    let block = provider
        .get_block_by_hash(hash)
        .await?
        .ok_or("mined block not found")?;
    let logs = provider
        .get_logs(
            &Filter::new()
                .at_block_hash(hash)
                .address(vec![executor, acl]),
        )
        .await?;
    assert_eq!(logs.len(), 2, "fixture must emit one FHE and one ACL log");
    assert_eq!(logs.iter().filter(|log| log.address() == acl).count(), 1);
    Ok(BlockLogs {
        summary: BlockSummary::from(block),
        logs,
        catchup: false,
        finalized: false,
    })
}

async fn ingest(
    db: &mut Database,
    logs: &BlockLogs<Log>,
    executor: Address,
    acl: Address,
) -> Result {
    ingest_block_logs(
        ChainId::try_from(CHAIN)?,
        db,
        logs,
        &Some(acl),
        &Some(executor),
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?
    .require_complete()?;
    Ok(())
}

async fn flags(pool: &PgPool, handle: B256) -> Result<(bool, bool, bool)> {
    Ok(sqlx::query_as(
        "SELECT is_allowed, is_completed, is_error FROM computations WHERE output_handle = $1",
    )
    .bind(handle.as_slice())
    .fetch_one(pool)
    .await?)
}

async fn block_state(pool: &PgPool, hash: B256) -> Result<(i32, i32, String)> {
    Ok(sqlx::query_as(
        "SELECT fhe_event_count, allow_event_count, block_status FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN as i64)
    .bind(hash.as_slice())
    .fetch_one(pool)
    .await?)
}

async fn ciphertext_count(pool: &PgPool, handles: &[Vec<u8>]) -> Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT count(*) FROM ciphertexts WHERE handle = ANY($1) AND ciphertext_version = $2",
    )
    .bind(handles)
    .bind(current_ciphertext_version())
    .fetch_one(pool)
    .await?)
}

async fn wait_for_ciphertexts(pool: &PgPool, handles: &[Vec<u8>]) -> Result {
    let result = timeout(WAIT, async {
        loop {
            let errors: Vec<(Vec<u8>, Option<String>)> = sqlx::query_as(
                "SELECT output_handle, error_message FROM computations
                 WHERE output_handle = ANY($1) AND is_error",
            )
            .bind(handles)
            .fetch_all(pool)
            .await?;
            assert!(
                errors.is_empty(),
                "worker marked fixture computations failed: {errors:?}"
            );
            if ciphertext_count(pool, handles).await? == handles.len() as i64 {
                return Ok::<_, Box<dyn Error>>(());
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await;
    if result.is_err() {
        let rows: Vec<(Vec<u8>, bool, bool, bool)> = sqlx::query_as(
            "SELECT output_handle, is_allowed, is_completed, is_error
             FROM computations WHERE output_handle = ANY($1)",
        )
        .bind(handles)
        .fetch_all(pool)
        .await?;
        panic!("ciphertexts still missing after {WAIT:?}; computation state: {rows:?}");
    }
    result??;
    Ok(())
}

async fn assert_plaintexts(pool: &PgPool, expected: &[(B256, u64)]) -> Result {
    let key = DbKeyCache::new(1)?.fetch_latest_from_pool(pool).await?;
    let mut ciphertexts = Vec::new();
    for (handle, value) in expected {
        let (bytes, ty): (Vec<u8>, i16) = sqlx::query_as(
            "SELECT ciphertext, ciphertext_type FROM ciphertexts
             WHERE handle = $1 AND ciphertext_version = $2",
        )
        .bind(handle.as_slice())
        .bind(current_ciphertext_version())
        .fetch_one(pool)
        .await?;
        ciphertexts.push((bytes, ty, *value));
    }
    tokio::task::spawn_blocking(move || {
        tfhe::set_server_key(key.sks);
        let client_key = key.cks.expect("test fixture has a client key");
        for (bytes, ty, expected) in ciphertexts {
            assert_eq!(ty, 5, "expected euint64 ciphertext");
            let ct = SupportedFheCiphertexts::decompress_no_memcheck(ty, &bytes).unwrap();
            assert_eq!(ct.decrypt(&client_key), expected.to_string());
        }
    })
    .await?;
    Ok(())
}

async fn reproduce_and_recover(replay: Replay) -> Result {
    let instance = setup_test_db(ImportMode::WithKeysNoSns).await?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await?;
    let mut db = Database::new(&instance.db_url, ChainId::try_from(CHAIN)?, 128).await?;
    let anvil = Anvil::new().chain_id(CHAIN).spawn();
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_http(anvil.endpoint_url());
    let acl = ACLTest::deploy(provider.clone()).await?;
    let executor = ArtifactStallTest::deploy(provider.clone(), *acl.address()).await?;
    let addresses = (*executor.address(), *acl.address());

    let root = executor.seed(7, 1).call().await?;
    let receipt = executor.seed(7, 1).send().await?.get_receipt().await?;
    let full_root = block_logs(&provider, &receipt, addresses.0, addresses.1).await?;
    let root_tx = receipt.transaction_hash;
    let mut partial_root = BlockLogs {
        summary: full_root.summary,
        logs: full_root.logs.clone(),
        catchup: false,
        finalized: false,
    };
    // Fault injection at the RPC delivery boundary: no fake DB/scheduler state.
    partial_root.logs.retain(|log| log.address() != addresses.1);
    assert_eq!(partial_root.logs.len(), 1);
    ingest(&mut db, &partial_root, addresses.0, addresses.1).await?;
    assert_eq!(flags(&pool, root).await?, (false, true, false));

    // A separate process runs the production startup, queue, scheduler and CPU
    // execution paths. kill_on_drop cleans up only this test's worker on failure.
    let mut worker = Command::new(env!("CARGO_BIN_EXE_tfhe_worker"))
        .args([
            "--run-bg-worker",
            "--database-url",
            instance.db_url(),
            "--worker-polling-interval-ms",
            "50",
            "--coprocessor-fhe-threads",
            "32",
            "--pg-pool-max-connections",
            "8",
            "--metrics-addr",
            "127.0.0.1:0",
            "--health-check-port",
            "0",
            "--dcid-cleanup-interval-sec",
            "1",
            "--dcid-ignore-dependency-count-threshold",
            "2",
            "--log-level",
            "warn",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;

    timeout(WAIT, async {
        loop {
            assert!(
                worker.try_wait()?.is_none(),
                "worker exited before processing producer"
            );
            let processed: bool = sqlx::query_scalar(
                "SELECT status = 'processed' FROM dependence_chain WHERE dependence_chain_id = $1",
            )
            .bind(root_tx.as_slice())
            .fetch_one(&pool)
            .await?;
            if processed {
                return Ok::<_, Box<dyn Error>>(());
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await??;
    assert_eq!(ciphertext_count(&pool, &[root.to_vec()]).await?, 0);

    // Fan-out and a second dependent level, each in a later transaction/block.
    let mut descendants = Vec::new();
    for (parent, scalar, nonce) in [(root, 13, 2), (root, 19, 3)] {
        let handle = executor.add(parent, scalar, nonce).call().await?;
        let receipt = executor
            .add(parent, scalar, nonce)
            .send()
            .await?
            .get_receipt()
            .await?;
        let logs = block_logs(&provider, &receipt, addresses.0, addresses.1).await?;
        ingest(&mut db, &logs, addresses.0, addresses.1).await?;
        descendants.push(handle);
    }
    let grandchild = executor.add(descendants[0], 5, 4).call().await?;
    let receipt = executor
        .add(descendants[0], 5, 4)
        .send()
        .await?
        .get_receipt()
        .await?;
    let logs = block_logs(&provider, &receipt, addresses.0, addresses.1).await?;
    ingest(&mut db, &logs, addresses.0, addresses.1).await?;
    descendants.push(grandchild);

    // This completion is a positive liveness witness, not an arbitrary sleep:
    // the worker is running and can produce ciphertexts while the graph is stuck.
    let canary = executor.seed(99, 5).call().await?;
    let receipt = executor.seed(99, 5).send().await?.get_receipt().await?;
    let logs = block_logs(&provider, &receipt, addresses.0, addresses.1).await?;
    ingest(&mut db, &logs, addresses.0, addresses.1).await?;
    wait_for_ciphertexts(&pool, &[canary.to_vec()]).await?;
    for &handle in &descendants {
        assert_eq!(flags(&pool, handle).await?, (true, false, false));
    }
    let graph: Vec<_> = std::iter::once(root)
        .chain(descendants.iter().copied())
        .map(|h| h.to_vec())
        .collect();
    assert_eq!(ciphertext_count(&pool, &graph).await?, 0);

    if matches!(replay, Replay::ColdFullBlock | Replay::ColdAclOnly) {
        db = Database::new(&instance.db_url, ChainId::try_from(CHAIN)?, 128).await?;
    }
    let mut repaired_root = full_root;
    repaired_root.catchup = true;
    repaired_root.finalized = true;
    if matches!(replay, Replay::ColdAclOnly) {
        repaired_root
            .logs
            .retain(|log| log.address() == addresses.1);
        assert_eq!(repaired_root.logs.len(), 1);
    }
    if matches!(replay, Replay::FailedThenRetried) {
        // Fail the ACL write inside the real ingestion transaction. A retry on
        // the same listener must work; failed ingestion must not poison its cache.
        sqlx::raw_sql(
            "CREATE FUNCTION artifact_fail_acl() RETURNS trigger LANGUAGE plpgsql AS $$
               BEGIN RAISE EXCEPTION 'artifact test ACL write failure'; END $$;
             CREATE TRIGGER artifact_fail_acl BEFORE INSERT ON allowed_handles
               FOR EACH ROW EXECUTE FUNCTION artifact_fail_acl();",
        )
        .execute(&pool)
        .await?;
        let error = ingest(&mut db, &repaired_root, addresses.0, addresses.1)
            .await
            .expect_err("ACL write failure must abort ingestion");
        assert!(
            error
                .to_string()
                .contains("artifact test ACL write failure"),
            "{error}"
        );
        assert_eq!(flags(&pool, root).await?, (false, true, false));
        assert_eq!(ciphertext_count(&pool, &graph).await?, 0);
        assert_eq!(
            block_state(&pool, repaired_root.summary.hash).await?,
            (1, 0, "pending".into()),
            "failed replay leaked event counts or finalization"
        );
        let allowances: i64 =
            sqlx::query_scalar("SELECT count(*) FROM allowed_handles WHERE handle = $1")
                .bind(root.as_slice())
                .fetch_one(&pool)
                .await?;
        assert_eq!(allowances, 0, "failed replay leaked an ACL write");
        sqlx::raw_sql(
            "DROP TRIGGER artifact_fail_acl ON allowed_handles; DROP FUNCTION artifact_fail_acl();",
        )
        .execute(&pool)
        .await?;
    }

    ingest(&mut db, &repaired_root, addresses.0, addresses.1).await?;
    let acl_persisted: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM allowed_handles WHERE handle = $1)")
            .bind(root.as_slice())
            .fetch_one(&pool)
            .await?;
    assert!(
        acl_persisted,
        "replay did not persist the missing ACL event"
    );
    assert!(
        flags(&pool, root).await?.0,
        "Artifact regression: replay stored the ACL but did not promote the existing producer"
    );
    assert_eq!(
        block_state(&pool, repaired_root.summary.hash).await?,
        (1, 1, "finalized".into()),
        "replay must repair event counts and finalize the recovered block"
    );
    wait_for_ciphertexts(&pool, &graph).await?;
    for handle in std::iter::once(root).chain(descendants.iter().copied()) {
        assert_eq!(flags(&pool, handle).await?, (true, true, false));
    }

    // Repeating recovery must preserve the canonical ciphertext bytes and must
    // not create duplicate computations or queue already completed work again.
    let before: Vec<(Vec<u8>, Vec<u8>)> = sqlx::query_as(
        "SELECT handle, ciphertext FROM ciphertexts WHERE handle = ANY($1) ORDER BY handle",
    )
    .bind(&graph)
    .fetch_all(&pool)
    .await?;
    ingest(&mut db, &repaired_root, addresses.0, addresses.1).await?;
    let after: Vec<(Vec<u8>, Vec<u8>)> = sqlx::query_as(
        "SELECT handle, ciphertext FROM ciphertexts WHERE handle = ANY($1) ORDER BY handle",
    )
    .bind(&graph)
    .fetch_all(&pool)
    .await?;
    assert_eq!(before, after, "duplicate replay changed ciphertexts");
    for handle in std::iter::once(root).chain(descendants.iter().copied()) {
        assert_eq!(flags(&pool, handle).await?, (true, true, false));
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM computations")
        .fetch_one(&pool)
        .await?;
    assert_eq!(count, 5);
    assert!(
        worker.try_wait()?.is_none(),
        "worker exited during recovery"
    );
    worker.kill().await?;
    worker.wait().await?;
    // Do not hold a second server key while the worker is still alive.
    assert_plaintexts(
        &pool,
        &[
            (root, 7),
            (descendants[0], 20),
            (descendants[1], 26),
            (grandchild, 25),
            (canary, 99),
        ],
    )
    .await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn artifact_full_block_replay_unblocks_descendants() -> Result {
    reproduce_and_recover(Replay::WarmFullBlock).await
}

#[tokio::test]
#[serial(db)]
async fn artifact_full_block_replay_after_listener_restart_unblocks_descendants() -> Result {
    reproduce_and_recover(Replay::ColdFullBlock).await
}

#[tokio::test]
#[serial(db)]
async fn artifact_acl_only_replay_after_listener_restart_unblocks_descendants() -> Result {
    reproduce_and_recover(Replay::ColdAclOnly).await
}

#[tokio::test]
#[serial(db)]
async fn artifact_failed_acl_replay_then_retry_unblocks_descendants() -> Result {
    reproduce_and_recover(Replay::FailedThenRetried).await
}

async fn recover_unreachable_with_parallel_workers(missing: bool) -> Result {
    let instance = setup_test_db(ImportMode::WithKeysNoSns).await?;
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(instance.db_url())
        .await?;
    let mut warm = Database::new(&instance.db_url, ChainId::try_from(CHAIN)?, 128).await?;
    let anvil = Anvil::new().chain_id(CHAIN).spawn();
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_http(anvil.endpoint_url());
    let acl = ACLTest::deploy(provider.clone()).await?;
    let executor = ArtifactStallTest::deploy(provider.clone(), *acl.address()).await?;
    let (exe, acl) = (*executor.address(), *acl.address());
    let root = executor.seed(7, 1).call().await?;
    let receipt = executor.seed(7, 1).send().await?.get_receipt().await?;
    let stored_chain = receipt.transaction_hash;
    ingest(
        &mut warm,
        &block_logs(&provider, &receipt, exe, acl).await?,
        exe,
        acl,
    )
    .await?;
    let producer = executor.add(root, 13, 2).call().await?;
    let receipt = executor
        .add(root, 13, 2)
        .send()
        .await?
        .get_receipt()
        .await?;
    let inferred_chain = receipt.transaction_hash;
    let mut replay = block_logs(&provider, &receipt, exe, acl).await?;
    ingest(&mut warm, &replay, exe, acl).await?;
    let stored: Vec<u8> =
        sqlx::query_scalar("SELECT dependence_chain_id FROM computations WHERE output_handle=$1")
            .bind(producer.as_slice())
            .fetch_one(&pool)
            .await?;
    assert_eq!(stored, stored_chain.as_slice());
    assert_ne!(
        stored_chain, inferred_chain,
        "cold replay must infer a different chain"
    );
    let mut expected = vec![(root, 7), (producer, 20)];
    for (parent, scalar, nonce, value) in [(producer, 5, 3, 25), (root, 19, 4, 26)] {
        let handle = executor.add(parent, scalar, nonce).call().await?;
        let receipt = executor
            .add(parent, scalar, nonce)
            .send()
            .await?
            .get_receipt()
            .await?;
        let mut cold = Database::new(&instance.db_url, ChainId::try_from(CHAIN)?, 128).await?;
        ingest(
            &mut cold,
            &block_logs(&provider, &receipt, exe, acl).await?,
            exe,
            acl,
        )
        .await?;
        expected.push((handle, value));
    }
    // Upgrade fixture: inject precisely the pre-fix scheduler state, leaving
    // every actual event, computation, permission and ciphertext untouched.
    let query = if missing {
        "DELETE FROM dependence_chain WHERE dependence_chain_id=$1"
    } else {
        "UPDATE dependence_chain SET status='processed' WHERE dependence_chain_id=$1"
    };
    assert_eq!(
        sqlx::query(query)
            .bind(stored_chain.as_slice())
            .execute(&pool)
            .await?
            .rows_affected(),
        1
    );
    let mut workers = [
        start_worker(instance.db_url())?,
        start_worker(instance.db_url())?,
    ];
    let canary = executor.seed(99, 5).call().await?;
    let receipt = executor.seed(99, 5).send().await?.get_receipt().await?;
    ingest(
        &mut warm,
        &block_logs(&provider, &receipt, exe, acl).await?,
        exe,
        acl,
    )
    .await?;
    wait_for_ciphertexts(&pool, &[canary.to_vec()]).await?;
    let graph: Vec<_> = expected.iter().map(|(h, _)| h.to_vec()).collect();
    assert_eq!(
        ciphertext_count(&pool, &graph).await?,
        0,
        "legacy chain must be unreachable before repair"
    );
    for (handle, _) in &expected {
        assert_eq!(flags(&pool, *handle).await?, (true, false, false));
    }
    replay.catchup = true;
    let mut cold = Database::new(&instance.db_url, ChainId::try_from(CHAIN)?, 128).await?;
    // Cold replay alone must find/recreate the stored chain; its inferred chain
    // is distinct, so upserting only the inferred chain cannot pass this test.
    ingest(&mut cold, &replay, exe, acl).await?;
    let reachable: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM dependence_chain d WHERE dependence_chain_id=$1
         AND (status != 'processed' OR NOT EXISTS (
             SELECT 1 FROM computations c WHERE c.dependence_chain_id=d.dependence_chain_id
             AND c.is_allowed AND NOT c.is_completed)))",
    )
    .bind(stored_chain.as_slice())
    .fetch_one(&pool)
    .await?;
    assert!(reachable, "cold replay did not re-arm the stored chain");
    // Concurrent duplicate deliveries while two workers run must preserve leases
    // and immutable execution metadata despite divergent cache grouping.
    let (a, b) = tokio::join!(
        ingest(&mut warm, &replay, exe, acl),
        ingest(&mut cold, &replay, exe, acl)
    );
    a?;
    b?;
    wait_for_ciphertexts(&pool, &graph).await?;
    for (handle, _) in &expected {
        assert_eq!(flags(&pool, *handle).await?, (true, true, false));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM computations")
        .fetch_one(&pool)
        .await?;
    assert_eq!(count, 5);
    stop_workers(&mut workers).await?;
    expected.push((canary, 99));
    assert_plaintexts(&pool, &expected).await
}

#[tokio::test]
#[serial(db)]
async fn artifact_parallel_workers_repair_processed_stored_chain() -> Result {
    recover_unreachable_with_parallel_workers(false).await
}

#[tokio::test]
#[serial(db)]
async fn artifact_parallel_workers_recreate_missing_stored_chain() -> Result {
    recover_unreachable_with_parallel_workers(true).await
}

fn emitted_seed(logs: &BlockLogs<Log>) -> Result<B256> {
    for log in &logs.logs {
        if let Ok(event) =
            host_listener::contracts::TfheContract::TrivialEncrypt::decode_log(&log.inner)
        {
            return Ok(event.data.result);
        }
    }
    Err("missing TrivialEncrypt event".into())
}

#[tokio::test]
#[serial(db)]
async fn artifact_orphan_completes_before_same_transaction_on_canonical_fork() -> Result {
    let instance = setup_test_db(ImportMode::WithKeysNoSns).await?;
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(instance.db_url())
        .await?;
    let mut db = Database::new(&instance.db_url, ChainId::try_from(CHAIN)?, 128).await?;
    let anvil = Anvil::new().chain_id(CHAIN).spawn();
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_http(anvil.endpoint_url());
    let acl = ACLTest::deploy(provider.clone()).await?;
    let executor = ArtifactStallTest::deploy(provider.clone(), *acl.address()).await?;
    let (exe, acl) = (*executor.address(), *acl.address());
    let snapshot: U256 = provider.raw_request("evm_snapshot".into(), ()).await?;
    let receipt = executor.seedBlock(7).send().await?.get_receipt().await?;
    let raw = provider
        .get_raw_transaction_by_hash(receipt.transaction_hash)
        .await?
        .ok_or("missing raw transaction")?;
    let orphan = block_logs(&provider, &receipt, exe, acl).await?;
    let orphan_handle = emitted_seed(&orphan)?;
    ingest(&mut db, &orphan, exe, acl).await?;
    let mut workers = [start_worker(instance.db_url())?];
    wait_for_ciphertexts(&pool, &[orphan_handle.to_vec()]).await?;
    timeout(WAIT, async {
        loop {
            let processed: bool = sqlx::query_scalar(
                "SELECT status='processed' FROM dependence_chain WHERE dependence_chain_id=$1",
            )
            .bind(receipt.transaction_hash.as_slice())
            .fetch_one(&pool)
            .await?;
            if processed {
                return Ok::<_, Box<dyn Error>>(());
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await??;
    let reverted: bool = provider
        .raw_request("evm_revert".into(), (snapshot,))
        .await?;
    assert!(reverted);
    provider
        .anvil_set_next_block_timestamp(orphan.summary.timestamp + 2)
        .await?;
    // Alloy can still confirm this transaction hash from its orphan-block cache.
    // Mine the replacement explicitly before reading its receipt: get_receipt()
    // can otherwise report NullResp while Anvil has not mined the new fork yet.
    provider.anvil_set_auto_mine(false).await?;
    let canonical_tx = provider.send_raw_transaction(&raw).await?;
    assert!(
        provider
            .get_transaction_receipt(*canonical_tx.tx_hash())
            .await?
            .is_none(),
        "replayed transaction must remain pending until explicitly mined"
    );
    provider.anvil_mine(Some(1), None).await?;
    let canonical_receipt = provider
        .get_transaction_receipt(*canonical_tx.tx_hash())
        .await?
        .ok_or("canonical replay was mined without a receipt")?;
    provider.anvil_set_auto_mine(true).await?;
    assert_eq!(canonical_receipt.transaction_hash, receipt.transaction_hash);
    let mut canonical = block_logs(&provider, &canonical_receipt, exe, acl).await?;
    let canonical_handle = emitted_seed(&canonical)?;
    assert_eq!(canonical.summary.number, orphan.summary.number);
    assert_ne!(canonical.summary.hash, orphan.summary.hash);
    assert_ne!(canonical_handle, orphan_handle);
    // Deliver the consumer before the canonical producer, after the orphan's
    // work really completed. The independently runnable canary proves liveness.
    let child = executor.add(canonical_handle, 13, 9).call().await?;
    let child_receipt = executor
        .add(canonical_handle, 13, 9)
        .send()
        .await?
        .get_receipt()
        .await?;
    ingest(
        &mut db,
        &block_logs(&provider, &child_receipt, exe, acl).await?,
        exe,
        acl,
    )
    .await?;
    let canary = executor.seed(99, 10).call().await?;
    let canary_receipt = executor.seed(99, 10).send().await?.get_receipt().await?;
    ingest(
        &mut db,
        &block_logs(&provider, &canary_receipt, exe, acl).await?,
        exe,
        acl,
    )
    .await?;
    wait_for_ciphertexts(&pool, &[canary.to_vec()]).await?;
    assert_eq!(
        ciphertext_count(&pool, &[canonical_handle.to_vec(), child.to_vec()]).await?,
        0
    );
    canonical.catchup = true;
    canonical.finalized = true;
    ingest(&mut db, &canonical, exe, acl).await?;
    wait_for_ciphertexts(&pool, &[canonical_handle.to_vec(), child.to_vec()]).await?;
    assert_eq!(block_state(&pool, orphan.summary.hash).await?.2, "orphaned");
    assert_eq!(
        block_state(&pool, canonical.summary.hash).await?.2,
        "finalized"
    );
    assert!(!fhevm_engine_common::chain_recovery::is_halted(&pool).await?);
    stop_workers(&mut workers).await?;
    assert_plaintexts(
        &pool,
        &[
            (orphan_handle, 7),
            (canonical_handle, 7),
            (child, 20),
            (canary, 99),
        ],
    )
    .await
}
