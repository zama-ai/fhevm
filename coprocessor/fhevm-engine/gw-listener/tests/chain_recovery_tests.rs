use std::{process::Stdio, time::Duration};

use alloy::{
    network::EthereumWallet,
    node_bindings::Anvil,
    primitives::{Address, U256},
    providers::{ext::AnvilApi, ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::{Context, Result};
use fhevm_engine_common::{
    chain_recovery::{is_halted, readiness_blocked},
    database::GCS_SCHEMA_QUOTED,
    versioning::{begin_write_guarded, GcsRollbackPolicy, WriteGuard},
};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::process::{Child, Command};

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

fn launch(db_url: &str, gw_url: &str, address: Address) -> Result<Child> {
    Ok(Command::new(env!("CARGO_BIN_EXE_gw_listener"))
        .args([
            "--database-url",
            db_url,
            "--gw-url",
            gw_url,
            "--input-verification-address",
            &address.to_string(),
            "--get-logs-poll-interval",
            "50ms",
            "--health-check-port",
            "0",
            "--metrics-addr",
            "127.0.0.1:0",
            "--log-level",
            "error",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?)
}

async fn stop(child: &mut Child) -> Result<()> {
    if child.try_wait()?.is_none() {
        child.start_kill()?;
    }
    tokio::time::timeout(Duration::from_secs(10), child.wait()).await??;
    Ok(())
}

#[tokio::test]
async fn gateway_checkpoints_anchor_neighboring_batches() -> Result<()> {
    use fhevm_engine_common::chain_recovery::{save_checkpoints, ListenerHeader, ListenerSource};
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let header = |number: i64| ListenerHeader {
        number,
        hash: vec![number as u8; 32],
        parent_hash: vec![(number - 1) as u8; 32],
    };
    save_checkpoints(&pool, ListenerSource::Gateway, false, 42, &[header(8)]).await?;
    // Identical concurrent admissions are idempotent; their identity checks
    // and writes must use the same per-chain lock.
    let left = pool.clone();
    let replica = tokio::spawn(async move {
        save_checkpoints(&left, ListenerSource::Gateway, false, 42, &[header(9)]).await
    });
    save_checkpoints(&pool, ListenerSource::Gateway, false, 42, &[header(9)]).await?;
    replica.await??;
    assert!(!is_halted(&pool).await?);
    let mut inconsistent = header(10);
    inconsistent.parent_hash = vec![0; 32];
    assert!(
        save_checkpoints(&pool, ListenerSource::Gateway, false, 42, &[inconsistent])
            .await
            .is_err()
    );
    assert!(is_halted(&pool).await?);
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM public.listener_block_checkpoints")
        .fetch_one(&pool)
        .await?;
    assert_eq!(count, 2);
    Ok(())
}

#[tokio::test]
async fn history_halt_blocks_automatic_drift_recovery() -> Result<()> {
    use fhevm_engine_common::{
        chain_recovery::{record_halt, HistoryMismatch, ListenerSource},
        drift_revert,
    };
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    record_halt(
        &pool,
        HistoryMismatch {
            source: ListenerSource::Gateway,
            gcs_mode: false,
            chain_id: 42,
            block_number: 8,
            expected_hash: None,
            observed_hash: None,
            reason: "test retained history",
        },
    )
    .await?;
    assert!(drift_revert::create_revert_signal(&pool, 42, 10)
        .await
        .is_err());
    let error = drift_revert::execute_revert(&pool, 42, 10)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("automatic drift revert refused"));
    assert!(drift_revert::latest_signal(&pool).await?.is_none());
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
async fn green_committed_gateway_fork_closes_readiness_and_acceptance() -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    upgrade_controller::create_gcs_schema(&pool).await?;
    sqlx::query(
        "UPDATE versioning SET consensus_version = consensus_version - 1 WHERE singleton = TRUE",
    )
    .execute(&pool)
    .await?;
    let anvil = Anvil::new().chain_id(12345).try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_ws(WsConnect::new(anvil.ws_endpoint_url()))
        .await?;
    let contract = InputVerification::deploy(&provider).await?;
    let snapshot = provider.anvil_snapshot().await?;
    let old = contract
        .verifyProofRequest(
            U256::from(42),
            Address::repeat_byte(17),
            Address::repeat_byte(18),
            vec![1_u8; 2048].into(),
            vec![].into(),
        )
        .send()
        .await?
        .get_receipt()
        .await?;
    anyhow::ensure!(old.status(), "old request reverted");
    let height = i64::try_from(old.block_number.context("request height missing")?)?;
    provider.anvil_mine(Some(1), None).await?;
    sqlx::query(
        "INSERT INTO upgrade_state (stack_role, host_chain_id, state, status, gw_start_block)
                 VALUES ('GCS', 42, 'UpgradeActivated', 'in_progress', $1)",
    )
    .bind(height)
    .execute(&pool)
    .await?;
    sqlx::query(
        "INSERT INTO public.gw_listener_last_block (dummy_id, last_block_num) VALUES (TRUE, $1)",
    )
    .bind(height)
    .execute(&pool)
    .await?;
    let url = anvil.ws_endpoint_url().to_string();
    let mut first = launch(db.db_url(), &url, *contract.address())?;
    let old_cursor = tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            anyhow::ensure!(first.try_wait()?.is_none(), "initial listener exited");
            let row: Option<Vec<u8>> = sqlx::query_scalar(&format!(
                "SELECT transaction_id FROM {GCS_SCHEMA_QUOTED}.verify_proofs WHERE zk_proof_id = 0"))
                .fetch_optional(&pool).await?;
            let cursor: Option<i64> = sqlx::query_scalar::<_, Option<i64>>(&format!(
                "SELECT last_block_num FROM {GCS_SCHEMA_QUOTED}.gw_listener_last_block WHERE dummy_id = TRUE"))
                .fetch_optional(&pool).await?.flatten();
            if row == Some(old.transaction_hash.to_vec()) && cursor.is_some_and(|n| n > height) {
                break Ok::<_, anyhow::Error>(cursor.unwrap());
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }).await??;
    assert!(upgrade_controller::check_gw_dry_run_ready(&pool, height).await?);
    sqlx::query("UPDATE upgrade_state SET gw_dry_run_started = TRUE WHERE stack_role = 'GCS'")
        .execute(&pool)
        .await?;
    stop(&mut first).await?;
    assert!(provider.anvil_revert(snapshot).await?);
    let replacement_signer: PrivateKeySigner = anvil.keys()[1].clone().into();
    let replacement_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(replacement_signer))
        .connect_ws(WsConnect::new(anvil.ws_endpoint_url()))
        .await?;
    let replacement_contract = InputVerification::new(*contract.address(), &replacement_provider);
    let new = replacement_contract
        .verifyProofRequest(
            U256::from(43),
            Address::repeat_byte(33),
            Address::repeat_byte(34),
            vec![2_u8; 2048].into(),
            vec![].into(),
        )
        .send()
        .await?
        .get_receipt()
        .await?;
    assert!(new.status());
    assert_eq!(new.block_number, old.block_number);
    assert_ne!(new.block_hash, old.block_hash);
    provider.anvil_mine(Some(2), None).await?;
    let mut second = launch(db.db_url(), &url, *contract.address())?;
    tokio::time::timeout(Duration::from_secs(30), async {
        while !is_halted(&pool).await? {
            anyhow::ensure!(
                second.try_wait()?.is_none(),
                "listener exited before persisting containment"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    stop(&mut second).await?;
    assert!(readiness_blocked(&pool).await?);
    assert!(!upgrade_controller::check_gw_dry_run_ready(&pool, height).await?);
    let version_before: (String, i64) = sqlx::query_as(
        "SELECT stack_version, consensus_version FROM versioning WHERE singleton = TRUE",
    )
    .fetch_one(&pool)
    .await?;
    upgrade_controller::execute_cutover(&pool).await?;
    let version_after: (String, i64) = sqlx::query_as(
        "SELECT stack_version, consensus_version FROM versioning WHERE singleton = TRUE",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        version_before, version_after,
        "containment must prevent cutover"
    );
    let (stored_tx, input): (Vec<u8>, Option<Vec<u8>>) = sqlx::query_as(&format!(
        "SELECT transaction_id, input FROM {GCS_SCHEMA_QUOTED}.verify_proofs WHERE zk_proof_id = 0"
    ))
    .fetch_one(&pool)
    .await?;
    assert_eq!(stored_tx, old.transaction_hash.to_vec());
    assert_eq!(input, Some(vec![1; 2048]));
    let cursor: i64 = sqlx::query_scalar(&format!(
        "SELECT last_block_num FROM {GCS_SCHEMA_QUOTED}.gw_listener_last_block WHERE dummy_id = TRUE"))
        .fetch_one(&pool).await?;
    assert_eq!(cursor, old_cursor);
    let latch: bool =
        sqlx::query_scalar("SELECT gw_dry_run_started FROM upgrade_state WHERE stack_role = 'GCS'")
            .fetch_one(&pool)
            .await?;
    assert!(!latch);
    for gcs in [false, true] {
        assert!(matches!(
            begin_write_guarded(&pool, gcs, GcsRollbackPolicy::RawIngestion).await?,
            WriteGuard::Skip
        ));
        assert!(matches!(
            begin_write_guarded(&pool, gcs, GcsRollbackPolicy::Continue).await?,
            WriteGuard::Skip
        ));
    }
    Ok(())
}

#[tokio::test]
async fn legacy_gateway_cursor_requires_verified_operator_baseline() -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    sqlx::query("INSERT INTO gw_listener_last_block (dummy_id, last_block_num) VALUES (TRUE, 1)")
        .execute(&pool)
        .await?;
    let anvil = Anvil::new().chain_id(12345).try_spawn()?;
    let mut listener = launch(
        db.db_url(),
        &anvil.ws_endpoint_url().to_string(),
        Address::ZERO,
    )?;
    tokio::time::timeout(Duration::from_secs(30), async {
        while !is_halted(&pool).await? {
            anyhow::ensure!(
                listener.try_wait()?.is_none(),
                "listener exited without legacy containment"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        Ok::<_, anyhow::Error>(())
    })
    .await??;
    stop(&mut listener).await?;
    let reason: String = sqlx::query_scalar(
        "SELECT reason FROM public.chain_recovery_halts WHERE resolved_at IS NULL",
    )
    .fetch_one(&pool)
    .await?;
    assert!(reason.contains("legacy Gateway cursor"));
    Ok(())
}
