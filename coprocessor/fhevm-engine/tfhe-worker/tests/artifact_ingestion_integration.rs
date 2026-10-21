//! Artifact stall regression: Anvil logs -> production ingestion -> PostgreSQL
//! -> the real CPU worker -> decrypted ciphertexts.
//!
//! Drop only the producer's ACL log on first delivery. Its computation is then
//! considered completed without a persisted ciphertext; downstream transactions
//! cannot run. Replay must repair that existing row and wake the dependency graph.
//! No computation, ciphertext, or scheduler flags are written by this test.

#![cfg(not(feature = "gpu"))]

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::{Address, B256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Filter, Log, TransactionReceipt},
    signers::local::PrivateKeySigner,
    sol,
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
use tokio::{process::Command, time::timeout};

sol!(
    #[sol(rpc)]
    ACLTest,
    "../host-listener/artifacts/ACLTest.sol/ACLTest.json"
);
sol!(
    #[sol(rpc)]
    ArtifactStallTest,
    "../host-listener/artifacts/ArtifactStallTest.sol/ArtifactStallTest.json"
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
};

#[derive(Clone, Copy, Debug)]
enum Replay {
    WarmFullBlock,
    ColdFullBlock,
    ColdAclOnly,
    FailedThenRetried,
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
    .await?;
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
            "0",
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
        let branch_allowed: bool = sqlx::query_scalar(
            "SELECT is_allowed FROM computations_branch
             WHERE output_handle = $1 AND producer_block_hash = $2",
        )
        .bind(root.as_slice())
        .bind(repaired_root.summary.hash.as_slice())
        .fetch_one(&pool)
        .await?;
        assert!(
            !branch_allowed,
            "failed replay leaked a branch-mirror update"
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
    let branch_allowed: bool = sqlx::query_scalar(
        "SELECT is_allowed FROM computations_branch
         WHERE output_handle = $1 AND producer_block_hash = $2",
    )
    .bind(root.as_slice())
    .bind(repaired_root.summary.hash.as_slice())
    .fetch_one(&pool)
    .await?;
    assert!(branch_allowed, "replay must also repair the branch mirror");
    let counts: (i32, i32) = sqlx::query_as(
        "SELECT fhe_event_count, allow_event_count FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN as i64)
    .bind(repaired_root.summary.hash.as_slice())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        counts,
        (1, 1),
        "replay must repair the partial block's event counts"
    );
    wait_for_ciphertexts(&pool, &graph).await?;
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
